use {
    crate::imports::status_imports::*
};

#[status_script(agent = "ryu", status = FIGHTER_STATUS_KIND_ATTACK_HI3, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn ryu_attackhi3_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, fighter::status::flag::JUMP_CANCEL) {
        FGCModule::jump_cancel_check_hit(fighter, false);
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        ryu_attackhi3_exec
    );
}