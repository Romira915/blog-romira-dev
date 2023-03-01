use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use yew::Reducible;

#[derive(EnumString, Clone, Copy, Debug, Display, PartialEq, Eq, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
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
    SetLight,
    SetDark,
}

impl Reducible for ColorMode {
    type Action = ColorModeActions;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        log::debug!("on reduce ColorMode");
        match action {
            ColorModeActions::Toggle => self.toggle(),
            ColorModeActions::SetLight => ColorMode::Light,
            ColorModeActions::SetDark => ColorMode::Dark,
        }
        .into()
    }
}
