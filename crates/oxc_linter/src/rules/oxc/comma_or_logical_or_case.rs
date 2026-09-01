use oxc_ast::{
    AstKind,
    ast::{Expression, LogicalOperator},
};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::GetSpan;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct CommaOrLogicalOrCase;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Reports switch cases that combine alternatives with expressions instead of separate labels.
    CommaOrLogicalOrCase,
    oxc,
    style,
    none,
    version = "0.0.1",
    short_description = "Prefer explicit switch case labels.",
);

impl Rule for CommaOrLogicalOrCase {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::SwitchCase(case) = node.kind() else { return };
        let Some(test) = &case.test else { return };
        let invalid = matches!(test, Expression::SequenceExpression(_))
            || matches!(test, Expression::LogicalExpression(logical) if logical.operator == LogicalOperator::Or);
        if invalid {
            ctx.diagnostic(
                OxcDiagnostic::warn("Explicitly specify separate cases that fall through.")
                    .with_help("Use one `case` clause for each accepted value.")
                    .with_label(test.span()),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        CommaOrLogicalOrCase::NAME,
        CommaOrLogicalOrCase::PLUGIN,
        vec!["switch (value) { case 1: run(); }"],
        vec!["switch (value) { case 1 || 2: run(); }"],
    )
    .test_and_snapshot();
}
