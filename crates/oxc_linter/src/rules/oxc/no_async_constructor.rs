use oxc_ast::{AstKind, ast::MethodDefinitionKind};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoAsyncConstructor;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows asynchronous class constructors.
    NoAsyncConstructor,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow async class constructors.",
);

impl Rule for NoAsyncConstructor {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::MethodDefinition(method) = node.kind() else {
            return;
        };
        if method.kind == MethodDefinitionKind::Constructor && method.value.r#async {
            ctx.diagnostic(
                OxcDiagnostic::warn("Class constructors cannot be asynchronous.")
                    .with_help("Move asynchronous initialization into a separate method.")
                    .with_label(method.span),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoAsyncConstructor::NAME,
        NoAsyncConstructor::PLUGIN,
        vec!["class Example { constructor() {} }"],
        vec!["class Example { async constructor() {} }"],
    )
    .test_and_snapshot();
}
