use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    custom_status::*,
    super::{
        helper::*,
        super::vars::*
    }
};

// Jab/Tilt common pre function

unsafe extern "C" fn marth_speciallw_attack_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW
            | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
            | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK

unsafe extern "C" fn marth_speciallw_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_lw_attack_11"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_attack_main_loop as *const () as _))
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK_LW3

unsafe extern "C" fn marth_speciallw_attack_lw3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_lw_attack_lw3"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_attack_main_loop as *const () as _))
}

// Jab/Tilt common main loop function

unsafe extern "C" fn marth_speciallw_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if marth_stance_cancel_helper(fighter).get_bool() {
        return 1.into();
    }
    if marth_stance_dash_cancel_helper(fighter, true).get_bool() {
        return 1.into();
    }
    marth_stance_mot_end_helper(fighter);
    0.into()
}

// Jab/Tilt common end function

unsafe extern "C" fn marth_speciallw_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_PARRY_XLU) {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_PARRY_XLU);
    }
    marth_speciallw_common_end(fighter);
    0.into()
}

pub fn install() {
    CustomStatusManager::add_new_agent_status_script(
        Hash40::new("fighter_kind_marth"),
        FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK,
        StatusInfo::new()
            .with_pre(marth_speciallw_attack_pre)
            .with_main(marth_speciallw_attack_main)
            .with_end(marth_speciallw_attack_end)
    );
    CustomStatusManager::add_new_agent_status_script(
        Hash40::new("fighter_kind_marth"),
        FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK_LW3,
        StatusInfo::new()
            .with_pre(marth_speciallw_attack_pre)
            .with_main(marth_speciallw_attack_lw3_main)
            .with_end(marth_speciallw_attack_end)
    );
}