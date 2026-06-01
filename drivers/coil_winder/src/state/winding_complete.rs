use crate::display::{DisplayManager, HardwareDisplay};
use crate::inputs::{InputState, ACT_CENTER, ACT_RESET, ACT_SET};
use crate::state::AppState;

pub fn update<D: HardwareDisplay>(
    ui: &mut DisplayManager,
    display: &mut D,
    inputs: &InputState,
) -> AppState {
    let _ = ui.draw(display, &["Winding Complete", "Press RST to end"]);

    if inputs.just_pressed_act(ACT_RESET)
        || inputs.just_pressed_act(ACT_CENTER)
        || inputs.just_pressed_act(ACT_SET)
    {
        AppState::Menu {
            id: crate::state::menu::MenuId::Main,
            selection: 0,
        }
    } else {
        AppState::WindingComplete
    }
}
