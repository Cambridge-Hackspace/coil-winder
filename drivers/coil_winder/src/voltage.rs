pub struct VoltageMonitor;

impl VoltageMonitor {
    /// Calculates the voltage in millivolts given an ADC reading.
    /// Assumes a voltage divider with 30k ohm to the source and 10k ohm to ground,
    /// and a 5.0 ADC reference.
    pub fn calculate_millivolts(adc_reading: u16) -> u16 {
        // V_pin = ADC * 5000 / 1024 (mV)
        // V_source = V_pin * (30k + 10k) / 10k = V_pin * 4
        // V_source = ADC * 20000 / 1024 = ADC * 625 / 32
        ((adc_reading as u32 * 625) / 32) as u16
    }
}
