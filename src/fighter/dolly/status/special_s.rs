use crate::imports::status_imports::*;

#[status("dolly", FIGHTER_STATUS_KIND_SPECIAL_S)]
unsafe fn dolly_specials_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_cancel = VarModule::is_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL);
    let ret = original(fighter);
    VarModule::set_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL, is_cancel);
    ret
}

#[status("dolly", FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND)]
unsafe fn dolly_specialscommand_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_cancel = VarModule::is_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL);
    let ret = original(fighter);
    VarModule::set_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL, is_cancel);
    ret
}

#[status("dolly", FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B)]
unsafe fn dolly_specialb_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_cancel = VarModule::is_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL);
    let ret = original(fighter);
    VarModule::set_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL, is_cancel);
    ret
}

#[status("dolly", FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND)]
unsafe fn dolly_specialbcommand_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_cancel = VarModule::is_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL);
    let ret = original(fighter);
    VarModule::set_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL, is_cancel);
    ret
}

#[status("dolly", FIGHTER_STATUS_KIND_SPECIAL_S)]
unsafe fn dolly_specials_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    dolly_specials_end_main(fighter)
}

#[status("dolly", FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND)]
unsafe fn dolly_specials_command_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    dolly_specials_end_main(fighter)
}

#[status("dolly", FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK)]
unsafe fn dolly_specialf_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL);
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END {
        WorkModule::set_customize_no(fighter.module_accessor, 0, 1);
    }
    0.into()
}

#[status("dolly", FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B)]
unsafe fn dolly_specialb_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    dolly_specials_end_main(fighter)
}

#[status("dolly", FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND)]
unsafe fn dolly_specialb_command_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    dolly_specials_end_main(fighter)
}

#[status("dolly", FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_ATTACK)]
unsafe fn dolly_specialb_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_LANDING {
        VarModule::off_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL);
        WorkModule::set_customize_no(fighter.module_accessor, 0, 1);
    }
    0.into()
}

unsafe extern "C" fn dolly_specials_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK
    && status != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_ATTACK {
        VarModule::off_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL);
        WorkModule::set_customize_no(fighter.module_accessor, 0, 1);
    }
    0.into()
}

pub fn install() {
    dolly_specials_pre::install();
    dolly_specialscommand_pre::install();
    dolly_specialb_pre::install();
    dolly_specialbcommand_pre::install();
    dolly_specials_end::install();
    dolly_specials_command_end::install();
    dolly_specialf_attack_end::install();
    dolly_specialb_end::install();
    dolly_specialb_command_end::install();
    dolly_specialb_attack_end::install();
}