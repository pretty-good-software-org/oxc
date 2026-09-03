use oxc_ast::{AstKind, ast::TSType};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::Span;

use crate::{AstNode, context::LintContext, rule::Rule};

fn no_unknown_parameters_diagnostic(span: Span, parameter: &str) -> OxcDiagnostic {
    OxcDiagnostic::warn(format!("Parameter `{parameter}` leaves input unparsed."))
        .with_help("Accept a named domain type and decode unknown input at its boundary.")
        .with_label(span)
}

#[derive(Debug, Default, Clone)]
pub struct NoUnknownParameters;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows explicitly `unknown` function parameters, except `cause`.
    ///
    /// ### Why is this bad?
    ///
    /// Unknown inputs should be decoded at the I/O boundary instead of being
    /// passed through application code.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```typescript
    /// function process(value: unknown) {}
    /// ```
    NoUnknownParameters,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow unparsed `unknown` parameters.",
);

impl Rule for NoUnknownParameters {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::FormalParameters(parameters) = node.kind() else {
            return;
        };
        for parameter in &parameters.items {
            let Some(annotation) = &parameter.type_annotation else {
                continue;
            };
            if !matches!(annotation.type_annotation, TSType::TSUnknownKeyword(_)) {
                continue;
            }
            let name = parameter.pattern.get_identifier_name();
            if name.as_deref() == Some("cause") {
                continue;
            }
            let name = name.as_deref().unwrap_or("parameter");
            ctx.diagnostic(no_unknown_parameters_diagnostic(annotation.span, name));
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec!["function process(value: string) {}", "function fail(cause: unknown) {}"];
    let fail =
        vec!["function process(value: unknown) {}", "const process = (value: unknown) => value;"];

    Tester::new(NoUnknownParameters::NAME, NoUnknownParameters::PLUGIN, pass, fail)
        .test_and_snapshot();
}
