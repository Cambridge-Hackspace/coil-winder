#![allow(dead_code)]

/// A generic resistor ladder decoder for reading multiple buttons on a single analog pin.
/// Supports up to 4 buttons (16 states) to keep memory footprint minimal.
pub struct ResistorLadder<const N: usize> {
    buckets: [(u16, u8); 16], // pairs of (expected_adc_value, button_state_bitmask)
    valid_count: usize,
}

impl<const N: usize> ResistorLadder<N> {
    /// Creates a new ResistorLadder.
    /// `r_pulldown`: Resistance of the common pull-up resistor in Ohms.
    /// `r_buttons`: Resistance of each button's resistor in Ohms.
    /// `exclusions`: Pairs of button indices that cannot be physically pressed simultaneously.
    pub const fn new(r_pulldown: u32, r_buttons: &[u32; N], exclusions: &[(usize, usize)]) -> Self {
        assert!(N <= 4, "ResistorLadder currently supports up to 4 buttons");

        let mut buckets = [(0, 0); 16];
        let mut valid_count = 0;

        let num_combinations = 1 << N;
        let mut state = 0;
        while state < num_combinations {
            let mut excluded = false;
            let mut excl_idx = 0;
            while excl_idx < exclusions.len() {
                let (a, b) = exclusions[excl_idx];
                if (state & (1 << a)) != 0 && (state & (1 << b)) != 0 {
                    excluded = true;
                    break;
                }
                excl_idx += 1;
            }
            if excluded {
                state += 1;
                continue;
            }

            // compute equivalent conductance algebraically transformed to avoid floats
            // s represents sum((R_down * 10000) / R_i)
            let mut s_total: u32 = 0;
            let mut i = 0;
            while i < N {
                if (state & (1 << i)) != 0 {
                    s_total += (r_pulldown * 10000) / r_buttons[i];
                }
                i += 1;
            }

            // V_out = V_cc * R_pd / (R_pd + R_eq)
            let adc = (1024 * s_total) / (s_total + 10000);
            let adc_clamped = if adc > 1023 { 1023 } else { adc as u16 };

            buckets[valid_count] = (adc_clamped, state as u8);
            valid_count += 1;

            state += 1;
        }

        Self {
            buckets,
            valid_count,
        }
    }

    /// Returns the bitmask of pressed buttons based on the ADC reading.
    pub fn resolve(&self, adc_reading: u16) -> u8 {
        let mut best_state = 0;
        let mut min_diff = u16::MAX;

        for i in 0..self.valid_count {
            let (expected, state) = self.buckets[i];
            let diff = expected.abs_diff(adc_reading);
            if diff < min_diff {
                min_diff = diff;
                best_state = state;
            }
        }

        best_state
    }
}
