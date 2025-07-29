use super::*;

unsafe extern "C" fn eflame_attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_HAS_ESWORD) {
        let attack_air_kind = ControlModule::get_attack_air_kind(fighter.module_accessor);
        if ![
            *FIGHTER_COMMAND_ATTACK_AIR_KIND_N,
            *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW,
        ].contains(&attack_air_kind) {
            ControlModule::set_attack_air_kind(fighter.module_accessor, *FIGHTER_COMMAND_ATTACK_AIR_KIND_N);
        }
    }
    fighter.status_AttackAir()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, eflame_attack_air_main);
}