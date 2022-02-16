use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    super::helper::*,
    wubor_utils::vars::*
};

pub unsafe extern "C" fn shizue_special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if shizue_check_rocket_fire(fighter)
    && shizue_check_attack_cancel(fighter) {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_SHIZUE_INSTANCE_WORK_ID_FLAG_FIRE_ROCKET_ANYTIME);
        ControlModule::clear_command_one(
            fighter.module_accessor,
            *FIGHTER_PAD_COMMAND_CATEGORY1,
            *FIGHTER_PAD_CMD_CAT1_SPECIAL_LW
        );
        return 0.into();
    }
    1.into()
}
