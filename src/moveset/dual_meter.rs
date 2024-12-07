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
pub struct TensionMeter {
    pub value: f32,
    pub gear_1_frame: f32,
    pub gear_2_frame: f32,
    pub flame_1_frame: f32,
    pub flame_2_frame: f32,
    pub is_available1: bool,
    pub is_available2: bool
}

impl TensionMeter {
    pub fn new() -> Self {
        Self {
            value: 50.0,
            gear_1_frame: 0.0,
            gear_2_frame: 0.0,
            flame_1_frame: 0.0,
            flame_2_frame: 0.0,
            is_available1: true,
            is_available2: false
        }
    }

    pub fn reset(&mut self) {
        self.value = 50.0;
        self.gear_1_frame = 0.0;
        self.gear_2_frame = 0.0;
        self.flame_1_frame = 0.0;
        self.flame_2_frame = 0.0;
        self.is_available1 = true;
        self.is_available2 = false;
    }

    pub fn add_meter(&mut self, amount: f32) {
        if amount >= 0.0{
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
        } else {
            if self.value + amount < 0.0{
                self.value = 0.0;
            }else{
                self.value += amount;
            }
            if self.value < 50.0{
                self.is_available1 = false;
            }
            if self.value < 100.0{
                self.is_available2 = false;
            }
        }
    }

    pub fn get_meter_frame(&mut self) -> f32 {
        return self.value;
    }

    pub fn consume_tension(&mut self) {
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

    pub fn is_tension1_available(&mut self) -> bool {
        return self.is_available1;
    }

    pub fn is_tension2_available(&mut self) -> bool {
        return self.is_available2;
    }

    pub fn get_gear_frame_1(&mut self) -> f32 {
        if self.gear_1_frame > 100.0 {
            self.gear_1_frame = 0.0;
        }
        self.gear_1_frame += 1.0;
        return self.gear_1_frame;
    }
    pub fn get_gear_frame_2(&mut self) -> f32 {
        if self.gear_2_frame > 100.0 {
            self.gear_2_frame = 0.0;
        }
        self.gear_2_frame += 1.0;
        return self.gear_2_frame;
    }
    pub fn get_flame_frame_1(&mut self) -> f32 {
        if self.is_available1{
            if self.flame_1_frame > 21.0 {
                self.flame_1_frame = 0.0;
            }
            self.flame_1_frame += 1.0;
            return self.flame_1_frame;
        }
        else{
            return 0.0
        }
    }
    pub fn get_flame_frame_2(&mut self) -> f32 {
        if self.is_available2{
            if self.flame_2_frame > 21.0 {
                self.flame_2_frame = 0.0;
            }
            self.flame_2_frame += 1.0;
            return self.flame_2_frame;
        }
        else{
            return 0.0
        }
    }
}

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct CutInUI {
    pub is_on: bool,
    pub bar_low_pos: f32,
    pub bar_hi_pos: f32,
    pub bars_alpha: f32,
    pub is_bar_out: bool,
    pub game_speed: f32,
    pub bar_in_out_duration: f32
}
impl CutInUI {
    pub fn reset() -> Self {
        Self {
            is_on: false,
            bar_low_pos: -550.0,
            bar_hi_pos: 550.0,
            bars_alpha: 0.0,
            is_bar_out: false,
            game_speed: 1.0,
            bar_in_out_duration: 30.0,
        }
    }
    pub fn reset_all(&mut self) {
        self.is_on= false;
        self.bar_low_pos = -550.0;
        self.bar_hi_pos = 550.0;
        self.bars_alpha = 0.0;
        self.is_bar_out= false;
        self.game_speed = 1.0;
        self.bar_in_out_duration = 30.0;
    }
    pub fn is_bar_on(&mut self) -> bool {
        return self.is_on;
    }
    pub fn set_bar_out(&mut self) -> bool {
        self.is_bar_out = true;
        return self.is_bar_out;
    }
    pub fn is_bar_out(&mut self) -> bool {
        return self.is_bar_out;
    }
    pub fn set_bar(&mut self,state:bool) -> bool {
        self.is_on = state;
        return self.is_on;
    }
    pub fn get_bar_in_pos(&mut self, pane: &str) -> f32 {
        if pane == "bar_hi"{
            if self.bar_hi_pos <= 490.0 {
                self.bar_hi_pos = 490.0;
            }else{
                self.bar_hi_pos -= (60.0/self.bar_in_out_duration) * self.game_speed;
            }
            return self.bar_hi_pos;
        }else{
            if self.bar_low_pos >= -490.0 {
                self.bar_low_pos = -490.0;
            }else{
                self.bar_low_pos += (60.0/self.bar_in_out_duration) * self.game_speed;
            }
            return self.bar_low_pos;
        }
    }
    pub fn get_bar_out_pos(&mut self, pane: &str) -> f32 {
        if pane == "bar_hi"{
            if self.bar_hi_pos >= 550.0 {
                self.set_bar(false);
                self.is_bar_out = false;
                self.bar_hi_pos = 550.0;
            }else{
                self.bar_hi_pos += (60.0/self.bar_in_out_duration) * self.game_speed;
            }
            return self.bar_hi_pos;
        }else{
            if self.bar_low_pos <= -550.0 {
                self.set_bar(false);
                self.is_bar_out = false;
                self.bar_low_pos = -550.0;
            }else{
                self.bar_low_pos -= (60.0/self.bar_in_out_duration) * self.game_speed;  // move this and similar up later since this doesnt seem entirely correct
            }
            return self.bar_low_pos;
        }
    }
    pub fn get_bar_in_alpha(&mut self) -> f32 {
        if self.bars_alpha >= 255.0 {
            self.bars_alpha = 255.0;
        }else{
            self.bars_alpha += (255.0/self.bar_in_out_duration) * self.game_speed;
        }
        return self.bars_alpha;
    }
    pub fn get_bar_out_alpha(&mut self) -> f32 {
        if self.bars_alpha <= 0.0 {
            self.bars_alpha = 0.0;
        }else{
            self.bars_alpha -= (255.0/self.bar_in_out_duration) * self.game_speed;
        }
        return self.bars_alpha;
    }

    pub fn set_game_speed(&mut self, speed:f32) {
        self.game_speed = speed;
    }

}