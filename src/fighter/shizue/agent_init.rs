use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    wubor_utils::table_const::*,
    super::{helper::*, vars::*}
};

unsafe extern "C" fn shizue_special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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

#[fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_SHIZUE {
            return;
        }
        fighter.global_table[SPECIAL_LW_PRE].assign(&L2CValue::Ptr(shizue_special_lw_pre as *const () as _));
    }
}

pub fn install() {
    install_agent_init_callbacks!(
        agent_init
    );
}
