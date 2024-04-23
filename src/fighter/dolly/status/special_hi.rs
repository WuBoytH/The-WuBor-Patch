use crate::imports::*;

unsafe extern "C" fn dolly_special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, dolly::instance::flag::SPECIAL_CANCEL) {
        VarModule::on_flag(fighter.module_accessor, dolly::status::flag::IS_SPECIAL_CANCEL);
        VarModule::off_flag(fighter.module_accessor, dolly::instance::flag::SPECIAL_CANCEL);
    }
    let status = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    let original = original_status(Main, fighter, status);
    original(fighter)
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

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, dolly_special_hi_main);
    agent.status(Main, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND, dolly_special_hi_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, dolly_special_hi_end);
    agent.status(End, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND, dolly_special_hi_command_end);

    agent.status(End, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_JUMP, dolly_special_hi_jump_end);
}