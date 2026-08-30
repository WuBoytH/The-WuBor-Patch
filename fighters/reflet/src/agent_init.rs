use super::*;

pub unsafe extern "C" fn reflet_status_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        VarModule::off_flag(fighter.module_accessor, vars::fighter::instance::flag::DISABLE_SPECIAL_HI);
    }
    0.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(reflet_status_end_control as *const () as _));
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
}
