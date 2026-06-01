use crate::display::{DisplayManager, HardwareDisplay};
use crate::inputs::{InputState, ACT_RESET};
use crate::state::AppState;
use crate::stepper::StepperMotor;
use crate::string_buffer::StringBuffer;
use crate::winder::WinderProxy;

pub fn update<D: HardwareDisplay>(
    wire_diameter_um: u16,
    total_turns: u32,
    start_pos: i32,
    end_pos: i32,
    ui: &mut DisplayManager,
    display: &mut D,
    inputs: &InputState,
    winder: &WinderProxy,
    spindle: &mut dyn StepperMotor,
    traverse: &mut dyn StepperMotor,
) -> AppState {
    let (turns_done, total) = winder.get_progress();
    let is_active = winder.is_active();

    let mut buf = StringBuffer::<16>::new();
    let _ = ufmt::uwrite!(&mut buf, "Turn {} / {}", turns_done, total);

    if is_active {
        let _ = ui.draw(display, &["Winding...", buf.as_str()]);
    } else {
        let _ = ui.draw(display, &["Winding Complete", buf.as_str()]);
    }

    if inputs.just_pressed_act(ACT_RESET) {
        winder.stop();
        spindle.set_moving(false);
        spindle.release();
        traverse.set_moving(false);
        traverse.release();
        return AppState::Menu {
            id: crate::state::menu::MenuId::Main,
            selection: 0,
        };
    }

    if !is_active && turns_done >= total && total > 0 {
        spindle.release();
        traverse.release();
        return AppState::WindingComplete;
    }

    AppState::Winding {
        wire_diameter_um,
        total_turns,
        start_pos,
        end_pos,
    }
}
