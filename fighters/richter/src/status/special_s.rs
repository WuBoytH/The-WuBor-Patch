use super::*;

unsafe extern "C" fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);

    let motion = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        Hash40::new("special_s")
    }
    else {
        Hash40::new("special_air_s")
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

    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_main_loop as *const () as _))
}

unsafe extern "C" fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            *FIGHTER_STATUS_KIND_WAIT
        }
        else {
            *FIGHTER_STATUS_KIND_FALL
        };
        fighter.change_status(status.into(), false.into());
        return 0.into();
    }

    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        let (motion, correct) = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            if VarModule::is_flag(fighter.module_accessor, vars::richter::status::flag::SPECIAL_S_ENABLE_LANDING) {
                WorkModule::set_float(fighter.module_accessor, 20.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
                return 0.into();
            }
            (Hash40::new("special_s"), *GROUND_CORRECT_KIND_GROUND)
        }
        else {
            if VarModule::is_flag(fighter.module_accessor, vars::richter::status::flag::SPECIAL_S_ENABLED_FALL) {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                return 0.into();
            }
            (Hash40::new("special_air_s"), *GROUND_CORRECT_KIND_AIR)
        };
        MotionModule::change_motion_inherit_frame(
            fighter.module_accessor,
            motion,
            -1.0,
            1.0,
            0.0,
            false,
            false
        );
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
    }

    0.into()
}

unsafe extern "C" fn special_s_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, vars::richter::status::flag::SPECIAL_S_START_DASH) {
        VarModule::off_flag(fighter.module_accessor, vars::richter::status::flag::SPECIAL_S_START_DASH);
        let situation = fighter.global_table[SITUATION_KIND].get_i32();
        VarModule::set_int(fighter.module_accessor, vars::richter::status::int::SPECIAL_S_START_SITUATION, situation);
        let (kinetic, correct) = if situation == *SITUATION_KIND_GROUND {
            (*FIGHTER_KINETIC_TYPE_GROUND_STOP, *GROUND_CORRECT_KIND_GROUND)
        }
        else {
            VarModule::on_flag(fighter.module_accessor, vars::fighter::instance::flag::DISABLE_SPECIAL_S);
            (*FIGHTER_KINETIC_TYPE_AIR_STOP, *GROUND_CORRECT_KIND_AIR)
        };

        KineticModule::change_kinetic(fighter.module_accessor, kinetic);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));

        let (speed, brake) = if situation == *SITUATION_KIND_GROUND {
            (3.5, 0.15)
        }
        else {
            (3.0, 0.15)
        };
        let lr = PostureModule::lr(fighter.module_accessor);

        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            0.0,
            0.0
        );
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            0.0,
            0.0
        );
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            -1.0,
            -1.0
        );
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            speed * lr,
            0.0
        );
        sv_kinetic_energy!(
            set_brake,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            brake,
            0.0
        );

        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, fighter.module_accessor);
    }
    if VarModule::is_flag(fighter.module_accessor, vars::richter::status::flag::SPECIAL_S_BRAKE_SHIFT) {
        VarModule::off_flag(fighter.module_accessor, vars::richter::status::flag::SPECIAL_S_BRAKE_SHIFT);
        let situation = VarModule::get_int(fighter.module_accessor, vars::richter::status::int::SPECIAL_S_START_SITUATION);
        let brake = if situation == *SITUATION_KIND_GROUND {
            0.05
        }
        else {
            0.02
        };

        sv_kinetic_energy!(
            set_brake,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            brake,
            0.0
        );
    }

    if VarModule::is_flag(fighter.module_accessor, vars::richter::status::flag::SPECIAL_S_ENABLE_FALL) {
        VarModule::off_flag(fighter.module_accessor, vars::richter::status::flag::SPECIAL_S_ENABLE_FALL);
        VarModule::on_flag(fighter.module_accessor, vars::richter::status::flag::SPECIAL_S_ENABLED_FALL);

        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
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
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
    }

    0.into()
}

unsafe extern "C" fn special_s_end(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_exec);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_end);
}