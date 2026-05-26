use crate::display::{DisplayManager, HardwareDisplay};
use crate::inputs::{InputState, ACT_CENTER, ACT_SET};
use crate::state::menu::MenuId;
use crate::state::AppState;

pub fn update<D: HardwareDisplay>(
    ui: &mut DisplayManager,
    display: &mut D,
    inputs: &InputState,
) -> AppState {
    let _ = ui.draw(display, &["Cambridge Hackspace", "Welcome!"]);

    if inputs.just_pressed_act(ACT_CENTER) || inputs.just_pressed_act(ACT_SET) {
        AppState::Menu {
            id: MenuId::Main,
            selection: 0,
        }
    } else {
        AppState::Home
    }
}
