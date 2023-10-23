use crate::imports::status_imports::*;

#[status_script(agent = "ridley", status = FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn ridley_special_hi_charge_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    ridley_special_hi_charge_pre_inner(fighter)
}

#[status_script(agent = "ridley", status = FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_F, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn ridley_special_hi_charge_f_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    ridley_special_hi_charge_pre_inner(fighter)
}

#[status_script(agent = "ridley", status = FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_B, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn ridley_special_hi_charge_b_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    ridley_special_hi_charge_pre_inner(fighter)
}

#[status_script(agent = "ridley", status = FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn ridley_special_hi_charge_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    ridley_special_hi_charge_pre_inner(fighter)
}

unsafe fn ridley_special_hi_charge_pre_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), // Was ALWAYS_BOTH_SIDES
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
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}

#[status_script(agent = "ridley", status = FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn ridley_special_hi_charge_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ridley_special_hi_charge_main_inner(
        fighter,
        hash40("special_air_hi_charge_hi").into(),
        hash40("charge_frame_hi").into(),
        hash40("charge_degree_hi").into(),
        hash40("charge_speed_hi").into()
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(ridley_special_hi_charge_hi_b_main_loop as *const () as _))
}

#[status_script(agent = "ridley", status = FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_F, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn ridley_special_hi_charge_f_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ridley_special_hi_charge_main_inner(
        fighter,
        hash40("special_air_hi_charge_f").into(),
        hash40("charge_frame_f").into(),
        hash40("charge_degree_f").into(),
        hash40("charge_speed_f").into()
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(ridley_special_hi_charge_f_main_loop as *const () as _))
}

#[status_script(agent = "ridley", status = FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_B, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn ridley_special_hi_charge_b_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ridley_special_hi_charge_main_inner(
        fighter,
        hash40("special_air_hi_charge_b").into(),
        hash40("charge_frame_b").into(),
        hash40("charge_degree_b").into(),
        hash40("charge_speed_b").into()
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(ridley_special_hi_charge_hi_b_main_loop as *const () as _))
}

unsafe fn ridley_special_hi_charge_main_inner(
    fighter: &mut L2CFighterCommon,
    motion: L2CValue,
    frame_param: L2CValue,
    degree_param: L2CValue,
    speed_param: L2CValue
) {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_CHARGE_DECCEL);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_WORK_INT_CHARGE_COUNT);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new_raw(motion.get_u64()),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    let frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), frame_param.get_u64());
    let end_frame = MotionModule::end_frame(fighter.module_accessor);
    MotionModule::set_rate(fighter.module_accessor, end_frame / frame as f32);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    let degree = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), degree_param.get_u64());
    let speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), speed_param.get_u64());
    let lr = PostureModule::lr(fighter.module_accessor);
    let rad = degree.to_radians();
    let speed_x = rad.cos() * speed * lr;
    let speed_y = rad.sin() * speed;
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        ENERGY_STOP_RESET_TYPE_AIR,
        speed_x,
        speed_y,
        0.0,
        0.0,
        0.0
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
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        speed_x,
        speed_y
    );
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        speed_x,
        speed_y
    );
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticUtility::reset_enable_energy(
        *FIGHTER_KINETIC_ENERGY_ID_MOTION,
        fighter.module_accessor,
        *ENERGY_MOTION_RESET_TYPE_AIR_TRANS,
        &Vector2f{x: 0.0, y:0.0},
        &Vector3f{x: 0.0, y: 0.0, z: 0.0}
    );
}

unsafe fn ridley_special_hi_charge_hi_b_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Normally has stuff for bonking but it's removed
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_END.into(), false.into());
    }
    0.into()
}

unsafe fn ridley_special_hi_charge_f_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Normally has stuff for bonking but it's removed
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_LANDING_F);
        fighter.change_status(FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_LANDING.into(), false.into());
        return 0.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_END.into(), false.into());
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        ridley_special_hi_charge_hi_pre,
        ridley_special_hi_charge_hi_main,

        ridley_special_hi_charge_f_pre,
        ridley_special_hi_charge_f_main,

        ridley_special_hi_charge_b_pre,
        ridley_special_hi_charge_b_main,

        ridley_special_hi_charge_lw_pre
    );
}