use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::WorkModule,
        lib::L2CValue
    },
    super::vars::*
};

pub unsafe extern "C" fn mario_speciallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::get_int(fighter.module_accessor, FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_KIND) == FIGHTER_MARIO_SPECIAL_LW_KIND_GROUND_POUND_CANCEL {
        return 0.into();
    }
    1.into()
}
