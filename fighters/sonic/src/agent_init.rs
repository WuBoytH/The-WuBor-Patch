use super::*;

extern "C" {
    #[link_name = "specials_pre_generic"]
    pub fn specials_pre_generic(fighter: &mut L2CFighterCommon) -> L2CValue;
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(specials_pre_generic as *const () as _));
    fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(sonic_status_end_control as *const () as _));
}

unsafe extern "C" fn sonic_status_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR
    || status == *FIGHTER_STATUS_KIND_REBIRTH {
        VarModule::off_flag(fighter.module_accessor, vars::fighter::instance::flag::DISABLE_SPECIAL_S);
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_start(on_start);
}
