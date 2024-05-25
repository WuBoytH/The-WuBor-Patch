use super::*;

unsafe extern "C" fn mario_speciallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::get_int(fighter.module_accessor, vars::mario::instance::int::SPECIAL_LW_KIND) == vars::mario::SPECIAL_LW_KIND_GROUND_POUND_CANCEL {
        return 0.into();
    }
    1.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[CHECK_SPECIAL_LW_UNIQ].assign(&L2CValue::Ptr(mario_speciallw_pre as *const () as _));
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
}
