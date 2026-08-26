use oxc_ast::{AstKind, ast::Expression};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::Span;

use crate::{AstNode, context::LintContext, rule::Rule};

fn no_reflect_apply_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Avoid `Reflect.apply`.")
        .with_help("Use a direct call or a typed wrapper instead.")
        .with_label(span)
}

#[derive(Debug, Default, Clone)]
pub struct NoReflectApply;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows global `Reflect.apply` calls.
    ///
    /// ### Why is this bad?
    ///
    /// `Reflect.apply` bypasses ordinary call syntax and hides useful type
    /// evidence at the call site.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```javascript
    /// Reflect.apply(fn, receiver, args);
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```javascript
    /// fn(...args);
    /// ```
    NoReflectApply,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow global `Reflect.apply` calls.",
);

impl Rule for NoReflectApply {
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
        if object.name == "Reflect"
            && ctx.is_reference_to_global_variable(object)
            && member.static_property_name().is_some_and(|name| name == "apply")
        {
            ctx.diagnostic(no_reflect_apply_diagnostic(call.span));
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec![
        "fn(...args)",
        "other.Reflect.apply(fn, receiver, args)",
        "function f(Reflect) { Reflect.apply(fn, receiver, args); }",
    ];
    let fail = vec!["Reflect.apply(fn, receiver, args)", "Reflect[\"apply\"](fn, receiver, args)"];

    Tester::new(NoReflectApply::NAME, NoReflectApply::PLUGIN, pass, fail).test_and_snapshot();
}
