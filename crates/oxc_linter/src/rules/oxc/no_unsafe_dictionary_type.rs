use oxc_ast::{
    AstKind,
    ast::{TSType, TSTypeName},
};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::Span;

use crate::{AstNode, context::LintContext, rule::Rule};

fn no_unsafe_dictionary_type_diagnostic(span: Span, value: &str) -> OxcDiagnostic {
    OxcDiagnostic::warn(format!("This dictionary's `{value}` value type is unsafe."))
        .with_help("Use an owner/schema-derived value type and parse payloads before insertion.")
        .with_label(span)
}

#[derive(Debug, Default, Clone)]
pub struct NoUnsafeDictionaryType;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows dictionary contracts whose direct value type is `unknown`,
    /// `any`, or `object`.
    ///
    /// ### Why is this bad?
    ///
    /// Broad dictionary values give callers no concrete contract and allow
    /// unchecked data to spread through the application.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```typescript
    /// type Values = Record<string, unknown>;
    /// ```
    NoUnsafeDictionaryType,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow unsafe dictionary value types.",
);

impl Rule for NoUnsafeDictionaryType {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::TSTypeReference(reference) = node.kind() else {
            return;
        };
        if matches!(ctx.nodes().parent_kind(node.id()), AstKind::TSTypeReference(_)) {
            return;
        }
        let Some(name) = TSTypeName::get_identifier_reference(&reference.type_name) else {
            return;
        };
        if !matches!(name.name.as_str(), "Record" | "Readonly") {
            return;
        }
        if name.name == "Record"
            && ctx.nodes().ancestors(node.id()).any(|ancestor| {
                matches!(ancestor.kind(), AstKind::TSTypeReference(reference)
                    if TSTypeName::get_identifier_reference(&reference.type_name)
                        .is_some_and(|name| name.name == "Readonly"))
            })
        {
            return;
        }
        let Some(arguments) = &reference.type_arguments else {
            return;
        };
        let params = &arguments.params;
        let value_type = if name.name == "Record" {
            params.get(1)
        } else if params.len() == 1 {
            match &params[0] {
                TSType::TSTypeReference(inner) => inner
                    .type_name
                    .get_identifier_reference()
                    .filter(|inner_name| inner_name.name == "Record")
                    .and_then(|_| inner.type_arguments.as_ref())
                    .and_then(|inner_args| inner_args.params.get(1)),
                _ => None,
            }
        } else {
            None
        };
        let Some(value_type) = value_type else {
            return;
        };
        let value_name = match value_type {
            TSType::TSUnknownKeyword(_) => "unknown",
            TSType::TSAnyKeyword(_) => "any",
            TSType::TSObjectKeyword(_) => "object",
            _ => return,
        };
        ctx.diagnostic(no_unsafe_dictionary_type_diagnostic(reference.span, value_name));
    }
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec!["type Values = Record<string, User>;", "type Values = Map<string, unknown>;"];
    let fail = vec![
        "type Values = Record<string, unknown>;",
        "type Values = Readonly<Record<string, any>>;",
    ];

    Tester::new(NoUnsafeDictionaryType::NAME, NoUnsafeDictionaryType::PLUGIN, pass, fail)
        .test_and_snapshot();
}
