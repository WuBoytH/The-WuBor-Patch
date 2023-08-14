use crate::imports::status_imports::*;

#[status("brave", FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_SELECT)]
unsafe fn brave_special_lw_select_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let index = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_SELECT_INDEX);
    VarModule::set_int(fighter.battle_object, brave::instance::int::NEXT_ROLL_INDEX, index);
    original(fighter)
}

pub fn install() {
    brave_special_lw_select_end::install();
}