use crate::imports::status_imports::*;
use super::helper::*;

#[status_script(agent = "luigi", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe extern "C" fn luigi_specials_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    luigi_remove_thunderhand_eff(fighter);
    0.into()
}

pub fn install() {
    install_status_scripts!(
        luigi_specials_end
    );
}