#![allow(non_upper_case_globals)]

pub mod param_private {
    pub const sp_single : f32 = 25.0;
    pub const sp_super : f32 = 50.0;
    pub const sp_ex_shadow : f32 = 10.0;
    pub const sp_onemore_shadow : f32 = 25.0;
    pub const sp_super_shadow : f32 = 20.0;
    pub const sp_effect_timer : i32 = 300;
    pub const sp_effect_timer_shadow : i32 = 600;
    pub const sp_gain_penalty : f32 = 360.0;
    pub const awakening_activation_damage : f32 = 100.0;
    pub const awakening_damage_mul : f32 = 0.8;
    pub const awakening_sp_max : f32 = 150.0;
    pub const awakening_sp_gain : f32 = 50.0;
    pub const shadow_type_damage_mul : f32 = 0.92;
    pub const shadow_type_attack_mul : f32 = 0.8;
    pub const shadow_frenzy_sp_drain : f32 = 1.0 / 12.0;
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