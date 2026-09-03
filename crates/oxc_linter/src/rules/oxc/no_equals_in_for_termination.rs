use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::GetSpan;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoEqualsInForTermination;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows equality tests used as the termination condition of incrementing loops.
    NoEqualsInForTermination,
    oxc,
    suspicious,
    none,
    version = "0.0.1",
    short_description = "Disallow equality loop termination checks.",
);

impl Rule for NoEqualsInForTermination {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::ForStatement(statement) = node.kind() else {
            return;
        };
        let (Some(test), Some(update)) = (&statement.test, &statement.update) else {
            return;
        };
        let test_text = ctx.source_range(test.span());
        let update_text = ctx.source_range(update.span());
        let equality = test_text.contains("===")
            || test_text.contains("==")
            || test_text.contains("!==")
            || test_text.contains("!=");
        let increments = update_text.contains("++")
            || update_text.contains("--")
            || update_text.contains("+=")
            || update_text.contains("-=");
        if equality && increments {
            ctx.diagnostic(
                OxcDiagnostic::warn(format!(
                    "Replace '{}' with a relational comparison for loop termination.",
                    test_text.trim()
                ))
                .with_help("Use `<`, `<=`, `>` or `>=` for an incrementing loop.")
                .with_label(test.span()),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoEqualsInForTermination::NAME,
        NoEqualsInForTermination::PLUGIN,
        vec!["for (let i = 0; i < 3; i++) {}"],
        vec!["for (let i = 0; i != 3; i++) {}"],
    )
    .test_and_snapshot();
}
