use crate::display::{DisplayManager, HardwareDisplay};
use crate::inputs::{InputState, ACT_CENTER, ACT_RESET, DIR_DOWN, DIR_LEFT, DIR_RIGHT, DIR_UP};
use crate::state::menu::MenuId;
use crate::state::AppState;
use crate::stepper::{Direction, StepperMotor};
use crate::string_buffer::StringBuffer;

pub fn update<D: HardwareDisplay>(
    ui: &mut DisplayManager,
    display: &mut D,
    inputs: &InputState,
    spindle: &mut dyn StepperMotor,
    traverse: &mut dyn StepperMotor,
) -> AppState {
    // cycle speed on center action button press
    if inputs.just_pressed_act(ACT_CENTER) {
        let next_speed = match spindle.speed() {
            25 => 50,
            50 => 75,
            75 => 100,
            100 => 25,
            _ => 100,
        };
        spindle.set_speed(next_speed);
        traverse.set_speed(next_speed);
    }

    let mut s_moving = false;
    let mut t_moving = false;

    // check if button is currently held down w/ `dir_curr` bitmask
    if (inputs.dir_curr & DIR_UP) != 0 {
        spindle.set_direction(Direction::Forward);
        s_moving = true;
    } else if (inputs.dir_curr & DIR_DOWN) != 0 {
        spindle.set_direction(Direction::Backward);
        s_moving = true;
    }

    if (inputs.dir_curr & DIR_RIGHT) != 0 {
        traverse.set_direction(Direction::Forward);
        t_moving = true;
    } else if (inputs.dir_curr & DIR_LEFT) != 0 {
        traverse.set_direction(Direction::Backward);
        t_moving = true;
    }

    // apply motor state
    spindle.set_moving(s_moving);
    traverse.set_moving(t_moving);

    // ui stuff
    let s_dir = if !s_moving {
        "STP"
    } else if spindle.direction() == Direction::Forward {
        "FWD"
    } else {
        "REV"
    };
    let t_dir = if !t_moving {
        "STP"
    } else if traverse.direction() == Direction::Forward {
        "FWD"
    } else {
        "REV"
    };

    let mut buf1 = StringBuffer::<16>::new();
    let mut buf2 = StringBuffer::<16>::new();

    let _ = ufmt::uwrite!(&mut buf1, "Spd:{} {}%", s_dir, spindle.speed());
    let _ = ufmt::uwrite!(&mut buf2, "Trv:{} {}%", t_dir, traverse.speed());

    let _ = ui.draw(display, &[buf1.as_str(), buf2.as_str()]);

    if inputs.just_pressed_act(ACT_RESET) {
        // kill torque
        spindle.release();
        traverse.release();
        AppState::Menu {
            id: MenuId::Maintenance,
            selection: 0,
        }
    } else {
        AppState::MotorTest
    }
}
