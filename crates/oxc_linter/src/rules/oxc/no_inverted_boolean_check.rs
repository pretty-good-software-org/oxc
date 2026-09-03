use oxc_ast::{AstKind, ast::Expression};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_syntax::operator::{BinaryOperator, UnaryOperator};

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoInvertedBooleanCheck;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows negating binary comparisons instead of using their opposite operator.
    NoInvertedBooleanCheck,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Prefer the opposite comparison operator.",
);

impl Rule for NoInvertedBooleanCheck {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::UnaryExpression(unary) = node.kind() else {
            return;
        };
        if unary.operator != UnaryOperator::LogicalNot {
            return;
        }
        let Expression::BinaryExpression(binary) = unary.argument.without_parentheses() else {
            return;
        };
        let Some(opposite) = opposite_operator(binary.operator) else {
            return;
        };
        ctx.diagnostic(
            OxcDiagnostic::warn(format!("Use the opposite operator `{opposite}` instead."))
                .with_help("Invert the comparison operator rather than the whole expression.")
                .with_label(unary.span),
        );
    }
}

fn opposite_operator(operator: BinaryOperator) -> Option<&'static str> {
    Some(match operator {
        BinaryOperator::Equality => "!=",
        BinaryOperator::Inequality => "==",
        BinaryOperator::StrictEquality => "!==",
        BinaryOperator::StrictInequality => "===",
        BinaryOperator::GreaterThan => "<=",
        BinaryOperator::LessThan => ">=",
        BinaryOperator::GreaterEqualThan => "<",
        BinaryOperator::LessEqualThan => ">",
        _ => return None,
    })
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoInvertedBooleanCheck::NAME,
        NoInvertedBooleanCheck::PLUGIN,
        vec!["if (value === expected) {}"],
        vec!["if (!(value === expected)) {}"],
    )
    .test_and_snapshot();
}
