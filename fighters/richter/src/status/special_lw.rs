use super::*;

unsafe extern "C" fn special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let (kinetic, correct, motion, main_loop) = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        (*FIGHTER_KINETIC_TYPE_MOTION_RUN_STOP, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK, Hash40::new("special_lw"), special_lw_main_loop as *const () as _)
    }
    else {
        (*FIGHTER_KINETIC_TYPE_RESET, *GROUND_CORRECT_KIND_AIR, Hash40::new("special_air_lw"), special_air_lw_main_loop as *const () as _)
    };

    KineticModule::change_kinetic(fighter.module_accessor, kinetic);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
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

    fighter.sub_shift_status_main(L2CValue::Ptr(main_loop))
}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor)
    && VarModule::is_flag(fighter.module_accessor, vars::richter::status::flag::SPECIAL_LW_SLIDE_ENABLE_JUMP)
    && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_JUMP_TRIGGER != 0 {
        fighter.change_status(FIGHTER_SIMON_STATUS_KIND_ATTACK_LW32.into(), true.into());
        return 1.into();
    }

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(true.into()).get_bool() {
            return 1.into();
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), false.into());
        return 0.into();
    }

    0.into()
}

unsafe extern "C" fn special_air_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, vars::richter::status::flag::SPECIAL_LW_CHECK_ANGLE) {
        VarModule::off_flag(fighter.module_accessor, vars::richter::status::flag::SPECIAL_LW_CHECK_ANGLE);
        let lr = PostureModule::lr(fighter.module_accessor);
        let stick_x = fighter.global_table[STICK_X].get_f32();
        if stick_x * lr >= 0.3 {
            VarModule::on_flag(fighter.module_accessor, vars::richter::status::flag::SPECIAL_LW_IS_ANGLED);
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                Hash40::new("special_air_lw_s"),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
    }
    if VarModule::is_flag(fighter.module_accessor, vars::richter::status::flag::SPECIAL_LW_BOUNCE)
    && !fighter.global_table[IS_STOP].get_bool() {
        fighter.change_status(vars::richter::status::SPECIAL_LW_BOUNCE.into(), false.into());
    }

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && !VarModule::is_flag(fighter.module_accessor, vars::richter::status::flag::SPECIAL_LW_BOUNCE) {
        let status = if VarModule::is_flag(fighter.module_accessor, vars::richter::status::flag::SPECIAL_LW_DIVE_ENABLE_LANDING) {
            vars::richter::status::SPECIAL_AIR_LW_LANDING
        }
        else {
            WorkModule::set_float(fighter.module_accessor, 15.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL
        };
        fighter.change_status(status.into(), false.into());
        return 0.into();
    }

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }

    0.into()
}

unsafe extern "C" fn special_lw_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::motion_kind(fighter.module_accessor) != hash40("special_lw") {
        if VarModule::is_flag(fighter.module_accessor, vars::richter::status::flag::SPECIAL_LW_DIVE) {
            VarModule::off_flag(fighter.module_accessor, vars::richter::status::flag::SPECIAL_LW_DIVE);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);

            let lr = PostureModule::lr(fighter.module_accessor);

            let speed: (f32, f32) = if VarModule::is_flag(fighter.module_accessor, vars::richter::status::flag::SPECIAL_LW_IS_ANGLED) {
                (1.8, 3.0)
            }
            else {
                (0.0, 3.0)
            };

            sv_kinetic_energy!(
                set_stable_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                -1.0,
                -1.0
            );
            sv_kinetic_energy!(
                set_limit_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                -1.0,
                -1.0
            );
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                0.0,
                0.0
            );
            sv_kinetic_energy!(
                set_brake,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                0.0,
                0.0
            );
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                speed.0 * lr,
                0.0
            );

            sv_kinetic_energy!(
                set_stable_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                speed.1
            );
            sv_kinetic_energy!(
                set_limit_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                speed.1
            );
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                -speed.1
            );
        }
    }

    0.into()
}

unsafe extern "C" fn special_lw_check_attack(fighter: &mut L2CFighterCommon, _param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let motion = MotionModule::motion_kind(fighter.module_accessor);
    if ![
        hash40("special_lw")
    ].contains(&motion) {
        let table = param_3.get_table() as *mut smash_rs::lib::L2CTable;
        let kind = MiscModule::get_table_value(table, "kind_").try_integer().unwrap() as i32;
    
        if ![
            *COLLISION_KIND_SHIELD,
        ].contains(&kind) {
            VarModule::on_flag(fighter.module_accessor, vars::richter::status::flag::SPECIAL_LW_BOUNCE_IS_HIT);
        }

        VarModule::on_flag(fighter.module_accessor, vars::richter::status::flag::SPECIAL_LW_BOUNCE);
    }

    0.into()
}

unsafe extern "C" fn special_lw_end(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_exec);
    agent.status(CheckAttack, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_check_attack);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_end);
}