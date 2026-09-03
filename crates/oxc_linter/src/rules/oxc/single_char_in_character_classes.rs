use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct SingleCharInCharacterClasses;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Reports character classes containing only one character.
    SingleCharInCharacterClasses,
    oxc,
    style,
    none,
    version = "0.0.1",
    short_description = "Disallow single-character regular-expression classes.",
);

impl Rule for SingleCharInCharacterClasses {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::RegExpLiteral(literal) = node.kind() else { return };
        let pattern = literal.regex.pattern.text.as_str();
        let has_single_character_class = pattern
            .as_bytes()
            .windows(3)
            .any(|window| window[0] == b'[' && window[2] == b']' && window[1] != b'^');
        if has_single_character_class {
            ctx.diagnostic(
                OxcDiagnostic::warn(
                    "Replace this single-character class with the character itself.",
                )
                .with_help("Remove the brackets around the single character.")
                .with_label(literal.span),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        SingleCharInCharacterClasses::NAME,
        SingleCharInCharacterClasses::PLUGIN,
        vec!["/[ab]/"],
        vec!["/[a]/"],
    )
    .test_and_snapshot();
}
