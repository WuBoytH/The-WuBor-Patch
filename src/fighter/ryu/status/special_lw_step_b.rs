use {
    crate::imports::status_imports::*,
    super::super::helper::*
};

unsafe extern "C" fn ryu_special_lw_step_b_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        (
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON |
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW
        ) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn ryu_special_lw_step_b_init(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn ryu_special_lw_step_b_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, ryu::instance::flag::DENJIN_CHARGE) {
        ryu_denjin_remover(fighter);
        VarModule::on_flag(fighter.module_accessor, ryu::status::flag::USED_DENJIN_CHARGE);
    }
    let mot = if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_GUARD_DAMAGE {
        VarModule::set_int(fighter.module_accessor, ryu::status::int::GUARD_SPECIAL_LW_KIND, ryu::GUARD_SPECIAL_LW_KIND_REVERSAL);
        hash40("special_lw_reversal")
    }
    else {
        hash40("special_lw_impact")
    };
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new_raw(mot),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        ENERGY_MOTION_RESET_TYPE_GROUND_TRANS,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0
    );
    if VarModule::get_int(fighter.module_accessor, ryu::status::int::GUARD_SPECIAL_LW_KIND) == ryu::GUARD_SPECIAL_LW_KIND_IMPACT {
        WorkModule::set_int(fighter.module_accessor, 2, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SUPER_ARMOUR_COUNT);
        DamageModule::set_no_reaction_mode_status(fighter.module_accessor, DamageNoReactionMode{_address: *DAMAGE_NO_REACTION_MODE_ALWAYS as u8}, -1.0, -1.0, -1);
        DamageModule::set_no_reaction_no_effect(fighter.module_accessor, true);
        let defense_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("defense_mul"));
        HitModule::set_defense_mul_status(fighter.module_accessor, defense_mul);
        let hit_stop_frame_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("hit_stop_frame_mul"));
        HitModule::set_hit_stop_mul(fighter.module_accessor, hit_stop_frame_mul, HitStopMulTarget{ _address: *HIT_STOP_MUL_TARGET_ALL as u8 }, 0.0);
        let wind_influence = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("wind_influence"));
        WorkModule::set_float(fighter.module_accessor, wind_influence, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_KINETIC_ENERGY_TYPE_ATTACK_SPEED_MUL);
        MotionModule::set_frame_partial(fighter.module_accessor, *FIGHTER_RYU_MOTION_PART_SET_KIND_INK, 0.0, true);
        VarModule::on_flag(fighter.module_accessor, ryu::status::flag::SPECIAL_LW_IMPACT_ENABLED_ARMOR);
    }
    else {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
    }
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_DAMAGE, fighter.module_accessor);

    fighter.sub_shift_status_main(L2CValue::Ptr(ryu_special_lw_step_b_main_loop as *const () as _))
}

unsafe extern "C" fn ryu_special_lw_step_b_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, ryu::status::flag::SPECIAL_LW_IMPACT_REMOVE_ARMOR) {
        DamageModule::reset_no_reaction_mode_status(fighter.module_accessor);
        HitModule::set_hit_stop_mul(fighter.module_accessor, 1.0, HitStopMulTarget{ _address: *HIT_STOP_MUL_TARGET_ALL as u8 }, 0.0);
        HitModule::set_defense_mul_status(fighter.module_accessor, 1.0);
        VarModule::off_flag(fighter.module_accessor, ryu::status::flag::SPECIAL_LW_IMPACT_ENABLED_ARMOR);
        VarModule::off_flag(fighter.module_accessor, ryu::status::flag::SPECIAL_LW_IMPACT_REMOVE_ARMOR);
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Pre, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B, ryu_special_lw_step_b_pre);
    agent.status(smashline::Init, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B, ryu_special_lw_step_b_init);
    agent.status(smashline::Main, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B, ryu_special_lw_step_b_main);
}