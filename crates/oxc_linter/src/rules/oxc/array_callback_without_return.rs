use oxc_ast::{AstKind, ast::Expression};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::GetSpan;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct ArrayCallbackWithoutReturn;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Detects callbacks for value-producing array methods that never return a value.
    ArrayCallbackWithoutReturn,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Require returns from value-producing array callbacks.",
);

impl Rule for ArrayCallbackWithoutReturn {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let Expression::StaticMemberExpression(member) = call.callee.without_parentheses() else {
            return;
        };
        if !matches!(
            member.property.name.as_str(),
            "map" | "filter" | "reduce" | "some" | "every" | "find" | "findIndex"
        ) {
            return;
        }
        let Some(callback) = call.arguments.first().and_then(|argument| argument.as_expression())
        else {
            return;
        };
        let is_function = matches!(
            callback,
            Expression::ArrowFunctionExpression(_) | Expression::FunctionExpression(_)
        );
        if is_function
            && ctx.source_range(callback.span()).contains('{')
            && !ctx.source_range(callback.span()).contains("return")
        {
            ctx.diagnostic(
                OxcDiagnostic::warn(format!(
                    "Return a value from the `{}` callback.",
                    member.property.name
                ))
                .with_help("Return the value that the array operation should use.")
                .with_label(callback.span()),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        ArrayCallbackWithoutReturn::NAME,
        ArrayCallbackWithoutReturn::PLUGIN,
        vec!["items.forEach((item) => { log(item); });"],
        vec!["items.map((item) => { log(item); });"],
    )
    .test_and_snapshot();
}
