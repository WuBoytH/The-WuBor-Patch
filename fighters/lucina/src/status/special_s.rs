use super::*;
use super::super::helper::*;

unsafe extern "C" fn lucina_special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_turn = if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() != vars::yu::status::SPECIAL_S_COMMAND {
        *FIGHTER_STATUS_ATTR_START_TURN as u32
    }
    else {
        0
    };
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
        *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        is_turn,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn lucina_special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT);
    VarModule::on_flag(fighter.module_accessor, vars::yu::instance::flag::DISABLE_SPECIAL_N_S);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_air_s1"),
        1.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_raginglion_loop as *const () as _))
}

unsafe extern "C" fn lucina_raginglion_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE) {
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2.into(), false.into());
        }
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn lucina_special_s_command_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let original = original_status(Init, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S);
    original(fighter)
}

unsafe extern "C" fn lucina_special_s_command_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if spent_meter(fighter.module_accessor, false) {
        let spent = VarModule::get_float(fighter.module_accessor, vars::yu::instance::float::SPENT_SP);
        let meter_max = VarModule::get_float(fighter.module_accessor, vars::yu::instance::float::SP_GAUGE_MAX);
        FGCModule::update_meter(fighter.module_accessor, -spent, meter_max, vars::yu::instance::float::SP_GAUGE);
        VarModule::set_int(fighter.module_accessor, vars::yu::instance::int::SP_FLASH_TIMER, 40);
        VarModule::on_flag(fighter.module_accessor, vars::yu::status::flag::IS_EX);
        sp_diff_checker(fighter.module_accessor);
    }
    lucina_special_s_main(fighter)
}

unsafe extern "C" fn lucina_special_s_command_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let original = original_status(Exec, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S);
    original(fighter)
}

unsafe extern "C" fn lucina_special_s_command_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let original = original_status(End, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S);
    original(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, lucina_special_s_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, lucina_special_s_main);

    agent.status(Pre, vars::yu::status::SPECIAL_S_COMMAND, lucina_special_s_pre);
    agent.status(Init, vars::yu::status::SPECIAL_S_COMMAND, lucina_special_s_command_init);
    agent.status(Main, vars::yu::status::SPECIAL_S_COMMAND, lucina_special_s_command_main);
    agent.status(Exec, vars::yu::status::SPECIAL_S_COMMAND, lucina_special_s_command_exec);
    agent.status(End, vars::yu::status::SPECIAL_S_COMMAND, lucina_special_s_command_end);
}