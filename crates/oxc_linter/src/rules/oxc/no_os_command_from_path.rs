use crate::{AstNode, context::LintContext, rule::Rule};
use oxc_ast::{AstKind, ast::Expression};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

#[derive(Debug, Default, Clone)]
pub struct NoOsCommandFromPath;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows shell commands assembled from a bare executable name.
    NoOsCommandFromPath,
    oxc,
    suspicious,
    none,
    version = "0.0.1",
    short_description = "Disallow commands resolved through PATH.",
);

impl Rule for NoOsCommandFromPath {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let Expression::Identifier(identifier) = &call.callee else {
            return;
        };
        if !matches!(identifier.name.as_str(), "exec" | "execFile" | "spawn") {
            return;
        }
        if matches!(call.arguments.first(), Some(argument) if matches!(argument.as_expression(), Some(Expression::StringLiteral(_))))
        {
            ctx.diagnostic(
                OxcDiagnostic::warn("Do not execute a command resolved from PATH.")
                    .with_help("Use a trusted absolute executable path or an allowlisted command.")
                    .with_label(call.span),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoOsCommandFromPath::NAME,
        NoOsCommandFromPath::PLUGIN,
        vec!["runTrustedCommand(command);"],
        vec!["exec(\"curl https://example.test\");"],
    )
    .test_and_snapshot();
}
