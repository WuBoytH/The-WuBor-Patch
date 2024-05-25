use super::*;

extern "C" {
    #[link_name = "specialhi_pre_generic"]
    pub fn specialhi_pre_generic(fighter: &mut L2CFighterCommon) -> L2CValue;
}

unsafe extern "C" fn status_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR
    || fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
        VarModule::off_flag(fighter.module_accessor, vars::fighter::instance::flag::DISABLE_SPECIAL_HI);
    }
    0.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(specialhi_pre_generic as *const () as _));
    fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(status_end_control as *const () as _));
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
}