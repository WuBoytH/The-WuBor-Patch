use crate::imports::status_imports::*;
use crate::fighter::belmont::status::special_n::*;

#[status_script(agent = "simon", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe extern "C" fn simon_special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    belmont_special_n_main_inner(fighter)
}

#[status_script(agent = "simon", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe extern "C" fn simon_special_n_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    belmont_special_n_end_inner(fighter)
}

pub fn install() {
    install_status_scripts!(
        simon_special_n_main,
        simon_special_n_end
    );
}