#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct LimitMeter {
    pub value: f32,
    pub glow_frame: f32,
    pub is_available: bool
}

impl LimitMeter {
    pub fn new() -> Self {
        Self {
            value: 0.0,
            glow_frame: 0.0,
            is_available: false
        }
    }

    pub fn reset(&mut self) {
        self.value = 0.0;
        self.glow_frame = 0.0;
        self.is_available = false;
    }

    pub fn add_meter(&mut self, amount: f32) {
        if self.value + amount > 100.0 {
            self.value = 100.0;
            self.is_available = true;
        }
        else {
            self.value += amount;
        }
    }

    pub fn is_available(&mut self) -> bool {
        return self.is_available;
    }

    pub fn get_glow_frame(&mut self) -> f32 {
        if self.glow_frame > 90.0 {
            self.glow_frame = 0.0;
        }
        self.glow_frame += 0.5;
        return self.glow_frame;
    }

    pub fn get_meter_frame(&mut self) -> f32 {
        return self.value;
    }
}

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct ATBMeter {
    pub value: f32,
    pub glow_frame: f32,
    pub is_available1: bool,
    pub is_available2: bool
}

impl ATBMeter {
    pub fn new() -> Self {
        Self {
            value: 50.0,
            glow_frame: 0.0,
            is_available1: true,
            is_available2: false
        }
    }

    pub fn reset(&mut self) {
        self.value = 50.0;
        self.glow_frame = 0.0;
        self.is_available1 = true;
        self.is_available2 = false;
    }

    pub fn add_meter(&mut self, amount: f32) {
        if self.value + amount > 100.0 {
            self.value = 100.0;
            self.is_available2 = true;
        }
        else {
            self.value += amount;
            if self.value >= 50.0 {
                self.is_available1 = true;
            }
        }
    }

    pub fn get_meter_frame(&mut self) -> f32 {
        return self.value;
    }

    pub fn consume_atb(&mut self) {
        self.value -= 50.0;
        self.is_available2 = false;
        if self.value < 50.0 {
            self.is_available1 = false;
        }
        if self.value < 0.0 {
            self.value = 0.0;
        }
    }

    pub fn is_available(&mut self) -> bool {
        return self.is_available1 || self.is_available2;
    }

    pub fn is_atb1_available(&mut self) -> bool {
        return self.is_available1;
    }

    pub fn is_atb2_available(&mut self) -> bool {
        return self.is_available2;
    }

    pub fn get_glow_frame(&mut self) -> f32 {
        if self.glow_frame > 90.0 {
            self.glow_frame = 0.0;
        }
        self.glow_frame += 0.5;
        return self.glow_frame;
    }
}