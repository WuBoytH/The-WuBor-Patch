use crate::imports::status_imports::*;

#[status_script(agent = "gekkouga", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn gekkouga_special_hi_init(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

#[status_script(agent = "gekkouga", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn gekkouga_special_hi_exec(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    install_status_scripts!(
        gekkouga_special_hi_init,
        gekkouga_special_hi_exec
    );
}