use oxc_ast::{
    AstKind,
    ast::{Expression, Statement},
};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::GetSpan;

use crate::{AstNode, context::LintContext, rule::Rule, utils::is_same_expression};

#[derive(Debug, Default, Clone)]
pub struct NoIdenticalConditions;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows repeated conditions in switch statements and else-if chains.
    NoIdenticalConditions,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow identical conditional tests.",
);

impl Rule for NoIdenticalConditions {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        match node.kind() {
            AstKind::SwitchStatement(statement) => {
                let tests = statement.cases.iter().filter_map(|case| case.test.as_ref());
                let mut previous: Vec<&Expression> = Vec::new();
                for test in tests {
                    if previous.iter().any(|other| is_same_expression(test, other, ctx)) {
                        report(ctx, test.span());
                    } else {
                        previous.push(test);
                    }
                }
            }
            AstKind::IfStatement(statement) => {
                let Some(alternate) = &statement.alternate else { return };
                let Statement::IfStatement(next) = alternate else { return };
                if is_same_expression(&statement.test, &next.test, ctx) {
                    report(ctx, next.test.span());
                }
            }
            _ => {}
        }
    }
}

fn report(ctx: &LintContext, span: oxc_span::Span) {
    ctx.diagnostic(
        OxcDiagnostic::warn("Remove or change this duplicated condition.")
            .with_help("Each conditional branch should test a distinct condition.")
            .with_label(span),
    );
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoIdenticalConditions::NAME,
        NoIdenticalConditions::PLUGIN,
        vec!["if (first) one(); else if (second) two();"],
        vec!["if (first) one(); else if (first) two();"],
    )
    .test_and_snapshot();
}
