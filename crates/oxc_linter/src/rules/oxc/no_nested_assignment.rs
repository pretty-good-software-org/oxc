use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_syntax::operator::AssignmentOperator;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoNestedAssignment;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows assignment expressions nested inside another assignment.
    NoNestedAssignment,
    oxc,
    suspicious,
    none,
    version = "0.0.1",
    short_description = "Disallow nested assignments.",
);

impl Rule for NoNestedAssignment {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::AssignmentExpression(assignment) = node.kind() else {
            return;
        };
        if assignment.operator != AssignmentOperator::Assign {
            return;
        }
        let parent = ctx.nodes().parent_node(node.id());
        if matches!(parent.kind(), AstKind::AssignmentExpression(_)) {
            ctx.diagnostic(
                OxcDiagnostic::warn("Do not nest assignments.")
                    .with_help("Split the assignments into separate statements.")
                    .with_label(assignment.span),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoNestedAssignment::NAME,
        NoNestedAssignment::PLUGIN,
        vec!["const value = 1;"],
        vec!["let left; let right; left = right = 1;"],
    )
    .test_and_snapshot();
}
