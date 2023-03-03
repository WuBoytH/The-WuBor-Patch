use crate::imports::status_imports::*;

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_APPEAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_appeal_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL);
    fighter.status_end_Appeal();
    0.into()
}

pub fn install() {
    install_status_scripts!(
        dolly_appeal_end
    );
}