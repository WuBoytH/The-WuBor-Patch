use crate::imports::*;
use super::super::helper::*;

unsafe extern "C" fn demon_attack_stand_2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !demon_attack_common(fighter).get_bool() {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_STAND_2_FLAG_CHECK_STEP);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_STAND_2_FLAG_INC_STEP);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_DEMON_STATUS_ATTACK_STAND_2_WORK_INT_COMBO);
        let mot = demon_attack_stand_2_get_mot(fighter).get_u64();
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new_raw(mot),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        notify_event_msc_cmd!(
            fighter,
            Hash40::new_raw(0x2b94de0d96),
            FIGHTER_LOG_ACTION_CATEGORY_ATTACK,
            FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_02
        );
        MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, false);
        fighter.sub_shift_status_main(L2CValue::Ptr(demon_attack_stand_2_main_loop as *const () as _))
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn demon_attack_stand_2_get_mot(fighter: &mut L2CFighterCommon) -> L2CValue {
    let combo = WorkModule::get_int(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_STAND_2_WORK_INT_COMBO);
    match combo {
        0 => hash40("attack_stand_21"),
        1 => {
            if VarModule::is_flag(fighter.module_accessor, demon::status::flag::ATTACK_STAND_2_SPECIAL_FINISHER) {
                hash40("attack_stand_2f")
            }
            else {
                hash40("attack_stand_22")
            }
        },
        2 => {
            if VarModule::is_flag(fighter.module_accessor, demon::status::flag::ATTACK_STAND_2_SPECIAL_FINISHER) {
                hash40("attack_stand_24")
            }
            else {
                hash40("attack_stand_23")
            }
        },
        _ => hash40("attack_stand_24")
    }.into()
}

unsafe extern "C" fn demon_attack_stand_2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if demon_attack_loop_common(fighter, FIGHTER_STATUS_KIND_WAIT.into()).get_bool() {
        return 0.into();
    }
    let combo = WorkModule::get_int(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_STAND_2_WORK_INT_COMBO);
    if combo < 3
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
    && !StatusModule::is_changing(fighter.module_accessor) {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            VarModule::on_flag(fighter.module_accessor, demon::status::flag::ATTACK_STAND_2_SPECIAL_FINISHER);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_STAND_2_FLAG_INC_STEP);
        }
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_STAND_2_FLAG_INC_STEP);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_STAND_2_FLAG_CHECK_STEP) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_STAND_2_FLAG_CHECK_STEP);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_STAND_2_FLAG_INC_STEP) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_STAND_2_FLAG_INC_STEP);
            WorkModule::inc_int(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_STAND_2_WORK_INT_COMBO);
            let mot = demon_attack_stand_2_get_mot(fighter).get_u64();
            let rate = MotionModule::rate(fighter.module_accessor);
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new_raw(mot),
                0.0,
                rate,
                false,
                0.0,
                false,
                false
            );
        }
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_2, demon_attack_stand_2_main);
}