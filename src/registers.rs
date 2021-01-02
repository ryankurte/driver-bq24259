/// Device registers for bq24259
/// From https://www.ti.com/lit/ds/symlink/bq24259.pdf
/// Copyright 2021 ryan kurte

/// Generated using [reggen](https://github.com/ryankurte/reggen)
/// (You probably don't want to edit this file directly)

/// Base register type
type Reg = u8;

/// bq24259 register addresses
pub enum RegisterAddress {
    /// Input Source Control Register REG00
    Reg00 = 0x00000000,
    /// Power-On Configuration Register REG01
    Reg01 = 0x00000001,
    /// Charge Current Control Register REG02
    Reg02 = 0x00000002,
}

/// Input Source Control Register REG00
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Reg00 {
    raw: Reg,
}

/// Default / Initialisation value (0x00000030)
impl Default for Reg00 {
    fn default() -> Self {
        Self::from(48)
    }
}

/// Create a Reg00 object from the provided raw value
impl From<Reg> for Reg00 {
    fn from(raw: Reg) -> Self {
        Self { raw }
    }
}

/// Convert a Reg00 object into a raw value
impl Into<Reg> for Reg00 {
    fn into(self) -> Reg {
        self.raw
    }
}

/// Input Current Limit
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum IinLim {
    I1000MA = 4,
    I100MA = 0,
    I1500MA = 5,
    I150MA = 1,
    I2000MA = 6,
    I3000MA = 7,
    I500MA = 2,
    I900MA = 3,
}

impl IinLim {
    /// Create an IinLim from a raw value,
    /// Internal use only
    fn from_raw(raw: Reg) -> Self {
        match raw {
            4 => IinLim::I1000MA,
            0 => IinLim::I100MA,
            5 => IinLim::I1500MA,
            1 => IinLim::I150MA,
            6 => IinLim::I2000MA,
            7 => IinLim::I3000MA,
            2 => IinLim::I500MA,
            3 => IinLim::I900MA,
            _ => panic!("Unexpected value: {:?}", raw),
        }
    }
}

impl Reg00 {
    /// Fetch 'en_hiz' field (shift: `7`, mask: `0b10000000`)
    pub fn get_en_hiz(&self) -> bool {
        let v = (self.raw & 0b10000000) >> 7;
        v != 0
    }
    /// Set 'en_hiz' field (shift: `7`, mask: `0b10000000`)
    pub fn set_en_hiz(&mut self, val: bool) {
        match val {
            true => self.raw |= 0b10000000,
            false => self.raw &= !0b10000000,
        }
    }

    /// Fetch 'iin_lim' field (shift: `0`, mask: `0b00000111`)
    pub fn get_iin_lim(&self) -> IinLim {
        let v = (self.raw & 0b00000111) >> 0;
        IinLim::from_raw(v)
    }
    /// Set 'iin_lim' field (shift: `0`, mask: `0b00000111`)
    pub fn set_iin_lim(&mut self, val: IinLim) {
        self.raw &= !0b00000111;
        self.raw |= ((val as Reg) << 0) & 0b00000111;
    }

    /// Fetch 'vin_lim' field (shift: `3`, mask: `0b01111000`)
    pub fn get_vin_lim(&self) -> i32 {
        let v = (self.raw & 0b01111000) >> 3;
        (v as i32) * 80
    }
    /// Set 'vin_lim' field (shift: `3`, mask: `0b01111000`)
    pub fn set_vin_lim(&mut self, val: i32) {
        self.raw &= !0b01111000;
        let val = val / 80;
        self.raw |= ((val as Reg) << 3) & 0b01111000;
    }
}

/// Power-On Configuration Register REG01
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Reg01 {
    raw: Reg,
}

/// Default / Initialisation value (0x0000001b)
impl Default for Reg01 {
    fn default() -> Self {
        Self::from(27)
    }
}

/// Create a Reg01 object from the provided raw value
impl From<Reg> for Reg01 {
    fn from(raw: Reg) -> Self {
        Self { raw }
    }
}

/// Convert a Reg01 object into a raw value
impl Into<Reg> for Reg01 {
    fn into(self) -> Reg {
        self.raw
    }
}

/// Boost Current Limit
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BoostLim {
    I1000MA = 0,
    I1500MA = 1,
}

impl BoostLim {
    /// Create an BoostLim from a raw value,
    /// Internal use only
    fn from_raw(raw: Reg) -> Self {
        match raw {
            0 => BoostLim::I1000MA,
            1 => BoostLim::I1500MA,
            _ => panic!("Unexpected value: {:?}", raw),
        }
    }
}

impl Reg01 {
    /// Fetch 'boost_lim' field (shift: `0`, mask: `0b00000111`)
    pub fn get_boost_lim(&self) -> BoostLim {
        let v = (self.raw & 0b00000111) >> 0;
        BoostLim::from_raw(v)
    }
    /// Set 'boost_lim' field (shift: `0`, mask: `0b00000111`)
    pub fn set_boost_lim(&mut self, val: BoostLim) {
        self.raw &= !0b00000111;
        self.raw |= ((val as Reg) << 0) & 0b00000111;
    }

    /// Fetch 'charge_en' field (shift: `4`, mask: `0b00010000`)
    pub fn get_charge_en(&self) -> bool {
        let v = (self.raw & 0b00010000) >> 4;
        v != 0
    }
    /// Set 'charge_en' field (shift: `4`, mask: `0b00010000`)
    pub fn set_charge_en(&mut self, val: bool) {
        match val {
            true => self.raw |= 0b00010000,
            false => self.raw &= !0b00010000,
        }
    }

    /// Fetch 'iic_wdt_reset' field (shift: `6`, mask: `0b01000000`)
    pub fn get_iic_wdt_reset(&self) -> bool {
        let v = (self.raw & 0b01000000) >> 6;
        v != 0
    }
    /// Set 'iic_wdt_reset' field (shift: `6`, mask: `0b01000000`)
    pub fn set_iic_wdt_reset(&mut self, val: bool) {
        match val {
            true => self.raw |= 0b01000000,
            false => self.raw &= !0b01000000,
        }
    }

    /// Fetch 'otg_en' field (shift: `5`, mask: `0b00100000`)
    pub fn get_otg_en(&self) -> bool {
        let v = (self.raw & 0b00100000) >> 5;
        v != 0
    }
    /// Set 'otg_en' field (shift: `5`, mask: `0b00100000`)
    pub fn set_otg_en(&mut self, val: bool) {
        match val {
            true => self.raw |= 0b00100000,
            false => self.raw &= !0b00100000,
        }
    }

    /// Fetch 'register_reset' field (shift: `7`, mask: `0b10000000`)
    pub fn get_register_reset(&self) -> bool {
        let v = (self.raw & 0b10000000) >> 7;
        v != 0
    }
    /// Set 'register_reset' field (shift: `7`, mask: `0b10000000`)
    pub fn set_register_reset(&mut self, val: bool) {
        match val {
            true => self.raw |= 0b10000000,
            false => self.raw &= !0b10000000,
        }
    }

    /// Fetch 'vin_lim' field  in mV (shift: `1`, mask: `0b00001110`)
    pub fn get_vin_lim_mv(&self) -> Reg {
        let v = (self.raw & 0b00001110) >> 1;
        v
    }
    /// Set 'vin_lim' field  in mV (shift: `1`, mask: `0b00001110`)
    pub fn set_vin_lim_mv(&mut self, val: Reg) {
        self.raw &= !0b00001110;
        self.raw |= ((val as Reg) << 1) & 0b00001110;
    }
}

/// Charge Current Control Register REG02
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Reg02 {
    raw: Reg,
}

/// Default / Initialisation value (0x00000060)
impl Default for Reg02 {
    fn default() -> Self {
        Self::from(96)
    }
}

/// Create a Reg02 object from the provided raw value
impl From<Reg> for Reg02 {
    fn from(raw: Reg) -> Self {
        Self { raw }
    }
}

/// Convert a Reg02 object into a raw value
impl Into<Reg> for Reg02 {
    fn into(self) -> Reg {
        self.raw
    }
}

impl Reg02 {
    /// Fetch 'bcold' field (shift: `1`, mask: `0b00000010`)
    pub fn get_bcold(&self) -> bool {
        let v = (self.raw & 0b00000010) >> 1;
        v != 0
    }
    /// Set 'bcold' field (shift: `1`, mask: `0b00000010`)
    pub fn set_bcold(&mut self, val: bool) {
        match val {
            true => self.raw |= 0b00000010,
            false => self.raw &= !0b00000010,
        }
    }

    /// Fetch 'charge_en' field (shift: `4`, mask: `0b00010000`)
    pub fn get_charge_en(&self) -> bool {
        let v = (self.raw & 0b00010000) >> 4;
        v != 0
    }
    /// Set 'charge_en' field (shift: `4`, mask: `0b00010000`)
    pub fn set_charge_en(&mut self, val: bool) {
        match val {
            true => self.raw |= 0b00010000,
            false => self.raw &= !0b00010000,
        }
    }

    /// Fetch 'force_20_pc' field (shift: `0`, mask: `0b00000001`)
    pub fn get_force_20_pc(&self) -> bool {
        let v = (self.raw & 0b00000001) >> 0;
        v != 0
    }
    /// Set 'force_20_pc' field (shift: `0`, mask: `0b00000001`)
    pub fn set_force_20_pc(&mut self, val: bool) {
        match val {
            true => self.raw |= 0b00000001,
            false => self.raw &= !0b00000001,
        }
    }

    /// Fetch 'otg_en' field (shift: `5`, mask: `0b00100000`)
    pub fn get_otg_en(&self) -> bool {
        let v = (self.raw & 0b00100000) >> 5;
        v != 0
    }
    /// Set 'otg_en' field (shift: `5`, mask: `0b00100000`)
    pub fn set_otg_en(&mut self, val: bool) {
        match val {
            true => self.raw |= 0b00100000,
            false => self.raw &= !0b00100000,
        }
    }

    /// Fetch 'vin_lim' field  in mA (shift: `2`, mask: `0b01111100`)
    pub fn get_vin_lim_ma(&self) -> i32 {
        let v = (self.raw & 0b01111100) >> 2;
        (v as i32) * 64 + 512
    }
    /// Set 'vin_lim' field  in mA (shift: `2`, mask: `0b01111100`)
    pub fn set_vin_lim_ma(&mut self, val: i32) {
        self.raw &= !0b01111100;
        let val = val - 512;
        let val = val / 64;
        self.raw |= ((val as Reg) << 2) & 0b01111100;
    }
}
