use super::*;

#[no_mangle]
unsafe extern "C" fn look_up_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_Squat()
}

#[no_mangle]
unsafe extern "C" fn look_up_main_common(fighter: &mut L2CFighterCommon, start_status: L2CValue, wait_status: L2CValue) {
    fighter.status_Squat_sub_param(hash40("look_up").into(), start_status, wait_status);
}

#[no_mangle]
unsafe extern "C" fn look_up_main_loop_common(fighter: &mut L2CFighterCommon, wait_status: L2CValue) -> L2CValue {
    if fighter.sub_fall().get_bool() {
        return 1.into();
    }

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(wait_status, false.into());
            return 0.into();
        }
    }

    if fighter.sub_squat_common_Main().get_bool() {
        return 0.into();
    }

    0.into()
}
