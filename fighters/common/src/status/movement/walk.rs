use super::*;

#[no_mangle]
unsafe extern "C" fn fgc_walk_back_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Walk_Common();

    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_walk_uniq_check();
    }
    fighter.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(L2CFighterCommon_sub_walk_uniq_check as *const () as _));

    fighter.sub_shift_status_main(L2CValue::Ptr(fgc_walk_back_main_loop as *const () as _))
}

#[no_mangle]
unsafe extern "C" fn fgc_walk_back_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_x = fighter.global_table[STICK_X].get_f32();
    fighter.status_Walk_Main_common(
        FIGHTER_RYU_STATUS_KIND_WALK_BRAKE_BACK.into(),
        (stick_x * -lr).into(),
        false.into(),
        L2CValue::Ptr(fgc_walk_back_main_loop_common_thing_help as *const () as _)
    );

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        let frame = fighter.global_table[STATUS_FRAME].get_f32();
        let turn_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_frame"));
        if frame != 0.0 && frame <= turn_frame {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH) {
                let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
                let dash_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("dash_stick_x"));
                if stick_x * -lr > dash_stick_x {
                    let flick_x = ControlModule::get_flick_x(fighter.module_accessor) & 0xff;
                    let dash_flick_x = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("dash_flick_x"));
                    if flick_x < dash_flick_x {
                        let lr_one_on_one = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
                        if lr_one_on_one != 0.0 {
                            if lr == lr_one_on_one {
                                fighter.change_status(FIGHTER_RYU_STATUS_KIND_DASH_BACK.into(), false.into());
                                return 0.into();
                            }
                        }
                    }
                }
            }
        }
    }

    0.into()
}

unsafe extern "C" fn fgc_walk_back_main_loop_common_thing_help(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr_one_on_one = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    if lr_one_on_one != 0.0 {
        if PostureModule::lr(fighter.module_accessor) != lr_one_on_one {
            fighter.change_status(FIGHTER_RYU_STATUS_KIND_TURN_AUTO.into(), false.into());
            return true.into();
        }
    }

    false.into()
}
