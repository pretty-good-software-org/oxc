use oxc_ast::{AstKind, ast::Expression};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::Span;

use crate::{AstNode, context::LintContext, rule::Rule};

fn no_reflect_get_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Avoid `Reflect.get`.")
        .with_help("Use typed property access or parse dynamic input into a domain type.")
        .with_label(span)
}

#[derive(Debug, Default, Clone)]
pub struct NoReflectGet;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows global `Reflect.get` calls.
    ///
    /// ### Why is this bad?
    ///
    /// `Reflect.get` bypasses ordinary property access and hides useful type
    /// evidence at the call site.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```javascript
    /// Reflect.get(value, key);
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```javascript
    /// value[key];
    /// ```
    NoReflectGet,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow global `Reflect.get` calls.",
);

impl Rule for NoReflectGet {
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
            && member.static_property_name().is_some_and(|name| name == "get")
        {
            ctx.diagnostic(no_reflect_get_diagnostic(call.span));
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec![
        "value[key]",
        "other.Reflect.get(value, key)",
        "function f(Reflect) { Reflect.get(value, key); }",
    ];
    let fail = vec!["Reflect.get(value, key)", "Reflect[\"get\"](value, key)"];

    Tester::new(NoReflectGet::NAME, NoReflectGet::PLUGIN, pass, fail).test_and_snapshot();
}
