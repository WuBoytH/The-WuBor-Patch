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

pub unsafe extern "C" fn yu_specialns_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if AIR_ACTION[entry_id(fighter.module_accessor)] {
        return 0.into();
    }
    1.into()
}

pub unsafe extern "C" fn yu_speciallw_pre(_fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}
