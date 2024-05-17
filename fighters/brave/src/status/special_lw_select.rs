use super::*;

unsafe extern "C" fn brave_special_lw_select_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let index = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_SELECT_INDEX);
    VarModule::set_int(fighter.module_accessor, vars::brave::instance::int::NEXT_ROLL_INDEX, index);
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status != *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START {
        let is_failure = status != *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_FAILURE;
        let fighta = fighter.global_table[FIGHTER].get_ptr() as *mut Fighter;
        FighterSpecializer_Brave::special_lw_close_window(fighta, false, true, is_failure);
        if status == *FIGHTER_BRAVE_STATUS_SPECIAL_LW_FLAG_AUTO_CANCEL {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_FLAG_AUTO_CANCEL) {
                SoundModule::play_se_no3d(fighter.module_accessor, Hash40::new("se_brave_special_l18_02"), false, false);
                let closing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), 0x2c50b3d6f0);
                WorkModule::set_int(fighter.module_accessor, closing_frame, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_WINDOW_CLOSING_FRAME);
            }
        }
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_DISABLE_SP_AUTO_RECOVER);
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_SELECT, brave_special_lw_select_end);
}