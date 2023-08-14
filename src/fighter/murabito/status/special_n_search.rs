use crate::imports::status_imports::*;
use super::super::helper::*;

#[status("murabito", FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH)]
unsafe fn murabito_special_n_pocket_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_pocket_set_flag(fighter)
}

pub fn install() {
    murabito_special_n_pocket_end::install();
}