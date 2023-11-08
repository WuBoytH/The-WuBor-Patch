use {
    crate::imports::status_imports::*,
    super::super::helper::*
};

unsafe extern "C" fn ryu_special_lw_step_f_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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

unsafe extern "C" fn ryu_special_lw_step_f_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, ryu::instance::flag::DENJIN_CHARGE) {
        ryu_denjin_remover(fighter);
        VarModule::on_flag(fighter.module_accessor, ryu::status::flag::USED_DENJIN_CHARGE);
    }

    let correct;
    let mot;
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        VarModule::on_flag(fighter.module_accessor, fighter::instance::flag::DISABLE_SPECIAL_LW);
        correct = *GROUND_CORRECT_KIND_AIR;
        mot = hash40("special_air_lw_rush");
    }
    else {
        correct = *GROUND_CORRECT_KIND_GROUND;
        mot = hash40("special_lw_rush");
    }
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
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
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        sv_kinetic_energy!(
            set_speed_mul,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            1.4,
            1.4
        );
    }
    else {
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }

    if !StopModule::is_stop(fighter.module_accessor) {
        ryu_special_lw_step_f_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(ryu_special_lw_step_f_substatus as *const () as _));

    fighter.sub_shift_status_main(L2CValue::Ptr(ryu_special_lw_step_f_main_loop as *const () as _))
}

unsafe extern "C" fn ryu_special_lw_step_f_substatus(fighter: &mut L2CFighterCommon, _param_1: L2CValue) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, ryu::status::flag::SPECIAL_LW_RUSH_ENABLE_ATTACK) {
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_CATCH);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
        VarModule::off_flag(fighter.module_accessor, ryu::status::flag::SPECIAL_LW_RUSH_ENABLE_ATTACK);
    }
    if VarModule::is_flag(fighter.module_accessor, ryu::status::flag::SPECIAL_LW_RUSH_RESUME_ENERGY) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                1.0
            );
            let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                -air_accel_y * 0.55
            );
        }
        VarModule::off_flag(fighter.module_accessor, ryu::status::flag::SPECIAL_LW_RUSH_RESUME_ENERGY);
    }
    0.into()
}

unsafe extern "C" fn ryu_special_lw_step_f_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_wait_ground_check_common(false.into()).get_bool()
    || fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }

    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        }
        return 0.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn ryu_special_lw_step_f_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ![
        *FIGHTER_STATUS_KIND_ATTACK,
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_LW4_START,
        *FIGHTER_STATUS_KIND_ATTACK_AIR,
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND
    ].contains(&fighter.global_table[STATUS_KIND].get_i32()) {
        VarModule::off_flag(fighter.module_accessor, ryu::instance::flag::DENJIN_RUSH_INHERIT);
    }
    if VarModule::is_flag(fighter.module_accessor, ryu::instance::flag::DENJIN_RUSH_INHERIT)
    && [
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND
    ].contains(&fighter.global_table[STATUS_KIND].get_i32()) {
        VarModule::off_flag(fighter.module_accessor, ryu::instance::flag::DENJIN_RUSH_INHERIT);
        VarModule::on_flag(fighter.module_accessor, ryu::instance::flag::DENJIN_CHARGE);
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Pre, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F, ryu_special_lw_step_f_pre);
    agent.status(smashline::Main, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F, ryu_special_lw_step_f_main);
    agent.status(smashline::End, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F, ryu_special_lw_step_f_end);
}