use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::{GetSpan, Span};
use std::net::IpAddr;

use crate::{AstNode, context::LintContext, rule::Rule};

fn diagnostic(value: &str, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn(format!("Make sure using a hardcoded IP address {value} is safe here."))
        .with_help("Use configuration or a named service instead of embedding an IP address.")
        .with_label(span)
}

#[derive(Debug, Default, Clone)]
pub struct NoHardcodedIp;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows IPv4 addresses in string literals.
    ///
    /// ### Why is this bad?
    ///
    /// Hard-coded addresses become stale and make deployments environment-specific.
    NoHardcodedIp,
    oxc,
    suspicious,
    none,
    version = "0.0.1",
    short_description = "Disallow hard-coded IP addresses.",
);

impl Rule for NoHardcodedIp {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let expression = match node.kind() {
            AstKind::StringLiteral(literal) => literal.value.as_str(),
            AstKind::TemplateLiteral(template) if template.expressions.is_empty() => {
                template.quasis.first().map_or("", |quasi| quasi.value.raw.as_str())
            }
            _ => return,
        };
        if is_ipv4(expression) {
            ctx.diagnostic(diagnostic(expression, node.span()));
        }
    }
}

fn is_ipv4(value: &str) -> bool {
    let ip = value
        .split_once('/')
        .filter(|(_, mask)| mask.chars().all(|character| character.is_ascii_digit()))
        .map_or(value, |(ip, _)| ip);
    if matches!(ip, "255.255.255.255" | "::" | "::1")
        || ip.starts_with("127.")
        || ip.starts_with("0.")
        || ip.starts_with("192.0.2.")
        || ip.starts_with("198.51.100.")
        || ip.starts_with("203.0.113.")
    {
        return false;
    }
    ip.parse::<IpAddr>().is_ok()
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoHardcodedIp::NAME,
        NoHardcodedIp::PLUGIN,
        vec![r#"const host = "service.internal";"#, r#"const host = "999.1.1.1";"#],
        vec![r#"const host = "192.168.0.1";"#, r#"const host = "10.0.0.1/24";"#],
    )
    .test_and_snapshot();
}
