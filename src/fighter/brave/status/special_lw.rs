use crate::imports::status_imports::*;

unsafe extern "C" fn brave_special_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let index = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_SELECT_INDEX);
    VarModule::set_int(fighter.module_accessor, brave::instance::int::NEXT_ROLL_INDEX, index);
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_SELECT {
        let fighta = fighter.global_table[FIGHTER].get_ptr() as *mut Fighter;
        FighterSpecializer_Brave::special_lw_close_window(fighta, false, true, false);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_DISABLE_SP_AUTO_RECOVER);
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::End, *FIGHTER_STATUS_KIND_SPECIAL_LW, brave_special_lw_end);
}