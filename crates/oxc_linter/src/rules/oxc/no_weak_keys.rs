use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoWeakKeys;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Reports obviously weak cryptographic key-generation options.
    NoWeakKeys,
    oxc,
    suspicious,
    none,
    version = "0.0.1",
    short_description = "Disallow weak cryptographic key parameters.",
);

impl Rule for NoWeakKeys {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let span = match node.kind() {
            AstKind::CallExpression(call) => call.span,
            AstKind::NewExpression(expression) => expression.span,
            _ => return,
        };
        let source = ctx.source_range(span);
        let crypto_call = ["generateKeyPair", "generateKey", "createCipher", "createSign"]
            .iter()
            .any(|name| source.contains(name));
        let weak_option = source.contains("modulusLength: 1024")
            || source.contains("modulusLength: 512")
            || source.contains("divisorLength: 160")
            || source.contains("divisorLength: 128")
            || source.contains("secp112")
            || source.contains("secp128")
            || source.contains("secp160");
        if crypto_call && weak_option {
            ctx.diagnostic(
                OxcDiagnostic::warn("Use cryptographic key parameters that provide sufficient security.")
                    .with_help("Use at least a 2048-bit RSA modulus, a 224-bit DSA divisor, or a stronger elliptic curve.")
                    .with_label(span),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoWeakKeys::NAME,
        NoWeakKeys::PLUGIN,
        vec!["crypto.generateKeyPair('rsa', { modulusLength: 4096 })"],
        vec!["crypto.generateKeyPair('rsa', { modulusLength: 1024 })"],
    )
    .test_and_snapshot();
}
