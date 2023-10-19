use crate::imports::status_imports::*;

#[status_script(agent = "ryu", status = FIGHTER_STATUS_KIND_ATTACK_LW3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn ryu_attack_lw3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, ryu::instance::flag::DENJIN_RUSH_INHERIT) {
        VarModule::on_flag(fighter.battle_object, ryu::status::flag::USED_DENJIN_CHARGE);
        VarModule::off_flag(fighter.battle_object, ryu::instance::flag::DENJIN_RUSH_INHERIT);
    }
    original!(fighter)
}

pub fn install() {
    install_status_scripts!(
        ryu_attack_lw3_main
    );
}