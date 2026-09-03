use oxc_ast::{
    AstKind,
    ast::{Expression, JSXAttributeName, JSXAttributeValue},
};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::GetSpan;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoUniqKey;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows generated React list keys based on random or current-time values.
    NoUniqKey,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow generated React list keys.",
);

impl Rule for NoUniqKey {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::JSXAttribute(attribute) = node.kind() else { return };
        let JSXAttributeName::Identifier(name) = &attribute.name else { return };
        if name.name != "key" {
            return;
        }
        let Some(JSXAttributeValue::ExpressionContainer(container)) = &attribute.value else {
            return;
        };
        if container.expression.as_expression().is_some_and(is_generated) {
            ctx.diagnostic(
                OxcDiagnostic::warn(
                    "Do not use generated values for keys of React list components.",
                )
                .with_help("Use a stable identifier from the list item.")
                .with_label(container.expression.span()),
            );
        }
    }
}

fn is_generated(expression: &Expression) -> bool {
    let Expression::CallExpression(call) = expression else { return false };
    let Some(member) = call.callee.as_member_expression() else { return false };
    matches!(member.object(), Expression::Identifier(object) if object.name == "Math" && member.static_property_name() == Some("random")
        || matches!(member.object(), Expression::Identifier(object) if object.name == "Date" && member.static_property_name() == Some("now")))
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoUniqKey::NAME,
        NoUniqKey::PLUGIN,
        vec!["<Item key={item.id} />"],
        vec!["<Item key={Math.random()} />", "<Item key={Date.now()} />"],
    )
    .test_and_snapshot();
}
