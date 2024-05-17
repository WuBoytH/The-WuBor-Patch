use super::*;
use super::helper::*;

unsafe extern "C" fn luigi_special_s_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    luigi_remove_thunderhand_eff(fighter);
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, luigi_special_s_end);
}