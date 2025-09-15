use super::*;

unsafe extern "C" fn dolly_special_lw_attack_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_AIR as u32,
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
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn dolly_special_lw_attack_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let angle = -60.0_f32.to_radians();
    let speed = 2.8;

    let speed_x = speed * angle.cos();
    let speed_y = speed * angle.sin();

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
        set_stable_speed,
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
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        speed_y
    );

    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        0.0
    );

    sv_kinetic_energy!(
        set_brake,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        0.0
    );

    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

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
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        speed_x,
        -1.0
    );

    sv_kinetic_energy!(
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        speed_x,
        -1.0
    );

    let lr = PostureModule::lr(fighter.module_accessor);
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        speed_x * lr,
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

    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);

    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);

    0.into()
}

unsafe extern "C" fn dolly_special_lw_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_lw_attack"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    let log = if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_07
    }
    else {
        let command_power_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("command_power_mul"));
        AttackModule::set_power_mul_status(fighter.module_accessor, command_power_mul);
        *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_08
    };
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), log - 1);
    fighter.sub_shift_status_main(L2CValue::Ptr(dolly_special_lw_attack_main_loop as *const () as _))
}

unsafe extern "C" fn dolly_special_lw_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 0.into();
        }
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_LW_WORK_FLAG_LANDING_HEAVY) {
            let param = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
                hash40("landing_heavy_frame_command")
            }
            else {
                hash40("landing_heavy_frame")
            };
            let landing_heavy_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), param);
            WorkModule::set_int(fighter.module_accessor, landing_heavy_frame, *FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_LANDING_STIFFNESS_FRAME);
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
            return 0.into();
        }
        let frame = fighter.global_table[STATUS_FRAME].get_f32() as i32;
        let start_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_LW_WORK_INT_START_FRAME);
        let attack_no_landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("attack_no_landing_frame"));
        if frame + start_frame >= attack_no_landing_frame {
            fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_LANDING.into(), false.into());
            return 0.into();
        }
    }
    0.into()
}

unsafe extern "C" fn dolly_special_lw_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let param = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_LW_WORK_FLAG_HIT) {
        hash40("landing_frame_hit")
    }
    else {
        hash40("landing_frame_fail")
    };
    let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), param);
    WorkModule::set_float(fighter.module_accessor, landing_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    VarModule::off_flag(fighter.module_accessor, vars::dolly::status::flag::IS_SPECIAL_CANCEL);
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK, dolly_special_lw_attack_pre);
    agent.status(Init, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK, dolly_special_lw_attack_init);
    agent.status(Main, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK, dolly_special_lw_attack_main);
    agent.status(End, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK, dolly_special_lw_attack_end);
}