use oxc_ast::{AstKind, ast::Expression};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::Span;

use crate::{AstNode, context::LintContext, rule::Rule};

fn no_module_mocking_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Avoid test-framework module mocking.")
        .with_help("Replace module mocking with dependency injection through a real interface.")
        .with_label(span)
}

#[derive(Debug, Default, Clone)]
pub struct NoModuleMocking;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows global Vitest and Jest module mocking calls.
    ///
    /// ### Why is this bad?
    ///
    /// Module mocks hide dependency seams and make tests diverge from runtime
    /// behavior. Prefer dependency injection or a faithful test implementation.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```javascript
    /// vi.mock("./client");
    /// jest.mock("./client");
    /// ```
    NoModuleMocking,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow test-framework module mocking.",
);

impl Rule for NoModuleMocking {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let Some(member) = call.callee.as_member_expression() else {
            return;
        };
        let Expression::Identifier(object) = member.object() else {
            return;
        };
        if !matches!(object.name.as_str(), "vi" | "jest")
            || !ctx.is_reference_to_global_variable(object)
            || !member
                .static_property_name()
                .is_some_and(|name| matches!(name, "mock" | "doMock" | "unstable_mockModule"))
        {
            return;
        }
        ctx.diagnostic(no_module_mocking_diagnostic(call.span));
    }
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec!["client.mock()", "function f(vi) { vi.mock('./client'); }"];
    let fail = vec!["vi.mock('./client')", "jest.doMock('./client')"];

    Tester::new(NoModuleMocking::NAME, NoModuleMocking::PLUGIN, pass, fail).test_and_snapshot();
}
