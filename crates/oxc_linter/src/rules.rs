mod no_array_constructor;
mod no_debugger;
mod no_empty;

pub use no_array_constructor::NoArrayConstructor;
pub use no_debugger::NoDebugger;
pub use no_empty::NoEmpty;

use crate::{context::LintContext, rule::Rule, AstNode};

lazy_static::lazy_static! {
    pub static ref RULES: Vec<RuleEnum> = vec![
        RuleEnum::NoDebugger(NoDebugger::default()),
        RuleEnum::NoEmpty(NoEmpty::default()),
        RuleEnum::NoArrayConstructor(NoArrayConstructor::default()),
    ];
}

#[derive(Debug, Clone)]
#[allow(clippy::enum_variant_names)]
pub enum RuleEnum {
    NoDebugger(NoDebugger),
    NoEmpty(NoEmpty),
    NoArrayConstructor(NoArrayConstructor),
}

impl RuleEnum {
    pub const fn name(&self) -> &'static str {
        match self {
            Self::NoDebugger(_) => NoDebugger::NAME,
            Self::NoEmpty(_) => NoEmpty::NAME,
            Self::NoArrayConstructor(_) => NoArrayConstructor::NAME,
        }
    }

    pub fn read_json(&self, maybe_value: Option<serde_json::Value>) -> Self {
        match self {
            Self::NoDebugger(_) => Self::NoDebugger(
                maybe_value.map(NoDebugger::from_configuration).unwrap_or_default(),
            ),
            Self::NoEmpty(_) => {
                Self::NoEmpty(maybe_value.map(NoEmpty::from_configuration).unwrap_or_default())
            }
            Self::NoArrayConstructor(_) => Self::NoArrayConstructor(
                maybe_value.map(NoArrayConstructor::from_configuration).unwrap_or_default(),
            ),
        }
    }

    pub fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        match self {
            Self::NoDebugger(rule) => rule.run(node, ctx),
            Self::NoEmpty(rule) => rule.run(node, ctx),
            Self::NoArrayConstructor(rule) => rule.run(node, ctx),
        }
    }
}