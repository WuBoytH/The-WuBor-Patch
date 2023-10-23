use crate::imports::status_imports::*;

#[status_script(agent = "brave", status = FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_SELECT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn brave_special_lw_select_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let index = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_SELECT_INDEX);
    VarModule::set_int(fighter.battle_object, brave::instance::int::NEXT_ROLL_INDEX, index);
    original!(fighter)
}

pub fn install() {
    install_status_scripts!(
        brave_special_lw_select_end
    );
}