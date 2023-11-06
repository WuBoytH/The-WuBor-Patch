use crate::imports::status_imports::*;

unsafe extern "C" fn dolly_special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_cancel = VarModule::is_flag(fighter.module_accessor, dolly::status::flag::IS_SPECIAL_CANCEL);
    let mut mask = *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK;
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND {
        mask |= *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI_COMMAND;
    }
    else {
        mask |= *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI;
    }
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        mask as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    VarModule::set_flag(fighter.module_accessor, dolly::status::flag::IS_SPECIAL_CANCEL, is_cancel);
    0.into()
}

unsafe extern "C" fn dolly_special_hi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    dolly_special_hi_end_main(fighter)
}

unsafe extern "C" fn dolly_special_hi_command_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_JUMP {
        if VarModule::is_flag(fighter.module_accessor, dolly::instance::flag::RISING_FORCE) {
            EffectModule::clear_screen(fighter.module_accessor, 1, 0x14);
        }
        VarModule::off_flag(fighter.module_accessor, dolly::instance::flag::RISING_FORCE);
    }
    dolly_special_hi_end_main(fighter)
}

unsafe extern "C" fn dolly_special_hi_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_JUMP
    && status != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_LANDING {
        VarModule::off_flag(fighter.module_accessor, dolly::status::flag::IS_SPECIAL_CANCEL);
        MotionAnimcmdModule::flush_current_motion(fighter.module_accessor);
        ItemModule::set_change_status_event(fighter.module_accessor, true);
    }
    0.into()
}

unsafe extern "C" fn dolly_special_hi_jump_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_JUMP
    && status != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_FALL
    && status != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_LANDING {
        MotionAnimcmdModule::flush_current_motion(fighter.module_accessor);
    }
    VarModule::off_flag(fighter.module_accessor, dolly::status::flag::IS_SPECIAL_CANCEL);
    ItemModule::set_change_status_event(fighter.module_accessor, true);
    if VarModule::is_flag(fighter.module_accessor, dolly::instance::flag::RISING_FORCE) {
        EffectModule::clear_screen(fighter.module_accessor, 1, 0x14);
    }
    VarModule::off_flag(fighter.module_accessor, dolly::instance::flag::RISING_FORCE);
    0.into()
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, dolly_special_hi_pre);
    agent.status(smashline::Pre, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND, dolly_special_hi_pre);
    agent.status(smashline::End, *FIGHTER_STATUS_KIND_SPECIAL_HI, dolly_special_hi_end);
    agent.status(smashline::End, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND, dolly_special_hi_command_end);

    agent.status(smashline::End, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_JUMP, dolly_special_hi_jump_end);
}