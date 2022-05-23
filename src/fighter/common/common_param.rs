#![allow(non_upper_case_globals)]

// Shield Params

pub const guard_off_attack_cancel_frame : i32 = 5;
pub const guard_off_invalid_capture_frame_add : i32 = 3;

pub mod jump {
    pub const special_jump_control_mul : f32 = 0.5;
    pub const hyper_hop_air_speed_x_stable_mul : f32 = 1.4;
    pub const super_jump_speed_x_mul : f32 = 0.8;
    pub const super_jump_speed_y_init : f32 = 1.825;
    pub const super_jump_gravity : f32 = 0.27;
}

// Command Input Params

// pub const command_input_window : i32 = 11;
// pub const super_command_input_window : i32 = 21;