use crate::imports::status_imports::*;

unsafe extern "C" fn tantan_fall_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_pre_fall().get_bool() {
        return 1.into();
    }
    let jump_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    let jump_count_max = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
    if jump_count_max <= jump_count {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_AERIAL);
        return 1.into();
    }

    let original = smashline::original_status(smashline::Pre, fighter, *FIGHTER_STATUS_KIND_FALL);
    original(fighter)
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Pre, *FIGHTER_STATUS_KIND_FALL, tantan_fall_pre);
}