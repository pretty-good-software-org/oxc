use oxc_ast::{AstKind, ast::Expression};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::GetSpan;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct PostMessage;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows wildcard origins in `postMessage` calls.
    PostMessage,
    oxc,
    suspicious,
    none,
    version = "0.0.1",
    short_description = "Require an explicit postMessage target origin.",
);

impl Rule for PostMessage {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::CallExpression(call) = node.kind() else { return };
        let Some(member) = call.callee.as_member_expression() else { return };
        if member.static_property_name() != Some("postMessage") || call.arguments.len() < 2 {
            return;
        }
        let Some(Expression::StringLiteral(origin)) = call.arguments[1].as_expression() else {
            return;
        };
        if origin.value != "*" {
            return;
        }
        ctx.diagnostic(
            OxcDiagnostic::warn("Specify a target origin for this message.")
                .with_help("Replace `*` with the exact origin that should receive the message.")
                .with_label(call.arguments[1].span()),
        );
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        PostMessage::NAME,
        PostMessage::PLUGIN,
        vec!["window.postMessage(message, 'https://example.com')"],
        vec!["window.postMessage(message, '*')"],
    )
    .test_and_snapshot();
}
