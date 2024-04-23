use crate::imports::*;

unsafe extern "C" fn rockman_ladder_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_LEAFSHIELD) {
        ControlModule::set_attack_air_kind(fighter.module_accessor, *FIGHTER_COMMAND_ATTACK_AIR_KIND_N);
    }
    fighter.status_LadderAttack_common();
    let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    if [
        hash40("attack_air_hi"), hash40("attack_air_lw")
    ].contains(&mot) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_SHOOT_NUM);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_LadderAttack_Main as *const () as _))
}

unsafe extern "C" fn rockman_ladder_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_LadderAttack()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_LADDER_ATTACK, rockman_ladder_attack_main);
    agent.status(End, *FIGHTER_STATUS_KIND_LADDER_ATTACK, rockman_ladder_attack_end);
}