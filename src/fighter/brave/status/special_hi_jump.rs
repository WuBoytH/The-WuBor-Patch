use crate::imports::*;
use super::helper::*;

unsafe extern "C" fn brave_special_hi_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.module_accessor, fighter::instance::flag::DISABLE_SPECIAL_HI);
    let hold_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_WORK_INT_HOLD_FRAME);
    let hold_frame_m = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("hold_frame_m"));
    let hold_frame_l = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("hold_frame_l"));
    let sp_param = if hold_frame < hold_frame_m {
        hash40("sp_special_hi1")
    }
    else if hold_frame < hold_frame_l {
        hash40("sp_special_hi2")
    }
    else {
        hash40("sp_special_hi3")
    };
    let sp = WorkModule::get_param_float(fighter.module_accessor, hash40("param_private"), sp_param);
    brave_special_check_sp(fighter, sp.into(), FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_SUCCESS_SP.into());
    let mot_g;
    let mot_a;
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_SUCCESS_SP) {
        if hold_frame < hold_frame_m {
            mot_g = hash40("special_hi1");
            mot_a = hash40("special_air_hi1");
        }
        else if hold_frame < hold_frame_l {
            mot_g = hash40("special_hi2");
            mot_a = hash40("special_air_hi2");
        }
        else {
            mot_g = hash40("special_hi3");
            mot_a = hash40("special_air_hi3");
        }
    }
    else {
        mot_g = hash40("special_hi_empty");
        mot_a = hash40("special_air_hi_empty");
    }
    fighter.sub_change_motion_by_situation(
        mot_g.into(),
        mot_a.into(),
        false.into()
    );
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        brave_special_air_hi_jump_speed_handler(fighter);
    }
    let landing_frame_param = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_SUCCESS_SP) {
        if hold_frame < hold_frame_m {
            hash40("landing_frame_s")
            
        }
        else if hold_frame < hold_frame_l {
            hash40("landing_frame_m")
            
        }
        else {
            hash40("landing_frame_l")
        }
    }
    else {
        hash40("landing_frame_empty")
    };
    let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), landing_frame_param);
    WorkModule::set_float(fighter.module_accessor, landing_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    fighter.sub_shift_status_main(L2CValue::Ptr(brave_special_hi_jump_main_loop as *const () as _))
}

unsafe extern "C" fn brave_special_air_hi_jump_speed_handler(fighter: &mut L2CFighterCommon) {
    let wait_jump_accel_y_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("wait_jump_accel_y_mul"));
    let wait_jump_max_speed_y_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("wait_jump_max_speed_y_mul"));
    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
    let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        air_accel_y * wait_jump_accel_y_mul * -1.0
    );
    sv_kinetic_energy!(
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        air_speed_y_stable * wait_jump_max_speed_y_mul
    );
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        air_speed_y_stable * wait_jump_max_speed_y_mul
    );
}

unsafe extern "C" fn brave_special_hi_jump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    let hold_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_WORK_INT_HOLD_FRAME);
    let hold_frame_m = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("hold_frame_m"));
    let hold_frame_l = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("hold_frame_l"));
    let can_act = hold_frame < hold_frame_m || !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_SUCCESS_SP);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_JUMP) {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_FALL) {
            let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            if sum_speed_y < 0.0 {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_FALL);
                let accel_param;
                let max_speed_param;
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_SUCCESS_SP) {
                    if hold_frame < hold_frame_m {
                        accel_param = hash40("end_accel_y_s");
                        max_speed_param = hash40("end_max_speed_y_s");
                    }
                    else if hold_frame < hold_frame_l {
                        accel_param = hash40("end_accel_y_m");
                        max_speed_param = hash40("end_max_speed_y_m");
                    }
                    else {
                        accel_param = hash40("end_accel_y_l");
                        max_speed_param = hash40("end_max_speed_y_l");
                    }
                }
                else {
                    accel_param = hash40("end_accel_y_empty");
                    max_speed_param = hash40("end_max_speed_y_empty");
                }
                let accel_y_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), accel_param);
                let max_speed_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), max_speed_param);
                let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
                let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
                sv_kinetic_energy!(
                    set_accel,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                    air_accel_y * accel_y_mul * -1.0
                );
                sv_kinetic_energy!(
                    set_limit_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                    air_speed_y_stable * max_speed_mul
                );
                sv_kinetic_energy!(
                    set_stable_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                    air_speed_y_stable * max_speed_mul
                );
            }
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_JUMP_START) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_JUMP_START);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_JUMP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        let distance_param;
        let speed_x_mul_param;
        let accel_y_param;
        let accel_x_param;
        let deccel_x_param;
        let max_speed_x_param;
        let max_degree_param;
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_SUCCESS_SP) {
            if hold_frame < hold_frame_m {
                distance_param = hash40("jump_distance_s");
                speed_x_mul_param = hash40("jump_speed_x_mul_s");
                accel_y_param = hash40("jump_accel_y_s");
                accel_x_param = hash40("jump_accel_x_s");
                deccel_x_param = hash40("jump_deccel_x_s");
                max_speed_x_param = hash40("jump_max_speed_x_s");
                max_degree_param = hash40("jump_max_degree_s");
            }
            else if hold_frame < hold_frame_l {
                distance_param = hash40("jump_distance_m");
                speed_x_mul_param = hash40("jump_speed_x_mul_m");
                accel_y_param = hash40("jump_accel_y_m");
                accel_x_param = hash40("jump_accel_x_m");
                deccel_x_param = hash40("jump_deccel_x_m");
                max_speed_x_param = hash40("jump_max_speed_x_m");
                max_degree_param = hash40("jump_max_degree_m");
            }
            else {
                distance_param = hash40("jump_distance_l");
                speed_x_mul_param = hash40("jump_speed_x_mul_l");
                accel_y_param = hash40("jump_accel_y_l");
                accel_x_param = hash40("jump_accel_x_l");
                deccel_x_param = hash40("jump_deccel_x_l");
                max_speed_x_param = hash40("jump_max_speed_x_l");
                max_degree_param = hash40("jump_max_degree_l");
            }
        }
        else {
            distance_param = hash40("jump_distance_empty");
            speed_x_mul_param = hash40("jump_speed_x_mul_empty");
            accel_y_param = hash40("jump_accel_y_empty");
            accel_x_param = hash40("jump_accel_x_empty");
            deccel_x_param = hash40("jump_deccel_x_empty");
            max_speed_x_param = hash40("jump_max_speed_x_empty");
            max_degree_param = hash40("jump_max_degree_empty");
        }
        let distance = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), distance_param);
        let speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), speed_x_mul_param);
        let accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), accel_y_param);
        let accel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), accel_x_param);
        let deccel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), deccel_x_param);
        let max_speed_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), max_speed_x_param);
        let max_degree = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), max_degree_param);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        sv_kinetic_energy!(
            unable,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP
        );
        let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
        let jump_speed_y = KineticUtility::get_jump_speed_y(distance, accel_y);
        let speed_x = sum_speed_x * speed_x_mul;
        let rad = (max_degree * stick_x).to_radians();
        let speed_x = jump_speed_y * rad.sin() + speed_x;
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            speed_x,
            0.0
        );
        sv_kinetic_energy!(
            set_brake,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            deccel_x,
            0.0
        );
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            max_speed_x,
            0.0
        );
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            max_speed_x,
            0.0
        );
        sv_kinetic_energy!(
            controller_set_accel_x_mul,
            fighter,
            0.0
        );
        sv_kinetic_energy!(
            controller_set_accel_x_add,
            fighter,
            accel_x
        );
        sv_kinetic_energy!(
            enable,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL
        );
        let speed_y = jump_speed_y * rad.cos();
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
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            speed_y
        );
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            speed_y
        );
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -accel_y
        );
        let lr = PostureModule::lr(fighter.module_accessor);
        let angle = max_degree * stick_x * lr;
        PostureModule::set_rot(fighter.module_accessor, &Vector3f{x: angle, y: 0.0, z: 0.0}, 0);
        WorkModule::set_float(fighter.module_accessor, max_degree * stick_x, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_WORK_FLOAT_INIT_DEGREE_X);
        WorkModule::set_float(fighter.module_accessor, max_degree * stick_x, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_WORK_FLOAT_DEGREE_X);
    }
    let disable_landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("disable_landing_frame"));
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[PREV_SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_JUMP) {
            if disable_landing_frame < WorkModule::get_int(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_WORK_INT_JUMP_COUNT) {
                let status = if can_act {
                    FIGHTER_STATUS_KIND_LANDING
                }
                else {
                    FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL
                };
                fighter.change_status(status.into(), false.into());
            }
        }
        else {
            brave_special_hi_jump_situation_change_handler(fighter);
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR
            && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                brave_special_air_hi_jump_speed_handler(fighter);
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            if can_act {
                FIGHTER_STATUS_KIND_LANDING
            }
            else {
                FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL
            }
        }
        else {
            if can_act {
                FIGHTER_STATUS_KIND_FALL
            }
            else {
                FIGHTER_STATUS_KIND_FALL_SPECIAL
            }
        };
        fighter.change_status(status.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn brave_special_hi_jump_situation_change_handler(fighter: &mut L2CFighterCommon) {
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    let situation_prev = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if situation == *SITUATION_KIND_GROUND
    && situation_prev == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        return;
    }
    if situation != *SITUATION_KIND_AIR {
        return;
    }
    if situation_prev != *SITUATION_KIND_GROUND {
        return;
    }
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_HI_JUMP, brave_special_hi_jump_main);
}