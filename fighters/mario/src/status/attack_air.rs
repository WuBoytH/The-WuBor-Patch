use super::*;
use super::super::helper::*;

unsafe extern "C" fn attack_air_check_attack(
    fighter: &mut L2CFighterCommon,
    _param_1: &L2CValue,
    _param_2: &L2CValue
) -> L2CValue {
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND) == hash40("attack_air_lw") {
        SoundModule::play_se_no3d(
            fighter.module_accessor,
            Hash40::new("se_mario_attackair_l02"),
            false,
            false
        );
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
            let speed: f32 = if VarModule::is_flag(fighter.module_accessor, vars::mario::status::flag::ATTACK_AIR_LW_LAST) {
                0.9
            }
            else {
                0.75
            };
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                speed
            );
        }
    }
    0.into()
}
unsafe extern "C" fn mario_attack_air_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    mario_remove_hammer(fighter);
    0.into()
}

unsafe extern "C" fn mario_landing_attack_air_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_LandingAttackAir();
    mario_remove_hammer(fighter);
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_check_attack);
    agent.status(End, *FIGHTER_STATUS_KIND_ATTACK_AIR, mario_attack_air_end);

    agent.status(End, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, mario_landing_attack_air_end);
}