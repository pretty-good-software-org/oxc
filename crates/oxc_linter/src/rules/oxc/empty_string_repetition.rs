use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct EmptyStringRepetition;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Reports repeated regular-expression groups that can match the empty string.
    EmptyStringRepetition,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Avoid repeating expressions that match the empty string.",
);

impl Rule for EmptyStringRepetition {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::RegExpLiteral(literal) = node.kind() else { return };
        let pattern = literal.regex.pattern.text.as_str();
        let has_empty_repetition =
            ["(?:)*", "(?:)+", "()?", "()*"].iter().any(|fragment| pattern.contains(fragment));
        if has_empty_repetition {
            ctx.diagnostic(
                OxcDiagnostic::warn("Rework this part of the regex to not match the empty string.")
                    .with_label(literal.span),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        EmptyStringRepetition::NAME,
        EmptyStringRepetition::PLUGIN,
        vec!["/(?:a)+/"],
        vec!["/(?:)*/"],
    )
    .test_and_snapshot();
}
