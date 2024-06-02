use super::*;

unsafe extern "C" fn mario_special_s_init(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn mario_special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    mario_special_s_motion_helper(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(mario_special_s_main_loop as *const () as _))
}

unsafe extern "C" fn mario_special_s_motion_helper(fighter: &mut L2CFighterCommon) {
    let cont = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_CONTINUE);
    let mot;
    let kinetic;
    let correct;
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        mot = Hash40::new("special_s");
        kinetic = *FIGHTER_KINETIC_TYPE_GROUND_STOP;
        correct = *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK;
    }
    else {
        mot = Hash40::new("special_air_s");
        kinetic = *FIGHTER_KINETIC_TYPE_AIR_STOP;
        correct = *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK;
    }
    KineticModule::change_kinetic(fighter.module_accessor, kinetic);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
    if cont {
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
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_CONTINUE);
    }
}

unsafe extern "C" fn mario_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
        && VarModule::is_flag(fighter.module_accessor, vars::mario::status::flag::SPECIAL_S_HOP) {
            WorkModule::set_float(fighter.module_accessor, 16.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            return 0.into();
        }
        mario_special_s_motion_helper(fighter);
    }

    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_WAIT
        }
        else {
            FIGHTER_STATUS_KIND_FALL
        };
        fighter.change_status(status.into(), false.into());
    }

    0.into()
}

unsafe extern "C" fn mario_special_s_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, vars::mario::status::flag::SPECIAL_S_TRY_HOP) {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP) {
            let lr = PostureModule::lr(fighter.module_accessor);
            let speed_x = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                vl::param_special_s::spin_hop_speed_xa
            }
            else {
                vl::param_special_s::spin_hop_speed_xg
            };
            let reset_type = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
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
                speed_x * lr,
                0.0,
                0.0,
                0.0,
                0.0
            );
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                sv_kinetic_energy!(
                    set_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                    vl::param_special_s::spin_hop_speed_y
                );
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP);
            }
            else {
                let ground_brake = WorkModule::get_param_float(fighter.module_accessor, hash40("ground_brake"), 0);
                sv_kinetic_energy!(
                    set_brake,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_STOP,
                    ground_brake
                );
            }
        }
        VarModule::off_flag(fighter.module_accessor, vars::mario::status::flag::SPECIAL_S_TRY_HOP);
        VarModule::on_flag(fighter.module_accessor, vars::mario::status::flag::SPECIAL_S_HOP);
    }

    if VarModule::is_flag(fighter.module_accessor, vars::mario::status::flag::SPECIAL_S_ENABLE_CONTROL) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
        VarModule::off_flag(fighter.module_accessor, vars::mario::status::flag::SPECIAL_S_ENABLE_CONTROL);
    }

    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
        let air_accel_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y_stable"), 0);
        let air_accel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP) {
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                -air_accel_y
            );
            sv_kinetic_energy!(
                set_accel_x_mul,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                air_accel_x * 0.2
            );
            sv_kinetic_energy!(
                set_limit_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                air_accel_y_stable
            );
        }
        else {
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                -air_accel_y / 1.5
            );
            sv_kinetic_energy!(
                set_accel_x_mul,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                air_accel_x * 0.2
            );
            sv_kinetic_energy!(
                set_limit_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                air_accel_y_stable / 2.0
            );
        }
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, mario_special_s_init);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, mario_special_s_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, mario_special_s_exec);
}