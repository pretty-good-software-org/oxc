use lazy_regex::regex;
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::{GetSpan, Span};

use crate::{AstNode, context::LintContext, rule::Rule};

fn diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Do not hard-code an IP address.")
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
            ctx.diagnostic(diagnostic(node.span()));
        }
    }
}

fn is_ipv4(value: &str) -> bool {
    let pattern = regex!(r"(?:^|[^\d])((?:\d{1,3}\.){3}\d{1,3})(?:$|[^\d])");
    pattern.captures(value).is_some_and(|captures| {
        captures
            .get(1)
            .is_some_and(|ip| ip.as_str().split('.').all(|part| part.parse::<u8>().is_ok()))
    })
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoHardcodedIp::NAME,
        NoHardcodedIp::PLUGIN,
        vec![r#"const host = "service.internal";"#, r#"const host = "999.1.1.1";"#],
        vec![r#"const host = "192.168.0.1";"#, r#"const url = "https://10.0.0.1/api";"#],
    )
    .test_and_snapshot();
}
