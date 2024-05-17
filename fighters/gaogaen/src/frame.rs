use super::*;

extern "C" {
    #[link_name = "common_fighter_frame"]
    pub fn common_fighter_frame(fighter: &mut L2CFighterCommon);
}

unsafe extern "C" fn gaogaen_lariat_jump_cancel(fighter: &mut L2CFighterCommon) {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
    && StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_N
    && VarModule::is_flag(fighter.module_accessor, vars::fighter::status::flag::JUMP_CANCEL)
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    && !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
    && !fighter.global_table[IS_STOP].get_bool() {
        FGCModule::jump_cancel_check_exception(fighter);
    }
}

unsafe extern "C" fn on_main(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    gaogaen_lariat_jump_cancel(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, on_main);
}