use crate::imports::status_imports::*;

#[status("dolly", FIGHTER_STATUS_KIND_SPECIAL_HI)]
unsafe fn dolly_specialhi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_cancel = VarModule::is_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL);
    let ret = original(fighter);
    VarModule::set_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL, is_cancel);
    ret
}

#[status("dolly", FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND)]
unsafe fn dolly_specialhicommand_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_cancel = VarModule::is_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL);
    let ret = original(fighter);
    VarModule::set_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL, is_cancel);
    ret
}

#[status("dolly", FIGHTER_STATUS_KIND_SPECIAL_HI)]
unsafe fn dolly_specialhi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    dolly_specialhi_end_main(fighter)
}

#[status("dolly", FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND)]
unsafe fn dolly_specialhi_command_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_JUMP {
        if VarModule::is_flag(fighter.battle_object, dolly::instance::flag::RISING_FORCE) {
            EffectModule::clear_screen(fighter.module_accessor, 1, 0x14);
        }
        VarModule::off_flag(fighter.battle_object, dolly::instance::flag::RISING_FORCE);
    }
    dolly_specialhi_end_main(fighter)
}

unsafe extern "C" fn dolly_specialhi_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_JUMP
    && status != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_LANDING {
        VarModule::off_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL);
        MotionAnimcmdModule::flush_current_motion(fighter.module_accessor);
        ItemModule::set_change_status_event(fighter.module_accessor, true);
    }
    0.into()
}

#[status("dolly", FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_JUMP)]
unsafe fn dolly_specialhi_jump_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_JUMP
    && status != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_FALL
    && status != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_LANDING {
        MotionAnimcmdModule::flush_current_motion(fighter.module_accessor);
    }
    VarModule::off_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL);
    ItemModule::set_change_status_event(fighter.module_accessor, true);
    if VarModule::is_flag(fighter.battle_object, dolly::instance::flag::RISING_FORCE) {
        EffectModule::clear_screen(fighter.module_accessor, 1, 0x14);
    }
    VarModule::off_flag(fighter.battle_object, dolly::instance::flag::RISING_FORCE);
    0.into()
}

pub fn install() {
    dolly_specialhi_pre::install();
    dolly_specialhicommand_pre::install();
    dolly_specialhi_end::install();
    dolly_specialhi_command_end::install();
    dolly_specialhi_jump_end::install();
}