pub mod about;
pub mod home;
pub mod menu;
pub mod not_implemented;
pub mod preflight;
pub mod switch_test;
pub mod voltage_test;

use crate::display::{DisplayManager, HardwareDisplay};
use crate::inputs::InputState;

#[derive(Clone, Copy)]
pub enum ReturnTarget {
    Home,
    Menu(menu::MenuId, u8),
}

impl ReturnTarget {
    pub fn to_state(&self) -> AppState {
        match self {
            ReturnTarget::Home => AppState::Home,
            ReturnTarget::Menu(id, sel) => AppState::Menu {
                id: *id,
                selection: *sel,
            },
        }
    }
}

#[derive(Clone, Copy)]
pub enum AppState {
    Preflight { ticks: u16 },
    Home,
    Menu { id: menu::MenuId, selection: u8 },
    NotImplemented { ticks: u16, prev: ReturnTarget },
    About,
    SwitchTest,
    VoltageTest,
}

impl AppState {
    pub fn initial() -> Self {
        AppState::Preflight { ticks: 100 }
    }

    pub fn update<D: HardwareDisplay>(
        &self,
        ui: &mut DisplayManager,
        display: &mut D,
        inputs: &InputState,
        switch_csv: &str,
        voltage_mv: u16,
    ) -> AppState {
        match self {
            AppState::Preflight { ticks } => preflight::update(*ticks, ui, display),
            AppState::Home => home::update(ui, display, inputs),
            AppState::Menu { id, selection } => menu::update(*id, *selection, ui, display, inputs),
            AppState::NotImplemented { ticks, prev } => {
                not_implemented::update(*ticks, *prev, ui, display, inputs)
            }
            AppState::About => about::update(ui, display, inputs),
            AppState::SwitchTest => switch_test::update(ui, display, inputs, switch_csv),
            AppState::VoltageTest => voltage_test::update(ui, display, inputs, voltage_mv),
        }
    }
}
