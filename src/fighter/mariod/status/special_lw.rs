use crate::imports::status_imports::*;

#[status_script(agent = "mariod", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn mariod_speciallw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    mariod_speciallw_mot_helper(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(mariod_speciallw_main_loop as *const () as _))
}

unsafe extern "C" fn mariod_speciallw_mot_helper(fighter: &mut L2CFighterCommon) {
    let mot;
    let kinetic;
    let correct;
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        mot = Hash40::new("special_air_lw");
        kinetic = *FIGHTER_KINETIC_TYPE_MARIOD_SPECIAL_AIR_LW;
        correct = *GROUND_CORRECT_KIND_AIR;
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_INT_MTRANS);
    }
    else {
        mot = Hash40::new("special_lw");
        kinetic = *FIGHTER_KINETIC_TYPE_MARIOD_SPECIAL_LW;
        correct = *GROUND_CORRECT_KIND_GROUND;
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_INT_MTRANS);
    }
    KineticModule::change_kinetic(fighter.module_accessor, kinetic);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_FIRST) {
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
    else {
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
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_FIRST);
    }
}

unsafe extern "C" fn mariod_speciallw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    else if VarModule::is_flag(fighter.battle_object, mariod::status::flag::SPECIAL_N_ENABLE_ACTIONS) {
        let situation = fighter.global_table[SITUATION_KIND].clone();
        if FGCModule::air_dash_cancel_check(fighter, false, true).get_bool()
        || special_cancel_common(fighter, situation, [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI].to_vec()).get_bool() {
            return 0.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 0.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 0.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        let mtrans = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_INT_MTRANS);
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() != mtrans
        && fighter.global_table[SITUATION_KIND].get_i32() == mtrans {
            mariod_speciallw_mot_helper(fighter);
        }
    }
    0.into()
}

#[status_script(agent = "mariod", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn mariod_speciallw_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC) {
        let limit_x_dec_curr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLOAT_LIMIT_X_DEC);
        let limit_x_dec = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("limit_x_dec"));
        let combined = limit_x_dec_curr + limit_x_dec;
        WorkModule::set_float(fighter.module_accessor, combined, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLOAT_LIMIT_X_DEC);
        let kinetic = KineticModule::get_kinetic_type(fighter.module_accessor);
        let limit_x_param = if kinetic != *FIGHTER_KINETIC_TYPE_MARIOD_SPECIAL_LW {
            hash40("air_limit_x")
        }
        else {
            hash40("limit_x")
        };
        let limit_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), limit_x_param);
        let limit_x = if combined < limit_x {
            limit_x - combined
        }
        else {
            0.0
        };
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            limit_x,
            0.0
        );
    }
    let mut rise_y = 0.0;
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIOD_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BUOYANCY) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_RISE) {
            let cont = if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
            || ControlModule::check_button_release(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                true
            }
            else {
                WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_RISE_PRECEDE)
            };
            if cont {
                rise_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("buoyancy"));
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_RISE_PRECEDE);
                if StatusModule::status_kind(fighter.module_accessor) != *SITUATION_KIND_AIR {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MARIOD_SPECIAL_AIR_LW);
                    fighter.set_situation(SITUATION_KIND_AIR.into());
                    GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                }
            }
        }
    }
    WorkModule::set_float(fighter.module_accessor, rise_y, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLOAT_RISE_Y);
    0.into()
}

#[status_script(agent = "mariod", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STOP)]
unsafe fn mariod_speciallw_exec_stop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_RISE_PRECEDE) {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        || ControlModule::check_button_release(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_RISE_PRECEDE);
        }
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        mariod_speciallw_main,
        mariod_speciallw_exec,
        mariod_speciallw_exec_stop
    );
}