use oxc_ast::{
    AstKind,
    ast::{BinaryOperator, Expression},
};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::GetSpan;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct InOperatorTypeError;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Reports `in` expressions whose right-hand side is a primitive literal.
    InOperatorTypeError,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow invalid primitive right-hand operands of `in`.",
);

impl Rule for InOperatorTypeError {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::BinaryExpression(expression) = node.kind() else { return };
        if expression.operator != BinaryOperator::In || !is_primitive(&expression.right) {
            return;
        }
        ctx.diagnostic(
            OxcDiagnostic::warn("The right-hand side of `in` must be an object.")
                .with_help("Check that the value is not null or a primitive before using `in`.")
                .with_label(expression.right.span()),
        );
    }
}

fn is_primitive(expression: &Expression) -> bool {
    matches!(
        expression,
        Expression::StringLiteral(_)
            | Expression::NumericLiteral(_)
            | Expression::BooleanLiteral(_)
            | Expression::NullLiteral(_)
            | Expression::BigIntLiteral(_)
    )
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        InOperatorTypeError::NAME,
        InOperatorTypeError::PLUGIN,
        vec!["'x' in object", "'x' in {}"],
        vec!["'x' in 1", "'x' in null"],
    )
    .test_and_snapshot();
}
