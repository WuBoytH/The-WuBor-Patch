use super::*;

pub unsafe extern "C" fn jack_special_s_catch_jump_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_NONE as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
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
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

pub unsafe extern "C" fn jack_special_s_catch_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::reverse_lr(fighter.module_accessor);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_DIDDY_STATUS_SPECIAL_S_TRANSITION_TERM_ID_GROUND);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_s1_catch_jump"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DIDDY_STATUS_MONKEY_FLIP_FLAG_JUMP_START);
    if !StopModule::is_stop(fighter.module_accessor) {
        jack_special_s_catch_jump_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(jack_special_s_catch_jump_substatus as *const () as _));

    let model_flag = LinkModule::get_model_constraint_flag(fighter.module_accessor) as u32;
    LinkModule::set_model_constraint_flag(fighter.module_accessor, model_flag | *CONSTRAINT_FLAG_OFFSET_ROT as u32);
    LinkModule::set_constraint_rot_offset_y(fighter.module_accessor, 180.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(jack_special_s_catch_jump_main_loop as *const () as _))
}

unsafe extern "C" fn jack_special_s_catch_jump_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DIDDY_STATUS_MONKEY_FLIP_FLAG_JUMP_START) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            fighter.set_situation(SITUATION_KIND_AIR.into());
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_DIDDY_STATUS_SPECIAL_S_TRANSITION_TERM_ID_GROUND);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DIDDY_STATUS_MONKEY_FLIP_FLAG_JUMP_START);
        }
    }
    0.into()
}

unsafe extern "C" fn jack_special_s_catch_jump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_DIDDY_STATUS_SPECIAL_S_TRANSITION_TERM_ID_GROUND)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        return 0.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn jack_special_s_catch_jump_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    CatchModule::cling_cut(fighter.module_accessor, false);
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, vars::jack::status::SPECIAL_S_CATCH_JUMP, jack_special_s_catch_jump_pre);
    agent.status(Main, vars::jack::status::SPECIAL_S_CATCH_JUMP, jack_special_s_catch_jump_main);
    agent.status(End, vars::jack::status::SPECIAL_S_CATCH_JUMP, jack_special_s_catch_jump_end);
}