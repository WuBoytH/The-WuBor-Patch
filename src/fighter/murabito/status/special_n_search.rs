use crate::imports::*;
use super::super::helper::*;

unsafe extern "C" fn murabito_special_n_pocket_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_pocket_set_flag(fighter)
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::End, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH, murabito_special_n_pocket_end);
}