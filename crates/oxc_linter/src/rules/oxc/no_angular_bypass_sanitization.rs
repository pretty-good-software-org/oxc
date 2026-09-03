use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::GetSpan;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoAngularBypassSanitization;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Reports Angular DOM sanitization bypass APIs.
    NoAngularBypassSanitization,
    oxc,
    suspicious,
    none,
    version = "0.0.1",
    short_description = "Disallow Angular sanitization bypass APIs.",
);

impl Rule for NoAngularBypassSanitization {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::CallExpression(call) = node.kind() else { return };
        let Some(member) = call.callee.as_member_expression() else { return };
        let Some(name) = member.static_property_name() else { return };
        if name.starts_with("bypassSecurityTrust") {
            ctx.diagnostic(
                OxcDiagnostic::warn("Review this Angular sanitization bypass.")
                    .with_help("Use Angular's standard sanitization instead of bypassing its security checks.")
                    .with_label(member.as_property_key().span()),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoAngularBypassSanitization::NAME,
        NoAngularBypassSanitization::PLUGIN,
        vec!["sanitizer.sanitize(value)"],
        vec!["sanitizer.bypassSecurityTrustHtml(value)"],
    )
    .test_and_snapshot();
}
