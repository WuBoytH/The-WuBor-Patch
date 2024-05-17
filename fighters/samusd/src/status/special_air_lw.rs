use super::*;
use super::super::vl;

unsafe extern "C" fn samusd_special_air_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    samusd_speciallw_helper(fighter);
    samusd_special_air_lw_mot_helper(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(samusd_special_air_lw_main_loop as *const () as _))
}

unsafe extern "C" fn samusd_speciallw_helper(fighter: &mut L2CFighterCommon) {
    let body = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_LW_BODY);
    if body != 1 {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x298600da7a));
    }
    else {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2993ae42dd));
    }
}

unsafe extern "C" fn samusd_special_air_lw_mot_helper(fighter: &mut L2CFighterCommon) {
    let mot = Hash40::new("special_air_lw");
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MOT_RESTART) {
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
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MOT_RESTART);
    }
    else {
        MotionModule::change_motion_inherit_frame(
            fighter.module_accessor,
            mot,
            -1.0,
            1.0,
            0.0,
            false,
            false
        );
    }
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
}

unsafe extern "C" fn samusd_special_air_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if samusd_special_air_lw_is_end_helper(fighter).get_i32() == 1 {
        return 1.into();
    }
    if fighter.global_table[STATUS_FRAME].get_f32() >= 1.0 {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            if AttackModule::is_attack(fighter.module_accessor, 0, false) {
                WorkModule::set_float(fighter.module_accessor, vl::param_special_lw::landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
                return 1.into();
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
                return 1.into();
            }
        }
    }
    0.into()
}

unsafe extern "C" fn samusd_special_air_lw_is_end_helper(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        let sit = fighter.global_table[SITUATION_KIND].get_i32();
        if sit == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, samusd_special_air_lw_main);
}