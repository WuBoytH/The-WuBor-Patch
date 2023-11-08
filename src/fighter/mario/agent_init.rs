use {
    smash::{
        lua2cpp::L2CFighterCommon,
        lib::L2CValue
    },
    custom_var::*,
    wubor_utils::{vars::*, table_const::*}
};

unsafe extern "C" fn mario_speciallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::get_int(fighter.module_accessor, mario::instance::int::SPECIAL_LW_KIND) == mario::SPECIAL_LW_KIND_GROUND_POUND_CANCEL {
        return 0.into();
    }
    1.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[CHECK_SPECIAL_LW_UNIQ].assign(&L2CValue::Ptr(mario_speciallw_pre as *const () as _));
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_start(on_start);
}
