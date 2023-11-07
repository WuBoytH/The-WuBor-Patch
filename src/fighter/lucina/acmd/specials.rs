use crate::imports::acmd_imports::*;
use super::super::{vl, helper::*};

unsafe extern "C" fn lucina_specialnstart_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if VarModule::is_flag(agent.module_accessor, yu::status::flag::IS_EX) {
        FGCModule::ex_flash(agent);
    }
}

unsafe extern "C" fn lucina_specialnstart_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if VarModule::is_flag(agent.module_accessor, yu::status::flag::IS_EX) {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_waza_ex_p4"));
        }
    }
}

unsafe extern "C" fn lucina_specialnloop(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 1.4);
}

unsafe extern "C" fn lucina_specialnend(agent: &mut L2CAgentBase) {
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY as u64 {
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 361, 20, 0, 95, 3.0, 0.0, 5.0, 8.0, Some(0.0), Some(5.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 9.0, 361, 20, 0, 95, 3.0, 0.0, 5.0, 25.0, Some(0.0), Some(5.0), Some(27.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 361, 20, 0, 95, 0.9, 0.0, 5.0, 17.0, Some(0.0), Some(5.0), Some(22.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 361, 20, 0, 95, 0.9, 0.0, 5.0, 23.5, Some(0.0), Some(5.0), Some(28.9), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            AttackModule::clear(agent.module_accessor, 0, false);
            AttackModule::clear(agent.module_accessor, 1, false);
        }
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    else {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            KineticModule::mul_speed(agent.module_accessor, &Vector3f {x: 0.4, y: 0.0, z: 0.0}, *FIGHTER_KINETIC_TYPE_UNIQ);
        }
        macros::FT_MOTION_RATE(agent, 0.125);
        frame(agent.lua_state_agent, 9.0);
        macros::FT_MOTION_RATE(agent, 1.0);
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            let ratio = get_damage_mul(agent.module_accessor);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0 * ratio, 361, 20, 0, 95, 3.3, 0.0, 7.7, 9.1, Some(0.0), Some(7.7), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("sword1"), 9.0 * ratio, 361, 20, 0, 95, 4.0, 2.0, 1.0, 1.5, Some(14.0), Some(1.0), Some(1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 2, 0, Hash40::new("sword1"), 9.0 * ratio, 361, 20, 0, 95, 3.5, 2.0, 1.0, 7.5, Some(9.5), Some(1.0), Some(7.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        frame(agent.lua_state_agent, 16.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
}

unsafe extern "C" fn lucina_specialnendmax(agent: &mut L2CAgentBase) {
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY as u64 {
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 60, 80, 0, 30, 3.0, 0.0, 5.0, 8.0, Some(0.0), Some(5.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 9.0, 60, 80, 0, 30, 3.0, 0.0, 5.0, 25.0, Some(0.0), Some(5.0), Some(27.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 60, 80, 0, 30, 0.9, 0.0, 5.0, 17.0, Some(0.0), Some(5.0), Some(22.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 60, 80, 0, 30, 0.9, 0.0, 5.0, 23.5, Some(0.0), Some(5.0), Some(28.9), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            AttackModule::clear(agent.module_accessor, 0, false);
            AttackModule::clear(agent.module_accessor, 1, false);
        }
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    else{
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            KineticModule::mul_speed(agent.module_accessor, &Vector3f {x: 0.4, y: 0.0, z: 0.0}, *FIGHTER_KINETIC_TYPE_UNIQ);
        }
        macros::FT_MOTION_RATE(agent, 0.125);
        frame(agent.lua_state_agent, 9.0);
        macros::FT_MOTION_RATE(agent, 1.25);
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            let ratio = get_damage_mul(agent.module_accessor);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0 * ratio, 60, 80, 0, 30, 3.3, 0.0, 7.7, 9.1, Some(0.0), Some(7.7), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("sword1"), 9.0 * ratio, 60, 80, 0, 30, 4.0, 2.0, 1.0, 1.5, Some(14.0), Some(1.0), Some(1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 2, 0, Hash40::new("sword1"), 9.0 * ratio, 60, 80, 0, 30, 3.5, 2.0, 1.0, 7.5, Some(9.5), Some(1.0), Some(7.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        frame(agent.lua_state_agent, 16.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 17.0);
        macros::FT_MOTION_RATE(agent, 1.0);
    }
}

unsafe extern "C" fn lucina_specialairs1(agent: &mut L2CAgentBase) {
    if VarModule::is_flag(agent.module_accessor, yu::instance::flag::COMMAND)
    && spent_meter(agent.module_accessor, false) {
        let spent = VarModule::get_float(agent.module_accessor, yu::instance::float::SPENT_SP);
        let meter_max = VarModule::get_float(agent.module_accessor, yu::instance::float::SP_GAUGE_MAX);
        FGCModule::update_meter(agent.module_accessor, -spent, meter_max, yu::instance::float::SP_GAUGE);
        VarModule::set_int(agent.module_accessor, yu::instance::int::SP_FLASH_TIMER, 40);
        VarModule::on_flag(agent.module_accessor, yu::status::flag::IS_EX);
        sp_diff_checker(agent.module_accessor);
    }
    else {
        VarModule::off_flag(agent.module_accessor, yu::status::flag::IS_EX);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
}

unsafe extern "C" fn lucina_specialairs1_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if VarModule::is_flag(agent.module_accessor, yu::status::flag::IS_EX) {
        FGCModule::ex_flash(agent);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn lucina_specialairs1_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if VarModule::is_flag(agent.module_accessor, yu::status::flag::IS_EX) {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_waza_ex_p4"));
        }
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_lucina_attack01"));
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucina_special_s01"));
    }
}

unsafe extern "C" fn lucina_specialairs2hi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        let mut dmg : f32;
        let kbg : i32;
        let velx : f32;
        let vely : f32;
        if VarModule::is_flag(agent.module_accessor, yu::status::flag::IS_EX) {
            dmg = 16.0;
            kbg = 71;
            velx = vl::param_special_s::dive_speed_x_ex;
            vely = vl::param_special_s::dive_speed_y_ex;
        }
        else {
            dmg = 11.0;
            kbg = 71;
            velx = vl::param_special_s::dive_speed_x;
            vely = vl::param_special_s::dive_speed_y;
        }
        if shadow_id(agent.module_accessor) {
            dmg *= 0.8;
        }
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(agent, velx, vely, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::suspend_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), dmg, 361, kbg, 0, 45, 2.5, 0.0, 8.5, 4.0, Some(0.0), Some(4.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), dmg, 361, kbg, 0, 45, 2.5, 0.0, 2.5, 3.0, Some(0.0), Some(11.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), dmg, 361, kbg, 0, 45, 2.5, 0.0, 2.5, 3.0, Some(0.0), Some(2.5), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
}

unsafe extern "C" fn lucina_specialairs2hi_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_sword_purple"), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_breaker_sting"), Hash40::new("top"), -0.0, 6, 8, 40, 0, 0, 0.9, true);
        macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 7.5, 6, 40, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.264, 0.47, 1.3);
        macros::LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_sp_flash"), Hash40::new("sword1"), -0.0, -0.0, 10, 0, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
}

unsafe extern "C" fn lucina_specialairs2hi_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucina_special_n03"));
    }
}

unsafe extern "C" fn lucina_specials2hi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn lucina_specials2hi_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn lucina_specials2hi_snd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_LANDING_SE(agent, Hash40::new("se_lucina_landing02"));
    }
}

unsafe extern "C" fn lucina_specials2hi_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn lucina_specialhi(agent: &mut L2CAgentBase) {
    if VarModule::is_flag(agent.module_accessor, yu::instance::flag::COMMAND)
    && spent_meter(agent.module_accessor, false) {
        let spent = VarModule::get_float(agent.module_accessor, yu::instance::float::SPENT_SP);
        let meter_max = VarModule::get_float(agent.module_accessor, yu::instance::float::SP_GAUGE_MAX);
        FGCModule::update_meter(agent.module_accessor, -spent, meter_max, yu::instance::float::SP_GAUGE);
        VarModule::set_int(agent.module_accessor, yu::instance::int::SP_FLASH_TIMER, 40);
        VarModule::on_flag(agent.module_accessor, yu::status::flag::IS_EX);
        sp_diff_checker(agent.module_accessor);
        full_invuln(agent.module_accessor, true);
        macros::FT_MOTION_RATE(agent, 2.0 / 3.0);
    }
    else {
        VarModule::off_flag(agent.module_accessor, yu::status::flag::IS_EX);
        upper_invuln(agent.module_accessor, true);
    }
    if VarModule::is_flag(agent.module_accessor, yu::status::flag::IS_EX) {
        frame(agent.lua_state_agent, 10.0);
        macros::FT_MOTION_RATE(agent, 1.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 79, 100, 100, 0, 5.1, 0.0, 11.0, 10.0, Some(0.0), Some(7.0), Some(10.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_SPECIAL_HI_SET_LR);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            full_invuln(agent.module_accessor, false);
            AttackModule::clear_all(agent.module_accessor);
            let ratio = get_damage_mul(agent.module_accessor);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0 * ratio, 40, 85, 0, 75, 6.5, 0.0, 12.0, 10.0, Some(0.0), Some(15.0), Some(10.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            // notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
        frame(agent.lua_state_agent, 24.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
    }
    else{
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            upper_invuln(agent.module_accessor, false);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 79, 100, 100, 0, 5.1, 0.0, 11.0, 10.0, Some(0.0), Some(7.0), Some(10.0), 1.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_SPECIAL_HI_SET_LR);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            let ratio = get_damage_mul(agent.module_accessor);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0 * ratio, 40, 80, 0, 70, 6.5, 0.0, 12.0, 10.0, Some(0.0), Some(15.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            // notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
        frame(agent.lua_state_agent, 24.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_KINETIC_CHANGE);
    }
}

unsafe extern "C" fn lucina_specialhi_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if VarModule::is_flag(agent.module_accessor, yu::status::flag::IS_EX) {
        FGCModule::ex_flash(agent);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_dolphin_swing"), Hash40::new("top"), 0, 12, -1, 14, -30, 37, 1, true);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 6, Hash40::new("sword1"), 0.0, 0.0, 0.5, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_dolphin_jump"), Hash40::new("top"), -0.0, 0, -5, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("lucina_dolphin_jump"), -1);
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("lucina_dolphin_swing"), -1);
        if VarModule::is_flag(agent.module_accessor, yu::status::flag::IS_EX) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_dolphin_shadow"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
}

unsafe extern "C" fn lucina_specialhi_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if VarModule::is_flag(agent.module_accessor, yu::status::flag::IS_EX) {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_waza_ex_p4"));
        }
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucina_special_h01"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_lucina_rnd_special_h"));
    }
}

unsafe extern "C" fn lucina_specialhi_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        if VarModule::is_flag(agent.module_accessor, yu::status::flag::IS_EX) {
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
        }
        else {
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
        }
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        if VarModule::is_flag(agent.module_accessor, yu::status::flag::IS_EX) {
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
        }
        else {
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
        }
    }
}

unsafe extern "C" fn lucina_specialairhi(agent: &mut L2CAgentBase) {
    if VarModule::is_flag(agent.module_accessor, yu::instance::flag::COMMAND)
    && spent_meter(agent.module_accessor, false) {
        let spent = VarModule::get_float(agent.module_accessor, yu::instance::float::SPENT_SP);
        let meter_max = VarModule::get_float(agent.module_accessor, yu::instance::float::SP_GAUGE_MAX);
        FGCModule::update_meter(agent.module_accessor, -spent, meter_max, yu::instance::float::SP_GAUGE);
        VarModule::set_int(agent.module_accessor, yu::instance::int::SP_FLASH_TIMER, 40);
        VarModule::on_flag(agent.module_accessor, yu::status::flag::IS_EX);
        sp_diff_checker(agent.module_accessor);
        full_invuln(agent.module_accessor, true);
        macros::FT_MOTION_RATE(agent, 2.0 / 3.0);
    }
    else {
        VarModule::off_flag(agent.module_accessor, yu::status::flag::IS_EX);
    }
    frame(agent.lua_state_agent, 3.0);
    if !VarModule::is_flag(agent.module_accessor, yu::status::flag::IS_EX) {
        if macros::is_excute(agent) {
            upper_invuln(agent.module_accessor, true);
        }
    }
    if VarModule::is_flag(agent.module_accessor, yu::status::flag::IS_EX) {
        frame(agent.lua_state_agent, 10.0);
        macros::FT_MOTION_RATE(agent, 1.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 79, 100, 100, 0, 5.1, 0.0, 11.0, 10.0, Some(0.0), Some(7.0), Some(10.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_SPECIAL_HI_SET_LR);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            full_invuln(agent.module_accessor, false);
            AttackModule::clear_all(agent.module_accessor);
            let ratio = get_damage_mul(agent.module_accessor);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0 * ratio, 40, 85, 0, 75, 6.5, 0.0, 12.0, 10.0, Some(0.0), Some(15.0), Some(10.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            // notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
        frame(agent.lua_state_agent, 24.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
    }
    else{
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            upper_invuln(agent.module_accessor, false);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 79, 100, 100, 0, 5.1, 0.0, 11.0, 10.0, Some(0.0), Some(7.0), Some(10.0), 1.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_SPECIAL_HI_SET_LR);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            let ratio = get_damage_mul(agent.module_accessor);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0 * ratio, 40, 80, 0, 70, 6.5, 0.0, 12.0, 10.0, Some(0.0), Some(15.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            // notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
        frame(agent.lua_state_agent, 24.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_KINETIC_CHANGE);
    }
}

unsafe extern "C" fn lucina_specialairhi_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if VarModule::is_flag(agent.module_accessor, yu::status::flag::IS_EX) {
        FGCModule::ex_flash(agent);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_dolphin_swing"), Hash40::new("top"), 0, 12, -1, 14, -30, 37, 1, true);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 6, Hash40::new("sword1"), 0.0, 0.0, 0.5, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_dolphin_jump"), Hash40::new("top"), -0.0, 0, -5, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("lucina_dolphin_jump"), -1);
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("lucina_dolphin_swing"), -1);
        if VarModule::is_flag(agent.module_accessor, yu::status::flag::IS_EX) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_dolphin_shadow"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
}

unsafe extern "C" fn lucina_specialairhi_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if VarModule::is_flag(agent.module_accessor, yu::status::flag::IS_EX) {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_waza_ex_p4"));
        }
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucina_special_h01"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_lucina_rnd_special_h"));
    }
}

unsafe extern "C" fn lucina_specialairhi_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        if VarModule::is_flag(agent.module_accessor, yu::status::flag::IS_EX) {
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
        }
        else {
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
        }
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        if VarModule::is_flag(agent.module_accessor, yu::status::flag::IS_EX) {
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
        }
        else {
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
        }
    }
}

unsafe extern "C" fn lucina_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let spent = VarModule::get_float(agent.module_accessor, yu::instance::float::SPENT_SP);
        let meter_max = VarModule::get_float(agent.module_accessor, yu::instance::float::SP_GAUGE_MAX);
        FGCModule::update_meter(agent.module_accessor, -spent, meter_max, yu::instance::float::SP_GAUGE);
        sp_diff_checker(agent.module_accessor);
        JostleModule::set_status(agent.module_accessor, false);
        KineticModule::unable_energy_all(agent.module_accessor);
        if VarModule::is_flag(agent.module_accessor, yu::instance::flag::ROMAN_ON_HIT) {
            VarModule::off_flag(agent.module_accessor, yu::instance::flag::DISABLE_SPECIAL_N_S);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 20.0, 0.0, 10.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 1, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            full_invuln(agent.module_accessor, true);
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, yu::status::flag::SPECIAL_LW_DECIDE_ROMAN_DIREC);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, yu::status::flag::SPECIAL_LW_ROMAN_MOVE);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 35.0, 0.0, 10.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 1, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    macros::FT_MOTION_RATE(agent, 5.0);
    frame(agent.lua_state_agent, 21.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        VarModule::off_flag(agent.module_accessor, yu::status::flag::SPECIAL_LW_ROMAN_MOVE);
        if VarModule::is_flag(agent.module_accessor, yu::instance::flag::ROMAN_ON_HIT) {
            full_invuln(agent.module_accessor, false);
        }
    }
}

unsafe extern "C" fn lucina_speciallw_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("lucina_counter_flash"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.3);
    }
}

unsafe extern "C" fn lucina_speciallw_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucina_special_l01"));
    }
}

unsafe extern "C" fn lucina_speciallwhit(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::FT_START_CUTIN(agent);
        macros::SLOW_OPPONENT(agent, 100.0, 12.0);
    }
    macros::FT_MOTION_RATE(agent, 2.0);
    frame(agent.lua_state_agent, 4.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 80, 60, 80, 30, 15.0, 0.0, 15.0, 0.0, None, None, None, 2.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 30.0, false);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
        VarModule::on_flag(agent.module_accessor, yu::instance::flag::SHADOW_FRENZY);
        VarModule::off_flag(agent.module_accessor, yu::instance::flag::DISABLE_SPECIAL_N_S);
        sp_diff_checker(agent.module_accessor);
    }
    macros::FT_MOTION_RATE(agent, 2.0);
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 20.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn lucina_specialairlwhit(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::FT_START_CUTIN(agent);
        macros::SLOW_OPPONENT(agent, 100.0, 12.0);
    }
    macros::FT_MOTION_RATE(agent, 2.0);
    frame(agent.lua_state_agent, 4.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 85, 60, 80, 35, 15.0, 0.0, 15.0, 0.0, None, None, None, 2.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 30.0, false);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
        VarModule::on_flag(agent.module_accessor, yu::instance::flag::SHADOW_FRENZY);
        VarModule::off_flag(agent.module_accessor, yu::instance::flag::DISABLE_SPECIAL_N_S);
        sp_diff_checker(agent.module_accessor);
    }
    macros::FT_MOTION_RATE(agent, 2.0);
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 20.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn lucina_speciallwhit_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("lucina_counter_flash"), Hash40::new("top"), 0, 15, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.3);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 1.0);
    }
}

unsafe extern "C" fn lucina_speciallwhit_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_lucina_special_l01"));
    }
}

unsafe extern "C" fn lucina_speciallwhit_exp(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitl"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_L);
    }
}

unsafe extern "C" fn lucina_lightningflash(agent: &mut L2CAgentBase) {
    let mut dmg : f32;
    let kbg : i32;
    if VarModule::is_flag(agent.module_accessor, yu::instance::flag::COMMAND)
    && spent_meter_super(agent.module_accessor) {
        let spent = VarModule::get_float(agent.module_accessor, yu::instance::float::SPENT_SP);
        let meter_max = VarModule::get_float(agent.module_accessor, yu::instance::float::SP_GAUGE_MAX);
        FGCModule::update_meter(agent.module_accessor, -spent, meter_max, yu::instance::float::SP_GAUGE);
        VarModule::set_int(agent.module_accessor, yu::instance::int::SP_FLASH_TIMER, 40);
        VarModule::on_flag(agent.module_accessor, yu::status::flag::IS_EX);
        sp_diff_checker(agent.module_accessor);
    }
    else {
        VarModule::off_flag(agent.module_accessor, yu::status::flag::IS_EX);
    }
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 2.0/3.0);
    frame(agent.lua_state_agent, 15.0);
    if VarModule::is_flag(agent.module_accessor, yu::status::flag::IS_EX) {
        macros::FT_MOTION_RATE(agent, 0.2);
    }
    else {
        macros::FT_MOTION_RATE(agent, 1.0);
    }
    frame(agent.lua_state_agent, 20.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 367, 140, 50, 0, 5.0, 0.0, 7.0, 7.0, Some(0.0), Some(7.0), Some(1.0), 0.4, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.5, 367, 140, 50, 0, 5.0, 0.0, 7.0, 7.0, Some(0.0), Some(7.0), Some(-5.0), 0.4, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 44.0);
    if macros::is_excute(agent) {
        if VarModule::is_flag(agent.module_accessor, yu::status::flag::IS_EX) {
            dmg = 13.0;
            kbg = 60;
        }
        else {
            dmg = 9.0;
            kbg = 66;
        }
        if shadow_id(agent.module_accessor) {
            dmg *= 0.8;
        }
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), dmg, 45, kbg, 0, 60, 2.5, 0.0, 8.5, -5.0, Some(0.0), Some(8.5), Some(20.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), dmg, 45, kbg, 0, 60, 2.5, 0.0, 8.5, 25.0, Some(0.0), Some(8.5), Some(27.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 47.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn lucina_lightningflash_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if VarModule::is_flag(agent.module_accessor, yu::status::flag::IS_EX) {
        FGCModule::ex_flash(agent);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_sword_purple"), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_breaker_sting"), Hash40::new("top"), -0.0, 11.0, 12, 0, 0, 0, 1.2, true);
        macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 9.5, 6, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.264, 0.47, 1.3);
        macros::LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(agent.lua_state_agent, 44.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_sp_flash"), Hash40::new("sword1"), -0.0, -0.0, 10, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 18, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 51.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucina_sword_purple"), false, true);
    }
}

unsafe extern "C" fn lucina_lightningflash_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if VarModule::is_flag(agent.module_accessor, yu::status::flag::IS_EX) {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_waza_super_p4"));
        }
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucina_jump02"));
    }
    frame(agent.lua_state_agent, 44.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_lucina_special_n01"));
        macros::PLAY_SE(agent, Hash40::new("se_lucina_special_n04"));
    }
}

unsafe extern "C" fn lucina_lightningflash_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("sword1"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohit11"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 44.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_piercel"), 0);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.effect_acmd("effect_specialnstart", lucina_specialnstart_eff);
    agent.sound_acmd("sound_specialnstart", lucina_specialnstart_snd);

    agent.effect_acmd("effect_specialairnstart", lucina_specialnstart_eff);
    agent.sound_acmd("sound_specialairnstart", lucina_specialnstart_snd);

    agent.game_acmd("game_specialnloop", lucina_specialnloop);

    agent.game_acmd("game_specialairnloop", lucina_specialnloop);

    agent.game_acmd("game_specialnend", lucina_specialnend);

    agent.game_acmd("game_specialairnend", lucina_specialnend);

    agent.game_acmd("game_specialnendmax", lucina_specialnendmax);

    agent.game_acmd("game_specialairnendmax", lucina_specialnendmax);

    agent.game_acmd("game_specialairs1", lucina_specialairs1);
    agent.effect_acmd("effect_specialairs1", lucina_specialairs1_eff);
    agent.sound_acmd("sound_specialairs1", lucina_specialairs1_snd);

    agent.game_acmd("game_specialairs2hi", lucina_specialairs2hi);
    agent.effect_acmd("effect_specialairs2hi", lucina_specialairs2hi_eff);
    agent.sound_acmd("sound_specialairs2hi", lucina_specialairs2hi_snd);

    agent.game_acmd("game_specials2hi", lucina_specials2hi);
    agent.effect_acmd("effect_specials2hi", lucina_specials2hi_eff);
    agent.sound_acmd("sound_specials2hi", lucina_specials2hi_snd);
    agent.expression_acmd("expression_specials2hi", lucina_specials2hi_exp);

    agent.game_acmd("game_specialhi", lucina_specialhi);
    agent.effect_acmd("effect_specialhi", lucina_specialhi_eff);
    agent.sound_acmd("sound_specialhi", lucina_specialhi_snd);
    agent.expression_acmd("expression_specialhi", lucina_specialhi_exp);

    agent.game_acmd("game_specialairhi", lucina_specialairhi);
    agent.effect_acmd("effect_specialairhi", lucina_specialairhi_eff);
    agent.sound_acmd("sound_specialairhi", lucina_specialairhi_snd);
    agent.expression_acmd("expression_specialairhi", lucina_specialairhi_exp);

    agent.game_acmd("game_speciallw", lucina_speciallw);
    agent.effect_acmd("effect_speciallw", lucina_speciallw_eff);
    agent.sound_acmd("sound_speciallw", lucina_speciallw_snd);

    agent.game_acmd("game_specialairlw", lucina_speciallw);
    agent.effect_acmd("effect_specialairlw", lucina_speciallw_eff);
    agent.sound_acmd("sound_specialairlw", lucina_speciallw_snd);

    agent.game_acmd("game_speciallwhit", lucina_speciallwhit);
    agent.effect_acmd("effect_speciallwhit", lucina_speciallwhit_eff);
    agent.sound_acmd("sound_speciallwhit", lucina_speciallwhit_snd);
    agent.expression_acmd("expression_speciallwhit", lucina_speciallwhit_exp);

    agent.game_acmd("game_specialairlwhit", lucina_specialairlwhit);
    agent.effect_acmd("effect_specialairlwhit", lucina_speciallwhit_eff);
    agent.sound_acmd("sound_specialairlwhit", lucina_speciallwhit_snd);
    agent.expression_acmd("expression_specialairlwhit", lucina_speciallwhit_exp);

    agent.game_acmd("game_specials1", lucina_lightningflash);
    agent.effect_acmd("effect_specials1", lucina_lightningflash_eff);
    agent.sound_acmd("sound_specials1", lucina_lightningflash_snd);
    agent.expression_acmd("expression_specials1", lucina_lightningflash_exp);
}