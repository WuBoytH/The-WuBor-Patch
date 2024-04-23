use crate::imports::*;
use super::helper;

unsafe extern "C" fn ike_special_s_dash_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.module_accessor, fighter::instance::flag::DISABLE_SPECIAL_S);
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false, // Disables Jostle
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
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn ike_special_s_dash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    GroundModule::select_cliff_hangdata(fighter.module_accessor, 1);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT);
    let special_s_dash_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_dash_frame"));
    WorkModule::set_int(fighter.module_accessor, special_s_dash_frame, *FIGHTER_IKE_STATUS_SPECIAL_S_WORK_INT_DASH_COUNT);
    if !StopModule::is_stop(fighter.module_accessor) {
        ike_special_s_dash_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(ike_special_s_dash_substatus as *const () as _));
    ike_special_s_dash_mot_helper(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(ike_special_s_dash_main_loop as *const () as _))
}

unsafe extern "C" fn ike_special_s_dash_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_S_WORK_INT_DASH_COUNT);
    }
    0.into()
}

unsafe extern "C" fn ike_special_s_dash_mot_helper(fighter: &mut L2CFighterCommon) {
    let mot = if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
        Hash40::new("special_air_s_dash")
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_NONE.into());
        Hash40::new("special_s_dash")
    };
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT) {
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
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT);
    }
    else {
        MotionModule::change_motion_inherit_frame(
            fighter.module_accessor,
            mot,
            -1.0,
            1.0,
            0.0,
            false,
            false
        );
    }
}

unsafe extern "C" fn ike_special_s_dash_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }
    if fighter.sub_ground_check_stop_wall().get_bool() {
        return 0.into();
    }
    let dash_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_S_WORK_INT_DASH_COUNT);
    if dash_count <= 0 {
        fighter.change_status(FIGHTER_IKE_STATUS_KIND_SPECIAL_S_END.into(), false.into());
        return 0.into();
    }
    // <WuBor>
    let pad_flag = fighter.global_table[PAD_FLAG].get_i32();
    // Press Attack or Special to attack
    if pad_flag & (*FIGHTER_PAD_FLAG_ATTACK_TRIGGER | *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER) != 0 {
        fighter.change_status(FIGHTER_IKE_STATUS_KIND_SPECIAL_S_ATTACK.into(), false.into());
        return 0.into();
    }
    // Press Shield to end the dash early
    if pad_flag & *FIGHTER_PAD_FLAG_GUARD_TRIGGER != 0 {
        fighter.change_status(FIGHTER_IKE_STATUS_KIND_SPECIAL_S_END.into(), false.into());
        return 0.into();
    }
    // </WuBor>
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        ike_special_s_dash_mot_helper(fighter);
    }
    helper::special_s::ike_special_s_main_loop_helper(fighter);
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_DASH, ike_special_s_dash_pre);
    agent.status(Main, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_DASH, ike_special_s_dash_main);
}