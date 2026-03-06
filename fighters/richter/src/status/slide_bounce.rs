use super::*;

unsafe extern "C" fn slide_bounce_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_NONE,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_1 as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn slide_bounce_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
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

    let (speed_x, speed_y) = if VarModule::is_flag(fighter.module_accessor, vars::richter::status::flag::SLIDE_BOUNCE_IS_HIT) {
        (-0.7, 1.8)
    }
    else {
        (-0.7, 0.8)
    };
    let lr = PostureModule::lr(fighter.module_accessor);
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        speed_x * lr,
        0.0
    );

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

    if VarModule::is_flag(fighter.module_accessor, vars::richter::status::flag::SLIDE_BOUNCE_IS_HIT) {
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

    0.into()
}

unsafe extern "C" fn slide_bounce_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let (motion, rate) = if VarModule::is_flag(fighter.module_accessor, vars::richter::status::flag::SLIDE_BOUNCE_IS_HIT) {
        (Hash40::new("fall_leaning_c"), 1.0)
    }
    else {
        let end_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new("fall_leaning_d"));
        (Hash40::new("fall_leaning_d"), end_frame / 28.0)
    };
    MotionModule::change_motion(
        fighter.module_accessor,
        motion,
        0.0,
        rate,
        false,
        0.0,
        false,
        false
    );

    fighter.sub_shift_status_main(L2CValue::Ptr(slide_bounce_main_loop as *const () as _))
}

unsafe extern "C" fn slide_bounce_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if VarModule::is_flag(fighter.module_accessor, vars::richter::status::flag::SLIDE_BOUNCE_IS_HIT) {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
            return 0.into();
        }
        else {
            WorkModule::set_float(fighter.module_accessor, 15.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            return 0.into();
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, vars::richter::status::SLIDE_BOUNCE, slide_bounce_pre);
    agent.status(Init, vars::richter::status::SLIDE_BOUNCE, slide_bounce_init);
    agent.status(Main, vars::richter::status::SLIDE_BOUNCE, slide_bounce_main);
}