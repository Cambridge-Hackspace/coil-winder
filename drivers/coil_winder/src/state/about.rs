use crate::display::{DisplayManager, HardwareDisplay};
use crate::inputs::{InputState, ACT_RESET};
use crate::state::menu::MenuId;
use crate::state::AppState;

pub fn update<D: HardwareDisplay>(
    ui: &mut DisplayManager,
    display: &mut D,
    inputs: &InputState,
) -> AppState {
    let _ = ui.draw(
        display,
        &["About", "Engineered by Cal for the Cambridge Hackspace"],
    );

    if inputs.just_pressed_act(ACT_RESET) {
        AppState::Menu {
            id: MenuId::Main,
            selection: 2,
        }
    } else {
        AppState::About
    }
}
