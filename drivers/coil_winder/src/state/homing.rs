use crate::display::{DisplayManager, HardwareDisplay};
use crate::inputs::{InputState, ACT_RESET};
use crate::state::menu::MenuId;
use crate::state::AppState;
use crate::stepper::{Direction, Speed, StepperMotor};

#[derive(Clone, Copy, PartialEq)]
pub enum HomingTarget {
    WindCoil,
    MaintenanceMenu,
}

pub fn update<D: HardwareDisplay>(
    phase: u8,
    home_pos: i32,
    start_pos: i32,
    target: HomingTarget,
    ui: &mut DisplayManager,
    display: &mut D,
    inputs: &InputState,
    limit_switch_pressed: bool,
    spindle: &mut dyn StepperMotor,
    traverse: &mut dyn StepperMotor,
) -> AppState {
    let _ = spindle;

    if inputs.just_pressed_act(ACT_RESET) {
        traverse.release();
        return AppState::Menu {
            id: MenuId::Main,
            selection: 0,
        };
    }

    match phase {
        1 => {
            let _ = ui.draw(display, &["Homing Carriage", "<- Fast"]);
            if limit_switch_pressed {
                traverse.set_moving(false);
                return AppState::CarriageHoming {
                    phase: 2,
                    home_pos,
                    start_pos,
                    target,
                };
            } else {
                traverse.set_speed(Speed::Fast);
                traverse.set_direction(Direction::Backward);
                traverse.set_moving(true);
            }
        }
        2 => {
            let _ = ui.draw(display, &["Homing Carriage", "Backing off..."]);
            if !limit_switch_pressed {
                traverse.set_moving(false);
                return AppState::CarriageHoming {
                    phase: 3,
                    home_pos,
                    start_pos,
                    target,
                };
            } else {
                traverse.set_speed(Speed::Moderate);
                traverse.set_direction(Direction::Forward);
                traverse.set_moving(true);
            }
        }
        3 => {
            let _ = ui.draw(display, &["Homing Carriage", "<- Slow"]);
            if limit_switch_pressed {
                traverse.set_moving(false);
                traverse.set_position(0);
                return AppState::CarriageHoming {
                    phase: 4,
                    home_pos: 0,
                    start_pos: 0,
                    target,
                };
            } else {
                traverse.set_speed(Speed::Slow);
                traverse.set_direction(Direction::Backward);
                traverse.set_moving(true);
            }
        }
        4 => {
            let _ = ui.draw(display, &["Homing Carriage", "Done"]);
            traverse.set_moving(false);

            return match target {
                HomingTarget::WindCoil => AppState::WindingSetup {
                    step: 0,
                    wire_diameter_um: 1000,
                    total_turns: 100,
                    start_pos: 0,
                    end_pos: 0,
                    held_ticks: 0,
                },
                HomingTarget::MaintenanceMenu => AppState::Menu {
                    id: MenuId::Maintenance,
                    selection: 0,
                },
            };
        }
        _ => {}
    }

    AppState::CarriageHoming {
        phase,
        home_pos,
        start_pos,
        target,
    }
}
