use crate::imports::status_imports::*;
use super::helper::*;

#[status_script(agent = "luigi", status = FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe extern "C" fn luigi_special_s_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    luigi_remove_thunderhand_eff(fighter);
    0.into()
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::End, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END, luigi_special_s_end_end);
}