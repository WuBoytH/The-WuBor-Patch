use super::*;

extern "C" {
    #[link_name = "common_fighter_frame"]
    pub fn common_fighter_frame(fighter: &mut L2CFighterCommon);
}

unsafe extern "C" fn richter_reset_vars(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.module_accessor, vars::fighter::instance::flag::DISABLE_SPECIAL_HI)
    && (StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH
    || StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
    || MiscModule::is_damage_check(fighter.module_accessor, false)) {
        VarModule::off_flag(fighter.module_accessor, vars::fighter::instance::flag::DISABLE_SPECIAL_HI);
    }
}

unsafe extern "C" fn on_main(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    richter_reset_vars(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, on_main);
}