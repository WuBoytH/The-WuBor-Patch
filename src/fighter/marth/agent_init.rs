use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::L2CValue
    },
    custom_status::*,
    super::vars::*
};

pub unsafe extern "C" fn marth_check_special_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_IS_STANCE) {
        let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ENTER);
        fighter.change_status(status.into(), true.into());
        return true.into();
    }
    false.into()
}

pub unsafe extern "C" fn marth_speciallw_pre(_fighter: &mut L2CFighterCommon) -> L2CValue {
    false.into()
}
