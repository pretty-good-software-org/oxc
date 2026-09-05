use oxc_ast::{AstKind, ast::Statement};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct LabelPosition;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Requires labels to be placed on loop or switch statements.
    LabelPosition,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow labels on non-iterative statements.",
);

impl Rule for LabelPosition {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::LabeledStatement(statement) = node.kind() else { return };
        if is_label_target(&statement.body) {
            return;
        }
        // Svelte uses top-level `$:` labels for reactive statements, not control flow.
        if statement.label.name == "$"
            && ctx.file_extension().is_some_and(|extension| extension == "svelte")
            && matches!(ctx.nodes().parent_kind(node.id()), AstKind::Program(_))
        {
            return;
        }
        ctx.diagnostic(
            OxcDiagnostic::warn("Move this label to a loop or switch statement.")
                .with_help("Labels should identify statements that can be continued or broken.")
                .with_label(statement.label.span),
        );
    }
}

fn is_label_target(statement: &Statement) -> bool {
    matches!(
        statement,
        Statement::DoWhileStatement(_)
            | Statement::ForInStatement(_)
            | Statement::ForOfStatement(_)
            | Statement::ForStatement(_)
            | Statement::WhileStatement(_)
            | Statement::SwitchStatement(_)
    )
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        LabelPosition::NAME,
        LabelPosition::PLUGIN,
        vec!["loop: for (;;) break loop;"],
        vec!["label: if (value) run();", "$: run();"],
    )
    .test_and_snapshot();
}

#[test]
fn test_svelte_reactivity() {
    use crate::tester::Tester;
    let pass = vec!["<script>$: run(); $: { run(); }</script>"];
    let fail = vec!["<script>label: run();</script>"];
    Tester::new(LabelPosition::NAME, LabelPosition::PLUGIN, pass, fail)
        .change_rule_path("test.svelte")
        .test();
}
