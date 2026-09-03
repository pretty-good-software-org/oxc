use oxc_ast::{AstKind, ast::Statement};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::{GetSpan, Span};

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoSameLineConditional;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Reports adjacent `if` statements that share a line without an `else`.
    NoSameLineConditional,
    oxc,
    style,
    none,
    version = "0.0.1",
    short_description = "Disallow same-line adjacent conditionals.",
);

impl Rule for NoSameLineConditional {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let statements = match node.kind() {
            AstKind::Program(program) => &program.body,
            AstKind::BlockStatement(block) => &block.body,
            AstKind::SwitchCase(case) => &case.consequent,
            _ => return,
        };
        for pair in statements.windows(2) {
            let (Statement::IfStatement(first), Statement::IfStatement(second)) =
                (&pair[0], &pair[1])
            else {
                continue;
            };
            if first.alternate.is_some()
                || line(ctx, first.span.end) != line(ctx, second.span.start)
            {
                continue;
            }
            ctx.diagnostic(
                OxcDiagnostic::warn("Move this `if` to a new line or add the missing `else`.")
                    .with_help("Put adjacent conditionals on separate lines or combine them with `else if`.")
                    .with_label(Span::new(second.span.start, second.test.span().end)),
            );
        }
    }
}

fn line(ctx: &LintContext, offset: u32) -> usize {
    ctx.source_range(Span::new(0, offset)).bytes().filter(|byte| *byte == b'\n').count()
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoSameLineConditional::NAME,
        NoSameLineConditional::PLUGIN,
        vec!["if (one) run();\nif (two) run();"],
        vec!["if (one) run(); if (two) run();"],
    )
    .test_and_snapshot();
}
