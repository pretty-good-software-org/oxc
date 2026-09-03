use oxc_ast::{AstKind, ast::TSType};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::Span;

use crate::{AstNode, context::LintContext, rule::Rule};

fn no_object_parameters_diagnostic(span: Span, parameter: &str) -> OxcDiagnostic {
    OxcDiagnostic::warn(format!("Parameter `{parameter}` uses the broad `object` type."))
        .with_help("Accept a named owner type and decode external input at its boundary.")
        .with_label(span)
}

#[derive(Debug, Default, Clone)]
pub struct NoObjectParameters;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows the broad `object` type on function parameters.
    ///
    /// ### Why is this bad?
    ///
    /// Broad object inputs hide ownership and prevent callers from expressing
    /// the domain contract of a value.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```typescript
    /// function process(value: object) {}
    /// ```
    NoObjectParameters,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow broad `object` parameters.",
);

impl Rule for NoObjectParameters {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::FormalParameters(parameters) = node.kind() else {
            return;
        };
        for parameter in &parameters.items {
            let Some(annotation) = &parameter.type_annotation else {
                continue;
            };
            if !matches!(annotation.type_annotation, TSType::TSObjectKeyword(_)) {
                continue;
            }
            let name = parameter.pattern.get_identifier_name();
            let name = name.as_deref().unwrap_or("parameter");
            ctx.diagnostic(no_object_parameters_diagnostic(annotation.span, name));
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec!["function process(value: User) {}", "function process(value: {}) {}"];
    let fail =
        vec!["function process(value: object) {}", "const process = (value: object) => value;"];

    Tester::new(NoObjectParameters::NAME, NoObjectParameters::PLUGIN, pass, fail)
        .test_and_snapshot();
}
