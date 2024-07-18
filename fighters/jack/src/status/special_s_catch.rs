use super::*;

pub unsafe extern "C" fn jack_special_s_catch_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_RESET,
        *GROUND_CORRECT_KIND_NONE as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
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
        true,
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

pub unsafe extern "C" fn jack_special_s_catch_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_s1_catch"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    WorkModule::set_float(fighter.module_accessor, -12.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CHECK_DEAD_OFFSET_Y);
    fighter.sub_shift_status_main(L2CValue::Ptr(jack_special_s_catch_main_loop as *const () as _))
}

unsafe extern "C" fn jack_special_s_catch_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(vars::jack::status::SPECIAL_S_CATCH_JUMP.into(), false.into());
        return 0.into();
    }
    CatchModule::update_pos_cling(fighter.module_accessor);
    0.into()
}

unsafe extern "C" fn jack_special_s_catch_check_dmg(fighter: &mut L2CFighterCommon, _param_1: &L2CValue) -> L2CValue {
    let ret = CatchModule::check_damage(fighter.module_accessor) != 0;
    ret.into()
}

unsafe extern "C" fn jack_special_s_catch_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CHECK_DEAD_OFFSET_Y);
    if fighter.global_table[STATUS_KIND].get_i32() != vars::jack::status::SPECIAL_S_CATCH_JUMP {
        CatchModule::cling_cut(fighter.module_accessor, false);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, vars::jack::status::SPECIAL_S_CATCH, jack_special_s_catch_pre);
    agent.status(Main, vars::jack::status::SPECIAL_S_CATCH, jack_special_s_catch_main);
    agent.status(CheckDamage, vars::jack::status::SPECIAL_S_CATCH, jack_special_s_catch_check_dmg);
    agent.status(End, vars::jack::status::SPECIAL_S_CATCH, jack_special_s_catch_end);
}