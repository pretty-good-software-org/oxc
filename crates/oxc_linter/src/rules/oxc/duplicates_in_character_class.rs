use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use rustc_hash::FxHashSet;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct DuplicatesInCharacterClass;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Detects repeated characters inside regular-expression character classes.
    DuplicatesInCharacterClass,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow duplicate characters in regex classes.",
);

impl Rule for DuplicatesInCharacterClass {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::RegExpLiteral(literal) = node.kind() else {
            return;
        };
        if has_duplicate_character_class(literal.regex.pattern.text.as_str()) {
            ctx.diagnostic(
                OxcDiagnostic::warn("Remove duplicate characters from this regex character class.")
                    .with_help("Keep each character or range only once.")
                    .with_label(literal.span),
            );
        }
    }
}

fn has_duplicate_character_class(pattern: &str) -> bool {
    let mut in_class = false;
    let mut escaped = false;
    let mut characters = FxHashSet::default();
    for character in pattern.chars() {
        if escaped {
            escaped = false;
            continue;
        }
        if character == '\\' {
            escaped = true;
            continue;
        }
        if character == '[' {
            in_class = true;
            characters.clear();
            continue;
        }
        if character == ']' && in_class {
            in_class = false;
            continue;
        }
        if in_class && !characters.insert(character) {
            return true;
        }
    }
    false
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        DuplicatesInCharacterClass::NAME,
        DuplicatesInCharacterClass::PLUGIN,
        vec!["const pattern = /[a-z]/;"],
        vec!["const pattern = /[aaz]/;"],
    )
    .test_and_snapshot();
}
