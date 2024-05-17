use super::*;

extern "C" {
    #[link_name = "common_fighter_frame"]
    pub fn common_fighter_frame(fighter: &mut L2CFighterCommon);
}

unsafe extern "C" fn daisy_handle_disable_special_s(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.module_accessor, vars::fighter::instance::flag::DISABLE_SPECIAL_S)
    && (StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_AIR
    || MiscModule::is_damage_check(fighter.module_accessor, false)) {
        VarModule::off_flag(fighter.module_accessor, vars::fighter::instance::flag::DISABLE_SPECIAL_S);
    }
}

unsafe extern "C" fn on_main(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    daisy_handle_disable_special_s(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, on_main);
}