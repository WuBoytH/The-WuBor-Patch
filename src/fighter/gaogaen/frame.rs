use {
    crate::imports::status_imports::*,
    crate::fighter::common::frame::common_fighter_frame
};

unsafe extern "C" fn gaogaen_lariat_jump_cancel(fighter: &mut L2CFighterCommon) {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
    && StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_N
    && VarModule::is_flag(fighter.module_accessor, fighter::status::flag::JUMP_CANCEL)
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    && !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
    && !fighter.global_table[IS_STOP].get_bool() {
        FGCModule::jump_cancel_check_exception(fighter);
    }
}

unsafe extern "C" fn gaogaen_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    gaogaen_lariat_jump_cancel(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, gaogaen_frame);
}