pub const DIR_LEFT: u8 = 1 << 0;
pub const DIR_RIGHT: u8 = 1 << 1;
pub const DIR_DOWN: u8 = 1 << 2;
pub const DIR_UP: u8 = 1 << 3;

pub const ACT_CENTER: u8 = 1 << 0;
pub const ACT_SET: u8 = 1 << 1;
pub const ACT_RESET: u8 = 1 << 2;

pub struct InputState {
    pub dir_curr: u8,
    pub dir_prev: u8,
    pub act_curr: u8,
    pub act_prev: u8,
}

impl InputState {
    /// Returns true only on the edge where the specified directional button was just pressed.
    pub fn just_pressed_dir(&self, mask: u8) -> bool {
        (self.dir_curr & mask) != 0 && (self.dir_prev & mask) == 0
    }

    /// Returns true only on the edge where the specified action button was just pressed.
    pub fn just_pressed_act(&self, mask: u8) -> bool {
        (self.act_curr & mask) != 0 && (self.act_prev & mask) == 0
    }

    /// Returns true iff no action button outside the mask is currently pressed.
    pub fn only_expected_act(&self, mask: u8) -> bool {
        0 == mask ^ (mask | self.act_curr)
    }

    /// Returns true iff no directional button outside the mask is currently pressed.
    pub fn only_expected_dir(&self, mask: u8) -> bool {
        0 == mask ^ (mask | self.dir_curr)
    }

    /// Returns true iff no action button outside the mask was previously pressed.
    pub fn only_expected_prev_act(&self, mask: u8) -> bool {
        0 == mask ^ (mask | self.act_prev)
    }

    /// Returns true iff no directional button outside the mask was previously is currently pressed.
    pub fn only_expected_prev_dir(&self, mask: u8) -> bool {
        0 == mask ^ (mask | self.dir_prev)
    }

    /// Returns true iff action intent (specified by mask) is newly present.
    pub fn intent_changed_act(&self, mask: u8) -> bool {
        self.only_expected_act(mask) && !self.only_expected_prev_act(mask)
    }

    /// Returns true iff directional intent (specified by mask) is newly present.
    pub fn intent_changed_dir(&self, mask: u8) -> bool {
        self.only_expected_dir(mask) && !self.only_expected_prev_dir(mask)
    }
}
