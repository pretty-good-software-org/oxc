use oxc_ast::{AstKind, ast::Expression};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_syntax::operator::AssignmentOperator;

use crate::{AstNode, ast_util::get_outer_member_expression, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoInsecureCookie;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows setting browser cookies without a Secure attribute.
    NoInsecureCookie,
    oxc,
    suspicious,
    none,
    version = "0.0.1",
    short_description = "Require secure browser cookies.",
);

impl Rule for NoInsecureCookie {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::AssignmentExpression(assignment) = node.kind() else {
            return;
        };
        if assignment.operator != AssignmentOperator::Assign {
            return;
        }
        let Some(target) = assignment.left.as_simple_assignment_target() else {
            return;
        };
        let Some(member) = get_outer_member_expression(target) else {
            return;
        };
        if !matches!(&member.object, Expression::Identifier(identifier) if identifier.name == "document")
            || member.property.name != "cookie"
        {
            return;
        }
        let Expression::StringLiteral(value) = &assignment.right else {
            return;
        };
        if !value.value.split(';').any(|part| part.trim().eq_ignore_ascii_case("secure")) {
            ctx.diagnostic(
                OxcDiagnostic::warn("Do not set a browser cookie without the Secure attribute.")
                    .with_help("Add `Secure` to the cookie attributes.")
                    .with_label(assignment.span),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoInsecureCookie::NAME,
        NoInsecureCookie::PLUGIN,
        vec![r#"document.cookie = "session=abc; Secure";"#],
        vec![r#"document.cookie = "session=abc";"#],
    )
    .test_and_snapshot();
}
