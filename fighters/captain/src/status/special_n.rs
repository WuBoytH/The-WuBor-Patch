use super::*;

unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int64(fighter.module_accessor, hash40("special_n") as i64, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_GROUND_MOT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n") as i64, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_AIR_MOT);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT);
    if !StopModule::is_stop(fighter.module_accessor) {
        special_n_substatus(fighter, false.into());
    }
    fighter.global_table[0x15].assign(&L2CValue::Ptr(special_n_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_main_loop as *const () as _))
}

unsafe extern "C" fn special_n_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_TURN) {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_CHECK_END) {
                let lr = PostureModule::lr(fighter.module_accessor);
                let stick_x = fighter.global_table[0x1A].get_f32();
                let turn_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_stick_x"));
                if lr * stick_x <= turn_stick_x {
                    fighter.change_status(FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN.into(), true.into());
                    return 0.into();
                }
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_CHECK_END);
            }
        }
    }
    0.into()
}

unsafe extern "C" fn special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_WAIT
        }
        else {
            FIGHTER_STATUS_KIND_FALL
        };
        fighter.change_status(status.into(), false.into());
        return 0.into();
    }

    special_n_bird_handler(fighter, WEAPON_CAPTAIN_FALCONPUNCH_STATUS_KIND_SPECIAL_N.into());

    if StatusModule::is_changing(fighter.module_accessor)
    || StatusModule::is_situation_changed(fighter.module_accessor) {
        special_n_situation_helper(fighter);
    }
    0.into()
}

unsafe extern "C" fn special_n_bird_handler(fighter: &mut L2CFighterCommon, status: L2CValue) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_GENERATE_BIRD) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH, false, -1);
        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH) {
            ArticleModule::change_status_exist(fighter.module_accessor, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH, status.get_i32());
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_VISIBLE_BIRD);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH, true, ArticleOperationTarget(0));
        }
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_GENERATE_BIRD);
    }

    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH) {
        let visible = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_VISIBLE_BIRD);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH, visible, ArticleOperationTarget(0));
    }
}

unsafe extern "C" fn special_n_situation_helper(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        special_n_motion_helper(fighter, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_GROUND_MOT);
        if !VarModule::is_flag(fighter.module_accessor, vars::captain::status::flag::SPECIAL_N_BOOST_POWER_KINETIC_SHIFTED) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        }
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
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        special_n_motion_helper(fighter, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_AIR_MOT);
        if !VarModule::is_flag(fighter.module_accessor, vars::captain::status::flag::SPECIAL_N_BOOST_POWER_KINETIC_SHIFTED) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        }
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            ENERGY_MOTION_RESET_TYPE_AIR_TRANS,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        if VarModule::is_flag(fighter.module_accessor, vars::captain::status::flag::SPECIAL_N_BOOST_POWER_KINETIC_SHIFTED) {
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
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    }
}

unsafe extern "C" fn special_n_motion_helper(fighter: &mut L2CFighterCommon, mot_const: i32) {
    let mot = WorkModule::get_int64(fighter.module_accessor, mot_const);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT) {
        MotionModule::change_motion_inherit_frame(
            fighter.module_accessor,
            Hash40::new_raw(mot),
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
            Hash40::new_raw(mot),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT);
    }
}

unsafe extern "C" fn special_n_turn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int64(fighter.module_accessor, hash40("special_n_turn") as i64, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_GROUND_MOT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_turn") as i64, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_AIR_MOT);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT);

    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH, false, -1);
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH) {
        ArticleModule::change_status_exist(
            fighter.module_accessor,
            *FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH,
            *WEAPON_CAPTAIN_FALCONPUNCH_STATUS_KIND_SPECIAL_N_TURN
        );
        let motion = if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            let lr = PostureModule::lr(fighter.module_accessor);
            if lr == -1.0 {
                Hash40::new("special_n_turn_l")
            }
            else {
                Hash40::new("special_n_turn_r")
            }
        }
        else {
            Hash40::new("special_air_n_turn")
        };
        ArticleModule::change_motion(
            fighter.module_accessor,
            *FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH,
            motion,
            false,
            -1.0
        );
        ArticleModule::set_frame(fighter.module_accessor, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH, 1.0);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH, false, ArticleOperationTarget(0));
    }

    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_turn_main_loop as *const () as _))
}

unsafe extern "C" fn special_n_turn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_WAIT
        }
        else {
            FIGHTER_STATUS_KIND_FALL
        };
        fighter.change_status(status.into(), false.into());
        return 0.into();
    }

    special_n_bird_handler(fighter, WEAPON_CAPTAIN_FALCONPUNCH_STATUS_KIND_SPECIAL_N_TURN.into());

    if StatusModule::is_changing(fighter.module_accessor)
    || StatusModule::is_situation_changed(fighter.module_accessor) {
        special_n_situation_helper(fighter);
    }
    0.into()
}

unsafe extern "C" fn special_n_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, vars::captain::status::flag::USED_BOOST_POWER) {
        if VarModule::is_flag(fighter.module_accessor, vars::captain::status::flag::SPECIAL_N_BOOST_POWER_KINETIC_SHIFT_END) {
            // println!("signal end");
            if !KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) {
                if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);

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
                    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
                    sv_kinetic_energy!(
                        set_accel,
                        fighter,
                        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                        -air_accel_y * 0.6
                    );
                    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                }
                else {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                }
            }
            else {
                let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
                sv_kinetic_energy!(
                    set_accel,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                    -air_accel_y * 0.4
                );
            }
        }
        if VarModule::is_flag(fighter.module_accessor, vars::captain::status::flag::SPECIAL_N_BOOST_POWER_KINETIC_SHIFT) {
            VarModule::off_flag(fighter.module_accessor, vars::captain::status::flag::SPECIAL_N_BOOST_POWER_KINETIC_SHIFT);
            VarModule::on_flag(fighter.module_accessor, vars::captain::status::flag::SPECIAL_N_BOOST_POWER_KINETIC_SHIFTED);

            sv_kinetic_energy!(
                reset_energy,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                ENERGY_STOP_RESET_TYPE_AIR,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0
            );
            sv_kinetic_energy!(
                set_limit_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                3.0,
                -1.0
            );
            sv_kinetic_energy!(
                set_brake,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                0.05,
                0.0
            );
            let lr = PostureModule::lr(fighter.module_accessor);
            let speed = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                3.0
            }
            else {
                2.2
            };
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                speed * lr,
                0.0
            );
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);

            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);
            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, fighter.module_accessor);
        }
    }
    else {
        let status = fighter.global_table[STATUS_KIND].get_i32();
        original_status(Exec, fighter, status)(fighter);
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_exec);

    agent.status(Main, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN, special_n_turn_main);
    agent.status(Exec, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN, special_n_exec);
}