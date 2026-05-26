use crate::display::{DisplayManager, HardwareDisplay};
use crate::inputs::{InputState, ACT_RESET};
use crate::state::menu::MenuId;
use crate::state::AppState;
use crate::string_buffer::StringBuffer;

pub fn update<D: HardwareDisplay>(
    ui: &mut DisplayManager,
    display: &mut D,
    inputs: &InputState,
    voltage_mv: u16,
) -> AppState {
    let mut buf = StringBuffer::<16>::new();
    let volts = voltage_mv / 1000;
    let frac = (voltage_mv % 1000) / 10;

    // fmt str manually, as `ufmt` lacks padding/precision
    if frac < 10 {
        let _ = ufmt::uwrite!(&mut buf, "{}.0{} V", volts, frac);
    } else {
        let _ = ufmt::uwrite!(&mut buf, "{}.{} V", volts, frac);
    }

    let _ = ui.draw(display, &["Buck Converter Output Voltage", buf.as_str()]);

    if inputs.just_pressed_act(ACT_RESET) {
        AppState::Menu {
            id: MenuId::Maintenance,
            selection: 2,
        }
    } else {
        AppState::VoltageTest
    }
}
