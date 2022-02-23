#![allow(non_upper_case_globals)]

// Yu Narukami Params

pub mod param_private {
    pub const sp_single : f32 = 25.0;
    pub const sp_super : f32 = 50.0;
    pub const sp_ex_shadow : f32 = 6.25;
    pub const sp_onemore_shadow : f32 = 12.5;
    pub const sp_super_shadow : f32 = 25.0;
    pub const sp_effect_timer : i32 = 300;
    pub const sp_effect_timer_shadow : i32 = 600;
    pub const sp_gain_penalty : f32 = 360.0;
}

pub mod param_special_s {
    pub const dive_speed_x : f32 = 2.5;
    pub const dive_speed_y : f32 = -2.1;
    pub const dive_speed_x_ex : f32 = 3.0;
    pub const dive_speed_y_ex : f32 = -2.4;
}

pub mod param_special_lw {
    pub const onemore_slowdown_mul : i32 = 50;
    pub const onemore_slowdown_frame : i32 = 19;
    pub const onemore_slowdown_mul_on_hit : i32 = 50;
    pub const onemore_slowdown_frame_on_hit : i32 = 20;
}