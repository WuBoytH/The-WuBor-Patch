use super::*;
use super::super::helper::*;

unsafe extern "C" fn lucina_special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let turn = if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() != vars::yu::status::SPECIAL_HI_COMMAND {
        *FIGHTER_STATUS_ATTR_START_TURN as u32
    }
    else {
        0
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
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI as u64,
        turn,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn lucina_special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_FALL_SPECIAL);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_hi"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    else {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_hi"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame"));
    WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    if !StopModule::is_stop(fighter.module_accessor) {
        lucina_special_hi_substatus(fighter);
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(lucina_special_hi_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_special_hi_main_loop as *const () as _))
}

unsafe extern "C" fn lucina_special_hi_substatus(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    0.into()
}

unsafe extern "C" fn lucina_special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if CustomCancelManager::execute_cancel(fighter) {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            return 0.into();
        }
    }
    else {
        if MotionModule::frame(fighter.module_accessor) >= 36.0 {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn lucina_special_hi_exec(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn lucina_special_hi_command_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let original = original_status(Init, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI);
    original(fighter)
}

unsafe extern "C" fn lucina_special_hi_command_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if spent_meter(fighter.module_accessor, false) {
        let spent = VarModule::get_float(fighter.module_accessor, vars::yu::instance::float::SPENT_SP);
        let meter_max = VarModule::get_float(fighter.module_accessor, vars::yu::instance::float::SP_GAUGE_MAX);
        FGCModule::update_meter(fighter.module_accessor, -spent, meter_max, vars::yu::instance::float::SP_GAUGE);
        VarModule::set_int(fighter.module_accessor, vars::yu::instance::int::SP_FLASH_TIMER, 40);
        VarModule::on_flag(fighter.module_accessor, vars::yu::status::flag::IS_EX);
        sp_diff_checker(fighter.module_accessor);
    }
    lucina_special_hi_main(fighter)
}

unsafe extern "C" fn lucina_special_hi_command_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let original = original_status(End, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI);
    original(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, lucina_special_hi_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, lucina_special_hi_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, lucina_special_hi_exec);

    agent.status(Pre, vars::yu::status::SPECIAL_HI_COMMAND, lucina_special_hi_pre);
    agent.status(Init, vars::yu::status::SPECIAL_HI_COMMAND, lucina_special_hi_command_init);
    agent.status(Main, vars::yu::status::SPECIAL_HI_COMMAND, lucina_special_hi_command_main);
    agent.status(End, vars::yu::status::SPECIAL_HI_COMMAND, lucina_special_hi_command_end);
}