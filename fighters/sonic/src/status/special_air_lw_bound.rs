use super::*;

unsafe extern "C" fn sonic_special_air_lw_bound_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
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
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn sonic_special_air_lw_bound_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let correct = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        VarModule::on_flag(fighter.module_accessor, vars::sonic::status::flag::SPECIAL_AIR_LW_BOUND_IS_GROUND);
        *GROUND_CORRECT_KIND_GROUND
    }
    else {
        *GROUND_CORRECT_KIND_AIR
    };
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));

    let angle = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        let normal = GroundModule::get_touch_normal(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
        normal.y.atan2(normal.x)
    }
    else {
        90.0_f32.to_radians()
    };

    VarModule::set_float(fighter.module_accessor, vars::sonic::status::float::SPECIAL_AIR_LW_BOUND_ANGLE, angle);

    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RESET);

    0.into()
}

unsafe extern "C" fn sonic_special_air_lw_bound_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.module_accessor, vars::fighter::instance::flag::DISABLE_SPECIAL_LW);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_air_lw_2_bound"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(sonic_special_air_lw_bound_main_bound as *const () as _))
}

unsafe extern "C" fn sonic_special_air_lw_bound_main_bound(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && VarModule::is_flag(fighter.module_accessor, vars::sonic::status::flag::SPECIAL_AIR_LW_BOUNDING) {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        return 0.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn sonic_special_air_lw_bound_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(fighter.module_accessor, vars::sonic::status::flag::SPECIAL_AIR_LW_BOUND_START) {
        return 0.into();
    }

    VarModule::off_flag(fighter.module_accessor, vars::sonic::status::flag::SPECIAL_AIR_LW_BOUND_START);
    VarModule::on_flag(fighter.module_accessor, vars::sonic::status::flag::SPECIAL_AIR_LW_BOUNDING);
    let speed_x_add = VarModule::get_float(fighter.module_accessor, vars::sonic::status::float::SPECIAL_AIR_LW_BOUND_SPEED_X);
    let speed = VarModule::get_float(fighter.module_accessor, vars::sonic::status::float::SPECIAL_AIR_LW_BOUND_SPEED_Y) * 0.6;
    let speed = speed.clamp(0.5, 10.0);

    fighter.set_situation(SITUATION_KIND_AIR.into());
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        -air_accel_y * 1.5
    );
    sv_kinetic_energy!(
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        -1.0
    );
    let angle = VarModule::get_float(fighter.module_accessor, vars::sonic::status::float::SPECIAL_AIR_LW_BOUND_ANGLE);
    let speed_x = angle.cos() * speed;
    let speed_y = angle.sin() * speed;
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        speed_x + speed_x_add,
        0.0
    );
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        speed_y.abs()
    );

    let jump_count = VarModule::get_int(fighter.module_accessor, vars::sonic::status::int::SPECIAL_AIR_LW_JUMP_RESERVE);
    WorkModule::set_int(fighter.module_accessor, jump_count, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, vars::sonic::status::SPECIAL_AIR_LW_BOUND, sonic_special_air_lw_bound_pre);
    agent.status(Init, vars::sonic::status::SPECIAL_AIR_LW_BOUND, sonic_special_air_lw_bound_init);
    agent.status(Main, vars::sonic::status::SPECIAL_AIR_LW_BOUND, sonic_special_air_lw_bound_main);
    agent.status(Exec, vars::sonic::status::SPECIAL_AIR_LW_BOUND, sonic_special_air_lw_bound_exec);
}