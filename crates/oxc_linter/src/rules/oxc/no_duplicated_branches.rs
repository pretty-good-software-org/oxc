use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule, utils::is_same_expression};

#[derive(Debug, Default, Clone)]
pub struct NoDuplicatedBranches;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows conditional branches with identical bodies.
    NoDuplicatedBranches,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow duplicated conditional branches.",
);

impl Rule for NoDuplicatedBranches {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::ConditionalExpression(expression) = node.kind() else {
            return;
        };
        if is_same_expression(&expression.consequent, &expression.alternate, ctx) {
            ctx.diagnostic(
                OxcDiagnostic::warn("Remove the duplicated conditional branch.")
                    .with_help("Keep one branch or replace it with the intended alternative.")
                    .with_label(expression.span),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoDuplicatedBranches::NAME,
        NoDuplicatedBranches::PLUGIN,
        vec!["const result = condition ? left : right;"],
        vec!["const result = condition ? value : value;"],
    )
    .test_and_snapshot();
}
