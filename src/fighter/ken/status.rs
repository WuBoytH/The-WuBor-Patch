use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    smashline::*,
    crate::{
        common_funcs::*,
        vars::*,
        table_const::*
    },
    super::super::{
        common::common_status::dash::fgc_dashback_main,
        ryu::helper::*
    }
};

#[status_script(agent = "ken", status = FIGHTER_STATUS_KIND_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn ken_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_AttackCommon();
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_NEAR_OPPONENT) {
        WorkModule::set_int64(fighter.module_accessor, 0x10556e6036, *FIGHTER_STATUS_ATTACK_WORK_INT_ATTACK11_MOTION);
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_LOG_ATTACK_KIND_ATTACK_NEAR, *FIGHTER_RYU_STATUS_ATTACK_INT_LOG_KIND);
    }
    else {
        WorkModule::set_int64(fighter.module_accessor, 0xb4f4e6f8f, *FIGHTER_STATUS_ATTACK_WORK_INT_ATTACK11_MOTION);
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_LOG_ATTACK_KIND_ATTACK11, *FIGHTER_RYU_STATUS_ATTACK_INT_LOG_KIND);
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        ryu_attack_main_uniq_chk(fighter);
    }
    fighter.global_table[SUB_STATUS3].assign(&L2CValue::Ptr(ryu_attack_main_uniq_chk as *const () as _));
    if !StopModule::is_stop(fighter.module_accessor) {
        ryu_attack_main_uniq_chk4(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(ryu_attack_main_uniq_chk4 as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(ken_attack_main_loop as *const () as _))
}

unsafe extern "C" fn ken_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !CancelModule::is_enable_cancel(fighter.module_accessor) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL) {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_HIT) {
                if ryu_final_hit_cancel(fighter, SITUATION_KIND_GROUND.into()).get_bool() {
                    return 1.into();
                }
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL) {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_HIT) {
                if ryu_hit_cancel(fighter, SITUATION_KIND_GROUND.into()).get_bool() {
                    return 1.into();
                }
            }
        }
    }
    if ComboModule::count(fighter.module_accessor) == 1 {
        if !CancelModule::is_enable_cancel(fighter.module_accessor) {
            let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
            let attack_start_cancel_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_private"), hash40("attack_start_cancel_frame"));
            if current_frame < attack_start_cancel_frame {
                if ryu_kara_cancel(fighter).get_bool() {
                    return 1.into();
                }
            }
        }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    let mot = MotionModule::motion_kind(fighter.module_accessor);
    if [
        hash40("attack_11_w"),
        hash40("attack_11_s"),
        hash40("attack_11_near_s")
    ].contains(&mot) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_CANCEL) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_BUTTON_TRIGGER) {
                if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                    let stick_y = fighter.global_table[STICK_Y].get_f32();
                    let attack_hi3_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_hi3_stick_y"));
                    let cont;
                    if !(stick_y < attack_hi3_stick_y) {
                        cont = false;
                    }
                    else {
                        let attack_lw3_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_lw3_stick_y"));
                        if !(attack_lw3_stick_y < stick_y) {
                            cont = false;
                        }
                        else {
                            let stick_x = fighter.global_table[STICK_X].get_f32();
                            let attack_s3_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_s3_stick_x"));
                            cont = stick_x < attack_s3_stick_x;
                        }
                    }
                    if cont {
                        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK.into(), false.into());
                        return 1.into();
                    }
                }
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_SAME_ATTACK_CANCEL) {
            let button_on_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_INT_BUTTON_ON_FRAME);
            let attack_11_s_button_on_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_private"), hash40("attack_11_s_button_on_frame"));
            if attack_11_s_button_on_frame <= button_on_frame as f32 {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK.into(), false.into());
                return 1.into();
            }
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    // if 0 < WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME) {
    //     if !StopModule::is_stop(fighter.module_accessor) {
    //         if fighter.sub_check_button_jump().get_bool() {
                
    //         }
    //     }
    // }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART) {
        if !MotionModule::is_end(fighter.module_accessor) {
            ryu_idkwhatthisis2(fighter);
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK.into(), false.into());
    }
    0.into()
}

#[status_script(agent = "ken", status = FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn ken_specialsloop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let start_sit = WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_START_SITUATION);
    if start_sit != *SITUATION_KIND_GROUND {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_s"),
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
            Hash40::new("special_s"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    let eff : u64;
    if !MotionModule::is_flip(fighter.module_accessor) {
        eff = 0x1479305a4a;
    }
    else {
        eff = 0x14833f6729;
    }
    fighter.clear_lua_stack();
    lua_args!(fighter, MA_MSC_EFFECT_REQUEST_FOLLOW, eff, hash40("rot"), 0.0, 1.5, 0.0, 0.0, 0.0, 0.0, 1.0, false, *EFFECT_SUB_ATTRIBUTE_SYNC_STOP, 0, -1);
    sv_module_access::effect(fighter.lua_state_agent);
    let spineffect = fighter.pop_lua_stack(1).get_u32();
    WorkModule::set_int(fighter.module_accessor, spineffect as i32, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_EFFECT_HANDLE);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
        let alpha = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("wind_alpha")) * 0.01;
        EffectModule::set_alpha(fighter.module_accessor, spineffect, alpha);
    }
    else {
        let alpha = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("command_wind_alpha")) * 0.01;
        EffectModule::set_alpha(fighter.module_accessor, spineffect, alpha);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(ken_specialsloop_main_loop as *const () as _))
}

unsafe extern "C" fn ken_specialsloop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            }
            else {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
                    WorkModule::set_float(fighter.module_accessor, 10.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                    fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
                    return 1.into();
                }
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            }
        }
    }
    else {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            let start_sit = WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_START_SITUATION);
            if start_sit != *SITUATION_KIND_GROUND {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
                sv_kinetic_energy::set_accel(fighter.lua_state_agent);
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
                sv_kinetic_energy::set_brake(fighter.lua_state_agent);
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
                sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_LOOP_COUNT);
        let loop_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_LOOP_COUNT);
        if loop_count > 0 {
            let start_sit = WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_START_SITUATION);
            if start_sit != *SITUATION_KIND_GROUND {
                MotionModule::change_motion(
                    fighter.module_accessor,
                    Hash40::new("special_air_s"),
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
                    Hash40::new("special_s"),
                    0.0,
                    1.0,
                    false,
                    0.0,
                    false,
                    false
                );
            }
        }
        else {
            fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END.into(), false.into());
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_FLAG_GROUND);
    }
    else {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_FLAG_GROUND);
    }
    0.into()
}

#[status_script(agent = "ken", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn ken_speciallw_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    fighter.clear_lua_stack();
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_AIR_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
    }
    else {
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
    }
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    else {
        let accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("accel_y"));
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -accel_y);
        sv_kinetic_energy::set_accel(fighter.lua_state_agent);
        let max_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("max_speed_y"));
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, max_y);
        sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    L2CValue::I32(0)
}

#[status_script(agent = "ken", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn ken_speciallw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        let mot;
        if V_GAUGE[entry_id(fighter.module_accessor)] < 900 || V_TRIGGER[entry_id(fighter.module_accessor)] {
            SPECIAL_LW_TYPE[entry_id(fighter.module_accessor)] = 0;
            QUICK_STEP_STATE[entry_id(fighter.module_accessor)] = 1;
            mot = Hash40::new("run");
        }
        else {
            SPECIAL_LW_TYPE[entry_id(fighter.module_accessor)] = 1;
            mot = Hash40::new("special_lw_step_f");
        }
        MotionModule::change_motion(
            fighter.module_accessor,
            mot,
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    else {
        if V_GAUGE[entry_id(fighter.module_accessor)] < 900 || V_TRIGGER[entry_id(fighter.module_accessor)] {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_UNIQ);
            macros::SET_SPEED_EX(fighter, 0.8, 0.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            QUICK_STEP_STATE[entry_id(fighter.module_accessor)] = 2;
            SPECIAL_LW_TYPE[entry_id(fighter.module_accessor)] = 0;
        }
        else {
            SPECIAL_LW_TYPE[entry_id(fighter.module_accessor)] = 1;
        }
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_lw_step_f"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    if SPECIAL_LW_TYPE[entry_id(fighter.module_accessor)] == 0 {
        fighter.sub_shift_status_main(L2CValue::Ptr(ken_quickstep_loop as *const () as _))
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RESET);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
        SlowModule::set_whole(fighter.module_accessor, 6, 0);
        macros::SLOW_OPPONENT(fighter, 100.0, 12.0);
        macros::FILL_SCREEN_MODEL_COLOR(fighter, 0, 3, 0.2, 0.2, 0.2, 0, 0, 0, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, 205);
        if OPPONENT_BOMA[entry_id(fighter.module_accessor)] != 0 {
            DIFF_X[entry_id(fighter.module_accessor)] = PostureModule::pos_x(OPPONENT_BOMA[entry_id(fighter.module_accessor)] as *mut BattleObjectModuleAccessor) - PostureModule::pos_x(fighter.module_accessor);
            if (DIFF_X[entry_id(fighter.module_accessor)] > 0.0 && PostureModule::lr(fighter.module_accessor) < 0.0)
            || (DIFF_X[entry_id(fighter.module_accessor)] < 0.0 && PostureModule::lr(fighter.module_accessor) > 0.0) {
                PostureModule::reverse_lr(fighter.module_accessor);
                PostureModule::reverse_rot_y_lr(fighter.module_accessor);
            }
            DIFF_X[entry_id(fighter.module_accessor)] = DIFF_X[entry_id(fighter.module_accessor)].abs();
            if DIFF_X[entry_id(fighter.module_accessor)] > 50.0 {
                DIFF_X[entry_id(fighter.module_accessor)] -= 5.0;
            }
            else {
                DIFF_X[entry_id(fighter.module_accessor)] = 0.0;
            }
            OPPONENT_BOMA[entry_id(fighter.module_accessor)] = 0;
        }
        else {
            DIFF_X[entry_id(fighter.module_accessor)] = 0.0;
        }
        macros::PLAY_SE(fighter, Hash40::new("se_ken_special_l01"));
        macros::PLAY_SE(fighter, Hash40::new("vc_ken_special_l01"));
        V_TRIGGER[entry_id(fighter.module_accessor)] = true;
        V_GAUGE[entry_id(fighter.module_accessor)] = 0;
        fighter.sub_shift_status_main(L2CValue::Ptr(ken_heatrush_loop as *const () as _))
    }
}

unsafe extern "C" fn ken_quickstep_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        fighter.sub_wait_ground_check_common(L2CValue::I32(0));
        fighter.sub_air_check_fall_common();
    }
    if STEP_KICK[entry_id(fighter.module_accessor)] {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("attack_s3_s_w"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        STEP_KICK[entry_id(fighter.module_accessor)] = false;
    }
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_lw_step_f") {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    L2CValue::I32(0)
}

unsafe extern "C" fn ken_heatrush_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        fighter.sub_wait_ground_check_common(L2CValue::I32(0));
        fighter.sub_air_check_fall_common();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    L2CValue::I32(0)
}

#[status_script(agent = "ken", status = FIGHTER_RYU_STATUS_KIND_DASH_BACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn ken_dashback_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_main(fighter)
}

pub fn install() {
    install_status_scripts!(
        ken_attack_main,
        ken_specialsloop_main,
        ken_speciallw_init,
        ken_speciallw_main,
        ken_dashback_main
    );
}