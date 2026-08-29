use oxc_ast::{AstKind, ast::Expression};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::GetSpan;
use oxc_syntax::operator::{BinaryOperator, LogicalOperator, UnaryOperator};

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoRedundantBoolean;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Detects boolean literals that make a comparison or logical expression redundant.
    NoRedundantBoolean,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow redundant boolean literals.",
);

impl Rule for NoRedundantBoolean {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let redundant = match node.kind() {
            AstKind::BinaryExpression(expression)
                if matches!(
                    expression.operator,
                    BinaryOperator::Equality | BinaryOperator::Inequality
                ) =>
            {
                is_boolean(&expression.left) || is_boolean(&expression.right)
            }
            AstKind::LogicalExpression(expression) => {
                is_boolean(&expression.left)
                    || (expression.operator == LogicalOperator::And
                        && is_boolean(&expression.right))
            }
            AstKind::UnaryExpression(expression) => {
                expression.operator == UnaryOperator::LogicalNot && is_boolean(&expression.argument)
            }
            _ => false,
        };
        if redundant {
            ctx.diagnostic(
                OxcDiagnostic::warn(
                    "Refactor the expression to avoid a redundant boolean literal.",
                )
                .with_help("Use the operand directly or invert it explicitly.")
                .with_label(node.kind().span()),
            );
        }
    }
}

fn is_boolean(expression: &Expression<'_>) -> bool {
    matches!(expression, Expression::BooleanLiteral(_))
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoRedundantBoolean::NAME,
        NoRedundantBoolean::PLUGIN,
        vec!["const result = value && other;"],
        vec!["const result = value == true;", "const result = value && false;"],
    )
    .test_and_snapshot();
}
