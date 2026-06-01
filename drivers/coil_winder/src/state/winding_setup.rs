use crate::display::{DisplayManager, HardwareDisplay};
use crate::inputs::{
    InputState, ACT_CENTER, ACT_RESET, ACT_SET, DIR_DOWN, DIR_LEFT, DIR_RIGHT, DIR_UP,
};
use crate::state::menu::MenuId;
use crate::state::AppState;
use crate::stepper::{Direction, Speed, StepperMotor};
use crate::string_buffer::StringBuffer;

pub fn update<D: HardwareDisplay>(
    step: u8,
    mut wire_diameter_um: u16,
    mut total_turns: u32,
    mut start_pos: i32,
    mut end_pos: i32,
    mut held_ticks: u16,
    ui: &mut DisplayManager,
    display: &mut D,
    inputs: &InputState,
    traverse: &mut dyn StepperMotor,
) -> AppState {
    fn process_numerical_config(
        val: u32,
        held_ticks: &mut u16,
        inputs: &InputState,
        step_small: u32,
        step_large: u32,
        min_val: u32,
    ) -> u32 {
        let mask_up = DIR_UP | DIR_RIGHT;
        let mask_down = DIR_DOWN | DIR_LEFT;

        let intent_up = (inputs.dir_curr & mask_up) != 0 && inputs.only_expected_dir(mask_up);
        let intent_down = (inputs.dir_curr & mask_down) != 0 && inputs.only_expected_dir(mask_down);

        let intent_changed = if intent_up {
            (inputs.dir_prev & mask_up) == 0 || !inputs.only_expected_prev_dir(mask_up)
        } else if intent_down {
            (inputs.dir_prev & mask_down) == 0 || !inputs.only_expected_prev_dir(mask_down)
        } else {
            false
        };

        if intent_up || intent_down {
            if intent_changed {
                *held_ticks = 0;
            }

            // tweak intensity of zoomies
            let is_tick = *held_ticks == 0 || (*held_ticks > 0 && *held_ticks % 100 == 0);

            // tweak grace period before zoomies
            let is_fast = *held_ticks >= 1000;

            let mut new_val = val;
            if is_tick {
                let amount = if is_fast { step_large } else { step_small };
                if intent_up {
                    new_val = val.saturating_add(amount);
                } else {
                    new_val = val.saturating_sub(amount).max(min_val);
                }
            }

            *held_ticks = held_ticks.saturating_add(1);
            new_val
        } else {
            *held_ticks = 0;
            val
        }
    }

    fn process_motor_config(traverse: &mut dyn StepperMotor, inputs: &InputState) {
        if inputs.just_pressed_dir(DIR_UP) {
            let next_speed = match traverse.speed() {
                Speed::Slow => Speed::Moderate,
                Speed::Moderate => Speed::Fast,
                Speed::Fast => Speed::Fast,
            };
            traverse.set_speed(next_speed);
        }
        if inputs.just_pressed_dir(DIR_DOWN) {
            let next_speed = match traverse.speed() {
                Speed::Fast => Speed::Moderate,
                Speed::Moderate => Speed::Slow,
                Speed::Slow => Speed::Slow,
            };
            traverse.set_speed(next_speed);
        }

        let mut t_moving = false;
        if (inputs.dir_curr & DIR_RIGHT) != 0 {
            traverse.set_direction(Direction::Forward);
            t_moving = true;
        } else if (inputs.dir_curr & DIR_LEFT) != 0 {
            traverse.set_direction(Direction::Backward);
            t_moving = true;
        }
        traverse.set_moving(t_moving);
    }

    match step {
        0 => {
            wire_diameter_um = process_numerical_config(
                wire_diameter_um as u32,
                &mut held_ticks,
                inputs,
                10,
                100,
                10,
            ) as u16;

            let mut buf = StringBuffer::<16>::new();
            let diam_mm = wire_diameter_um / 1000;
            let diam_frac = (wire_diameter_um % 1000) / 10;
            if diam_frac < 10 {
                let _ = ufmt::uwrite!(&mut buf, "{}.0{} mm", diam_mm, diam_frac);
            } else {
                let _ = ufmt::uwrite!(&mut buf, "{}.{} mm", diam_mm, diam_frac);
            }
            let _ = ui.draw(display, &["Wire Diameter", buf.as_str()]);

            if inputs.just_pressed_act(ACT_RESET) {
                return AppState::Menu {
                    id: MenuId::Main,
                    selection: 0,
                };
            } else if inputs.just_pressed_act(ACT_CENTER) || inputs.just_pressed_act(ACT_SET) {
                return AppState::WindingSetup {
                    step: 1,
                    wire_diameter_um,
                    total_turns,
                    start_pos,
                    end_pos,
                    held_ticks: 0,
                };
            }
        }
        1 => {
            total_turns =
                process_numerical_config(total_turns, &mut held_ticks, inputs, 10, 100, 10);

            let mut buf = StringBuffer::<16>::new();
            let _ = ufmt::uwrite!(&mut buf, "{}", total_turns);
            let _ = ui.draw(display, &["Total Turns", buf.as_str()]);

            if inputs.just_pressed_act(ACT_RESET) {
                return AppState::WindingSetup {
                    step: 0,
                    wire_diameter_um,
                    total_turns,
                    start_pos,
                    end_pos,
                    held_ticks: 0,
                };
            } else if inputs.just_pressed_act(ACT_CENTER) || inputs.just_pressed_act(ACT_SET) {
                start_pos = traverse.position();
                return AppState::WindingSetup {
                    step: 2,
                    wire_diameter_um,
                    total_turns,
                    start_pos,
                    end_pos,
                    held_ticks: 0,
                };
            }
        }
        2 => {
            process_motor_config(traverse, inputs);
            start_pos = traverse.position();

            let mut buf = StringBuffer::<16>::new();
            let _ = ufmt::uwrite!(&mut buf, "<- Pos: {} ->", start_pos);
            let _ = ui.draw(display, &["Set Start Pos", buf.as_str()]);

            if inputs.just_pressed_act(ACT_RESET) {
                traverse.release();
                return AppState::WindingSetup {
                    step: 1,
                    wire_diameter_um,
                    total_turns,
                    start_pos,
                    end_pos,
                    held_ticks: 0,
                };
            } else if inputs.just_pressed_act(ACT_SET) || inputs.just_pressed_act(ACT_CENTER) {
                traverse.release();
                end_pos = start_pos;
                return AppState::WindingSetup {
                    step: 3,
                    wire_diameter_um,
                    total_turns,
                    start_pos,
                    end_pos,
                    held_ticks: 0,
                };
            }
        }
        3 => {
            process_motor_config(traverse, inputs);
            end_pos = traverse.position();

            let mut buf = StringBuffer::<16>::new();
            let _ = ufmt::uwrite!(&mut buf, "<- Pos: {} ->", end_pos);
            let _ = ui.draw(display, &["Set End Pos", buf.as_str()]);

            if inputs.just_pressed_act(ACT_RESET) {
                traverse.release();
                return AppState::WindingSetup {
                    step: 2,
                    wire_diameter_um,
                    total_turns,
                    start_pos,
                    end_pos,
                    held_ticks: 0,
                };
            } else if inputs.just_pressed_act(ACT_SET) || inputs.just_pressed_act(ACT_CENTER) {
                traverse.release();
                return AppState::WindingSetup {
                    step: 4,
                    wire_diameter_um,
                    total_turns,
                    start_pos,
                    end_pos,
                    held_ticks: 0,
                };
            }
        }
        4 => {
            let mut buf = StringBuffer::<16>::new();
            let diam_mm = wire_diameter_um / 1000;
            let diam_frac = (wire_diameter_um % 1000) / 10;
            if diam_frac < 10 {
                let _ = ufmt::uwrite!(&mut buf, "{}T, {}.0{}mm", total_turns, diam_mm, diam_frac);
            } else {
                let _ = ufmt::uwrite!(&mut buf, "{}T, {}.{}mm", total_turns, diam_mm, diam_frac);
            }
            let _ = ui.draw(display, &["Ready to Wind?", buf.as_str()]);

            if inputs.just_pressed_act(ACT_RESET) {
                return AppState::WindingSetup {
                    step: 3,
                    wire_diameter_um,
                    total_turns,
                    start_pos,
                    end_pos,
                    held_ticks: 0,
                };
            } else if inputs.just_pressed_act(ACT_CENTER) || inputs.just_pressed_act(ACT_SET) {
                return AppState::Winding {
                    wire_diameter_um,
                    total_turns,
                    start_pos,
                    end_pos,
                };
            }
        }
        _ => {}
    }

    AppState::WindingSetup {
        step,
        wire_diameter_um,
        total_turns,
        start_pos,
        end_pos,
        held_ticks,
    }
}
