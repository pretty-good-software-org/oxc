use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_semantic::IsGlobalReference;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoGlobalThis;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows direct use of the global `globalThis` object.
    NoGlobalThis,
    oxc,
    suspicious,
    none,
    version = "0.0.1",
    short_description = "Disallow direct globalThis access.",
);

impl Rule for NoGlobalThis {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::IdentifierReference(identifier) = node.kind() else {
            return;
        };
        if identifier.name == "globalThis" && identifier.is_global_reference(ctx.scoping()) {
            ctx.diagnostic(
                OxcDiagnostic::warn("Avoid direct use of `globalThis`.")
                    .with_help("Use an explicit platform dependency instead.")
                    .with_label(identifier.span),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoGlobalThis::NAME,
        NoGlobalThis::PLUGIN,
        vec!["const globalThis = {}; globalThis.value;"],
        vec!["globalThis.value;"],
    )
    .test_and_snapshot();
}
