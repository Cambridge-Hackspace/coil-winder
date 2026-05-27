use crate::display::{DisplayManager, HardwareDisplay};
use crate::inputs::{InputState, ACT_CENTER, ACT_RESET, ACT_SET, DIR_LEFT, DIR_RIGHT};
use crate::state::AppState;

#[derive(Clone, Copy, PartialEq)]
pub enum MenuId {
    Main,
    Maintenance,
}

impl MenuId {
    pub fn header(&self) -> &'static str {
        match self {
            MenuId::Main => "Main Menu",
            MenuId::Maintenance => "Maintenance",
        }
    }

    pub fn items(&self) -> &'static [&'static str] {
        match self {
            MenuId::Main => &["Wind a Coil", "Maintenance", "About"],
            MenuId::Maintenance => &[
                "Carriage Homing",
                "Motor Test",
                "Switch Test",
                "Voltage Test",
            ],
        }
    }

    pub fn prev(&self) -> AppState {
        match self {
            MenuId::Main => AppState::Home,
            MenuId::Maintenance => AppState::Menu {
                id: MenuId::Main,
                selection: 1,
            },
        }
    }

    pub fn select(&self, selection: u8) -> AppState {
        match self {
            MenuId::Main => match selection {
                0 => AppState::CarriageHoming {
                    phase: 1,
                    home_pos: 0,
                    start_pos: 0,
                    target: crate::state::homing::HomingTarget::WindCoil,
                },
                1 => AppState::Menu {
                    id: MenuId::Maintenance,
                    selection: 0,
                },
                2 => AppState::About,
                _ => AppState::Menu {
                    id: *self,
                    selection,
                },
            },
            MenuId::Maintenance => match selection {
                0 => AppState::CarriageHoming {
                    phase: 1,
                    home_pos: 0,
                    start_pos: 0,
                    target: crate::state::homing::HomingTarget::MaintenanceMenu,
                },
                1 => AppState::MotorTest,
                2 => AppState::SwitchTest,
                3 => AppState::VoltageTest,
                _ => AppState::Menu {
                    id: *self,
                    selection,
                },
            },
        }
    }
}

pub fn update<D: HardwareDisplay>(
    id: MenuId,
    selection: u8,
    ui: &mut DisplayManager,
    display: &mut D,
    inputs: &InputState,
) -> AppState {
    let items = id.items();
    let header = id.header();
    let _ = ui.draw(display, &[header, items[selection as usize]]);

    let mut new_sel = selection;

    if inputs.just_pressed_dir(DIR_LEFT) {
        if new_sel > 0 {
            new_sel -= 1;
        } else {
            new_sel = (items.len() - 1) as u8;
        }
    }

    if inputs.just_pressed_dir(DIR_RIGHT) {
        if new_sel < (items.len() - 1) as u8 {
            new_sel += 1;
        } else {
            new_sel = 0;
        }
    }

    if inputs.just_pressed_act(ACT_RESET) {
        id.prev()
    } else if inputs.just_pressed_act(ACT_SET) || inputs.just_pressed_act(ACT_CENTER) {
        id.select(new_sel)
    } else {
        AppState::Menu {
            id,
            selection: new_sel,
        }
    }
}
