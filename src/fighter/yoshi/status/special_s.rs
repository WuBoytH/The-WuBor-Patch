use crate::imports::status_imports::*;

unsafe extern "C" fn yoshi_special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
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
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON |
            *FIGHTER_LOG_MASK_FLAG_SHOOT
        ) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn yoshi_special_s_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let y_spd_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("y_spd_mul"));
    let y_spd_add = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("y_spd_add"));
    let y_up_spd_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("y_up_spd_max"));
    let y_spd_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("y_spd_max"));
    let y_acl = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("y_acl"));
    let hop_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("hop_max")) as i32;
    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    let x_spd_max_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("x_spd_max_mul"));
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    else {
        let hop_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_YOSHI_INSTANCE_WORK_ID_INT_HOP_COUNT);
        let hop_speed = if hop_count < hop_max {
            if hop_count != 0 {
                WorkModule::get_float(fighter.module_accessor, *FIGHTER_YOSHI_INSTANCE_WORK_ID_FLOAT_HOP_SPEED)
            }
            else {
                y_spd_add
            }
        }
        else {
            0.0
        };
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let mut speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
        speed_y *= y_spd_mul;
        speed_y += hop_speed;
        speed_y = speed_y.min(y_up_spd_max);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            speed_y
        );
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -y_acl
        );
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            y_spd_max
        );
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            y_up_spd_max
        );
        let hop_y_spd_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("hop_y_spd_mul"));
        WorkModule::set_float(fighter.module_accessor, hop_speed * hop_y_spd_mul, *FIGHTER_YOSHI_INSTANCE_WORK_ID_FLOAT_HOP_SPEED);
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_YOSHI_INSTANCE_WORK_ID_INT_HOP_COUNT);
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            air_speed_x_stable * x_spd_max_mul,
            -1.0
        );
    }
    0.into()
}

unsafe extern "C" fn yoshi_special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_YOSHI_STATUS_SPECIAL_HI_FLAG_EGG_APPEAR);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_YOSHI_STATUS_SPECIAL_HI_FLAG_EGG_SHOOT);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_YOSHI_STATUS_SPECIAL_HI_WORK_INT_EGG_COUNT);
    let motion = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        hash40("special_hi")
    }
    else {
        hash40("special_air_hi")
    };
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new_raw(motion),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    if !StopModule::is_stop(fighter.module_accessor) {
        yoshi_special_s_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(yoshi_special_s_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(yoshi_special_s_main_loop as *const () as _))
}

unsafe extern "C" fn yoshi_special_s_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            WorkModule::inc_int(fighter.module_accessor, *FIGHTER_YOSHI_STATUS_SPECIAL_HI_WORK_INT_EGG_COUNT);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_YOSHI_STATUS_SPECIAL_HI_FLAG_EGG_APPEAR) {
        if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_YOSHI_GENERATE_ARTICLE_TAMAGO) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_YOSHI_STATUS_SPECIAL_HI_FLAG_EGG_APPEAR);
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_YOSHI_GENERATE_ARTICLE_TAMAGO, false, -1);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_YOSHI_STATUS_SPECIAL_HI_FLAG_EGG_SHOOT) {
        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_YOSHI_GENERATE_ARTICLE_TAMAGO) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_YOSHI_STATUS_SPECIAL_HI_FLAG_EGG_SHOOT);
            ArticleModule::shoot(fighter.module_accessor, *FIGHTER_YOSHI_GENERATE_ARTICLE_TAMAGO, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        }
    }
    0.into()
}

unsafe extern "C" fn yoshi_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
            FIGHTER_STATUS_KIND_WAIT
        }
        else {
            FIGHTER_STATUS_KIND_FALL
        };
        fighter.change_status(status.into(), false.into());
        return 0.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                Hash40::new("special_hi"),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
            let x_spd_max_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("x_spd_max_mul"));
            sv_kinetic_energy!(
                set_stable_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                air_speed_x_stable * x_spd_max_mul,
                0.0
            );
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                Hash40::new("special_air_hi"),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        }
    }
    0.into()
}

unsafe extern "C" fn yoshi_special_s_exec(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn yoshi_special_s_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_YOSHI_GENERATE_ARTICLE_TAMAGO, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, yoshi_special_s_pre);
    agent.status(smashline::Init, *FIGHTER_STATUS_KIND_SPECIAL_S, yoshi_special_s_init);
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_SPECIAL_S, yoshi_special_s_main);
    agent.status(smashline::Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, yoshi_special_s_exec);
    agent.status(smashline::End, *FIGHTER_STATUS_KIND_SPECIAL_S, yoshi_special_s_end);
}