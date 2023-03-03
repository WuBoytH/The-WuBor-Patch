use crate::imports::status_imports::*;
use crate::fighter::murabito::helper::*;

#[status_script(agent = "shizue", status = FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn shizue_special_n_search_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_pocket_set_flag(fighter)
}

pub fn install() {
    install_status_scripts!(
        shizue_special_n_search_end
    );
}