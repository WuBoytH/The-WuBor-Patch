use crate::imports::status_imports::*;
use super::super::{vl, helper::*};

unsafe extern "C" fn lucario_special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
        *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn lucario_special_lw_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let start_x_spd_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("start_x_spd_mul"));
    let speed_x = speed_x * start_x_spd_mul;
    lua_bind::KineticEnergy::reset_energy(
        stop_energy as *mut smash::app::KineticEnergy,
        *ENERGY_STOP_RESET_TYPE_AIR,
        &Vector2f{x: speed_x, y: 0.0},
        &Vector3f{x: 0.0, y: 0.0, z: 0.0},
        fighter.module_accessor
    );
    lua_bind::KineticEnergy::enable(stop_energy as *mut smash::app::KineticEnergy);
    let gravity = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    let speed_y = if !VarModule::is_flag(fighter.module_accessor, lucario::instance::flag::USED_AURA_CHARGE_AIR) {
        0.0
    }
    else {
        KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
    };
    lua_bind::FighterKineticEnergyGravity::set_speed(
        gravity as *mut smash::app::FighterKineticEnergyGravity,
        speed_y
    );
    let fall_acc_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("fall_acc_y"));
    lua_bind::FighterKineticEnergyGravity::set_accel(
        gravity as *mut smash::app::FighterKineticEnergyGravity,
        -fall_acc_y
    );
    let fall_spd_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("fall_spd_max"));
    lua_bind::FighterKineticEnergyGravity::set_limit_speed(
        gravity as *mut smash::app::FighterKineticEnergyGravity,
        fall_spd_max
    );
    lua_bind::KineticEnergy::enable(gravity as *mut smash::app::KineticEnergy);
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
    0.into()
}

unsafe extern "C" fn lucario_special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::set_int(fighter.module_accessor, lucario::status::int::SPECIAL_LW_STEP, lucario::SPECIAL_LW_STEP_START);
    VarModule::set_int(fighter.module_accessor, lucario::status::int::SPECIAL_LW_CHARGE_TIME, 0);
    VarModule::set_int(fighter.module_accessor, lucario::status::int::SPECIAL_LW_CHARGES_GAINED, 0);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_lw") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AIR_MOT);
    lucario_special_lw_set_kinetic(fighter);
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(lucario_special_lw_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(lucario_special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn lucario_special_lw_set_kinetic(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        lucario_special_set_air(fighter);
        lucario_special_air_mot_helper(fighter);
    }
    else {
        lucario_special_set_ground(fighter);
        lucario_special_ground_mot_helper(fighter);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RUN_STOP);
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
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    }
}

unsafe extern "C" fn lucario_special_lw_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    let step = VarModule::get_int(fighter.module_accessor, lucario::status::int::SPECIAL_LW_STEP);
    if param_1.get_bool() {
        if VarModule::is_flag(fighter.module_accessor, lucario::status::flag::SPECIAL_LW_ENABLE_CANCEL) {
            if !WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON) {
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
            }
            let pad_flag = fighter.global_table[PAD_FLAG].get_i32();
            if pad_flag & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 {
                ControlModule::clear_command(fighter.module_accessor, false);
                VarModule::off_flag(fighter.module_accessor, lucario::status::flag::SPECIAL_LW_ENABLE_CANCEL);
                VarModule::on_flag(fighter.module_accessor, lucario::status::flag::SPECIAL_LW_ATTACK);
                VarModule::on_flag(fighter.module_accessor, lucario::status::flag::SPECIAL_LW_CHARGE_END);
            }
            if pad_flag & *FIGHTER_PAD_FLAG_GUARD_TRIGGER != 0
            || fighter.sub_check_button_jump().get_bool()
            || fighter.sub_check_button_frick().get_bool() {
                ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
                VarModule::off_flag(fighter.module_accessor, lucario::status::flag::SPECIAL_LW_ENABLE_CANCEL);
                VarModule::on_flag(fighter.module_accessor, lucario::status::flag::SPECIAL_LW_CANCEL);
                VarModule::on_flag(fighter.module_accessor, lucario::status::flag::SPECIAL_LW_CHARGE_END);
                if fighter.sub_check_button_jump().get_bool()
                || fighter.sub_check_button_frick().get_bool() {
                    VarModule::on_flag(fighter.module_accessor, lucario::status::flag::SPECIAL_LW_CANCEL_FORCE_JUMP);
                }
            }
        }
        if step == lucario::SPECIAL_LW_STEP_CHARGE {
            lucario_special_lw_effect_helper(fighter);
            let charges_gained = VarModule::get_int(fighter.module_accessor, lucario::status::int::SPECIAL_LW_CHARGES_GAINED);
            if VarModule::get_int(fighter.module_accessor, lucario::status::int::SPECIAL_LW_CHARGE_TIME) == 0 {
                if charges_gained == 0 {
                    macros::EFFECT_OFF_KIND(fighter, Hash40::new("lucario_aura"), false, true);
                }
                macros::EFFECT_FOLLOW(fighter, Hash40::new("lucario_aura"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
                let rate_add = vl::special_lw::CHARGE_FRAME_SUBSEQUENT_MUL * VarModule::get_int(fighter.module_accessor, lucario::status::int::SPECIAL_LW_CHARGES_GAINED) as f32;
                macros::LAST_EFFECT_SET_RATE(fighter, 0.5 + rate_add);
                macros::PLAY_SE(fighter, Hash40::new("se_lucario_special_l01"));
            }
            VarModule::inc_int(fighter.module_accessor, lucario::status::int::SPECIAL_LW_CHARGE_TIME);
            let charge_max = vl::special_lw::CHARGE_FRAME - (vl::special_lw::CHARGE_FRAME * charges_gained as f32 * vl::special_lw::CHARGE_FRAME_SUBSEQUENT_MUL);
            if VarModule::get_int(fighter.module_accessor, lucario::status::int::SPECIAL_LW_CHARGE_TIME) >= charge_max as i32 {
                VarModule::set_int(fighter.module_accessor, lucario::status::int::SPECIAL_LW_CHARGE_TIME, 0);
                lucario_gain_aura(fighter);
                lucario_special_lw_effect_helper(fighter);
                if VarModule::get_int(fighter.module_accessor, lucario::instance::int::AURA_LEVEL) >= vl::private::AURA_CHARGE_MAX
                || !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                    VarModule::off_flag(fighter.module_accessor, lucario::status::flag::SPECIAL_LW_ENABLE_CANCEL);
                    VarModule::on_flag(fighter.module_accessor, lucario::status::flag::SPECIAL_LW_CHARGE_END);
                }
            }
        }
    }
    0.into()
}

unsafe extern "C" fn lucario_special_lw_effect_helper(fighter: &mut L2CFighterCommon) {
    let level = VarModule::get_int(fighter.module_accessor, lucario::instance::int::AURA_LEVEL);
    if level > 2 {
        let eff = VarModule::get_int(fighter.module_accessor, lucario::status::int::SPECIAL_LW_EFF3) as u32;
        if !EffectModule::is_exist_effect(fighter.module_accessor, eff) {
            EffectModule::req_follow(
                fighter.module_accessor,
                Hash40::new("lucario_hadou_l_l"),
                Hash40::new("top"),
                &Vector3f{x: 0.0, y: 25.0, z: 0.0},
                &ZERO_VECTOR,
                1.0,
                false,
                0,
                0,
                0,
                0,
                0,
                false,
                false
            );
            let eff = EffectModule::get_last_handle(fighter.module_accessor) as u32;
            VarModule::set_int(fighter.module_accessor, lucario::status::int::SPECIAL_LW_EFF3, eff as i32);
        }
    }
    if level > 1 {
        let eff = VarModule::get_int(fighter.module_accessor, lucario::status::int::SPECIAL_LW_EFF2) as u32;
        if !EffectModule::is_exist_effect(fighter.module_accessor, eff) {
            let lr = PostureModule::lr(fighter.module_accessor);
            EffectModule::req_follow(
                fighter.module_accessor,
                Hash40::new("lucario_hadou_m"),
                Hash40::new("top"),
                &Vector3f{x: 0.0, y: 15.0, z: 12.0 * lr},
                &ZERO_VECTOR,
                1.0,
                false,
                0,
                0,
                0,
                0,
                0,
                false,
                false
            );
            let eff = EffectModule::get_last_handle(fighter.module_accessor) as u32;
            VarModule::set_int(fighter.module_accessor, lucario::status::int::SPECIAL_LW_EFF2, eff as i32);
        }
    }
    if level > 0 {
        let eff = VarModule::get_int(fighter.module_accessor, lucario::status::int::SPECIAL_LW_EFF1) as u32;
        if !EffectModule::is_exist_effect(fighter.module_accessor, eff) {
            let lr = PostureModule::lr(fighter.module_accessor);
            EffectModule::req_follow(
                fighter.module_accessor,
                Hash40::new("lucario_hadou"),
                Hash40::new("top"),
                &Vector3f{x: 0.0, y: 15.0, z: -12.0 * lr},
                &ZERO_VECTOR,
                1.0,
                false,
                0,
                0,
                0,
                0,
                0,
                false,
                false
            );
            let eff = EffectModule::get_last_handle(fighter.module_accessor) as u32;
            VarModule::set_int(fighter.module_accessor, lucario::status::int::SPECIAL_LW_EFF1, eff as i32);
        }
    }
}

unsafe extern "C" fn lucario_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let step = VarModule::get_int(fighter.module_accessor, lucario::status::int::SPECIAL_LW_STEP);
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        lucario_special_lw_set_kinetic(fighter);
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if VarModule::is_flag(fighter.module_accessor, lucario::status::flag::SPECIAL_LW_CANCEL_FORCE_JUMP) {
            let jump = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                true
            }
            else {
                let jumps = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
                let jumps_max = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
                jumps < jumps_max
            };
            if jump {
                let status = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                    FIGHTER_STATUS_KIND_JUMP_SQUAT
                }
                else {
                    FIGHTER_STATUS_KIND_JUMP_AERIAL
                };
                fighter.change_status(status.into(), true.into());
                return 0.into();
            }
            VarModule::off_flag(fighter.module_accessor, lucario::status::flag::SPECIAL_LW_CANCEL_FORCE_JUMP);
        }
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if VarModule::is_flag(fighter.module_accessor, lucario::status::flag::SPECIAL_LW_CHARGE_END) {
        let mot_g;
        let mot_a;
        if VarModule::is_flag(fighter.module_accessor, lucario::status::flag::SPECIAL_LW_ATTACK) {
            mot_g = hash40("special_lw_attack");
            mot_a = hash40("special_air_lw_attack");
        }
        else if VarModule::is_flag(fighter.module_accessor, lucario::status::flag::SPECIAL_LW_CANCEL) {
            mot_g = hash40("special_lw_cancel");
            mot_a = hash40("special_air_lw_cancel");
        }
        else {
            mot_g = hash40("special_lw_end");
            mot_a = hash40("special_air_lw_end");
        }
        let lr = PostureModule::lr(fighter.module_accessor);
        if fighter.global_table[STICK_X].get_f32() * lr < -0.5 {
            PostureModule::reverse_lr(fighter.module_accessor);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
        }
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
        WorkModule::set_int64(fighter.module_accessor, mot_g as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
        WorkModule::set_int64(fighter.module_accessor, mot_a as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AIR_MOT);
        lucario_special_lw_set_kinetic(fighter);
        VarModule::set_int(fighter.module_accessor, lucario::status::int::SPECIAL_LW_STEP, lucario::SPECIAL_LW_STEP_END);
        VarModule::off_flag(fighter.module_accessor, lucario::status::flag::SPECIAL_LW_ATTACK);
        VarModule::off_flag(fighter.module_accessor, lucario::status::flag::SPECIAL_LW_CANCEL);
        VarModule::off_flag(fighter.module_accessor, lucario::status::flag::SPECIAL_LW_CHARGE_END);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if step == lucario::SPECIAL_LW_STEP_END {
            let status = if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                FIGHTER_STATUS_KIND_FALL
            }
            else {
                FIGHTER_STATUS_KIND_WAIT
            };
            fighter.change_status(status.into(), false.into());
            return 0.into();
        }
        else if step == lucario::SPECIAL_LW_STEP_START {
            VarModule::inc_int(fighter.module_accessor, lucario::status::int::SPECIAL_LW_STEP);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
            WorkModule::set_int64(fighter.module_accessor, hash40("special_lw_charge") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
            WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw_charge") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AIR_MOT);
            lucario_special_lw_set_kinetic(fighter);
        }
    }
    0.into()
}

unsafe extern "C" fn lucario_special_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    lucario_special_lw_eff_remover(fighter);
    0.into()
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, lucario_special_lw_pre);
    agent.status(smashline::Init, *FIGHTER_STATUS_KIND_SPECIAL_LW, lucario_special_lw_init);
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, lucario_special_lw_main);
    agent.status(smashline::End, *FIGHTER_STATUS_KIND_SPECIAL_LW, lucario_special_lw_end);
}