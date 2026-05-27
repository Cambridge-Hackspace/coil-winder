use crate::display::{DisplayManager, HardwareDisplay};
use crate::inputs::{InputState, ACT_RESET};
use crate::state::AppState;

pub fn update<D: HardwareDisplay>(
    msg: &'static str,
    ui: &mut DisplayManager,
    display: &mut D,
    inputs: &InputState,
) -> AppState {
    let _ = ui.draw(display, &["Homing Error", msg]);

    if inputs.just_pressed_act(ACT_RESET) {
        AppState::Home
    } else {
        AppState::HomingError { msg }
    }
}
