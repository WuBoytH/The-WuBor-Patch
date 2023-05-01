use crate::imports::status_imports::*;

#[skyline::hook(replace = L2CFighterCommon_status_AttackDash)]
unsafe fn status_attackdash(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::set_float(fighter.battle_object, attack_dash::float::FALL_SPEED_Y_MUL, -1.0);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("attack_dash"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
    let catch_dash_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("catch_dash_frame"));
    WorkModule::set_int(fighter.module_accessor, catch_dash_frame, *FIGHTER_STATUS_ATTACK_DASH_WORK_INT_CATCH_FRAME);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_DASH_FLAG_ATTACK_HI4_DISABLE);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
        let jump_mini_attack_enable_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("jump_mini_attack_enable_frame"));
        WorkModule::set_int(fighter.module_accessor, jump_mini_attack_enable_frame + 1, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    }
    let log = fighter.status_attack()["log_infos"]["attack_dash"].get_int();
    WorkModule::set_int64(fighter.module_accessor, log as i64, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK)
    && WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND) > 0 {
        let log = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, log);
        WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_attack_dash_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_sub_attack_dash_uniq as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_AttackDash_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_status_AttackDash_Main)]
unsafe fn status_attackdash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool()
    || fighter.sub_air_check_fall_common().get_bool() {
        return 0.into();
    }
    if CustomCancelManager::execute_cancel(fighter) {
        return 1.into();
    }
    // <WuBor>
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    // This block is to force a ground correct type depending on if you enable sliding off or not.
    if situation == *SITUATION_KIND_GROUND {
        if VarModule::is_flag(fighter.battle_object, attack_dash::flag::ENABLE_AIR_FALL) {
            if GroundModule::get_correct(fighter.module_accessor) != *GROUND_CORRECT_KIND_GROUND {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            }
        }
        else {
            if GroundModule::get_correct(fighter.module_accessor) != *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            }
        }
    }
    // This block checks if you want to enable gravity. This code will only run once, and only while in the air,
    // but it enables another flag that will be checked when your situation changes and renable gravity there.
    if VarModule::is_flag(fighter.battle_object, attack_dash::flag::ENABLE_GRAVITY)
    && situation != *SITUATION_KIND_GROUND {
        VarModule::off_flag(fighter.battle_object, attack_dash::flag::ENABLE_GRAVITY);
        VarModule::on_flag(fighter.battle_object, attack_dash::flag::GRAVITY_ENABLED);
        let fall_mul = VarModule::get_float(fighter.battle_object, attack_dash::float::FALL_SPEED_Y_MUL);
        if fall_mul.signum() != -1.0 {
            let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                -air_accel_y * fall_mul
            );
        }
    }
    // This is to check if you want a Dash Attack to either slide off or continue off of ledges.
    // This is all behind an `is_situation_changed` check because we don't want this code running all of the time.
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            // Checks if your dash attack should roll off or not.
            if VarModule::is_flag(fighter.battle_object, attack_dash::flag::ENABLE_AIR_CONTINUE) {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                // Enables gravity.
                sv_kinetic_energy!(
                    reset_energy,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                    ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                );
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                let fall_mul = VarModule::get_float(fighter.battle_object, attack_dash::float::FALL_SPEED_Y_MUL);
                if fall_mul.signum() != -1.0 {
                    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
                    sv_kinetic_energy!(
                        set_accel,
                        fighter,
                        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                        -air_accel_y * fall_mul
                    );
                }
                // Checks if you have an attack_air_dash motion_kind in your motion_list. If so, change to it.
                if MotionModule::is_anim_resource(fighter.module_accessor, Hash40::new("attack_air_dash")) {
                    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                    MotionModule::change_motion_inherit_frame(
                        fighter.module_accessor,
                        Hash40::new("attack_air_dash"),
                        -1.0,
                        1.0,
                        0.0,
                        false,
                        false
                    );
                }
                // The previously mentioned flags that's only checked when your situation changes.
                if VarModule::is_flag(fighter.battle_object, attack_dash::flag::GRAVITY_ENABLED) {
                    let fall_mul = VarModule::get_float(fighter.battle_object, attack_dash::float::FALL_SPEED_Y_MUL);
                    if fall_mul.signum() != -1.0 {
                        let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
                        sv_kinetic_energy!(
                            set_accel,
                            fighter,
                            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                            -air_accel_y * fall_mul
                        );
                    }
                }
            }
            else {
                fighter.change_status(
                    L2CValue::I32(*FIGHTER_STATUS_KIND_FALL),
                    L2CValue::Bool(false)
                );
                return 0.into();
            }
        }
        else {
            // Checks if ENABLE_AIR_LANDING is enabled, and if it is, cancel into LANDING instead of continuing the dash attack.
            if VarModule::is_flag(fighter.battle_object, attack_dash::flag::ENABLE_AIR_LANDING) {
                fighter.change_status(
                    FIGHTER_STATUS_KIND_LANDING.into(),
                    false.into()
                );
                return 0.into();
            }
            else {
                // Changes motion back to attack_dash, just in case you've changed to attack_air_dash at some point
                MotionModule::change_motion_inherit_frame(
                    fighter.module_accessor,
                    Hash40::new("attack_dash"),
                    -1.0,
                    1.0,
                    0.0,
                    false,
                    false
                );
                KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            }
        }
    }
    // </WuBor>
    let jump_attack_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    if 0 < jump_attack_frame {
        if !StopModule::is_stop(fighter.module_accessor)
        && fighter.sub_check_button_jump().get_bool() {
            let log = fighter.status_attack();
            let info = log[0x10f40d7b92u64].get_i64();
            let mot = MotionModule::motion_kind(fighter.module_accessor);
            MotionAnimcmdModule::call_script_single(
                fighter.module_accessor,
                *FIGHTER_ANIMCMD_EXPRESSION,
                Hash40::new_raw(mot),
                -1
            );
            WorkModule::set_int64(fighter.module_accessor, info, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            fighter.change_status_jump_mini_attack(true.into());
            return 1.into();
        }
    }
    if 1 == jump_attack_frame {
        if !fighter.global_table[IS_STOP].get_bool()
        && WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND) > 0 {
            let log = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, log);
            WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN)
    && {
        let stick_x = fighter.global_table[STICK_X].get_f32();
        let lr = PostureModule::lr(fighter.module_accessor);
        let turn_run_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_run_stick_x"));
        stick_x * lr <= turn_run_stick_x
    }
    && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    && !ItemModule::is_have_item(fighter.module_accessor, 0) {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_TURN.into(), true.into());
        return 0.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH)
    && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    && !ItemModule::is_have_item(fighter.module_accessor, 0) {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_DASH.into(), true.into());
        return 0.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_FALL
        }
        else if WorkModule::get_param_int(fighter.module_accessor, 0x17e10662a4, 0) == *FIGHTER_ATTACK_DASH_TYPE_SQUAT_WAIT {
            FIGHTER_STATUS_KIND_SQUAT_WAIT
        }
        else {
            FIGHTER_STATUS_KIND_WAIT
        };
        fighter.change_status(status.into(), false.into());
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_attackdash,
            status_attackdash_main
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}