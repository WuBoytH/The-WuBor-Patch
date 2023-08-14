use crate::imports::status_imports::*;
use super::super::helper::*;

#[status("dolly", FIGHTER_STATUS_KIND_ATTACK_HI3)]
unsafe fn dolly_attackhi3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
    fighter.clear_lua_stack();
    let mut mot = sv_fighter_util::get_attack_hi3_motion(fighter.lua_state_agent);
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_ESCAPE
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ESCAPE_ATTACK) {
        mot = Hash40::new("escape_attack");
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_16 - 1);
    }
    fighter.status_AttackHi3_Common(mot.into(), mot.into());
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_attack3_uniq_check(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_sub_attack3_uniq_check as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(dolly_attackhi3_main_loop as *const () as _))
}

unsafe extern "C" fn dolly_attackhi3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if dolly_hit_cancel(fighter).get_i32() == 0 {
        if fighter.global_table[PREV_STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_ESCAPE
        && !VarModule::is_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL) {
            if dolly_attack_start_cancel(fighter).get_i32() == 1 {
                return 1.into();
            }
        }
        else {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ESCAPE_ATTACK) {
                if dolly_attack_start_cancel(fighter).get_i32() == 1 {
                    return 1.into();
                }
            }
        }
        fighter.status_AttackHi3_Main();
        0.into()
    }
    else {
        1.into()
    }
}

pub fn install() {
    dolly_attackhi3_main::install();
}