use crate::display::{DisplayManager, HardwareDisplay};
use crate::state::AppState;

pub fn update<D: HardwareDisplay>(
    ticks: u16,
    high: bool,
    ui: &mut DisplayManager,
    display: &mut D,
) -> AppState {
    let show_text = (ticks % 400) >= 200;

    if !show_text {
        let _ = ui.draw(display, &["", ""]);
    } else if high {
        let _ = ui.draw(display, &[" !!! DANGER !!!", "  HIGH VOLTAGE"]);
    } else {
        let _ = ui.draw(display, &[" !!! DANGER !!!", "  LOW VOLTAGE"]);
    }

    if ticks == 0 {
        AppState::VoltageTest
    } else {
        AppState::VoltageDanger {
            ticks: ticks - 1,
            high,
        }
    }
}
