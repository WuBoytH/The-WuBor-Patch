use super::*;
use super::super::helper::*;

unsafe extern "C" fn murabito_special_n_pocket_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_pocket_set_flag(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH, murabito_special_n_pocket_end);
}