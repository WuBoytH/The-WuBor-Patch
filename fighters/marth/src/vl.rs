#![allow(non_upper_case_globals)]

pub mod param_stance {
    pub const stance_change_lockout_frame : i32 = 20;
    pub const enter_air_speed_x_mul : f32 = 0.3;
    pub const fall_speed_x_mul : f32 = 0.3;
    pub const dash_attack_cancel_frame : f32 = 8.0;
    pub const special_s_start_x_mul : f32 = 0.5;
    pub const special_s_start_y_mul : f32 = 0.2;
    pub const special_s2_start_x : f32 = 1.3;
    pub const special_s2_start_y : f32 = 1.7;
    pub const special_s2_x_accel_mul_air : f32 = 0.3;
    pub const special_s2_y_accel_mul_fall : f32 = 0.5;
    pub const special_s2_y_stable_mul : f32 = 1.4;
    pub const special_s2_ground_loop_max : i32 = 5;
    pub const special_s2_air_loop_max : i32 = 4;
}
