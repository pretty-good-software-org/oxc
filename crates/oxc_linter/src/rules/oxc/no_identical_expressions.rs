use oxc_ast::{AstKind, ast::Expression};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::{GetSpan, Span};
use oxc_syntax::operator::{BinaryOperator, LogicalOperator};

use crate::{AstNode, context::LintContext, rule::Rule, utils::is_same_expression};

fn no_identical_expressions_diagnostic(span: Span, operator: &str) -> OxcDiagnostic {
    OxcDiagnostic::warn(format!(
        "Correct one of the identical sub-expressions around `{operator}`."
    ))
    .with_help("Replace one side of the expression with the intended operand.")
    .with_label(span)
}

#[derive(Debug, Default, Clone)]
pub struct NoIdenticalExpressions;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows identical sub-expressions on both sides of selected operators.
    ///
    /// ### Why is this bad?
    ///
    /// Repeating the same expression in a binary operation is usually a typo
    /// and means one branch of the intended condition is missing.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```javascript
    /// if (value === value) {}
    /// const result = value && value;
    /// ```
    NoIdenticalExpressions,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow identical binary sub-expressions.",
);

impl Rule for NoIdenticalExpressions {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let (left, right, operator) = match node.kind() {
            AstKind::BinaryExpression(expression) => {
                if !is_relevant_binary_operator(expression.operator)
                    || both_identifiers(&expression.left, &expression.right)
                    || is_one_shift(expression.operator, &expression.left)
                {
                    return;
                }
                (&expression.left, &expression.right, binary_operator_text(expression.operator))
            }
            AstKind::LogicalExpression(expression) => {
                (&expression.left, &expression.right, logical_operator_text(expression.operator))
            }
            _ => return,
        };
        if is_same_expression(left, right, ctx) {
            ctx.diagnostic(no_identical_expressions_diagnostic(node.span(), operator));
        }
    }
}

fn both_identifiers(left: &Expression<'_>, right: &Expression<'_>) -> bool {
    matches!((left, right), (Expression::Identifier(_), Expression::Identifier(_)))
}

fn is_relevant_binary_operator(operator: BinaryOperator) -> bool {
    matches!(
        operator,
        BinaryOperator::Addition
            | BinaryOperator::Division
            | BinaryOperator::Subtraction
            | BinaryOperator::ShiftLeft
            | BinaryOperator::ShiftRight
            | BinaryOperator::LessThan
            | BinaryOperator::LessEqualThan
            | BinaryOperator::GreaterThan
            | BinaryOperator::GreaterEqualThan
            | BinaryOperator::Equality
            | BinaryOperator::StrictEquality
            | BinaryOperator::Inequality
            | BinaryOperator::StrictInequality
    )
}

fn is_one_shift(operator: BinaryOperator, left: &Expression<'_>) -> bool {
    operator == BinaryOperator::ShiftLeft
        && matches!(left, Expression::NumericLiteral(literal) if literal.value.to_bits() == 1.0f64.to_bits())
}

fn binary_operator_text(operator: BinaryOperator) -> &'static str {
    match operator {
        BinaryOperator::Addition => "+",
        BinaryOperator::Division => "/",
        BinaryOperator::Subtraction => "-",
        BinaryOperator::ShiftLeft => "<<",
        BinaryOperator::ShiftRight => ">>",
        BinaryOperator::LessThan => "<",
        BinaryOperator::LessEqualThan => "<=",
        BinaryOperator::GreaterThan => ">",
        BinaryOperator::GreaterEqualThan => ">=",
        BinaryOperator::Equality => "==",
        BinaryOperator::StrictEquality => "===",
        BinaryOperator::Inequality => "!=",
        BinaryOperator::StrictInequality => "!==",
        _ => "operator",
    }
}

fn logical_operator_text(operator: LogicalOperator) -> &'static str {
    match operator {
        LogicalOperator::And => "&&",
        LogicalOperator::Or => "||",
        LogicalOperator::Coalesce => "??",
    }
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec!["value === value", "1 << 1", "left && right"];
    let fail = vec!["1 + 1", "left && left"];

    Tester::new(NoIdenticalExpressions::NAME, NoIdenticalExpressions::PLUGIN, pass, fail)
        .test_and_snapshot();
}
