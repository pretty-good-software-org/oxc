use oxc_ast::{
    AstKind,
    ast::{BinaryOperator, Expression},
};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct DifferentTypesComparison;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Reports comparisons between statically different primitive literal types.
    DifferentTypesComparison,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow comparisons between different literal types.",
);

impl Rule for DifferentTypesComparison {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::BinaryExpression(expression) = node.kind() else { return };
        if !matches!(
            expression.operator,
            BinaryOperator::Equality
                | BinaryOperator::StrictEquality
                | BinaryOperator::Inequality
                | BinaryOperator::StrictInequality
                | BinaryOperator::LessThan
                | BinaryOperator::LessEqualThan
                | BinaryOperator::GreaterThan
                | BinaryOperator::GreaterEqualThan
        ) {
            return;
        }
        let Some(left_type) = literal_type(&expression.left) else { return };
        let Some(right_type) = literal_type(&expression.right) else { return };
        if left_type == right_type {
            return;
        }
        ctx.diagnostic(
            OxcDiagnostic::warn("Do not compare values of different primitive types.")
                .with_help("Convert the values explicitly or compare compatible types.")
                .with_label(expression.span),
        );
    }
}

fn literal_type(expression: &Expression) -> Option<&'static str> {
    Some(match expression {
        Expression::StringLiteral(_) => "string",
        Expression::NumericLiteral(_) => "number",
        Expression::BooleanLiteral(_) => "boolean",
        Expression::NullLiteral(_) => "null",
        Expression::BigIntLiteral(_) => "bigint",
        _ => return None,
    })
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        DifferentTypesComparison::NAME,
        DifferentTypesComparison::PLUGIN,
        vec!["1 === 1", "'1' === '1'"],
        vec!["1 === '1'"],
    )
    .test_and_snapshot();
}
