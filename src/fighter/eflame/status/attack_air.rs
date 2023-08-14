use crate::imports::status_imports::*;

#[status("eflame", FIGHTER_STATUS_KIND_ATTACK_AIR)]
unsafe fn eflame_attackair_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_HAS_ESWORD) {
        ControlModule::set_attack_air_kind(fighter.module_accessor, *FIGHTER_COMMAND_ATTACK_AIR_KIND_N);
    }
    fighter.status_AttackAir()
}

pub fn install() {
    eflame_attackair_main::install();
}