#![no_std]
#![no_main]

mod display;
mod inputs;
mod ladder;
mod lcd_i2c;
mod state;

use arduino_hal::prelude::*;
use display::{DisplayManager, HardwareDisplay};
use inputs::InputState;
use ladder::ResistorLadder;
use lcd_i2c::LcdI2c;
use panic_halt as _;
use state::AppState;

struct StringBuffer<const N: usize> {
    buf: [u8; N],
    len: usize,
}

impl<const N: usize> StringBuffer<N> {
    fn new() -> Self {
        Self {
            buf: [0; N],
            len: 0,
        }
    }

    fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.buf[..self.len]) }
    }
}

impl<const N: usize> ufmt::uWrite for StringBuffer<N> {
    type Error = ();
    fn write_str(&mut self, s: &str) -> Result<(), ()> {
        let bytes = s.as_bytes();
        if self.len + bytes.len() > N {
            return Err(());
        }
        self.buf[self.len..self.len + bytes.len()].copy_from_slice(bytes);
        self.len += bytes.len();
        Ok(())
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut led = pins.d13.into_output();

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let a0 = pins.a0.into_pull_up_input().into_analog_input(&mut adc);
    let a1 = pins.a1.into_pull_up_input().into_analog_input(&mut adc);

    let i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        100000,
    );

    arduino_hal::delay_ms(500);

    ufmt::uwriteln!(&mut serial, "Initializing LCD...").unwrap();
    let mut lcd = LcdI2c::new(i2c, 0x27).unwrap();
    let mut ui = DisplayManager::new(16, 2);

    const LADDER_DIR: ResistorLadder<4> =
        ResistorLadder::<4>::new(1000, &[10000, 4700, 2200, 1000], &[(0, 1), (2, 3)]);
    const LADDER_ACT: ResistorLadder<3> = ResistorLadder::<3>::new(1000, &[1000, 2200, 4700], &[]);

    let mut inputs = InputState {
        dir_curr: 0,
        dir_prev: 0,
        act_curr: 0,
        act_prev: 0,
    };
    let mut app_state = AppState::initial();
    let mut loop_counter: u8 = 0;

    loop {
        led.toggle();

        let _ = a0.analog_read(&mut adc);
        arduino_hal::delay_ms(2);
        let val_a0 = a0.analog_read(&mut adc);

        let _ = a1.analog_read(&mut adc);
        arduino_hal::delay_ms(2);
        let val_a1 = a1.analog_read(&mut adc);

        let state_act = LADDER_ACT.resolve(val_a0);
        let state_dir = LADDER_DIR.resolve(val_a1);

        inputs.act_prev = inputs.act_curr;
        inputs.act_curr = state_act;
        inputs.dir_prev = inputs.dir_curr;
        inputs.dir_curr = state_dir;

        let mut csv_buf = StringBuffer::<64>::new();
        let mut first = true;

        macro_rules! add_switch {
            ($label:expr) => {
                if !first {
                    let _ = ufmt::uwrite!(&mut csv_buf, ", ");
                }
                let _ = ufmt::uwrite!(&mut csv_buf, "{}", $label);
                first = false;
            };
        }

        if (state_dir & (1 << 0)) != 0 {
            add_switch!("LEFT");
        }
        if (state_dir & (1 << 1)) != 0 {
            add_switch!("RIGHT");
        }
        if (state_dir & (1 << 2)) != 0 {
            add_switch!("DOWN");
        }
        if (state_dir & (1 << 3)) != 0 {
            add_switch!("UP");
        }

        if (state_act & (1 << 0)) != 0 {
            add_switch!("CENTER");
        }
        if (state_act & (1 << 1)) != 0 {
            add_switch!("SET");
        }
        if (state_act & (1 << 2)) != 0 {
            add_switch!("RESET");
        }

        let _ = ufmt::uwriteln!(
            &mut serial,
            "A0: {}, A1: {} | ACT: {}, DIR: {} | {}",
            val_a0,
            val_a1,
            state_act,
            state_dir,
            if first { "None" } else { csv_buf.as_str() }
        );

        if first {
            let _ = ufmt::uwrite!(&mut csv_buf, "None");
        }

        app_state = app_state.update(&mut ui, &mut lcd, &inputs, csv_buf.as_str());
        if loop_counter >= 3 {
            ui.tick();
            loop_counter = 0;
        }

        arduino_hal::delay_ms(50);
        loop_counter += 1;
    }
}
