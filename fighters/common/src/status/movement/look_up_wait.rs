use super::*;

#[no_mangle]
unsafe extern "C" fn look_up_wait_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_SquatWait()
}

#[no_mangle]
unsafe extern "C" fn look_up_wait_main_common(fighter: &mut L2CFighterCommon, start_status: L2CValue, wait_status: L2CValue) {
    look_up_wait_common(fighter, start_status, wait_status);
}

#[no_mangle]
unsafe extern "C" fn look_up_wait_common(fighter: &mut L2CFighterCommon, start_status: L2CValue, wait_status: L2CValue) {
    let cursor_wait_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("cursor_wait_frame"));
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), true, cursor_wait_frame);

    VarModule::set_int(fighter.module_accessor, vars::look_up::int::HOLD_COUNT, 120);

    fighter.sub_squat_common_param(start_status, wait_status);

    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT_RV);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK);

    if MotionModule::motion_kind(fighter.module_accessor) == hash40("look_up")
    && MotionModule::is_end(fighter.module_accessor) {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("look_up_wait"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }

    if !StopModule::is_stop(fighter.module_accessor) {
        look_up_wait_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(look_up_wait_substatus as *const () as _));
}

unsafe extern "C" fn look_up_wait_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        if VarModule::countdown_int(fighter.module_accessor, vars::look_up::int::HOLD_COUNT, 0) {
            VarModule::on_flag(fighter.module_accessor, vars::look_up::flag::SHIFT_CAMERA);
            CameraModule::add_camera_range_offset(fighter.module_accessor, &Vector3f{x: 0.0, y: 200.0, z: 0.0}, 0);
        }
    }
    0.into()
}

#[no_mangle]
unsafe extern "C" fn look_up_wait_main_loop_common(fighter: &mut L2CFighterCommon, rv_status: L2CValue) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }

    if fighter.sub_squat_common_Main().get_bool() {
        return 1.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT_RV) {
        let stick_y = fighter.global_table[STICK_Y].get_f32();
        let squat_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("squat_stick_y"));
        if stick_y < -squat_stick_y {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                fighter.change_status(rv_status, true.into());
                return 1.into();
            }
        }
    }

    if MotionModule::motion_kind(fighter.module_accessor) == hash40("look_up")
    && MotionModule::is_end(fighter.module_accessor) {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("look_up_wait"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }

    0.into()
}

#[no_mangle]
unsafe extern "C" fn look_up_wait_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionAnimcmdModule::flush(fighter.module_accessor, false);
    if VarModule::is_flag(fighter.module_accessor, vars::look_up::flag::SHIFT_CAMERA) {
        CameraModule::reset_camera_range(fighter.module_accessor, 0);
    }
    0.into()
}