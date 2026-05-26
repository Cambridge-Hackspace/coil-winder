use crate::display::{DisplayManager, HardwareDisplay};
use crate::inputs::{InputState, ACT_RESET};
use crate::state::menu::MenuId;
use crate::state::AppState;

pub fn update<D: HardwareDisplay>(
    ui: &mut DisplayManager,
    display: &mut D,
    inputs: &InputState,
    switch_csv: &str,
) -> AppState {
    let _ = ui.draw(display, &["Active Switches:", switch_csv]);

    if inputs.just_pressed_act(ACT_RESET) {
        AppState::Menu {
            id: MenuId::Maintenance,
            selection: 1,
        }
    } else {
        AppState::SwitchTest
    }
}
