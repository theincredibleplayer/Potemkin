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
    pub is_bar_in: bool,
    pub is_bar_out: bool,
    pub bar_visibility: bool,
    pub is_jetpack_on: bool,
    pub is_explosion_1_on: bool,
    pub is_explosion_2_on: bool,
    pub game_speed: f32,
    pub jetpack_lr: f32,
    pub bar_in_frame: f32,
    pub jetpack_frame: f32,
    pub explosion_1_frame: f32,
    pub explosion_2_frame: f32
}
impl CutInUI {
    pub fn reset() -> Self {
        Self {
            is_on: false,
            is_bar_in: false,
            is_bar_out: false,
            bar_visibility: false,
            is_jetpack_on: false,
            is_explosion_1_on: false,
            is_explosion_2_on: false,
            game_speed: 1.0,
            jetpack_lr: 1.0,
            bar_in_frame: 0.0,
            jetpack_frame: 0.0,
            explosion_1_frame: 0.0,
            explosion_2_frame: 0.0
        }
    }
    pub fn reset_all(&mut self) {
        self.is_on= false;
        self.is_bar_in= false;
        self.is_bar_out= false;
        self.bar_visibility = false;
        self.is_jetpack_on = false;
        self.is_explosion_1_on = false;
        self.is_explosion_2_on = false;
        self.game_speed = 1.0;
        self.jetpack_lr = 1.0;
        self.bar_in_frame = 0.0;
        self.jetpack_frame = 0.0;
        self.explosion_1_frame = 0.0;
        self.explosion_2_frame = 0.0;
    }
    pub fn set_jetpack_lr(&mut self, lr:f32) {
        self.jetpack_lr = lr;
    }
    pub fn get_jetpack_lr(&mut self) ->f32 {
        return self.jetpack_lr;
    }
    pub fn set_game_speed(&mut self, speed:f32) {
        self.game_speed = speed;
    }
    //bar functions
    pub fn is_bar_on(&mut self) -> bool {
        return self.bar_visibility;
    }
    pub fn set_bar_on(&mut self) -> bool {
        self.is_bar_in = true;
        self.bar_visibility = true;
        return self.is_bar_in;
    }
    pub fn set_bar_off(&mut self) -> bool {
        self.is_bar_out = true;
        self.is_bar_in = false;
        return self.is_bar_out;
    }
    pub fn get_bar_frame(&mut self) -> f32 {
        if self.bar_in_frame < 9.0 && self.is_bar_in {
            self.bar_in_frame += 0.1 * self.game_speed;
        }else if self.bar_in_frame >= 9.0  && self.is_bar_in {
            self.bar_in_frame = 9.0;
        }else if (self.bar_in_frame > 0.0) && (self.is_bar_out) {
            self.bar_in_frame -= 0.1 * self.game_speed;
        }else{
            self.bar_in_frame = 0.0;
            self.is_bar_out = false;
        }

        if self.bar_in_frame == 0.0 {
            self.bar_visibility = false;
        } else {
            self.bar_visibility = true;
        }
        return self.bar_in_frame;
    }



    //jetpack functions
    pub fn set_jetpack_on(&mut self) -> bool {
        self.is_jetpack_on = true;
        return self.is_jetpack_on;
    }
    pub fn is_jetpack_on(&mut self) -> bool {
        return self.is_jetpack_on;
    }
    pub fn get_jetpack_frame(&mut self) -> f32 {
        if self.is_jetpack_on{
            self.jetpack_frame += 1.0 * self.game_speed;
            if self.jetpack_frame > 84.0 {
                self.jetpack_frame = 0.0 ;
                self.is_jetpack_on = false;
            }
        }
        return self.jetpack_frame;
    }
    //explosion_1 functions
    pub fn set_explosion_1_on(&mut self) -> bool {
        self.is_explosion_1_on = true;
        return self.is_explosion_1_on;
    }
    pub fn is_explosion_1_on(&mut self) -> bool {
        return self.is_explosion_1_on;
    }
    pub fn get_explosion_1_frame(&mut self) -> f32 {
        if self.is_explosion_1_on{
            self.explosion_1_frame += 1.0 * self.game_speed;
            if self.explosion_1_frame > 16.0 {
                self.explosion_1_frame = 0.0 ;
                self.is_explosion_1_on = false;
            }
        }
        return self.explosion_1_frame;
    }
    //explosion_2 functions
    pub fn set_explosion_2_on(&mut self) -> bool {
        self.is_explosion_2_on = true;
        return self.is_explosion_2_on;
    }
    pub fn is_explosion_2_on(&mut self) -> bool {
        return self.is_explosion_2_on;
    }
    pub fn get_explosion_2_frame(&mut self) -> f32 {
        if self.is_explosion_2_on{
            self.explosion_2_frame += 1.0 * self.game_speed;
            if self.explosion_2_frame > 9.0 {
                self.explosion_2_frame = 0.0 ;
                self.is_explosion_2_on = false;
            }
        }
        return self.explosion_2_frame;
    }
}




//for jetpack is_jetpack_on for visibility and get_jetpack_frame for frame