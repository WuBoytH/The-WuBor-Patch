use super::*;

unsafe extern "C" fn sonic_special_air_s_launch_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP),
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
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn sonic_special_air_s_launch_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.module_accessor, vars::sonic::status::flag::SPECIAL_AIR_S_CHECK_HIT);

    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_air_s_launch"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    fighter.sub_shift_status_main(L2CValue::Ptr(sonic_special_air_s_launch_main_loop as *const () as _))
}

unsafe extern "C" fn sonic_special_air_s_launch_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        WorkModule::set_float(fighter.module_accessor, 16.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        return 0.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, vars::sonic::status::SPECIAL_AIR_S_LAUNCH, sonic_special_air_s_launch_pre);
    agent.status(Main, vars::sonic::status::SPECIAL_AIR_S_LAUNCH, sonic_special_air_s_launch_main);
}
