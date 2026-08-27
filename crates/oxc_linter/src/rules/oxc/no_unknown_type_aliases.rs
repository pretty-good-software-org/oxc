use rustc_hash::FxHashMap;

use oxc_ast::{
    AstKind,
    ast::{TSType, TSTypeAliasDeclaration, TSTypeName},
};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::Span;

use crate::{context::LintContext, rule::Rule};

fn no_unknown_type_aliases_diagnostic(span: Span, alias: &str) -> OxcDiagnostic {
    OxcDiagnostic::warn(format!("Type alias `{alias}` hides `unknown`."))
        .with_help("Keep `unknown` explicit at the parsing boundary or use a parsed owner type.")
        .with_label(span)
}

#[derive(Debug, Default, Clone)]
pub struct NoUnknownTypeAliases;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows named aliases that merely conceal TypeScript's `unknown` type.
    ///
    /// ### Why is this bad?
    ///
    /// An alias hides the boundary where an unknown value should be decoded.
    /// Keep `unknown` visible or replace it with a parsed domain type.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```typescript
    /// type UnknownValue = unknown;
    /// ```
    NoUnknownTypeAliases,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow aliases that conceal `unknown`.",
);

impl Rule for NoUnknownTypeAliases {
    fn run_once(&self, ctx: &LintContext) {
        let aliases: FxHashMap<String, &TSTypeAliasDeclaration<'_>> = ctx
            .nodes()
            .iter()
            .filter_map(|node| match node.kind() {
                AstKind::TSTypeAliasDeclaration(alias) => Some((alias.id.name.to_string(), alias)),
                _ => None,
            })
            .collect();

        let mut alias_values: Vec<_> = aliases.values().copied().collect();
        alias_values.sort_by_key(|alias| alias.id.name.as_str());
        for alias in alias_values {
            let mut visited = Vec::new();
            if resolves_to_unknown(&alias.type_annotation, &aliases, &mut visited) {
                ctx.diagnostic(no_unknown_type_aliases_diagnostic(
                    alias.id.span,
                    alias.id.name.as_str(),
                ));
            }
        }
    }
}

fn resolves_to_unknown<'a>(
    type_annotation: &'a TSType<'a>,
    aliases: &FxHashMap<String, &TSTypeAliasDeclaration<'a>>,
    visited: &mut Vec<String>,
) -> bool {
    match type_annotation {
        TSType::TSUnknownKeyword(_) => true,
        TSType::TSParenthesizedType(parenthesized) => {
            resolves_to_unknown(&parenthesized.type_annotation, aliases, visited)
        }
        TSType::TSTypeReference(reference) => {
            let Some(identifier) = TSTypeName::get_identifier_reference(&reference.type_name)
            else {
                return false;
            };
            if reference.type_arguments.as_ref().is_some_and(|args| !args.params.is_empty())
                || visited.iter().any(|name| name == identifier.name.as_str())
            {
                return false;
            }
            let name = identifier.name.to_string();
            let Some(alias) = aliases.get(&name) else {
                return false;
            };
            visited.push(name);
            let result = resolves_to_unknown(&alias.type_annotation, aliases, visited);
            visited.pop();
            result
        }
        _ => false,
    }
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec!["type User = { name: string };", "type Value = unknown | string;"];
    let fail = vec![
        "type UnknownValue = unknown;",
        "type Alias = UnknownValue;\ntype UnknownValue = unknown;",
    ];

    Tester::new(NoUnknownTypeAliases::NAME, NoUnknownTypeAliases::PLUGIN, pass, fail)
        .test_and_snapshot();
}
