use crate::imports::status_imports::*;

#[status_script(agent = "ryu", status = FIGHTER_STATUS_KIND_REBIRTH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn ryu_rebirth_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, ryu::instance::flag::DENJIN_CHARGE);
    VarModule::set_int(fighter.battle_object, ryu::instance::int::DENJIN_EFF_HANDLE, 0);
    fighter.status_pre_Rebirth()
}

pub fn install() {
    install_status_scripts!(
        ryu_rebirth_pre
    );
}