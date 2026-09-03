use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoSmallSwitch;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows switch statements that can be expressed more simply.
    NoSmallSwitch,
    oxc,
    suspicious,
    none,
    version = "0.0.1",
    short_description = "Disallow trivial switch statements.",
);

impl Rule for NoSmallSwitch {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::SwitchStatement(statement) = node.kind() else {
            return;
        };
        let has_default = statement.cases.iter().any(|case| case.test.is_none());
        if statement.cases.len() < 2 || (statement.cases.len() == 2 && has_default) {
            ctx.diagnostic(
                OxcDiagnostic::warn("Replace this trivial switch statement.")
                    .with_help(
                        "Use an if statement when there are fewer than two meaningful cases.",
                    )
                    .with_label(statement.span),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoSmallSwitch::NAME,
        NoSmallSwitch::PLUGIN,
        vec!["switch (value) { case 1: one(); case 2: two(); }"],
        vec![
            "switch (value) { case 1: one(); }",
            "switch (value) { case 1: one(); default: two(); }",
        ],
    )
    .test_and_snapshot();
}
