use {
    crate::imports::status_imports::*,
    super::super::helper::*
};

#[status_script(agent = "ryu", status = FIGHTER_STATUS_KIND_ATTACK_LW4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn ryu_attack_lw4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_attack_reset(fighter);
    fighter.status_AttackLw4()
}

pub fn install() {
    install_status_scripts!(
        ryu_attack_lw4_main
    );
}