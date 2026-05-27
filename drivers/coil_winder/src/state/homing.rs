use crate::display::{DisplayManager, HardwareDisplay};
use crate::inputs::{InputState, ACT_RESET};
use crate::state::menu::MenuId;
use crate::state::{AppState, ReturnTarget};
use crate::stepper::{Direction, Speed, StepperMotor};

#[derive(Clone, Copy, PartialEq)]
pub enum HomingTarget {
    MaintenanceMenu,
    WindCoil,
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
    let phase_str = match phase {
        1 => "Phase 1: Seeking",
        2 => "Phase 2: BackOff",
        3 => "Phase 3: Homing ",
        4 => "Phase 4: Return ",
        _ => "                ",
    };
    let _ = ui.draw(
        display,
        &["Homing Carriage... Press RST to cancel...", phase_str],
    );

    if inputs.just_pressed_act(ACT_RESET) {
        spindle.release();
        traverse.release();
        return AppState::HomingError { msg: "Canceled!" };
    }

    let mut next_phase = phase;
    let mut next_home = home_pos;
    let mut next_start = start_pos;

    match phase {
        1 => {
            if limit_switch_pressed {
                traverse.set_moving(false);
                next_phase = 2;
                next_start = traverse.position();
            } else {
                traverse.set_speed(Speed::Fast);
                traverse.set_direction(Direction::Backward);
                traverse.set_moving(true);
            }
        }
        2 => {
            if !limit_switch_pressed {
                traverse.set_moving(false);
                next_phase = 3;
                next_home = traverse.position();
            } else {
                let traveled = (traverse.position() - start_pos).abs();
                if traveled >= 2048 {
                    spindle.release();
                    traverse.release();
                    return AppState::HomingError {
                        msg: "Limit switch malfunction!",
                    };
                }
                traverse.set_speed(Speed::Slow);
                traverse.set_direction(Direction::Forward);
                traverse.set_moving(true);
            }
        }
        3 => {
            if limit_switch_pressed {
                traverse.set_moving(false);
                let current = traverse.position();
                let diff = next_home - current;
                next_home = next_home + (diff / 2);
                next_phase = 4;
            } else {
                traverse.set_speed(Speed::Slow);
                traverse.set_direction(Direction::Backward);
                traverse.set_moving(true);
            }
        }
        4 => {
            let current = traverse.position();
            if current == next_home {
                spindle.release();
                traverse.release();
                return match target {
                    HomingTarget::MaintenanceMenu => AppState::Menu {
                        id: MenuId::Maintenance,
                        selection: 0,
                    },
                    HomingTarget::WindCoil => AppState::NotImplemented {
                        ticks: 1200,
                        prev: ReturnTarget::Menu(MenuId::Main, 0),
                    },
                };
            } else {
                traverse.set_speed(Speed::Moderate);
                if current < next_home {
                    traverse.set_direction(Direction::Forward);
                } else {
                    traverse.set_direction(Direction::Backward);
                }
                traverse.set_moving(true);
            }
        }
        _ => {}
    }

    AppState::CarriageHoming {
        phase: next_phase,
        home_pos: next_home,
        start_pos: next_start,
        target,
    }
}
