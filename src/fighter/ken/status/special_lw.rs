use crate::imports::status_imports::*;

unsafe extern "C" fn ken_special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn ken_special_lw_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN).clamp(-0.5, 0.5);
    let reset_type = if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        let lr = PostureModule::lr(fighter.module_accessor);
        speed_x = 1.4 * lr;
        ENERGY_STOP_RESET_TYPE_AIR
    }
    else {
        ENERGY_STOP_RESET_TYPE_GROUND
    };
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        reset_type,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0
    );
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        speed_x,
        0.0
    );
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            ENERGY_MOTION_RESET_TYPE_GROUND_TRANS,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    }
    else {
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
        let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
            0.0,
            speed_y.clamp(-1.0, 1.0),
            0.0,
            0.0,
            0.0
        );
        let accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("accel_y"));
        sv_kinetic_energy!(
            set_brake,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            accel_y
        );
        let max_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("max_speed_y"));
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            max_y
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    0.into()
}

unsafe extern "C" fn ken_special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let correct;
    let mot;
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        correct = *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP;
        mot = hash40("special_lw_start");
    }
    else {
        VarModule::on_flag(fighter.module_accessor, fighter::instance::flag::DISABLE_SPECIAL_LW);
        correct = *GROUND_CORRECT_KIND_AIR;
        mot = hash40("special_air_lw_start");
    }
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
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
    fighter.sub_shift_status_main(L2CValue::Ptr(ken_special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn ken_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, ken::status::flag::SPECIAL_LW_ENABLE_ACTION) {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
        VarModule::off_flag(fighter.module_accessor, ken::status::flag::SPECIAL_LW_ENABLE_ACTION);
        VarModule::on_flag(fighter.module_accessor, ken::status::flag::SPECIAL_LW_ENABLED_ACTION);
    }
    if VarModule::is_flag(fighter.module_accessor, ken::status::flag::SPECIAL_LW_UNABLE_ACTION) {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
        VarModule::off_flag(fighter.module_accessor, ken::status::flag::SPECIAL_LW_UNABLE_ACTION);
        VarModule::off_flag(fighter.module_accessor, ken::status::flag::SPECIAL_LW_ENABLED_ACTION);
    }
    if VarModule::is_flag(fighter.module_accessor, ken::status::flag::SPECIAL_LW_RESET_GRAVITY) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
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
        }
        VarModule::off_flag(fighter.module_accessor, ken::status::flag::SPECIAL_LW_RESET_GRAVITY);
    }
    if fighter.sub_wait_ground_check_common(false.into()).get_bool()
    || fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }

    // Special handling to allow Dragonlash to be easier performed during Quick Dash
    if VarModule::is_flag(fighter.module_accessor, ken::status::flag::SPECIAL_LW_ENABLED_ACTION)
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_S_UNIQ].clone()).get_bool()
    && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0 {
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND.into(), true.into());
        return 1.into();
    }

    if VarModule::is_flag(fighter.module_accessor, ken::status::flag::SPECIAL_LW_ENABLED_ACTION) {
        if fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_GUARD_TRIGGER != 0 {
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
            let mot = if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                hash40("special_air_lw_step_f")
            }
            else {
                hash40("special_lw_step_f")
            };
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
            ControlModule::clear_command(fighter.module_accessor, false);
            VarModule::off_flag(fighter.module_accessor, ken::status::flag::SPECIAL_LW_ENABLED_ACTION);
            return 0.into();
        }
        if fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 {
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
            let mot = if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                hash40("special_air_lw")
            }
            else {
                hash40("special_lw")
            };
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
            ControlModule::clear_command(fighter.module_accessor, false);
            VarModule::off_flag(fighter.module_accessor, ken::status::flag::SPECIAL_LW_ENABLED_ACTION);
            return 0.into();
        }
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        }
        return 0.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn ken_special_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if VarModule::is_flag(fighter.module_accessor, ken::status::flag::SPECIAL_LW_ENABLED_ACTION)
    && [
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND
    ].contains(&status) {
        VarModule::on_flag(fighter.module_accessor, ken::instance::flag::QUICK_STEP_INHERIT)
    }
    else {
        VarModule::off_flag(fighter.module_accessor, ken::instance::flag::QUICK_STEP_INHERIT)
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, ken_special_lw_pre);
    agent.status(smashline::Init, *FIGHTER_STATUS_KIND_SPECIAL_LW, ken_special_lw_init);
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, ken_special_lw_main);
    agent.status(smashline::End, *FIGHTER_STATUS_KIND_SPECIAL_LW, ken_special_lw_end);
}