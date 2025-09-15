use super::*;

extern "C" {
    #[link_name = "common_fighter_frame"]
    pub fn common_fighter_frame(fighter: &mut L2CFighterCommon);
}

unsafe extern "C" fn mario_reset_special_lw_kind(fighter: &mut L2CFighterCommon) {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
    || StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_CLIFF
    || fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_REBIRTH {
        VarModule::off_flag(fighter.module_accessor, vars::fighter::instance::flag::DISABLE_SPECIAL_LW);
    }
}

unsafe extern "C" fn on_main(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    mario_reset_special_lw_kind(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, on_main);
}