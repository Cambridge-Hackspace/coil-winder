use crate::display::{DisplayManager, HardwareDisplay};
use crate::state::AppState;

pub fn update<D: HardwareDisplay>(
    ticks: u16,
    ui: &mut DisplayManager,
    display: &mut D,
) -> AppState {
    let _ = ui.draw(display, &["Cambridge Hackspace", "Loading..."]);

    if ticks == 0 {
        AppState::Home
    } else {
        AppState::Preflight { ticks: ticks - 1 }
    }
}
