use crate::imports::status_imports::*;
use super::super::helper::*;

unsafe extern "C" fn dolly_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_AttackCommon();
    if !StopModule::is_stop(fighter.module_accessor) {
        dolly_attack_substatus3(fighter);
    }
    fighter.global_table[SUB_STATUS3].assign(&L2CValue::Ptr(dolly_attack_substatus3 as *const () as _));
    fighter.sub_status_AttackComboCommon();
    fighter.sub_shift_status_main(L2CValue::Ptr(dolly_attack_main_loop as *const () as _))
}

unsafe extern "C" fn dolly_attack_substatus3(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
        let combo = ComboModule::count(fighter.module_accessor) as i32;
        let attack_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_combo_max"), 0);
        if combo >= attack_combo_max {
            return 0.into()
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO) {
            return 0.into();
        }
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
    }
    else {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
    }
    fighter.attack_mtrans();
    0.into()
}

unsafe extern "C" fn dolly_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if dolly_hit_cancel(fighter).get_i32() == 0 {
        let combo = ComboModule::count(fighter.module_accessor);
        if combo < 3
        && !CancelModule::is_enable_cancel(fighter.module_accessor)
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL) {
            let stick_dir = ControlModule::get_stick_dir(fighter.module_accessor);
            let attack_s3_stick_dir_hi = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_s3_stick_dir_hi"));
            if (fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 == 0
            && FGCModule::cancel_exceptions(
                fighter,
                *FIGHTER_STATUS_KIND_ATTACK_HI3,
                *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3,
                true
            ).get_bool())
            || (fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 == 0
            && attack_s3_stick_dir_hi < stick_dir
            && FGCModule::cancel_exceptions(
                fighter,
                *FIGHTER_STATUS_KIND_ATTACK_S3,
                *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3,
                true
            ).get_bool()) {
                VarModule::on_flag(fighter.module_accessor, dolly::status::flag::IS_SPECIAL_CANCEL);
                return 1.into();
            }
        }
        if combo == 1
        && dolly_attack_start_cancel(fighter).get_i32() == 1 {
            return 1.into();
        }
        fighter.status_Attack_Main();
        0.into()
    }
    else {
        1.into()
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_ATTACK, dolly_attack_main);
}