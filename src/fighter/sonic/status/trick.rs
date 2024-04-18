use crate::imports::*;

unsafe extern "C" fn sonic_trick_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_AIR as u32,
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
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ADDITIONS_ATTACK_01 |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        0,
        0,
        0
    );
    0.into()
}

unsafe extern "C" fn sonic_trick_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RESET);
    0.into()
}

unsafe extern "C" fn sonic_trick_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let lr = PostureModule::lr(fighter.module_accessor);
    let mot = if stick_x * lr > 0.5 {
        Hash40::new("trick_f")
    }
    else if stick_x * lr < -0.5 {
        Hash40::new("trick_b")
    }
    else {
        Hash40::new("trick_hi")
    };
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
    fighter.sub_shift_status_main(L2CValue::Ptr(sonic_trick_main_loop as *const () as _))
}

unsafe extern "C" fn sonic_trick_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        WorkModule::set_float(fighter.module_accessor, 20.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        return 0.into();
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn sonic_trick_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, sonic::status::flag::TRICK_ENABLE_MOVEMENT) {
        VarModule::off_flag(fighter.module_accessor, sonic::status::flag::TRICK_ENABLE_MOVEMENT);
        let mot = MotionModule::motion_kind(fighter.module_accessor);
        let vel: (f32, f32) = if mot == hash40("trick_f") {
            (2.5, 0.5)
        }
        else if mot == hash40("trick_b") {
            (-2.0, 1.0)
        }
        else {
            (0.0, 2.4)
        };
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            vel.1
        );
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            0.0,
            0.0
        );
        let brake_x = if mot == hash40("trick_b") {
            0.05
        }
        else {
            0.025
        };
        sv_kinetic_energy!(
            set_brake,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            brake_x,
            0.0
        );
        let lr = PostureModule::lr(fighter.module_accessor);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            vel.0 * lr,
            0.0
        );
    }
    if VarModule::is_flag(fighter.module_accessor, sonic::status::flag::TRICK_ENABLE_CONTROL) {
        VarModule::off_flag(fighter.module_accessor, sonic::status::flag::TRICK_ENABLE_CONTROL);
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
            controller_set_accel_x_add,
            fighter,
            0.01
        );
        sv_kinetic_energy!(
            controller_set_accel_x_mul,
            fighter,
            0.01
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Pre, sonic::status::TRICK, sonic_trick_pre);
    agent.status(smashline::Init, sonic::status::TRICK, sonic_trick_init);
    agent.status(smashline::Main, sonic::status::TRICK, sonic_trick_main);
    agent.status(smashline::Exec, sonic::status::TRICK, sonic_trick_exec);
}