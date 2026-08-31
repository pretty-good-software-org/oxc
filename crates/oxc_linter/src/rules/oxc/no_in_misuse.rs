use oxc_ast::{
    AstKind,
    ast::{BinaryOperator, Expression},
};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoInMisuse;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows using `in` to search an array by value.
    NoInMisuse,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Use array search methods instead of `in`.",
);

impl Rule for NoInMisuse {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::BinaryExpression(expression) = node.kind() else { return };
        if expression.operator != BinaryOperator::In {
            return;
        }
        let Expression::ArrayExpression(_) = &expression.right else { return };
        if matches!(&expression.left, Expression::StringLiteral(_) | Expression::Identifier(_)) {
            ctx.diagnostic(
                OxcDiagnostic::warn("Use `indexOf` or `includes` instead of `in` for arrays.")
                    .with_help("Use `array.includes(value)` to search by value.")
                    .with_label(expression.span),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoInMisuse::NAME,
        NoInMisuse::PLUGIN,
        vec!["0 in array", "key in object"],
        vec!["'key' in [key]"],
    )
    .test_and_snapshot();
}
