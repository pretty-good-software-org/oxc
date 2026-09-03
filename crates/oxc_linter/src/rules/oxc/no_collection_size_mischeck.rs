use oxc_ast::{AstKind, ast::Expression};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::GetSpan;
use oxc_syntax::operator::BinaryOperator;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoCollectionSizeMischeck;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Detects impossible negative collection-size comparisons.
    NoCollectionSizeMischeck,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow impossible collection-size comparisons.",
);

impl Rule for NoCollectionSizeMischeck {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::BinaryExpression(expression) = node.kind() else {
            return;
        };
        if !matches!(
            expression.operator,
            BinaryOperator::LessThan | BinaryOperator::GreaterEqualThan
        ) || !matches!(&expression.right, Expression::NumericLiteral(number) if number.value.to_bits() == 0.0f64.to_bits())
        {
            return;
        }
        let Expression::StaticMemberExpression(member) = expression.left.without_parentheses()
        else {
            return;
        };
        if !matches!(member.property.name.as_str(), "length" | "size") {
            return;
        }
        let object = ctx.source_range(member.object.span());
        let operator = if expression.operator == BinaryOperator::LessThan { "<" } else { ">=" };
        ctx.diagnostic(
            OxcDiagnostic::warn(format!(
                "The `{}` property of `{object}` is always greater than or equal to zero.",
                member.property.name
            ))
            .with_help(format!("Use `{operator}` only with a meaningful non-negative comparison."))
            .with_label(expression.span),
        );
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoCollectionSizeMischeck::NAME,
        NoCollectionSizeMischeck::PLUGIN,
        vec!["if (items.length === 0) {}"],
        vec!["if (items.length < 0) {}", "if (items.size >= 0) {}"],
    )
    .test_and_snapshot();
}
