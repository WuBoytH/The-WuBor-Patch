use {
    crate::imports::status_imports::*,
    super::super::helper::*
};

#[status_script(agent = "ryu", status = FIGHTER_STATUS_KIND_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn ryu_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_attack_main_inner(fighter)
}

pub fn install() {
    install_status_scripts!(
        ryu_attack_main
    );
}