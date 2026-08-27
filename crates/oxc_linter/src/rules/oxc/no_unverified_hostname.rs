use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::GetSpan;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoUnverifiedHostname;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows hostname verification callbacks that return `undefined`.
    NoUnverifiedHostname,
    oxc,
    suspicious,
    none,
    version = "0.0.1",
    short_description = "Disallow disabled TLS hostname verification.",
);

impl Rule for NoUnverifiedHostname {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::ObjectProperty(property) = node.kind() else {
            return;
        };
        if property.key.static_name().is_some_and(|name| name == "checkServerIdentity")
            && matches!(property.value, oxc_ast::ast::Expression::ArrowFunctionExpression(_))
            && ctx.source_range(property.value.span()).contains("undefined")
        {
            ctx.diagnostic(
                OxcDiagnostic::warn("Do not disable TLS hostname verification.")
                    .with_help("Validate the peer hostname against the certificate.")
                    .with_label(property.span),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoUnverifiedHostname::NAME,
        NoUnverifiedHostname::PLUGIN,
        vec!["const options = { checkServerIdentity: verify };"],
        vec!["const options = { checkServerIdentity: () => undefined };"],
    )
    .test_and_snapshot();
}
