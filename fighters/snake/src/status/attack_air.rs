use super::*;

unsafe extern "C" fn snake_attack_air_check_attack(fighter: &mut L2CFighterCommon, _param_1: &L2CValue, _param_2: &L2CValue) -> L2CValue {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_lw") {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("attack_air_lw2"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_AIR, snake_attack_air_check_attack);
}