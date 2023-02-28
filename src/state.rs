use serde::{Deserialize, Serialize};
use strum_macros::Display;
use yew::Reducible;
use yewdux::store::{Reducer, Store};

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, Serialize, Deserialize, Store)]
#[strum(serialize_all = "snake_case")]
#[store(storage = "local")]
pub enum ColorMode {
    Light,
    Dark,
}

impl ColorMode {
    pub(crate) fn toggle(&self) -> Self {
        match self {
            ColorMode::Light => ColorMode::Dark,
            ColorMode::Dark => ColorMode::Light,
        }
    }
}

impl Default for ColorMode {
    fn default() -> Self {
        ColorMode::Light
    }
}

pub enum ColorModeActions {
    Toggle,
}

impl Reducer<ColorMode> for ColorModeActions {
    fn apply(self, state: std::rc::Rc<ColorMode>) -> std::rc::Rc<ColorMode> {
        match self {
            ColorModeActions::Toggle => state.toggle().into(),
        }
    }
}
