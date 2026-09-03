use oxc_ast::{AstKind, ast::Expression};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct ProductionDebug;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Reports unconditional installation of the `errorhandler` middleware.
    ProductionDebug,
    oxc,
    suspicious,
    none,
    version = "0.0.1",
    short_description = "Disallow debug error middleware in production code.",
);

impl Rule for ProductionDebug {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::CallExpression(call) = node.kind() else { return };
        let Some(member) = call.callee.as_member_expression() else { return };
        if member.static_property_name() != Some("use") {
            return;
        }
        let Some(Expression::CallExpression(middleware)) =
            call.arguments.first().and_then(|argument| argument.as_expression())
        else {
            return;
        };
        let Expression::Identifier(name) = &middleware.callee else { return };
        if name.name != "errorhandler" {
            return;
        }
        ctx.diagnostic(
            OxcDiagnostic::warn("Make sure this debug feature is deactivated before delivering the code in production.")
                .with_help("Install the error handler only in a development-only conditional.")
                .with_label(middleware.span),
        );
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        ProductionDebug::NAME,
        ProductionDebug::PLUGIN,
        vec!["app.use(otherMiddleware())"],
        vec!["app.use(errorhandler())"],
    )
    .test_and_snapshot();
}
