use oxc_ast::{AstKind, ast::Expression};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::GetSpan;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct PseudoRandom;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Flags use of the non-cryptographic `Math.random` generator.
    PseudoRandom,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Ensure pseudorandom number generation is safe for its use.",
);

impl Rule for PseudoRandom {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::CallExpression(call) = node.kind() else { return };
        let Expression::StaticMemberExpression(member) = call.callee.without_parentheses() else {
            return;
        };
        let Expression::Identifier(object) = member.object.without_parentheses() else { return };
        if object.name == "Math" && member.property.name == "random" {
            ctx.diagnostic(
                OxcDiagnostic::warn(
                    "Make sure that using this pseudorandom number generator is safe here.",
                )
                .with_label(call.span()),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        PseudoRandom::NAME,
        PseudoRandom::PLUGIN,
        vec!["crypto.getRandomValues(buffer);"],
        vec!["Math.random();"],
    )
    .test_and_snapshot();
}
