use crate::imports::status_imports::*;

unsafe extern "C" fn ken_special_s_loop_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_sit = fighter.global_table[SITUATION_KIND].get_i32();
    WorkModule::set_int(fighter.module_accessor, current_sit, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_START_SITUATION);
    let command = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND);
    let strength = WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    let loop_count_hash;
    let speed_x_hash;
    if VarModule::is_flag(fighter.module_accessor, ken::status::flag::QUICK_STEP_INHERITED) {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        loop_count_hash = hash40("air_loop_num_s");
        speed_x_hash = hash40("air_speed_x_w");
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
    }
    else if current_sit != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if strength == *FIGHTER_RYU_STRENGTH_W {
            loop_count_hash = hash40("air_loop_num_w");
            speed_x_hash = hash40("air_speed_x_w");
        }
        else if strength == *FIGHTER_RYU_STRENGTH_M {
            loop_count_hash = hash40("air_loop_num_m");
            speed_x_hash = hash40("air_speed_x_m");
        }
        else {
            loop_count_hash = hash40("air_loop_num_s");
            speed_x_hash = hash40("air_speed_x_s");
        }
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if strength == *FIGHTER_RYU_STRENGTH_W {
            loop_count_hash = hash40("loop_num_w");
            speed_x_hash = hash40("speed_x_w");
        }
        else if strength == *FIGHTER_RYU_STRENGTH_M {
            loop_count_hash = hash40("loop_num_m");
            speed_x_hash = hash40("speed_x_m");
        }
        else {
            loop_count_hash = hash40("loop_num_s");
            speed_x_hash = hash40("speed_x_s");
        }
    }
    let lr = PostureModule::lr(fighter.module_accessor);
    let speed_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), speed_x_hash) * lr;
    let current_sit = fighter.global_table[SITUATION_KIND].get_i32();
    let loops = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), loop_count_hash);
    WorkModule::set_int(fighter.module_accessor, loops, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_LOOP_COUNT);
    let stop_type = if current_sit != *SITUATION_KIND_GROUND {
        ENERGY_STOP_RESET_TYPE_AIR
    }
    else {
        ENERGY_STOP_RESET_TYPE_NONE
    };
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        stop_type,
        speed_x,
        0.0,
        0.0,
        0.0,
        0.0
    );
    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        0.0,
        0.0
    );
    sv_kinetic_energy!(
        set_brake,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        0.0,
        0.0
    );
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        0.0,
        0.0
    );
    if current_sit == *SITUATION_KIND_GROUND {
        let ground_speed_limit = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("ground_speed_limit"));
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            ground_speed_limit,
            0.0
        );
    }
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    if current_sit != *SITUATION_KIND_GROUND {
        let speed_y = if !VarModule::is_flag(fighter.module_accessor, ken::status::flag::QUICK_STEP_INHERITED) {
            KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
        }
        else {
            0.3
        };
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
            0.0,
            speed_y, 
            0.0,
            0.0,
            0.0
        );
        let air_accel_y = if !VarModule::is_flag(fighter.module_accessor, ken::status::flag::QUICK_STEP_INHERITED) {
            WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("air_accel_y"))
        }
        else {
            0.0
        };
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -air_accel_y
        );
        let air_max_speed_y = if !VarModule::is_flag(fighter.module_accessor, ken::status::flag::QUICK_STEP_INHERITED) {
            WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("air_max_speed_y"))
        }
        else {
            WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0)
        };
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            air_max_speed_y
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    if command {
        let command_power_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("command_power_mul"));
        AttackModule::set_power_mul_status(fighter.module_accessor, command_power_mul);
    }
    let boma = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, boma);
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, boma);
    0.into()
}

unsafe extern "C" fn ken_special_s_loop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(fighter.module_accessor, ken::status::flag::QUICK_STEP_INHERITED) {
        let loops = WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_LOOP_COUNT);
        WorkModule::set_int(fighter.module_accessor, loops - 1, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_LOOP_COUNT);
        if loops - 1 <= 0 {
            fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END.into(), false.into());
            return 1.into();
        }
        let original = smashline::original_status(smashline::Main, fighter, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP);
        return original(fighter);
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_FLAG_GROUND);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_air_s2"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    let eff = if !MotionModule::is_flip(fighter.module_accessor) {
        hash40("ken_tatsumaki_wind_r")
    }
    else {
        hash40("ken_tatsumaki_wind_l")
    };
    fighter.clear_lua_stack();
    lua_args!(fighter, MA_MSC_EFFECT_REQUEST_FOLLOW, eff, hash40("rot"), 0.0, 1.5, 0.0, 0.0, 0.0, 0.0, 1.0, false, *EFFECT_SUB_ATTRIBUTE_SYNC_STOP, 0, -1);
    sv_module_access::effect(fighter.lua_state_agent);
    let spineffect = fighter.pop_lua_stack(1).get_u32();
    EffectModule::set_rgb(fighter.module_accessor, spineffect, 0.8, 0.2, 0.2);
    WorkModule::set_int(fighter.module_accessor, spineffect as i32, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_EFFECT_HANDLE);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
        let alpha = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("wind_alpha")) * 0.01;
        EffectModule::set_alpha(fighter.module_accessor, spineffect, alpha);
    }
    else {
        let alpha = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("command_wind_alpha")) * 0.01;
        EffectModule::set_alpha(fighter.module_accessor, spineffect, alpha);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(ken_special_s2_loop_main_loop as *const () as _))
}

unsafe extern "C" fn ken_special_s2_loop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        WorkModule::set_float(fighter.module_accessor, 16.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        return 0.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_LOOP_COUNT);
        let loop_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_LOOP_COUNT);
        if loop_count > 0 {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("special_air_s2"),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
        }
        else {
            fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn ken_special_s_loop_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ken_special_s_loop_end_inner(fighter)
}

pub unsafe extern "C" fn ken_special_s_loop_end_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END {
        EffectModule::kill_kind(
            fighter.module_accessor,
            Hash40::new("ken_syoryuken_fire"),
            false,
            true
        );
    }
    let eff = WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_EFFECT_HANDLE);
    if eff != *EFFECT_HANDLE_NULL {
        EffectModule::kill(fighter.module_accessor, eff as u32, false, false);
        WorkModule::set_int(fighter.module_accessor, *EFFECT_HANDLE_NULL, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_EFFECT_HANDLE);
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Init, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP, ken_special_s_loop_init);
    agent.status(smashline::Main, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP, ken_special_s_loop_main);
    agent.status(smashline::End, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP, ken_special_s_loop_end);
}