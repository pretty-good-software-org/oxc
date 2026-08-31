use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::GetSpan;

use crate::{AstNode, context::LintContext, rule::Rule, utils::is_same_expression};

#[derive(Debug, Default, Clone)]
pub struct NoAllDuplicatedBranches;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows conditional structures whose branches all produce the same result.
    NoAllDuplicatedBranches,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow conditional structures with identical branches.",
);

impl Rule for NoAllDuplicatedBranches {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        match node.kind() {
            AstKind::ConditionalExpression(expression)
                if is_same_expression(&expression.consequent, &expression.alternate, ctx) =>
            {
                report(ctx, expression.span);
            }
            AstKind::IfStatement(statement) => {
                let Some(alternate) = &statement.alternate else {
                    return;
                };
                if ctx.source_range(statement.consequent.span()).trim()
                    == ctx.source_range(alternate.span()).trim()
                {
                    report(ctx, statement.span);
                }
            }
            _ => {}
        }
    }
}

fn report(ctx: &LintContext, span: oxc_span::Span) {
    ctx.diagnostic(
        OxcDiagnostic::warn("Remove this conditional structure or edit its branches.")
            .with_help("Make the branches produce different results or simplify the condition.")
            .with_label(span),
    );
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoAllDuplicatedBranches::NAME,
        NoAllDuplicatedBranches::PLUGIN,
        vec!["const result = condition ? left : right;"],
        vec!["const result = condition ? value : value;"],
    )
    .test_and_snapshot();
}
