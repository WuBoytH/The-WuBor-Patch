use crate::imports::status_imports::*;
use super::helper::*;

#[status_script(agent = "luigi", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe extern "C" fn luigi_special_s_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    luigi_remove_thunderhand_eff(fighter);
    0.into()
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::End, *FIGHTER_STATUS_KIND_SPECIAL_S, luigi_special_s_end);
}