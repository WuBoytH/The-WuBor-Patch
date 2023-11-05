use {
    crate::imports::status_imports::*,
    super::super::vl
};

#[status_script(agent = "daisy", status = FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn daisy_uniqfloatstart_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_RESET,
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
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64,
        (*FIGHTER_STATUS_ATTR_CLEAR_MOTION_ENERGY | *FIGHTER_STATUS_ATTR_START_TURN) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

#[status_script(agent = "daisy", status = FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn daisy_uniqfloatstart_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_float(fighter.module_accessor, 10.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("fuwafuwa_start"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(daisy_uniqfloatstart_main_loop as *const () as _))
}

unsafe extern "C" fn daisy_uniqfloatstart_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.sub_transition_group_check_air_cliff().get_bool()
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
    }
    0.into()
}

#[status_script(agent = "daisy", status = FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn daisy_uniqfloatstart_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_FALL {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            vl::param_special_lw::limit_speed_y
        );
        fighter.clear_lua_stack();
    }
    0.into()
}

#[status_script(agent = "daisy", status = FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn daisy_uniqfloatstart_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL
    && ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR) {
        ArticleModule::remove(fighter.module_accessor, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    0.into()
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Pre, *FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START, daisy_uniqfloatstart_pre);
    agent.status(smashline::Main, *FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START, daisy_uniqfloatstart_main);
    agent.status(smashline::Exec, *FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START, daisy_uniqfloatstart_exec);
    agent.status(smashline::End, *FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START, daisy_uniqfloatstart_end);
}