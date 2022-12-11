use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    AnyJsFunctionBody, JsArrowFunctionExpression, JsFunctionDeclaration, JsFunctionExpression,
    JsLanguage, JsMethodClassMember, JsPropertyClassMember, JsSyntaxKind,
};
use rome_rowan::{declare_node_union, AstNode, NodeOrToken, SyntaxNode, SyntaxToken};

declare_rule! {
    /// Disallow async functions which have no `await` expression.
    ///
    /// Asynchronous functions in JavaScript behave differently than other functions in two important ways:
    /// 1. The return value is always a `Promise`.
    /// 2. You can use the `await` operator inside of them.
    ///
    /// Asynchronous functions that don’t use `await` might not need to be asynchronous functions and could be the unintentional result of refactoring.
    ///
    /// Note: This rule ignores async generator functions. This is because generators yield rather than return a value and async generators might yield all the values of another async generator without ever actually needing to use `await`.
    ///
    /// Source: [require-await](https://eslint.org/docs/latest/rules/require-await).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// async function foo() {
    ///     doSomething();
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// bar(async () => {
    /// 	doSomething();
    /// });
    /// ```
    ///
    /// ### Valid
    /// ```js
    /// async function foo() {
    ///     await doSomething();
    /// }
    ///
    /// bar(async () => {
    ///     await doSomething();
    /// });
    ///
    /// function foo() {
    ///     doSomething();
    /// }
    ///
    /// bar(() => {
    ///     doSomething();
    /// });
    ///
    /// // Allow empty functions.
    /// async function noop() {}
    /// ```
    ///
    pub(crate) UseAwait {
        version: "12.0.0",
        name: "useAwait",
        recommended: true,
    }
}

declare_node_union! {
    pub(crate) AnyFunction =
        JsPropertyClassMember
        | JsFunctionDeclaration
        | JsFunctionExpression
        | JsMethodClassMember
        | JsArrowFunctionExpression
}

impl Rule for UseAwait {
    type Query = Ast<AnyFunction>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let (async_token, start_token, function_body_syntax) = match node {
            AnyFunction::JsArrowFunctionExpression(arrow_func) => {
                let syntax_body = get_function_body_syntax(arrow_func.body().ok()?);
                Some((arrow_func.async_token(), None, syntax_body))
            }
            AnyFunction::JsFunctionDeclaration(func_declaration) => Some((
                func_declaration.async_token(),
                func_declaration.star_token(),
                func_declaration.body().ok()?.statements().into_syntax(),
            )),
            AnyFunction::JsFunctionExpression(func_expression) => Some((
                func_expression.async_token(),
                func_expression.star_token(),
                func_expression.body().ok()?.statements().into_syntax(),
            )),
            AnyFunction::JsMethodClassMember(class_method) => Some((
                class_method.async_token(),
                class_method.star_token(),
                class_method.body().ok()?.statements().into_syntax(),
            )),
            AnyFunction::JsPropertyClassMember(class_property) => {
                let binding = class_property.value()?.expression().ok()?;
                let arrow_func = binding.as_js_arrow_function_expression()?;
                let syntax_body = get_function_body_syntax(arrow_func.body().ok()?);
                Some((arrow_func.async_token(), None, syntax_body))
            }
        }?;

        if async_token.is_some()
            && start_token.is_none()
            && !function_body_syntax.clone().into_list().is_empty()
            && !has_await_kw(NodeOrToken::from(function_body_syntax))?
        {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! {"Async function has no "<Emphasis>"await"</Emphasis>" expression."},
        ))
    }
}

fn get_function_body_syntax(body: AnyJsFunctionBody) -> SyntaxNode<JsLanguage> {
    match body {
        AnyJsFunctionBody::AnyJsExpression(js_expression) => js_expression.into_syntax(),
        AnyJsFunctionBody::JsFunctionBody(function_body) => {
            function_body.statements().into_syntax()
        }
    }
}

static FUNCTION_KINDS: [JsSyntaxKind; 2] = [
    JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION,
    JsSyntaxKind::FUNCTION_KW,
];

/// Traverses the syntax tree and verifies the presence of the await keyword.
fn has_await_kw(
    node: NodeOrToken<SyntaxNode<JsLanguage>, SyntaxToken<JsLanguage>>,
) -> Option<bool> {
    if node.kind() == JsSyntaxKind::AWAIT_KW {
        return Some(true);
    }

    if FUNCTION_KINDS.contains(&node.kind()) || node.as_token().is_some() {
        return Some(false);
    }

    for child in node.as_node()?.children_with_tokens() {
        if !has_await_kw(child)? {
            continue;
        }

        return Some(true);
    }

    Some(false)
}
