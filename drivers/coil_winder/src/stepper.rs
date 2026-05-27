use avr_device::interrupt::Mutex;
use core::cell::RefCell;
use embedded_hal::digital::OutputPin;

#[derive(Clone, Copy, PartialEq)]
pub enum Speed {
    Fast,
    Moderate,
    Slow,
}

impl Speed {
    pub fn as_str(&self) -> &'static str {
        match self {
            Speed::Fast => "FST",
            Speed::Moderate => "MOD",
            Speed::Slow => "SLW",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Forward,
    Backward,
}

pub trait StepperMotor {
    fn set_speed(&mut self, speed: Speed);
    fn set_direction(&mut self, dir: Direction);
    fn set_moving(&mut self, moving: bool);
    fn release(&mut self);
    fn tick(&mut self);
    fn speed(&self) -> Speed;
    fn direction(&self) -> Direction;
    fn is_moving(&self) -> bool;
    fn position(&self) -> i32;
    fn set_position(&mut self, pos: i32);
}

pub struct Stepper<P1, P2, P3, P4> {
    p1: P1,
    p2: P2,
    p3: P3,
    p4: P4,
    step_index: u8,
    speed: Speed,
    direction: Direction,
    moving: bool,
    tick_count: u16,
    ticks_per_step: u16,
    position: i32,
}

impl<P1, P2, P3, P4> Stepper<P1, P2, P3, P4>
where
    P1: OutputPin,
    P2: OutputPin,
    P3: OutputPin,
    P4: OutputPin,
{
    pub fn new(p1: P1, p2: P2, p3: P3, p4: P4) -> Self {
        Self {
            p1,
            p2,
            p3,
            p4,
            step_index: 0,
            speed: Speed::Fast,
            direction: Direction::Forward,
            moving: false,
            tick_count: 0,
            ticks_per_step: 3,
            position: 0,
        }
    }

    /// Dual-phase full-stepping procedure.
    fn apply_step(&mut self) {
        let seq = [
            (true, true, false, false),
            (false, true, true, false),
            (false, false, true, true),
            (true, false, false, true),
        ];

        let (s1, s2, s3, s4) = seq[self.step_index as usize];

        if s1 {
            let _ = self.p1.set_high();
        } else {
            let _ = self.p1.set_low();
        }
        if s2 {
            let _ = self.p2.set_high();
        } else {
            let _ = self.p2.set_low();
        }
        if s3 {
            let _ = self.p3.set_high();
        } else {
            let _ = self.p3.set_low();
        }
        if s4 {
            let _ = self.p4.set_high();
        } else {
            let _ = self.p4.set_low();
        }
    }
}

impl<P1, P2, P3, P4> StepperMotor for Stepper<P1, P2, P3, P4>
where
    P1: OutputPin,
    P2: OutputPin,
    P3: OutputPin,
    P4: OutputPin,
{
    fn set_speed(&mut self, speed: Speed) {
        self.speed = speed;
        self.ticks_per_step = match speed {
            Speed::Fast => 3,     // 3 * 0.5ms = 1.5ms
            Speed::Moderate => 5, // 5 * 0.5ms = 2.5ms
            Speed::Slow => 8,     // 8 * 0.5ms = 4.0ms
        };
    }

    fn set_direction(&mut self, dir: Direction) {
        self.direction = dir;
    }

    fn set_moving(&mut self, moving: bool) {
        if moving && !self.moving {
            // on start assert current step
            self.apply_step();
            self.tick_count = 0;
        }
        self.moving = moving
    }

    fn release(&mut self) {
        self.moving = false;
        // kill torque
        let _ = self.p1.set_low();
        let _ = self.p2.set_low();
        let _ = self.p3.set_low();
        let _ = self.p4.set_low();
    }

    fn tick(&mut self) {
        if !self.moving {
            return;
        }

        self.tick_count += 1;
        if self.tick_count >= self.ticks_per_step {
            self.tick_count = 0;

            match self.direction {
                Direction::Forward => {
                    self.step_index = (self.step_index + 1) % 4;
                    self.position = self.position.wrapping_add(1);
                }
                Direction::Backward => {
                    self.step_index = if self.step_index == 0 {
                        3
                    } else {
                        self.step_index - 1
                    };
                    self.position = self.position.wrapping_sub(1);
                }
            }

            self.apply_step();
        }
    }

    fn speed(&self) -> Speed {
        self.speed
    }

    fn direction(&self) -> Direction {
        self.direction
    }

    fn is_moving(&self) -> bool {
        self.moving
    }

    fn position(&self) -> i32 {
        self.position
    }

    fn set_position(&mut self, pos: i32) {
        self.position = pos;
    }
}

/// A proxy to safely control a StepperMotor that lives inside a hardware interrupt Mutex.
pub struct StepperProxy<'a, S> {
    mutex: &'a Mutex<RefCell<Option<S>>>,
}

impl<'a, S> StepperProxy<'a, S> {
    pub fn new(mutex: &'a Mutex<RefCell<Option<S>>>) -> Self {
        Self { mutex }
    }
}

impl<'a, S: StepperMotor> StepperMotor for StepperProxy<'a, S> {
    fn set_speed(&mut self, speed: Speed) {
        avr_device::interrupt::free(|cs| {
            if let Some(s) = self.mutex.borrow(cs).borrow_mut().as_mut() {
                s.set_speed(speed);
            }
        });
    }

    fn set_direction(&mut self, dir: Direction) {
        avr_device::interrupt::free(|cs| {
            if let Some(s) = self.mutex.borrow(cs).borrow_mut().as_mut() {
                s.set_direction(dir);
            }
        });
    }

    fn set_moving(&mut self, moving: bool) {
        avr_device::interrupt::free(|cs| {
            if let Some(s) = self.mutex.borrow(cs).borrow_mut().as_mut() {
                s.set_moving(moving);
            }
        });
    }

    fn release(&mut self) {
        avr_device::interrupt::free(|cs| {
            if let Some(s) = self.mutex.borrow(cs).borrow_mut().as_mut() {
                s.release();
            }
        });
    }

    fn tick(&mut self) {} // nop; handled by hardware interrupt

    fn speed(&self) -> Speed {
        avr_device::interrupt::free(|cs| {
            self.mutex
                .borrow(cs)
                .borrow()
                .as_ref()
                .map_or(Speed::Fast, |s| s.speed())
        })
    }

    fn direction(&self) -> Direction {
        avr_device::interrupt::free(|cs| {
            self.mutex
                .borrow(cs)
                .borrow()
                .as_ref()
                .map_or(Direction::Forward, |s| s.direction())
        })
    }

    fn is_moving(&self) -> bool {
        avr_device::interrupt::free(|cs| {
            self.mutex
                .borrow(cs)
                .borrow()
                .as_ref()
                .map_or(false, |s| s.is_moving())
        })
    }

    fn position(&self) -> i32 {
        avr_device::interrupt::free(|cs| {
            self.mutex
                .borrow(cs)
                .borrow()
                .as_ref()
                .map_or(0, |s| s.position())
        })
    }

    fn set_position(&mut self, pos: i32) {
        avr_device::interrupt::free(|cs| {
            if let Some(s) = self.mutex.borrow(cs).borrow_mut().as_mut() {
                s.set_position(pos);
            }
        });
    }
}
