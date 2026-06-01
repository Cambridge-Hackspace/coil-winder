use crate::stepper::{Direction, StepperMotor};
use avr_device::interrupt::Mutex;
use core::cell::RefCell;

pub struct WinderState {
    pub active: bool,
    pub turns_done: u32,
    pub total_turns: u32,
    pub ratio: u32,
    pub accumulator: u32,
    pub spindle_steps: u32,
    pub carriage_pos: i32,
    pub traverse_start: i32,
    pub traverse_end: i32,
    pub direction: Direction,
    pub ghost_steps_remaining: u32,
    pub backlash_steps: u32,
}

impl WinderState {
    pub const fn new() -> Self {
        Self {
            active: false,
            turns_done: 0,
            total_turns: 0,
            ratio: 0,
            accumulator: 0,
            spindle_steps: 0,
            carriage_pos: 0,
            traverse_start: 0,
            traverse_end: 0,
            direction: Direction::Forward,
            ghost_steps_remaining: 0,
            backlash_steps: 50,
        }
    }

    pub fn tick<S: StepperMotor, T: StepperMotor>(&mut self, spindle: &mut S, traverse: &mut T) {
        if !self.active {
            spindle.tick();
            traverse.tick();
            return;
        }

        if !spindle.tick() {
            return;
        }

        self.spindle_steps += 1;
        if self.spindle_steps >= 2048 {
            self.spindle_steps -= 2048;
            self.turns_done += 1;
            if self.turns_done >= self.total_turns {
                self.active = false;
                spindle.set_moving(false);
                traverse.set_moving(false);
                return;
            }
        }

        self.accumulator += self.ratio;
        if self.accumulator >= 2048 {
            self.accumulator -= 2048;
            traverse.step_once(self.direction);

            if self.ghost_steps_remaining > 0 {
                self.ghost_steps_remaining -= 1;
            } else {
                match self.direction {
                    Direction::Forward => {
                        self.carriage_pos += 1;
                        if self.carriage_pos >= self.traverse_end {
                            self.direction = Direction::Backward;
                            self.ghost_steps_remaining = self.backlash_steps;
                        }
                    }
                    Direction::Backward => {
                        self.carriage_pos -= 1;
                        if self.carriage_pos <= self.traverse_start {
                            self.direction = Direction::Forward;
                            self.ghost_steps_remaining = self.backlash_steps;
                        }
                    }
                }
            }
        }
    }
}

pub struct WinderProxy<'a> {
    mutex: &'a Mutex<RefCell<WinderState>>,
}

impl<'a> WinderProxy<'a> {
    pub fn new(mutex: &'a Mutex<RefCell<WinderState>>) -> Self {
        Self { mutex }
    }

    pub fn start(&self, total_turns: u32, ratio: u32, start: i32, end: i32, backlash: u32) {
        avr_device::interrupt::free(|cs| {
            let mut w = self.mutex.borrow(cs).borrow_mut();
            w.active = true;
            w.turns_done = 0;
            w.total_turns = total_turns;
            w.ratio = ratio;
            w.accumulator = 0;
            w.spindle_steps = 0;
            w.carriage_pos = start;
            w.traverse_start = start.min(end);
            w.traverse_end = start.max(end);
            w.direction = if start <= end {
                Direction::Forward
            } else {
                Direction::Backward
            };
            w.ghost_steps_remaining = 0;
            w.backlash_steps = backlash;
        });
    }

    pub fn get_progress(&self) -> (u32, u32) {
        avr_device::interrupt::free(|cs| {
            let w = self.mutex.borrow(cs).borrow();
            (w.turns_done, w.total_turns)
        })
    }

    pub fn is_active(&self) -> bool {
        avr_device::interrupt::free(|cs| self.mutex.borrow(cs).borrow().active)
    }

    pub fn stop(&self) {
        avr_device::interrupt::free(|cs| {
            self.mutex.borrow(cs).borrow_mut().active = false;
        });
    }
}

/// Computes the ratio logic based on the assumption that an M8 rod needs
/// 1638.4 traverse steps per millimeter (2048 steps / 1.25 mm thread pitch).
pub fn calculate_ratio(wire_diameter_um: u16) -> u32 {
    (wire_diameter_um as u32 * 16384) / 10000
}
