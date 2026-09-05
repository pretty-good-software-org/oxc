// Port of sonarjs/index-of-compare-to-positive-number (S2692).
// https://github.com/SonarSource/eslint-plugin-sonarjs/blob/v3.0.7/packages/jsts/src/rules/S2692
//
// Upstream uses full TypeScript type information to confirm the `indexOf` receiver is an
// `Array`, so it never flags `String.prototype.indexOf`. Oxc's linter does not have a type
// checker, so this port instead relies on a conservative syntactic heuristic: it only fires
// when the receiver is unambiguously array-producing syntax (an array literal, `new Array()`,
// `Array.from`/`Array.of`, `Object.keys`/`values`/`entries`, or common array-returning methods
// such as `.map`/`.filter`/`.slice`/`.concat`/`.flat`/`.flatMap`). This trades recall for
// avoiding false positives on strings and other `indexOf`-bearing types.

use oxc_ast::{AstKind, ast::Expression};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::{GetSpan, Span};
use oxc_syntax::operator::BinaryOperator;

use crate::{AstNode, context::LintContext, rule::Rule};

fn index_of_compare_to_positive_number_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn(
        "This check ignores index 0; consider using 'includes' method to make this check safe and explicit.",
    )
    .with_label(span)
}

#[derive(Debug, Default, Clone)]
pub struct IndexOfCompareToPositiveNumber;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Reports `array.indexOf(x) > 0` checks, which incorrectly ignore a match at index `0`.
    ///
    /// ### Why is this bad?
    ///
    /// `indexOf` returns `0` when the searched value is the first element of the array, so
    /// `array.indexOf(x) > 0` is `false` for a match at index `0`, even though the value is
    /// present. Use `array.includes(x)` (or `array.indexOf(x) !== -1` / `>= 0`) to make the
    /// intent safe and explicit.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```javascript
    /// if (array.indexOf(x) > 0) {
    ///   // ...
    /// }
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```javascript
    /// if (array.includes(x)) {
    ///   // ...
    /// }
    /// if (array.indexOf(x) >= 0) {
    ///   // ...
    /// }
    /// ```
    IndexOfCompareToPositiveNumber,
    oxc,
    style,
    none,
    version = "0.0.1",
    short_description = "Reports `indexOf(x) > 0` checks that ignore a match at index 0.",
);

impl Rule for IndexOfCompareToPositiveNumber {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::BinaryExpression(expr) = node.kind() else { return };
        if expr.operator != BinaryOperator::GreaterThan || !is_zero(&expr.right) {
            return;
        }
        if is_array_index_of_call(&expr.left) {
            ctx.diagnostic(index_of_compare_to_positive_number_diagnostic(expr.span()));
        }
    }
}

fn is_zero(expr: &Expression) -> bool {
    matches!(expr, Expression::NumericLiteral(lit) if lit.value == 0.0)
}

fn is_array_index_of_call(expr: &Expression) -> bool {
    let Expression::CallExpression(call) = expr else { return false };
    if call.arguments.len() != 1 {
        return false;
    }
    let Expression::StaticMemberExpression(member) = &call.callee else { return false };
    member.property.name == "indexOf"
        && is_array_like_expression(member.object.without_parentheses())
}

fn is_array_like_expression(expr: &Expression) -> bool {
    match expr {
        Expression::ArrayExpression(_) => true,
        Expression::NewExpression(new_expr) => {
            matches!(&new_expr.callee, Expression::Identifier(ident) if ident.name == "Array")
        }
        Expression::CallExpression(call) => is_array_producing_call(call),
        _ => false,
    }
}

fn is_array_producing_call(call: &oxc_ast::ast::CallExpression) -> bool {
    let Expression::StaticMemberExpression(member) = &call.callee else { return false };
    let Expression::Identifier(object) = member.object.without_parentheses() else {
        return array_returning_method(&member.property.name);
    };
    match object.name.as_str() {
        "Array" => matches!(member.property.name.as_str(), "from" | "of"),
        "Object" => matches!(member.property.name.as_str(), "keys" | "values" | "entries"),
        _ => array_returning_method(&member.property.name),
    }
}

fn array_returning_method(name: &str) -> bool {
    matches!(name, "map" | "filter" | "slice" | "concat" | "flat" | "flatMap")
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec![
        "array.indexOf(x) >= 0;",
        "array.indexOf(x) !== -1;",
        "array.includes(x);",
        "str.indexOf(x) > 0;",
        "value.indexOf(x) > 0;",
        "array.indexOf(x, y) > 0;",
        "array.lastIndexOf(x) > 0;",
        "array.indexOf(x) > 1;",
    ];

    let fail = vec![
        "[1, 2, 3].indexOf(x) > 0;",
        "new Array(1, 2, 3).indexOf(x) > 0;",
        "Array.from(iterable).indexOf(x) > 0;",
        "Array.of(1, 2).indexOf(x) > 0;",
        "Object.keys(obj).indexOf(x) > 0;",
        "Object.values(obj).indexOf(x) > 0;",
        "Object.entries(obj).indexOf(x) > 0;",
        "array.map(f).indexOf(x) > 0;",
        "array.filter(f).indexOf(x) > 0;",
        "array.slice(1).indexOf(x) > 0;",
        "array.concat(other).indexOf(x) > 0;",
        "array.flat().indexOf(x) > 0;",
        "array.flatMap(f).indexOf(x) > 0;",
    ];

    Tester::new(
        IndexOfCompareToPositiveNumber::NAME,
        IndexOfCompareToPositiveNumber::PLUGIN,
        pass,
        fail,
    )
    .test_and_snapshot();
}
