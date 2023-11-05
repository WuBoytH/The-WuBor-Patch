use {
    crate::imports::status_imports::*,
    super::super::helper::*
};

#[status_script(agent = "ryu", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe extern "C" fn ryu_specialhi(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_specialhi_main(fighter);
    0.into()
}

#[status_script(agent = "ryu", status = FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe extern "C" fn ryu_specialhi_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_specialhi_main(fighter);
    0.into()
}

pub fn install() {
    install_status_scripts!(
        ryu_specialhi,
        ryu_specialhi_command
    );
}