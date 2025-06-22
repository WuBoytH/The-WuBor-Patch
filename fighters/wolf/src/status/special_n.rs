use super::*;

unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    let motion = if situation == *SITUATION_KIND_GROUND {
        Hash40::new("special_n")
    }
    else {
        Hash40::new("special_air_n")
    };

    MotionModule::change_motion(
        fighter.module_accessor,
        motion,
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    VarModule::set_int64(fighter.module_accessor, vars::wolf::status::int::SPECIAL_N_MOTION, hash40("special_n"));
    VarModule::set_int64(fighter.module_accessor, vars::wolf::status::int::SPECIAL_N_MOTION_AIR, hash40("special_air_n"));

    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_BLASTER_STATUS_WORK_ID_FLAG_CONTINUE);

    if situation == *SITUATION_KIND_GROUND {
        let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            ENERGY_STOP_RESET_TYPE_GROUND,
            sum_speed_x,
            0.0,
            0.0,
            0.0,
            0.0
        );
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    } else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }

    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_main_loop as *const () as _))
}

unsafe extern "C" fn special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_WAIT
        } else {
            FIGHTER_STATUS_KIND_FALL
        };
        fighter.change_status(status.into(), false.into());
        return 0.into();
    }

    if VarModule::is_flag(fighter.module_accessor, vars::wolf::status::flag::SPECIAL_N_CHECK_ANGLE) {
        VarModule::off_flag(fighter.module_accessor, vars::wolf::status::flag::SPECIAL_N_CHECK_ANGLE);

        let cancel = ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD);
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
        && cancel {
            fighter.change_status(vars::wolf::status::SPECIAL_N_CANCEL.into(), true.into());
            return 0.into();
        }

        let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
        let (g, a) = if stick_y >= 0.5 {
            (hash40("special_n_hi"), hash40("special_air_n_hi"))
        }
        else if stick_y <= -0.5 {
            (hash40("special_n_lw"), hash40("special_air_n_lw"))
        }
        else {
            (hash40("invalid"), hash40("invalid"))
        };

        if g != hash40("invalid") {
            VarModule::set_int64(fighter.module_accessor, vars::wolf::status::int::SPECIAL_N_MOTION, g);
            VarModule::set_int64(fighter.module_accessor, vars::wolf::status::int::SPECIAL_N_MOTION_AIR, a);
            special_n_mot_helper(fighter);
        }
    }

    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        if VarModule::is_flag(fighter.module_accessor, vars::wolf::status::flag::SPECIAL_N_ENABLE_LANDING)
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
            return 0.into();
        }
        special_n_mot_helper(fighter);
    }

    0.into()
}

unsafe extern "C" fn special_n_mot_helper(fighter: &mut L2CFighterCommon) {
    let situation = fighter.global_table[SITUATION_KIND].get_i32();

    if situation == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    } else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }

    let motion = if situation == *SITUATION_KIND_GROUND {
        VarModule::get_int64(fighter.module_accessor, vars::wolf::status::int::SPECIAL_N_MOTION)
    }
    else {
        VarModule::get_int64(fighter.module_accessor, vars::wolf::status::int::SPECIAL_N_MOTION_AIR)
    };
    MotionModule::change_motion_inherit_frame_keep_rate(
        fighter.module_accessor,
        Hash40::new_raw(motion),
        -1.0,
        1.0,
        0.0
    );
}

unsafe extern "C" fn special_n_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != vars::wolf::status::SPECIAL_N_CANCEL {
        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER) {
            ArticleModule::remove(fighter.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, ArticleOperationTarget(0));
        }
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_end);
}