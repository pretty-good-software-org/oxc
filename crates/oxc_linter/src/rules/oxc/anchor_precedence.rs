use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct AnchorPrecedence;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Reports anchored regex alternations whose precedence may be unclear.
    AnchorPrecedence,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Make regex anchor precedence explicit.",
);

impl Rule for AnchorPrecedence {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::RegExpLiteral(literal) = node.kind() else { return };
        let pattern = literal.regex.pattern.text.as_str();
        let starts_anchored_alternative = pattern.starts_with('^') && pattern.contains('|');
        let ends_anchored_alternative = pattern.ends_with('$') && pattern.contains('|');
        if starts_anchored_alternative || ends_anchored_alternative {
            ctx.diagnostic(
                OxcDiagnostic::warn(
                    "Group parts of the regex together to make anchor precedence explicit.",
                )
                .with_help("Use parentheses to show which alternatives the anchor applies to.")
                .with_label(literal.span),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        AnchorPrecedence::NAME,
        AnchorPrecedence::PLUGIN,
        vec!["/^foo$/"],
        vec!["/^foo|bar/"],
    )
    .test_and_snapshot();
}
