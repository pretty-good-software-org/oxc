use oxc_ast::{AstKind, ast::Expression};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::GetSpan;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoUndefinedArgument;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows passing `undefined` explicitly as a function argument.
    NoUndefinedArgument,
    oxc,
    suspicious,
    none,
    version = "0.0.1",
    short_description = "Disallow explicit undefined arguments.",
);

impl Rule for NoUndefinedArgument {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let Some(argument) = call.arguments.last() else {
            return;
        };
        if matches!(argument.as_expression(), Some(Expression::Identifier(identifier)) if identifier.name == "undefined")
        {
            ctx.diagnostic(
                OxcDiagnostic::warn("Do not pass `undefined` explicitly as the final argument.")
                    .with_help("Omit the argument when the parameter is optional.")
                    .with_label(argument.span()),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoUndefinedArgument::NAME,
        NoUndefinedArgument::PLUGIN,
        vec!["call(value);"],
        vec!["call(undefined);"],
    )
    .test_and_snapshot();
}
