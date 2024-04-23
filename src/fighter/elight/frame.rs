use {
    crate::imports::*,
    crate::fighter::common::frame::common_fighter_frame
};

unsafe extern "C" fn elight_up_special_turnaround(fighter: &mut L2CFighterCommon) {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_hi_jump") {
        let stick_x = fighter.global_table[STICK_X].get_f32();
        let lr = PostureModule::lr(fighter.module_accessor);
        if stick_x * lr < -0.75 {
            PostureModule::reverse_lr(fighter.module_accessor);
        }
    }
}

unsafe extern "C" fn on_main(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    elight_up_special_turnaround(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, on_main);
}