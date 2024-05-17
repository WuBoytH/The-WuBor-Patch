use super::*;
use super::super::super::vl;
use super::super::helper::*;

// Note to self: Implement PARRY_XLU when dash anims are done

// Used for both dashes

unsafe extern "C" fn marth_speciallw_dash_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_DASH_F

unsafe extern "C" fn marth_speciallw_dash_f_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::clear_command_one(
        fighter.module_accessor,
        *FIGHTER_PAD_COMMAND_CATEGORY1,
        *FIGHTER_PAD_CMD_CAT1_DASH
    );
    sv_kinetic_energy!(
        set_speed_mul,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        0.8
    );
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_lw_dash_f"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_dash_main_loop as *const () as _))
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_DASH_B

unsafe extern "C" fn marth_speciallw_dash_b_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::clear_command_one(
        fighter.module_accessor,
        *FIGHTER_PAD_COMMAND_CATEGORY1,
        *FIGHTER_PAD_CMD_CAT1_TURN_DASH
    );
    sv_kinetic_energy!(
        set_speed_mul,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        0.9
    );
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_lw_dash_b"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_dash_main_loop as *const () as _))
}

// Dash Main Loop

unsafe extern "C" fn marth_speciallw_dash_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        let cancel = CancelModule::is_enable_cancel(fighter.module_accessor);
        if fighter.global_table[STATUS_FRAME].get_f32() >= vl::param_stance::dash_attack_cancel_frame
        || cancel {
            if marth_stance_cancel_helper(fighter).get_bool() {
                return 1.into();
            }
        }
        if cancel {
            if marth_stance_dash_cancel_helper(fighter, true).get_bool() {
                return 1.into();
            }
        }
        let cat1 = fighter.global_table[CMD_CAT1].get_i32();
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S != 0 {
            fighter.change_status(vars::marth::status::STANCE_SPECIAL_S.into(), true.into());
            return true.into();
        }
        marth_stance_mot_end_helper(fighter);
    }
    else {
        if !VarModule::is_flag(fighter.module_accessor, vars::marth::instance::flag::IS_STANCE) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return true.into();
        }
        else {
            fighter.change_status(vars::marth::status::STANCE_WAIT.into(), false.into());
        }
    }
    0.into()
}

// Dash End

unsafe extern "C" fn marth_speciallw_dash_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.module_accessor, vars::marth::instance::flag::PARRY_XLU);
    marth_stance_common_end(fighter);
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, vars::marth::status::STANCE_DASH_F, marth_speciallw_dash_pre);
    agent.status(Main, vars::marth::status::STANCE_DASH_F, marth_speciallw_dash_f_main);
    agent.status(End, vars::marth::status::STANCE_DASH_F, marth_speciallw_dash_end);

    agent.status(Pre, vars::marth::status::STANCE_DASH_B, marth_speciallw_dash_pre);
    agent.status(Main, vars::marth::status::STANCE_DASH_B, marth_speciallw_dash_b_main);
    agent.status(End, vars::marth::status::STANCE_DASH_B, marth_speciallw_dash_end);
}