use {
    smash::{
        lua2cpp::L2CFighterCommon,
        lib::{lua_const::*, L2CValue}
    },
    crate::{
        common_funcs::*,
        vars::*,
        table_const::*
    }
};

pub unsafe extern "C" fn ken_speciallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_SPECIAL_LW
    && QUICK_STEP_STATE[entry_id(fighter.module_accessor)] != 2 {
        return 1.into();
    }
    0.into()
}
