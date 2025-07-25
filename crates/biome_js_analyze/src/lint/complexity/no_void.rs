use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::JsUnaryExpression;
use biome_rowan::AstNode;
use biome_rule_options::no_void::NoVoidOptions;

declare_lint_rule! {
    /// Disallow the use of `void` operators, which is not a familiar operator.
    ///
    /// > The `void` operator is often used merely to obtain the undefined primitive value,
    /// > usually using `void(0)` (which is equivalent to `void 0`). In these cases, the global variable `undefined` can be used.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// void 0;
    /// ```
    ///
    pub NoVoid {
        version: "1.0.0",
        name: "noVoid",
        language: "js",
        sources: &[RuleSource::Eslint("no-void").same()],
        recommended: false,
        severity: Severity::Warning,
    }
}

impl Rule for NoVoid {
    type Query = Ast<JsUnaryExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoVoidOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let expression = ctx.query();
        if expression.is_void().ok()? {
            Some(())
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "The use of "<Emphasis>"void"</Emphasis>" is not allowed."
            },
        ).note(
			markup!{
				"If you use "<Emphasis>"void"</Emphasis>" to alter the return type of a function or return `undefined`, use the global `undefined` instead."
			}
		))
    }
}
