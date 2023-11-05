use crate::imports::status_imports::*;
use super::super::helper::*;

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe extern "C" fn dolly_wait_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.status_pre_Wait_check_interrupt().get_i32() != 0 {
        return 1.into();
    }
    FighterSpecializer_Dolly::update_opponent_lr_1on1(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT);
    let opponent_lr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    if opponent_lr != 0.0
    && opponent_lr != PostureModule::lr(fighter.module_accessor) {
        let prev_status = fighter.global_table[PREV_STATUS_KIND].get_i32();
        if dolly_status_end_control_lr_status_check(prev_status.into()).get_bool() {
            if GroundModule::is_ottotto(fighter.module_accessor, 0.86)
            && fighter.global_table[STICK_X].get_f32().abs() < 0.75
            && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_OTTOTTO) {
                StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_RYU_STATUS_KIND_TURN_AUTO);
            }
        }
    }
    fighter.status_pre_Wait_main();
    0.into()
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Pre, *FIGHTER_STATUS_KIND_WAIT, dolly_wait_pre);
}