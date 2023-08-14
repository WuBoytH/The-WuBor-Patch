use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::*,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    custom_var::*,
    wubor_utils::{vars::*, table_const::*}
};

unsafe extern "C" fn ken_speciallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_SPECIAL_LW
    && VarModule::get_int(fighter.battle_object, ken::instance::int::QUICK_STEP_STATE) != ken::QUICK_STEP_STATE_DISABLE {
        return 1.into();
    }
    0.into()
}

#[event("ken", initialize)]
unsafe fn agent_init(fighter: &mut L2CFighterCommon) {
    fighter.global_table[CHECK_SPECIAL_LW_UNIQ].assign(&L2CValue::Ptr(ken_speciallw_pre as *const () as _));
}

pub fn install() {
    agent_init::install();
}
