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
    super::super::common::common_status::fgc_dashback_main
};

#[status_script(agent = "ken", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn ken_speciallw_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.clear_lua_stack();
    lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    fighter.clear_lua_stack();
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, *ENERGY_MOTION_RESET_TYPE_AIR_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
    }
    else {
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, *ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
    }
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    fighter.clear_lua_stack();
    lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    else {
        let accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("accel_y"));
        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -accel_y);
        sv_kinetic_energy::set_accel(fighter.lua_state_agent);
        let max_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("max_speed_y"));
        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, max_y);
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
        ken_speciallw_init,
        ken_speciallw_main,
        ken_dashback_main
    );
}