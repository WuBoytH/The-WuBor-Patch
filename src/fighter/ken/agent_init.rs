use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::WorkModule,
        lib::{lua_const::*, L2CValue}
    },
    wubor_utils::{
        vars::*,
        table_const::*
    }
};

pub unsafe extern "C" fn ken_speciallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_SPECIAL_LW
    && WorkModule::get_int(fighter.module_accessor, FIGHTER_KEN_INSTANCE_WORK_ID_INT_QUICK_STEP_STATE) != FIGHTER_KEN_QUICK_STEP_STATE_DISABLE {
        return 1.into();
    }
    0.into()
}
