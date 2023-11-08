use crate::imports::status_imports::*;

unsafe extern "C" fn marth_special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let attack_type;
    let power_up;
    if !VarModule::is_flag(fighter.module_accessor, marth::instance::flag::IS_STANCE) {
        attack_type = (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW
            | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
            | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64;
        power_up = *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32
    }
    else {
        attack_type = *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI as u64;
        power_up = *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32;
    };
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
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
        attack_type,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        power_up,
        0
    );
    0.into()
}

unsafe extern "C" fn marth_special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.module_accessor, marth::status::flag::DISABLE_STANCE_CHANGE);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_FALL_SPECIAL);
    let is_stance = VarModule::is_flag(fighter.module_accessor, marth::instance::flag::IS_STANCE);
    let mot;
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if is_stance {
            mot = Hash40::new("special_lw_special_hi");
        }
        else {
            mot = Hash40::new("special_hi");
        }
    }
    else {
        if is_stance {
            mot = Hash40::new("special_lw_special_hi");
        }
        else {
            mot = Hash40::new("special_air_hi");
        }
    }
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
    let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame"));
    WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    if !StopModule::is_stop(fighter.module_accessor) {
        marth_special_hi_substatus(fighter);
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(marth_special_hi_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_special_hi_main_loop as *const () as _))
}

unsafe extern "C" fn marth_special_hi_substatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(fighter.module_accessor, marth::instance::flag::IS_STANCE) {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_KINETIC_CHANGE) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE) {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                fighter.set_situation(SITUATION_KIND_AIR.into());
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_KINETIC_CHANGE);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE_SET_ANGLE);
            }
        }
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE) {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            fighter.set_situation(SITUATION_KIND_AIR.into());
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_KINETIC_CHANGE) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_KINETIC_CHANGE);
        }
    }
    0.into()
}

unsafe extern "C" fn marth_special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            return 0.into();
        }
    }
    else {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_FALL_SPECIAL)
        && MotionModule::frame(fighter.module_accessor) >= 20.0 {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn marth_special_hi_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(fighter.module_accessor, marth::instance::flag::IS_STANCE) {
        let original = smashline::original_status(smashline::Exec, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI);
        original(fighter);
    }
    0.into()
}

unsafe extern "C" fn marth_special_hi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame"));
    WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_CANCEL);
    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
        let fall_x_mul_value = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("fall_x_mul_value"));
        WorkModule::set_float(fighter.module_accessor, fall_x_mul_value, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
    }
    VarModule::off_flag(fighter.module_accessor, marth::instance::flag::IS_STANCE);
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, marth_special_hi_pre);
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, marth_special_hi_main);
    agent.status(smashline::Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, marth_special_hi_exec);
    agent.status(smashline::End, *FIGHTER_STATUS_KIND_SPECIAL_HI, marth_special_hi_end);
}