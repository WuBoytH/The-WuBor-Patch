use super::*;
use super::super::vl;

unsafe extern "C" fn special_lw_jump_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS),
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
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn special_lw_jump_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let lr = PostureModule::lr(fighter.module_accessor);
    let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

    let is_blj = fighter.global_table[PREV_STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_SPECIAL_LW
        && sum_speed_x * lr < 0.0
        && stick_x * lr < 0.0;
    
    VarModule::set_flag(fighter.module_accessor, vars::mario::status::flag::SPECIAL_LW_IS_BLJ, is_blj);

    let speed_x = if sum_speed_x * lr >= 0.0 && sum_speed_x.abs() < vl::param_special_lw::long_jump_speed_min_x {
        vl::param_special_lw::long_jump_speed_min_x * lr
    }
    else {
        sum_speed_x * 1.5
    };

    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_STOP, fighter.module_accessor);

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
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        5.0,
        -1.0
    );
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        speed_x,
        0.0
    );

    let air_accel_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_add"), 0);
    let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
    sv_kinetic_energy!(
        controller_set_accel_x_add,
        fighter,
        air_accel_x_add * 1.25
    );
    sv_kinetic_energy!(
        controller_set_accel_x_mul,
        fighter,
        air_accel_x_mul * 1.25
    );

    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);

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

    let speed_y = if is_blj {
        vl::param_special_lw::long_jump_speed_y_back
    }
    else {
        vl::param_special_lw::long_jump_speed_y
    };
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        speed_y
    );

    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    0.into()
}

unsafe extern "C" fn special_lw_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_lw_jump"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_jump_main_loop as *const () as _))
}

unsafe extern "C" fn special_lw_jump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_lw_jump_power_handler(fighter);

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(vars::mario::status::SPECIAL_LW_LANDING.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn special_lw_jump_power_handler(fighter: &mut L2CFighterCommon) {
    if AttackModule::is_attack(fighter.module_accessor, 0, false) {
        let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN).abs();
        AttackModule::set_power(fighter.module_accessor, 0, 4.0 * sum_speed_x, false);
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, vars::mario::status::SPECIAL_LW_JUMP, special_lw_jump_pre);
    agent.status(Init, vars::mario::status::SPECIAL_LW_JUMP, special_lw_jump_init);
    agent.status(Main, vars::mario::status::SPECIAL_LW_JUMP, special_lw_jump_main);
}