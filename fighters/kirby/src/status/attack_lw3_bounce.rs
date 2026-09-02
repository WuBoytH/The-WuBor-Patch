use super::*;

pub unsafe extern "C" fn kirby_attacklw3_bounce_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_FALL,
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
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_3 as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn kirby_attacklw3_bounce_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    FighterControlModuleImpl::update_attack_air_kind(fighter.module_accessor, true);

    let rate = if VarModule::is_flag(fighter.module_accessor, vars::kirby::status::flag::ATTACK_LW3_HIT) {
        31.0 / 20.0
    }
    else {
        1.0
    };
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("jump_b"), 22.0, rate, false, 0.0, false, false);

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
    let lr = PostureModule::lr(fighter.module_accessor);
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        -0.5 * lr,
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
        1.5
    );

    fighter.sub_shift_status_main(L2CValue::Ptr(kirby_attacklw3_bounce_main_loop as *const () as _))
}

unsafe extern "C" fn kirby_attacklw3_bounce_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    else if MotionModule::frame(fighter.module_accessor) >= vl::param_special_lw::slide_bounce_cancel_frame
    && VarModule::is_flag(fighter.module_accessor, vars::kirby::status::flag::ATTACK_LW3_HIT) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        return 0.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, vars::kirby::status::ATTAK_LW3_BOUNCE, kirby_attacklw3_bounce_pre);
    agent.status(Main, vars::kirby::status::ATTAK_LW3_BOUNCE, kirby_attacklw3_bounce_main);
}