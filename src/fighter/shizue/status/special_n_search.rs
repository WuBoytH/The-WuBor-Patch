use crate::imports::status_imports::*;
use crate::fighter::murabito::helper::*;

#[status("shizue", FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH)]
unsafe fn shizue_special_n_search_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_pocket_set_flag(fighter)
}

pub fn install() {
    shizue_special_n_search_end::install();
}