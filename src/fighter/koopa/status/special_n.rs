use crate::imports::status_imports::*;

unsafe extern "C" fn koopa_special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    koopa_special_n_mtrans_reset(fighter);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_BREATH_FLAG_CONTINUE_START);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_BREATH_FLAG_CONTINUE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_BREATH_FLAG_CONTINUE_END);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_BREATH_STEP_START, *FIGHTER_KOOPA_STATUS_BREATH_WORK_INT_STEP);
    koopa_special_n_handle_motion(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(koopa_special_n_main_loop as *const () as _))
}

unsafe extern "C" fn koopa_special_n_mtrans_reset(fighter: &mut L2CFighterCommon) {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KOOPA_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KOOPA_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KOOPA_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_MOTION_END);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KOOPA_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_EX1);
}

unsafe extern "C" fn koopa_special_n_handle_motion(fighter: &mut L2CFighterCommon) {
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    let step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_BREATH_WORK_INT_STEP);
    let mot;
    let cont_const = if step == *FIGHTER_KOOPA_STATUS_BREATH_STEP_START {
        *FIGHTER_KOOPA_STATUS_BREATH_FLAG_CONTINUE_START
    }
    else {
        *FIGHTER_KOOPA_STATUS_BREATH_FLAG_CONTINUE_END
    };
    if situation == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        if step == *FIGHTER_KOOPA_STATUS_BREATH_STEP_START {
            mot = hash40("special_n_start");
        }
        else {
            mot = hash40("special_n_end");
        }
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if step == *FIGHTER_KOOPA_STATUS_BREATH_STEP_START {
            mot = hash40("special_air_n_start");
        }
        else {
            mot = hash40("special_air_n_end");
        }
    }
    if !WorkModule::is_flag(fighter.module_accessor, cont_const) {
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
        WorkModule::on_flag(fighter.module_accessor, cont_const);
    }
    else {
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
}

unsafe extern "C" fn koopa_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
            return 1.into();
        }
        koopa_special_n_handle_motion(fighter);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_BREATH_WORK_INT_STEP);
        if step == *FIGHTER_KOOPA_STATUS_BREATH_STEP_START {
            WorkModule::set_int(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_BREATH_STEP_END, *FIGHTER_KOOPA_STATUS_BREATH_WORK_INT_STEP);
            koopa_special_n_handle_motion(fighter);
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KOOPA_GENERATE_ARTICLE_BREATH, false, -1);
            let speed_mul_min = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("fire_speed_mul_min"));
            let scale_min = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("fire_scale_min"));
            WorkModule::set_float(fighter.module_accessor, speed_mul_min, *FIGHTER_KOOPA_INSTANCE_WORK_ID_FLOAT_BREATH_SPEED_MUL);
            WorkModule::set_float(fighter.module_accessor, scale_min, *FIGHTER_KOOPA_INSTANCE_WORK_ID_FLOAT_BREATH_SCALE);
        }
        else {
            let status = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                FIGHTER_STATUS_KIND_WAIT
            }
            else {
                FIGHTER_STATUS_KIND_FALL
            };
            fighter.change_status(status.into(), false.into());
            return 0.into();
        }
    }
    0.into()
}

unsafe extern "C" fn koopa_special_n_exec(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn koopa_special_n_exec_stop(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_SPECIAL_N, koopa_special_n_main);
    agent.status(smashline::Exec, *FIGHTER_STATUS_KIND_SPECIAL_N, koopa_special_n_exec);
    agent.status(smashline::ExecStop, *FIGHTER_STATUS_KIND_SPECIAL_N, koopa_special_n_exec_stop);
}