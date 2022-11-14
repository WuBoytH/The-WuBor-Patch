use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    crate::fighter::ryu::helper::*
};

#[status_script(agent = "ken", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn ken_specials_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_specials_init_main(fighter)
}

#[status_script(agent = "ken", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn ken_specials(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_specials_main(fighter);
    0.into()
}

#[status_script(agent = "ken", status = FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn ken_specials_command_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND);
    ryu_specials_init_main(fighter)
}

#[status_script(agent = "ken", status = FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn ken_specials_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_specials_main(fighter);
    0.into()
}

pub fn install() {
    install_status_scripts!(
        ken_specials_init,
        ken_specials,
        ken_specials_command_init,
        ken_specials_command
    );
}