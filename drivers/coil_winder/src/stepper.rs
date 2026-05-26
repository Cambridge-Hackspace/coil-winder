use embedded_hal::digital::OutputPin;

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Forward,
    Backward,
}

pub trait StepperMotor {
    fn set_speed(&mut self, speed_percent: u8);
    fn set_direction(&mut self, dir: Direction);
    fn set_moving(&mut self, moving: bool);
    fn release(&mut self);
    fn tick(&mut self);
    fn speed(&self) -> u8;
    fn direction(&self) -> Direction;
    fn is_moving(&self) -> bool;
}

pub struct Stepper<P1, P2, P3, P4> {
    p1: P1,
    p2: P2,
    p3: P3,
    p4: P4,
    step_index: u8,
    speed: u8,
    direction: Direction,
    moving: bool,
    tick_count: u16,
    ticks_per_step: u16,
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
            speed: 100,
            direction: Direction::Forward,
            moving: false,
            tick_count: 0,
            ticks_per_step: 1,
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
    fn set_speed(&mut self, speed_percent: u8) {
        self.speed = speed_percent;
        self.ticks_per_step = match speed_percent {
            100 => 1, // ~1ms/step
            75 => 2,  // ~2ms/step
            50 => 4,  // ~4ms/step
            25 => 8,  // ~8ms/step
            _ => 1,
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
                }
                Direction::Backward => {
                    self.step_index = if self.step_index == 0 {
                        3
                    } else {
                        self.step_index - 1
                    };
                }
            }

            self.apply_step();
        }
    }

    fn speed(&self) -> u8 {
        self.speed
    }

    fn direction(&self) -> Direction {
        self.direction
    }

    fn is_moving(&self) -> bool {
        self.moving
    }
}
