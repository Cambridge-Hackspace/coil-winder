use crate::display::{DisplayManager, HardwareDisplay};
use crate::inputs::{InputState, ACT_RESET};
use crate::state::{AppState, ReturnTarget};

pub fn update<D: HardwareDisplay>(
    ticks: u16,
    prev: ReturnTarget,
    ui: &mut DisplayManager,
    display: &mut D,
    inputs: &InputState,
) -> AppState {
    let _ = ui.draw(display, &["Notice:", "Not yet implemented."]);

    if ticks == 0 || inputs.just_pressed_act(ACT_RESET) {
        prev.to_state()
    } else {
        AppState::NotImplemented {
            ticks: ticks - 1,
            prev,
        }
    }
}
