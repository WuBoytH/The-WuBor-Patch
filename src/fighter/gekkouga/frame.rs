use {
    crate::imports::*,
    crate::fighter::common::frame::common_fighter_frame
};

unsafe extern "C" fn gekkouga_taunt_cancel(fighter: &mut L2CFighterCommon) {
    if [
        hash40("appeal_lw_l"),
        hash40("appeal_lw_r")
    ].contains(&MotionModule::motion_kind(fighter.module_accessor))
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
    && !fighter.global_table[IS_STOP].get_bool() {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

unsafe extern "C" fn gekkouga_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    gekkouga_taunt_cancel(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, gekkouga_frame);
}