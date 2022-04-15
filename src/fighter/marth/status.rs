use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    wubor_utils::table_const::*,
    custom_status::*,
    super::vars::*
};

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ENTER

unsafe extern "C" fn marth_speciallw_enter_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
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
        0,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        0,
        0
    );
    0.into()
}

unsafe extern "C" fn marth_speciallw_enter_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_lw_enter"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_enter_main_loop as *const () as _))
}

unsafe extern "C" fn marth_speciallw_enter_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if marth_stance_cancel_helper(fighter).get_bool() {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_WAIT);
        fighter.change_status(status.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn marth_speciallw_enter_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    marth_speciallw_common_end(fighter);
    0.into()
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_WAIT

unsafe extern "C" fn marth_speciallw_wait_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        0,
        (*FIGHTER_STATUS_KIND_ITEM_ASSIST_HOIST | *FIGHTER_STATUS_ATTR_INTO_DOOR | *FIGHTER_STATUS_ATTR_ENABLE_ROCKETBELT_EJECT) as u32,
        0,
        0
    );
    0.into()
}

unsafe extern "C" fn marth_speciallw_wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_lw_wait"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_wait_main_loop as *const () as _))
}

unsafe extern "C" fn marth_speciallw_wait_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_IS_STANCE) {
        let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_EXIT);
        fighter.change_status(status.into(), true.into());
        return 1.into();
    }
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 {
        let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK);
        fighter.change_status(status.into(), true.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn marth_speciallw_wait_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    marth_speciallw_common_end(fighter);
    0.into()
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_EXIT

unsafe extern "C" fn marth_speciallw_exit_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
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
        0,
        0,
        0,
        0
    );
    0.into()
}

unsafe extern "C" fn marth_speciallw_exit_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_lw_exit"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_exit_main_loop as *const () as _))
}

unsafe extern "C" fn marth_speciallw_exit_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn marth_speciallw_exit_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    marth_speciallw_common_end(fighter);
    0.into()
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK

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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ADDITIONS_ATTACK_01 | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

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

unsafe extern "C" fn marth_speciallw_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if marth_stance_cancel_helper(fighter).get_bool() {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_WAIT);
        fighter.change_status(status.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn marth_speciallw_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    marth_speciallw_common_end(fighter);
    0.into()
}

// Some common functions used for the stance

unsafe extern "C" fn marth_stance_cancel_helper(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
    && !fighter.global_table[IN_HITLAG].get_bool() {
        CancelModule::enable_cancel(fighter.module_accessor);
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_IS_STANCE) {
            let cat1 = fighter.global_table[CMD_CAT1].get_i32();
            let curr_status = fighter.global_table[STATUS_KIND].get_i32();
            let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK);
            if curr_status < status
            && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 {
                fighter.change_status(status.into(), true.into());
                return true.into();
            }
        }
        else {
            if fighter.sub_transition_group_check_ground_special().get_bool()
            || fighter.sub_transition_group_check_ground_attack().get_bool()
            || fighter.sub_transition_group_check_air_special().get_bool() 
            || fighter.sub_transition_group_check_air_attack().get_bool() {
                return true.into();
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor)
    && !WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_IS_STANCE) {
        let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_EXIT);
        fighter.change_status(status.into(), true.into());
        return true.into();
    }
    false.into()
}

unsafe extern "C" fn marth_speciallw_common_end(fighter: &mut L2CFighterCommon) {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status < CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ENTER)
    && status != *FIGHTER_STATUS_KIND_SPECIAL_LW {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_IS_STANCE);
    }
}

pub fn install() {
    CustomStatusManager::add_new_agent_status_script(
        Hash40::new("fighter_kind_marth"),
        FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ENTER,
        StatusInfo::new()
            .with_pre(marth_speciallw_enter_pre)
            .with_main(marth_speciallw_enter_main)
            .with_end(marth_speciallw_enter_end)
    );
    CustomStatusManager::add_new_agent_status_script(
        Hash40::new("fighter_kind_marth"),
        FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_WAIT,
        StatusInfo::new()
            .with_pre(marth_speciallw_wait_pre)
            .with_main(marth_speciallw_wait_main)
            .with_end(marth_speciallw_wait_end)
    );
    CustomStatusManager::add_new_agent_status_script(
        Hash40::new("fighter_kind_marth"),
        FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_EXIT,
        StatusInfo::new()
            .with_pre(marth_speciallw_exit_pre)
            .with_main(marth_speciallw_exit_main)
            .with_end(marth_speciallw_exit_end)
    );
    CustomStatusManager::add_new_agent_status_script(
        Hash40::new("fighter_kind_marth"),
        FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK,
        StatusInfo::new()
            .with_pre(marth_speciallw_attack_pre)
            .with_main(marth_speciallw_attack_main)
            .with_end(marth_speciallw_attack_end)
    );
}