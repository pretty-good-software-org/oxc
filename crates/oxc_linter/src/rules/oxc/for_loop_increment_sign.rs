use oxc_ast::{
    AstKind,
    ast::{BinaryOperator, Expression, SimpleAssignmentTarget, UpdateOperator},
};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::GetSpan;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct ForLoopIncrementSign;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Reports for loops whose update moves away from their termination condition.
    ForLoopIncrementSign,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Ensure for-loop increments move toward termination.",
);

impl Rule for ForLoopIncrementSign {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::ForStatement(statement) = node.kind() else { return };
        let (Some(test), Some(update)) = (&statement.test, &statement.update) else { return };
        let Expression::BinaryExpression(condition) = test else { return };
        let Expression::UpdateExpression(increment) = update else { return };
        let SimpleAssignmentTarget::AssignmentTargetIdentifier(identifier) = &increment.argument
        else {
            return;
        };
        let Expression::Identifier(test_identifier) = &condition.left else { return };
        if identifier.name != test_identifier.name {
            return;
        }
        let wrong = match condition.operator {
            BinaryOperator::LessThan | BinaryOperator::LessEqualThan => {
                increment.operator == UpdateOperator::Decrement
            }
            BinaryOperator::GreaterThan | BinaryOperator::GreaterEqualThan => {
                increment.operator == UpdateOperator::Increment
            }
            _ => false,
        };
        if wrong {
            ctx.diagnostic(
                OxcDiagnostic::warn(format!(
                    "`{}` moves away from its stop condition.",
                    identifier.name
                ))
                .with_label(update.span()),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        ForLoopIncrementSign::NAME,
        ForLoopIncrementSign::PLUGIN,
        vec!["for (let i = 0; i < 10; i++) {}"],
        vec!["for (let i = 0; i < 10; i--) {}", "for (let i = 10; i > 0; i++) {}"],
    )
    .test_and_snapshot();
}
