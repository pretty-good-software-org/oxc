use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoNestedTemplateLiterals;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows template literals nested inside another template literal.
    NoNestedTemplateLiterals,
    oxc,
    suspicious,
    none,
    version = "0.0.1",
    short_description = "Disallow nested template literals.",
);

impl Rule for NoNestedTemplateLiterals {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::TemplateLiteral(template) = node.kind() else {
            return;
        };
        if ctx
            .nodes()
            .ancestors(node.id())
            .any(|ancestor| matches!(ancestor.kind(), AstKind::TemplateLiteral(_)))
        {
            ctx.diagnostic(
                OxcDiagnostic::warn("Do not nest template literals.")
                    .with_help("Extract the inner template or simplify the interpolation.")
                    .with_label(template.span),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoNestedTemplateLiterals::NAME,
        NoNestedTemplateLiterals::PLUGIN,
        vec!["const value = `${prefix}${suffix}`;"],
        vec!["const value = `${`${prefix}${suffix}`}`;"],
    )
    .test_and_snapshot();
}
