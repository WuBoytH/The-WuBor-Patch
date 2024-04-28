use crate::imports::*;

unsafe extern "C" fn sonic_special_air_s_start_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_AIR as u32,
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
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn sonic_special_air_s_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.module_accessor, fighter::instance::flag::DISABLE_SPECIAL_S);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_air_s_start"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(sonic_special_air_s_start_main_loop as *const () as _))
}

unsafe extern "C" fn sonic_special_air_s_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(sonic::status::SPECIAL_AIR_S_HOLD.into(), false.into());
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, sonic::status::SPECIAL_AIR_S_START, sonic_special_air_s_start_pre);
    agent.status(Main, sonic::status::SPECIAL_AIR_S_START, sonic_special_air_s_start_main);
}