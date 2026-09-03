use oxc_ast::{AstKind, ast::Expression};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoInternalApiUse;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows importing dependencies through `node_modules` implementation paths.
    NoInternalApiUse,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow direct use of dependency internal APIs.",
);

impl Rule for NoInternalApiUse {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        match node.kind() {
            AstKind::ImportDeclaration(import) if import.source.value.contains("node_modules") => {
                report(ctx, import.source.span);
            }
            AstKind::CallExpression(call) => {
                let Expression::Identifier(identifier) = &call.callee else { return };
                if identifier.name != "require" || call.arguments.len() != 1 {
                    return;
                }
                let Some(Expression::StringLiteral(source)) = call.arguments[0].as_expression()
                else {
                    return;
                };
                if source.value.contains("node_modules") {
                    report(ctx, source.span);
                }
            }
            _ => {}
        }
    }
}

fn report(ctx: &LintContext, span: oxc_span::Span) {
    ctx.diagnostic(
        OxcDiagnostic::warn("Do not use internal APIs of your dependencies.")
            .with_help("Import the dependency through its public package API.")
            .with_label(span),
    );
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoInternalApiUse::NAME,
        NoInternalApiUse::PLUGIN,
        vec!["import api from 'package';"],
        vec![
            "import api from 'package/node_modules/internal/api';",
            "require('package/node_modules/internal/api');",
        ],
    )
    .test_and_snapshot();
}
