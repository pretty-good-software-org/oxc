use oxc_ast::{AstKind, ast::Expression};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoUnverifiedCertificate;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows disabling TLS certificate verification.
    NoUnverifiedCertificate,
    oxc,
    suspicious,
    none,
    version = "0.0.1",
    short_description = "Disallow disabled TLS certificate verification.",
);

impl Rule for NoUnverifiedCertificate {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::ObjectProperty(property) = node.kind() else {
            return;
        };
        if property.key.static_name().is_some_and(|name| name == "rejectUnauthorized")
            && matches!(&property.value, Expression::BooleanLiteral(value) if !value.value)
        {
            ctx.diagnostic(
                OxcDiagnostic::warn("Do not disable TLS certificate verification.")
                    .with_help("Keep `rejectUnauthorized` enabled and use a trusted certificate.")
                    .with_label(property.span),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoUnverifiedCertificate::NAME,
        NoUnverifiedCertificate::PLUGIN,
        vec!["const options = { rejectUnauthorized: true };"],
        vec!["const options = { rejectUnauthorized: false };"],
    )
    .test_and_snapshot();
}
