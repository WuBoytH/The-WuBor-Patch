use crate::imports::status_imports::*;
use super::super::helper::*;

// Jab/Tilt common pre function

unsafe extern "C" fn marth_speciallw_attack_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW
            | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
            | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK

unsafe extern "C" fn marth_speciallw_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_lw_attack_11"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_attack_main_loop as *const () as _))
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK_LW3

unsafe extern "C" fn marth_speciallw_attack_lw3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.module_accessor, marth::status::flag::ATTACK_3_CHANGE_MOTION);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_lw_attack_lw3"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_attack_lw3_main_loop as *const () as _))
}

unsafe extern "C" fn marth_speciallw_attack_lw3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if VarModule::is_flag(fighter.module_accessor, marth::status::flag::ATTACK_3_CHANGE_MOTION) {
            VarModule::off_flag(fighter.module_accessor, marth::status::flag::ATTACK_3_CHANGE_MOTION);
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                fighter.change_status(marth::status::STANCE_ATTACK_LW4.into(), true.into());
                return 1.into();
            }
        }
        if CancelModule::is_enable_cancel(fighter.module_accessor)
        || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
        && !fighter.global_table[IS_STOP].get_bool() {
            if marth_stance_cancel_helper(fighter).get_bool()
            || marth_stance_dash_cancel_helper(fighter, true).get_bool() {
                return 1.into();
            }
        }
        if MotionModule::is_end(fighter.module_accessor) {
            if !VarModule::is_flag(fighter.module_accessor, marth::instance::flag::IS_STANCE) {
                let clear_buffer = fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0;
                fighter.change_status(marth::status::STANCE_EXIT.into(), clear_buffer.into());
                return 1.into();
            }
            else {
                fighter.change_status(marth::status::STANCE_SQUAT_WAIT.into(), false.into());
            }
        }
    }
    else {
        if !VarModule::is_flag(fighter.module_accessor, marth::instance::flag::IS_STANCE) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
        else {
            fighter.change_status(marth::status::STANCE_WAIT.into(), false.into());
        }
    }
    0.into()
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK_LW4

unsafe extern "C" fn marth_speciallw_attack_lw4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.module_accessor, marth::status::flag::ATTACK_3_CHANGE_MOTION);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_lw_attack_lw4"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_attack_lw4_main_loop as *const () as _))
}

unsafe extern "C" fn marth_speciallw_attack_lw4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if VarModule::is_flag(fighter.module_accessor, marth::status::flag::ATTACK_3_CHANGE_MOTION) {
            VarModule::off_flag(fighter.module_accessor, marth::status::flag::ATTACK_3_CHANGE_MOTION);
            let dir = FGCModule::get_command_stick_direction(fighter, true);
            let mot;
            if [1, 4, 7].contains(&dir) {
                mot = Hash40::new("special_lw_attack_lw4_b");
            }
            else if [3, 6, 9].contains(&dir) {
                mot = Hash40::new("special_lw_attack_lw4_f");
            }
            else {
                mot = Hash40::new("invalid");
            }
            if mot.hash != hash40("invalid") {
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
    }
    marth_speciallw_attack_main_loop(fighter)
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK_HI3

unsafe extern "C" fn marth_speciallw_attack_hi3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.module_accessor, marth::status::flag::ATTACK_3_CHANGE_MOTION);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_lw_attack_hi3"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_attack_hi3_main_loop as *const () as _))
}

unsafe extern "C" fn marth_speciallw_attack_hi3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if VarModule::is_flag(fighter.module_accessor, marth::status::flag::ATTACK_3_CHANGE_MOTION) {
            VarModule::off_flag(fighter.module_accessor, marth::status::flag::ATTACK_3_CHANGE_MOTION);
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                MotionModule::change_motion_inherit_frame(
                    fighter.module_accessor,
                    Hash40::new("special_lw_attack_hi4"),
                    -1.0,
                    1.0,
                    0.0,
                    false,
                    false
                );
            }
        }
    }
    marth_speciallw_attack_main_loop(fighter)
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK_F3

unsafe extern "C" fn marth_speciallw_attack_f3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.module_accessor, marth::status::flag::ATTACK_3_CHANGE_MOTION);
    VarModule::off_flag(fighter.module_accessor, marth::status::flag::ATTACK_F3_HEAVY);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_lw_attack_f3"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_attack_f3_main_loop as *const () as _))
}

unsafe extern "C" fn marth_speciallw_attack_f3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if VarModule::is_flag(fighter.module_accessor, marth::status::flag::ATTACK_3_CHANGE_MOTION) {
            VarModule::off_flag(fighter.module_accessor, marth::status::flag::ATTACK_3_CHANGE_MOTION);
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                sv_kinetic_energy!(
                    set_speed_mul,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_MOTION,
                    1.5
                );
                VarModule::on_flag(fighter.module_accessor, marth::status::flag::ATTACK_F3_HEAVY);
            }
        }
    }
    marth_speciallw_attack_main_loop(fighter)
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK_B3

unsafe extern "C" fn marth_speciallw_attack_b3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_lw_attack_b3"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_attack_main_loop as *const () as _))
}

// Jab/Tilt common main loop function

unsafe extern "C" fn marth_speciallw_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if CancelModule::is_enable_cancel(fighter.module_accessor)
        || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
        && !fighter.global_table[IS_STOP].get_bool() {
            if marth_stance_cancel_helper(fighter).get_bool()
            || marth_stance_dash_cancel_helper(fighter, true).get_bool() {
                return 1.into();
            }
        }
        marth_stance_mot_end_helper(fighter);
    }
    else {
        if !VarModule::is_flag(fighter.module_accessor, marth::instance::flag::IS_STANCE) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return true.into();
        }
        else {
            fighter.change_status(marth::status::STANCE_WAIT.into(), false.into());
        }
    }
    0.into()
}

// Jab/Tilt common end function

unsafe extern "C" fn marth_speciallw_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.module_accessor, marth::instance::flag::PARRY_XLU);
    marth_stance_common_end(fighter);
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Pre, marth::status::STANCE_ATTACK, marth_speciallw_attack_pre);
    agent.status(smashline::Main, marth::status::STANCE_ATTACK, marth_speciallw_attack_main);
    agent.status(smashline::End, marth::status::STANCE_ATTACK, marth_speciallw_attack_end);

    agent.status(smashline::Pre, marth::status::STANCE_ATTACK_LW3, marth_speciallw_attack_pre);
    agent.status(smashline::Main, marth::status::STANCE_ATTACK_LW3, marth_speciallw_attack_lw3_main);
    agent.status(smashline::End, marth::status::STANCE_ATTACK_LW3, marth_speciallw_attack_end);

    agent.status(smashline::Pre, marth::status::STANCE_ATTACK_LW4, marth_speciallw_attack_pre);
    agent.status(smashline::Main, marth::status::STANCE_ATTACK_LW4, marth_speciallw_attack_lw4_main);
    agent.status(smashline::End, marth::status::STANCE_ATTACK_LW4, marth_speciallw_attack_end);

    agent.status(smashline::Pre, marth::status::STANCE_ATTACK_HI3, marth_speciallw_attack_pre);
    agent.status(smashline::Main, marth::status::STANCE_ATTACK_HI3, marth_speciallw_attack_hi3_main);
    agent.status(smashline::End, marth::status::STANCE_ATTACK_HI3, marth_speciallw_attack_end);

    agent.status(smashline::Pre, marth::status::STANCE_ATTACK_F3, marth_speciallw_attack_pre);
    agent.status(smashline::Main, marth::status::STANCE_ATTACK_F3, marth_speciallw_attack_f3_main);
    agent.status(smashline::End, marth::status::STANCE_ATTACK_F3, marth_speciallw_attack_end);

    agent.status(smashline::Pre, marth::status::STANCE_ATTACK_B3, marth_speciallw_attack_pre);
    agent.status(smashline::Main, marth::status::STANCE_ATTACK_B3, marth_speciallw_attack_b3_main);
    agent.status(smashline::End, marth::status::STANCE_ATTACK_B3, marth_speciallw_attack_end);
}