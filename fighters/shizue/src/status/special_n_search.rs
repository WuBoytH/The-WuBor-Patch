use super::*;
use crate::fighter::murabito::helper::*;

unsafe extern "C" fn shizue_special_n_search_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_pocket_set_flag(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH, shizue_special_n_search_end);
}