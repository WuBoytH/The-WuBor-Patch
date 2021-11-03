use {
    smash::{
        lua2cpp::L2CFighterCommon,
        lib::L2CValue
    },
    crate::{
        common_funcs::*,
        vars::*
    }
};

pub unsafe extern "C" fn mario_speciallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if SPECIAL_LW_TYPE[entry_id(fighter.module_accessor)] == 2 {
        return 0.into();
    }
    1.into()
}
