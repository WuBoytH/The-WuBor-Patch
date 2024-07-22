use super::*;

unsafe extern "C" fn sonic_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_jump_item_rocketbelt();
    let motion = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI)
    && fighter.global_table[STICK_X].get_f32() == 0.0 {
        Hash40::new("jump_mini")
    }
    else {
        Hash40::new("invalid")
    };
    fighter.status_Jump_sub(motion.into(), 0.0_f32.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_Jump_Main as *const () as _))
}

unsafe extern "C" fn sonic_jump_aerial_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < 1 {
        WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ROCKETBELT) {
        let energy = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_ROCKETBELT_BURNER_ENERGY_VALUE);
        if 0.0 < energy {
            ItemModule::set_attach_item_action(fighter.module_accessor, ItemKind(*ITEM_KIND_ROCKETBELT), *ITEM_ROCKETBELT_ACTION_JUMP_JET_FIRE, 1.0);
        }
    }
    fighter.status_JumpAerialSub(false.into(), false.into());
    // if [
    //     *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH,
    //     *FIGHTER_SONIC_STATUS_KIND_SPIN_JUMP
    // ].contains(&fighter.global_table[PREV_STATUS_KIND].get_i32()) {
    //     sonic_set_jumps(fighter);
    // }
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_JumpAerial_Main as *const () as _))
}

unsafe extern "C" fn sonic_screw_jump_aerial_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < 1 {
        WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    }
    fighter.status_ItemScrewJumpAerialSub();
    // if [
    //     *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH,
    //     *FIGHTER_SONIC_STATUS_KIND_SPIN_JUMP
    // ].contains(&fighter.global_table[PREV_STATUS_KIND].get_i32()) {
    //     sonic_set_jumps(fighter);
    // }
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_ItemScrewJumpAerial_Main as *const () as _))
}

// unsafe extern "C" fn sonic_set_jumps(fighter: &mut L2CFighterCommon) {
//     let jump_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
//     let jump_count_max = WorkModule::get_param_int(fighter.module_accessor, hash40("jump_count_max"), 0);
//     if jump_count < jump_count_max {
//         WorkModule::set_int(fighter.module_accessor, jump_count_max, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
//     }
//     let jump_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SCREW_JUMP_COUNT);
//     if jump_count < *FIGHTER_STATUS_SCREW_JUMP_COUNT_MAX {
//         WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_SCREW_JUMP_COUNT_MAX, *FIGHTER_INSTANCE_WORK_ID_INT_SCREW_JUMP_COUNT);
//     }
// }

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_JUMP, sonic_jump_main);

    agent.status(Main, *FIGHTER_STATUS_KIND_JUMP_AERIAL, sonic_jump_aerial_main);

    agent.status(Main, *FIGHTER_STATUS_KIND_ITEM_SCREW_JUMP_AERIAL, sonic_screw_jump_aerial_main);
}