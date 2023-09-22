#![allow(
    non_upper_case_globals,
    dead_code
)]

pub mod shield {
    pub const guard_off_invalid_capture_frame_add : i32 = 9;
}

pub mod passive {
    pub const invalid_passive_damage_add : f32 = 33.0;
    pub const invalid_passive_reaction : f32 = 45.0;
}

pub mod jump {
    pub const special_jump_control_mul : f32 = 0.5;
    pub const super_jump_speed_x_mul : f32 = 0.8;
}

pub mod damage {
    pub const damage_speed_up_speed_min : f32 = 3.5;
    pub const damage_speed_up_speed_max : f32 = 6.0;
}

// Command Input Params

// pub const command_input_window : i32 = 11;
// pub const super_command_input_window : i32 = 21;