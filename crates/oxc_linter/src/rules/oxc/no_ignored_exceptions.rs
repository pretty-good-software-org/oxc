use oxc_ast::{AstKind, ast::BindingPattern};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoIgnoredExceptions;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Reports catch clauses that bind an exception without using it.
    NoIgnoredExceptions,
    oxc,
    suspicious,
    none,
    version = "0.0.1",
    short_description = "Disallow ignored caught exceptions.",
);

impl Rule for NoIgnoredExceptions {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::CatchClause(clause) = node.kind() else { return };
        let Some(parameter) = clause.param.as_ref() else { return };
        let BindingPattern::BindingIdentifier(identifier) = &parameter.pattern else { return };
        let body = ctx.source_range(clause.body.span);
        let used = body
            .split(|character: char| !character.is_ascii_alphanumeric() && character != '_')
            .any(|word| word == identifier.name.as_str());
        if !used {
            ctx.diagnostic(
                OxcDiagnostic::warn("Handle this exception or do not catch it at all.")
                    .with_help("Use the caught exception or remove the catch clause.")
                    .with_label(clause.span),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoIgnoredExceptions::NAME,
        NoIgnoredExceptions::PLUGIN,
        vec!["try { run(); } catch (error) { log(error); }"],
        vec!["try { run(); } catch (error) { recover(); }"],
    )
    .test_and_snapshot();
}
