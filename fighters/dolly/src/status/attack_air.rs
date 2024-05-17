use super::*;
use super::super::helper::*;

unsafe extern "C" fn dolly_attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let aerial = ControlModule::get_attack_air_kind(fighter.module_accessor);
    let mot= match aerial {
        2 => Hash40::new("attack_air_f"),
        3 => Hash40::new("attack_air_b"),
        4 => Hash40::new("attack_air_hi"),
        5 => Hash40::new("attack_air_lw"),
        _ => Hash40::new("attack_air_n")
    };
    MotionModule::change_motion(
        fighter.module_accessor,
        mot,
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    WorkModule::set_int64(fighter.module_accessor, mot.hash as i64, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    fighter.sub_attack_air_common(false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(dolly_attack_air_main_loop as *const () as _))
}

unsafe extern "C" fn dolly_attack_air_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !CancelModule::is_enable_cancel(fighter.module_accessor) {
        if dolly_hit_cancel(fighter).get_i32() == 1 {
            return 1.into();
        }
    }
    if dolly_attack_start_cancel(fighter).get_i32() == 1 {
        return 1.into();
    }
    let frame = fighter.global_table[STATUS_FRAME].get_f32();
    let attack_cancel_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_private"), hash40("attack_start_cancel_frame")) as f32;
    if frame == attack_cancel_frame {
        let mot = MotionModule::motion_kind(fighter.module_accessor);
        if mot == hash40("attack_air_f") {
            notify_event_msc_cmd!(
                fighter,
                Hash40::new_raw(0x2b94de0d96),
                FIGHTER_LOG_ACTION_CATEGORY_ATTACK,
                FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_F
            );
        }
        else if mot == hash40("attack_air_b") {
            notify_event_msc_cmd!(
                fighter,
                Hash40::new_raw(0x2b94de0d96),
                FIGHTER_LOG_ACTION_CATEGORY_ATTACK,
                FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_B
            );
        }
        else if mot == hash40("attack_air_hi") {
            notify_event_msc_cmd!(
                fighter,
                Hash40::new_raw(0x2b94de0d96),
                FIGHTER_LOG_ACTION_CATEGORY_ATTACK,
                FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_HI
            );
        }
        else if mot == hash40("attack_air_lw") {
            notify_event_msc_cmd!(
                fighter,
                Hash40::new_raw(0x2b94de0d96),
                FIGHTER_LOG_ACTION_CATEGORY_ATTACK,
                FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_LW
            );
        }
        else {
            notify_event_msc_cmd!(
                fighter,
                Hash40::new_raw(0x2b94de0d96),
                FIGHTER_LOG_ACTION_CATEGORY_ATTACK,
                FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_N
            );
        }
    }
    fighter.status_AttackAir_Main();
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, dolly_attack_air_main);
}