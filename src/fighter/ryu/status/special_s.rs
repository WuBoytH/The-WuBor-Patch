use {
    crate::imports::status_imports::*,
    super::super::helper::*
};

#[status_script(agent = "ryu", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn ryu_specials_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_specials_init_main(fighter)
}

#[status_script(agent = "ryu", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn ryu_specials(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_specials_main(fighter);
    0.into()
}

#[status_script(agent = "ryu", status = FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn ryu_specials_command_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND);
    ryu_specials_init_main(fighter)
}

#[status_script(agent = "ryu", status = FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn ryu_specials_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_specials_main(fighter);
    0.into()
}

pub fn install() {
    install_status_scripts!(
        ryu_specials_init,
        ryu_specials,
        ryu_specials_command_init,
        ryu_specials_command
    );
}