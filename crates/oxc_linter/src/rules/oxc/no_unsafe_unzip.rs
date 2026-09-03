use oxc_ast::{AstKind, ast::Expression};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::GetSpan;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoUnsafeUnzip;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Reports known archive extraction APIs that lack an explicit safety policy.
    NoUnsafeUnzip,
    oxc,
    suspicious,
    none,
    version = "0.0.1",
    short_description = "Review archive extraction for path traversal safety.",
);

impl Rule for NoUnsafeUnzip {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::CallExpression(call) = node.kind() else { return };
        let Some(member) = call.callee.as_member_expression() else { return };
        let Some(method) = member.static_property_name() else { return };
        let object_name = match member.object() {
            Expression::Identifier(identifier) => identifier.name.as_str(),
            _ => return,
        };
        let known = matches!(
            (object_name, method),
            ("jszip", "loadAsync") | ("yauzl", "open") | ("admZip", "extractAllTo") | ("tar", "x")
        );
        if known && !(method == "loadAsync" && ctx.source_range(call.span).contains("onEntry")) {
            ctx.diagnostic(
                OxcDiagnostic::warn("Make sure that expanding this archive file is safe here.")
                    .with_help(
                        "Validate archive entries and prevent path traversal before extraction.",
                    )
                    .with_label(member.as_property_key().span()),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoUnsafeUnzip::NAME,
        NoUnsafeUnzip::PLUGIN,
        vec!["jszip.loadAsync(data, { onEntry })"],
        vec!["jszip.loadAsync(data)"],
    )
    .test_and_snapshot();
}
