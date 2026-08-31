use oxc_ast::{AstKind, ast::Expression};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::GetSpan;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct LinkWithTargetBlank;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Requires `noopener` when opening an HTTP URL in a new window.
    LinkWithTargetBlank,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Require noopener for HTTP window.open calls.",
);

impl Rule for LinkWithTargetBlank {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::CallExpression(call) = node.kind() else { return };
        let Some(member) = call.callee.as_member_expression() else { return };
        if member.static_property_name() != Some("open") || !is_window(member.object()) {
            return;
        }
        let Some(Expression::StringLiteral(url)) =
            call.arguments.first().and_then(|argument| argument.as_expression())
        else {
            return;
        };
        if !(url.value.starts_with("http://") || url.value.starts_with("https://")) {
            return;
        }
        let has_noopener = call
            .arguments
            .get(2)
            .and_then(|argument| argument.as_expression())
            .and_then(|expression| match expression {
                Expression::StringLiteral(value) => Some(value.value.contains("noopener")),
                _ => None,
            })
            .unwrap_or(false);
        if !has_noopener {
            ctx.diagnostic(
                OxcDiagnostic::warn("Make sure not using `noopener` is safe here.")
                    .with_help("Include `noopener` in the window features argument.")
                    .with_label(member.as_property_key().span()),
            );
        }
    }
}

fn is_window(expression: &Expression) -> bool {
    if matches!(expression, Expression::Identifier(identifier) if identifier.name == "window") {
        return true;
    }
    expression.as_member_expression().is_some_and(|member| {
        matches!(member.object(), Expression::ThisExpression(_))
            && member.static_property_name() == Some("window")
    })
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        LinkWithTargetBlank::NAME,
        LinkWithTargetBlank::PLUGIN,
        vec!["window.open('https://example.com', '_blank', 'noopener,noreferrer')"],
        vec!["window.open('https://example.com', '_blank')"],
    )
    .test_and_snapshot();
}
