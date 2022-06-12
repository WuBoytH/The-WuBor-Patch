use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::{Hash40, Vector3f},
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    custom_var::*,
    wubor_utils::{vars::*, wua_bind::*},
    super::super::{
        vl,
        helper::*
    }
};

#[acmd_script( agent = "lucina", scripts = [ "game_specialnloop", "game_specialairnloop" ], category = ACMD_GAME, low_priority )]
unsafe fn lucina_specialnloop(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, 1.4);
}

#[acmd_script( agent = "lucina", scripts = [ "game_specialnend", "game_specialairnend" ], category = ACMD_GAME, low_priority )]
unsafe fn lucina_specialnend(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY as u64 {
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0, 361, 20, 0, 95, 3.0, 0.0, 5.0, 8.0, Some(0.0), Some(5.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 9.0, 361, 20, 0, 95, 3.0, 0.0, 5.0, 25.0, Some(0.0), Some(5.0), Some(27.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 361, 20, 0, 95, 0.9, 0.0, 5.0, 17.0, Some(0.0), Some(5.0), Some(22.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 361, 20, 0, 95, 0.9, 0.0, 5.0, 23.5, Some(0.0), Some(5.0), Some(28.9), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 9.0);
        if macros::is_excute(fighter) {
            AttackModule::clear(fighter.module_accessor, 0, false);
            AttackModule::clear(fighter.module_accessor, 1, false);
        }
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }
    else {
        frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            KineticModule::mul_speed(fighter.module_accessor, &Vector3f {x: 0.4, y: 0.0, z: 0.0}, *FIGHTER_KINETIC_TYPE_UNIQ);
        }
        macros::FT_MOTION_RATE(fighter, 0.125);
        frame(fighter.lua_state_agent, 9.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            let ratio;
            if shadow_id(fighter.module_accessor) {
                ratio = vl::param_private::shadow_type_attack_mul;
            }
            else {
                ratio = 1.0;
            }
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0 * ratio, 361, 20, 0, 95, 3.3, 0.0, 7.7, 9.1, Some(0.0), Some(7.7), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 9.0 * ratio, 361, 20, 0, 95, 4.0, 2.0, 1.0, 1.5, Some(14.0), Some(1.0), Some(1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 9.0 * ratio, 361, 20, 0, 95, 3.5, 2.0, 1.0, 7.5, Some(9.5), Some(1.0), Some(7.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }
}

#[acmd_script( agent = "lucina", scripts = [ "game_specialnendmax", "game_specialairnendmax" ], category = ACMD_GAME, low_priority )]
unsafe fn lucina_specialnendmax(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY as u64 {
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0, 60, 80, 0, 30, 3.0, 0.0, 5.0, 8.0, Some(0.0), Some(5.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 9.0, 60, 80, 0, 30, 3.0, 0.0, 5.0, 25.0, Some(0.0), Some(5.0), Some(27.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 60, 80, 0, 30, 0.9, 0.0, 5.0, 17.0, Some(0.0), Some(5.0), Some(22.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 60, 80, 0, 30, 0.9, 0.0, 5.0, 23.5, Some(0.0), Some(5.0), Some(28.9), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 9.0);
        if macros::is_excute(fighter) {
            AttackModule::clear(fighter.module_accessor, 0, false);
            AttackModule::clear(fighter.module_accessor, 1, false);
        }
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }
    else{
        frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            KineticModule::mul_speed(fighter.module_accessor, &Vector3f {x: 0.4, y: 0.0, z: 0.0}, *FIGHTER_KINETIC_TYPE_UNIQ);
        }
        macros::FT_MOTION_RATE(fighter, 0.125);
        frame(fighter.lua_state_agent, 9.0);
        macros::FT_MOTION_RATE(fighter, 1.25);
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            let ratio;
            if shadow_id(fighter.module_accessor) {
                ratio = vl::param_private::shadow_type_attack_mul;
            }
            else {
                ratio = 1.0;
            }
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0 * ratio, 60, 80, 0, 30, 3.3, 0.0, 7.7, 9.1, Some(0.0), Some(7.7), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 9.0 * ratio, 60, 80, 0, 30, 4.0, 2.0, 1.0, 1.5, Some(14.0), Some(1.0), Some(1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 9.0 * ratio, 60, 80, 0, 30, 3.5, 2.0, 1.0, 7.5, Some(9.5), Some(1.0), Some(7.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 17.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
}

#[acmd_script( agent = "lucina", script = "game_specialairs1", category = ACMD_GAME, low_priority )]
unsafe fn lucina_specialairs1(fighter: &mut L2CAgentBase) {
    if VarModule::is_flag(fighter.battle_object, yu::instance::flag::COMMAND)
    && spent_meter(fighter.battle_object, false) {
        let spent = VarModule::get_float(fighter.battle_object, yu::instance::float::SPENT_SP);
        let meter_max = VarModule::get_float(fighter.battle_object, yu::instance::float::SP_GAUGE_MAX);
        FGCModule::update_meter(fighter.battle_object, -spent, meter_max, yu::instance::float::SP_GAUGE);
        VarModule::set_int(fighter.battle_object, yu::instance::int::SP_FLASH_TIMER, 40);
        VarModule::on_flag(fighter.battle_object, yu::status::flag::IS_EX);
        sp_diff_checker(fighter.module_accessor);
    }
    else {
        VarModule::off_flag(fighter.battle_object, yu::status::flag::IS_EX);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
}

#[acmd_script( agent = "lucina", script = "effect_specialairs1", category = ACMD_EFFECT, low_priority )]
unsafe fn lucina_specialairs1_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "lucina", script = "game_specialairs2hi", category = ACMD_GAME, low_priority )]
unsafe fn lucina_specialairs2hi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        let mut dmg : f32;
        let kbg : i32;
        let velx : f32;
        let vely : f32;
        if VarModule::is_flag(fighter.battle_object, yu::status::flag::IS_EX) {
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
        if shadow_id(fighter.module_accessor) {
            dmg *= 0.8;
        }
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(fighter, velx, vely, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), dmg, 361, kbg, 0, 45, 2.5, 0.0, 8.5, 4.0, Some(0.0), Some(4.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), dmg, 361, kbg, 0, 45, 2.5, 0.0, 2.5, 3.0, Some(0.0), Some(11.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), dmg, 361, kbg, 0, 45, 2.5, 0.0, 2.5, 3.0, Some(0.0), Some(2.5), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
}

#[acmd_script( agent = "lucina", script = "effect_specialairs2hi", category = ACMD_EFFECT, low_priority )]
unsafe fn lucina_specialairs2hi_eff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("lucina_sword_purple"), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("lucina_breaker_sting"), Hash40::new("top"), -0.0, 6, 8, 40, 0, 0, 0.9, true);
        macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 7.5, 6, 40, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.264, 0.47, 1.3);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_sp_flash"), Hash40::new("sword1"), -0.0, -0.0, 10, 0, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
}

#[acmd_script( agent = "lucina", script = "sound_specialairs2hi", category = ACMD_SOUND, low_priority )]
unsafe fn lucina_specialairs2hi_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucina_special_n03"));
    }
}

#[acmd_script( agent = "lucina", script = "game_specials2hi", category = ACMD_GAME, low_priority )]
unsafe fn lucina_specials2hi(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "lucina", script = "effect_specials2hi", category = ACMD_EFFECT, low_priority )]
unsafe fn lucina_specials2hi_eff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "lucina", script = "sound_specials2hi", category = ACMD_SOUND, low_priority )]
unsafe fn lucina_specials2hi_snd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_LANDING_SE(fighter, Hash40::new("se_lucina_landing02"));
    }
}

#[acmd_script( agent = "lucina", script = "expression_specials2hi", category = ACMD_EXPRESSION, low_priority )]
unsafe fn lucina_specials2hi_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "lucina", script = "game_specialhi", category = ACMD_GAME, low_priority )]
unsafe fn lucina_specialhi(fighter: &mut L2CAgentBase) {
    if VarModule::is_flag(fighter.battle_object, yu::instance::flag::COMMAND)
    && spent_meter(fighter.battle_object, false) {
        let spent = VarModule::get_float(fighter.battle_object, yu::instance::float::SPENT_SP);
        let meter_max = VarModule::get_float(fighter.battle_object, yu::instance::float::SP_GAUGE_MAX);
        FGCModule::update_meter(fighter.battle_object, -spent, meter_max, yu::instance::float::SP_GAUGE);
        VarModule::set_int(fighter.battle_object, yu::instance::int::SP_FLASH_TIMER, 40);
        VarModule::on_flag(fighter.battle_object, yu::status::flag::IS_EX);
        sp_diff_checker(fighter.module_accessor);
        full_invuln(fighter.module_accessor, true);
        macros::FT_MOTION_RATE(fighter, 2.0 / 3.0);
    }
    else {
        VarModule::off_flag(fighter.battle_object, yu::status::flag::IS_EX);
        upper_invuln(fighter.module_accessor, true);
    }
    if VarModule::is_flag(fighter.battle_object, yu::status::flag::IS_EX) {
        frame(fighter.lua_state_agent, 10.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 79, 100, 200, 0, 5.1, 0.0, 11.0, 10.0, Some(0.0), Some(7.0), Some(10.0), 1.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_SPECIAL_HI_SET_LR);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            full_invuln(fighter.module_accessor, false);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 79, 100, 90, 0, 6.5, 0.0, 12.0, 10.0, Some(0.0), Some(21.0), Some(10.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
        wait(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            let ratio;
            if shadow_id(fighter.module_accessor) {
                ratio = vl::param_private::shadow_type_attack_mul;
            }
            else {
                ratio = 1.0;
            }
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0 * ratio, 40, 85, 0, 75, 6.5, 0.0, 12.0, 10.0, Some(0.0), Some(21.0), Some(10.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 24.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }
    else{
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            upper_invuln(fighter.module_accessor, false);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 79, 100, 200, 0, 5.1, 0.0, 11.0, 10.0, Some(0.0), Some(7.0), Some(10.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_SPECIAL_HI_SET_LR);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 79, 100, 90, 0, 6.5, 0.0, 12.0, 10.0, Some(0.0), Some(21.0), Some(10.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
        wait(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            let ratio;
            if shadow_id(fighter.module_accessor) {
                ratio = vl::param_private::shadow_type_attack_mul;
            }
            else {
                ratio = 1.0;
            }
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0 * ratio, 40, 80, 0, 70, 6.5, 0.0, 12.0, 10.0, Some(0.0), Some(21.0), Some(10.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 24.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }
    frame(fighter.lua_state_agent, 36.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_KINETIC_CHANGE);
    }
}

#[acmd_script( agent = "lucina", script = "effect_specialhi", category = ACMD_EFFECT, low_priority )]
unsafe fn lucina_specialhi_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if VarModule::is_flag(fighter.battle_object, yu::status::flag::IS_EX) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -2, -10, 15, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("lucina_dolphin_swing"), Hash40::new("top"), 0, 12, -1, 14, -30, 37, 1, true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 6, Hash40::new("sword1"), 0.0, 0.0, 0.5, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("lucina_dolphin_jump"), Hash40::new("top"), -0.0, 0, -5, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_DETACH_KIND(fighter, Hash40::new("lucina_dolphin_jump"), -1);
        macros::EFFECT_DETACH_KIND(fighter, Hash40::new("lucina_dolphin_swing"), -1);
        if VarModule::is_flag(fighter.battle_object, yu::status::flag::IS_EX) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("lucina_dolphin_shadow"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        }
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE_OFF(fighter, 4);
    }
}

#[acmd_script( agent = "lucina", script = "sound_specialhi", category = ACMD_SOUND, low_priority )]
unsafe fn lucina_specialhi_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucina_special_h01"));
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_lucina_rnd_special_h"));
    }
}

#[acmd_script( agent = "lucina", script = "expression_specialhi", category = ACMD_EXPRESSION, low_priority )]
unsafe fn lucina_specialhi_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_slashm"), 0);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, yu::status::flag::IS_EX) {
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_slashl"), 0);
        }
        else {
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_slashm"), 0);
        }
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, yu::status::flag::IS_EX) {
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_slashl"), 0);
        }
        else {
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_slashm"), 0);
        }
    }
}

#[acmd_script( agent = "lucina", script = "game_specialairhi", category = ACMD_GAME, low_priority )]
unsafe fn lucina_specialairhi(fighter: &mut L2CAgentBase) {
    if VarModule::is_flag(fighter.battle_object, yu::instance::flag::COMMAND)
    && spent_meter(fighter.battle_object, false) {
        let spent = VarModule::get_float(fighter.battle_object, yu::instance::float::SPENT_SP);
        let meter_max = VarModule::get_float(fighter.battle_object, yu::instance::float::SP_GAUGE_MAX);
        FGCModule::update_meter(fighter.battle_object, -spent, meter_max, yu::instance::float::SP_GAUGE);
        VarModule::set_int(fighter.battle_object, yu::instance::int::SP_FLASH_TIMER, 40);
        VarModule::on_flag(fighter.battle_object, yu::status::flag::IS_EX);
        sp_diff_checker(fighter.module_accessor);
        full_invuln(fighter.module_accessor, true);
        macros::FT_MOTION_RATE(fighter, 2.0 / 3.0);
    }
    else {
        VarModule::off_flag(fighter.battle_object, yu::status::flag::IS_EX);
    }
    frame(fighter.lua_state_agent, 3.0);
    if !VarModule::is_flag(fighter.battle_object, yu::status::flag::IS_EX) {
        if macros::is_excute(fighter) {
            upper_invuln(fighter.module_accessor, true);
        }
    }
    if VarModule::is_flag(fighter.battle_object, yu::status::flag::IS_EX) {
        frame(fighter.lua_state_agent, 10.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 79, 100, 200, 0, 5.1, 0.0, 11.0, 10.0, Some(0.0), Some(7.0), Some(10.0), 1.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_SPECIAL_HI_SET_LR);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            full_invuln(fighter.module_accessor, false);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 79, 100, 90, 0, 6.5, 0.0, 12.0, 10.0, Some(0.0), Some(21.0), Some(10.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
        wait(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            let ratio;
            if shadow_id(fighter.module_accessor) {
                ratio = vl::param_private::shadow_type_attack_mul;
            }
            else {
                ratio = 1.0;
            }
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0 * ratio, 40, 85, 0, 75, 6.5, 0.0, 12.0, 10.0, Some(0.0), Some(21.0), Some(10.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 24.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }
    else{
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            upper_invuln(fighter.module_accessor, false);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 79, 100, 200, 0, 5.1, 0.0, 11.0, 10.0, Some(0.0), Some(7.0), Some(10.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_SPECIAL_HI_SET_LR);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 79, 100, 90, 0, 6.5, 0.0, 12.0, 10.0, Some(0.0), Some(21.0), Some(10.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
        wait(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            let ratio;
            if shadow_id(fighter.module_accessor) {
                ratio = vl::param_private::shadow_type_attack_mul;
            }
            else {
                ratio = 1.0;
            }
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0 * ratio, 40, 80, 0, 70, 6.5, 0.0, 12.0, 10.0, Some(0.0), Some(21.0), Some(10.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 24.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }
    frame(fighter.lua_state_agent, 36.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_KINETIC_CHANGE);
    }
}

#[acmd_script( agent = "lucina", script = "effect_specialairhi", category = ACMD_EFFECT, low_priority )]
unsafe fn lucina_specialairhi_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if VarModule::is_flag(fighter.battle_object, yu::status::flag::IS_EX) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -2, -10, 15, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("lucina_dolphin_swing"), Hash40::new("top"), 0, 12, -1, 14, -30, 37, 1, true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 6, Hash40::new("sword1"), 0.0, 0.0, 0.5, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("lucina_dolphin_jump"), Hash40::new("top"), -0.0, 0, -5, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_DETACH_KIND(fighter, Hash40::new("lucina_dolphin_jump"), -1);
        macros::EFFECT_DETACH_KIND(fighter, Hash40::new("lucina_dolphin_swing"), -1);
        if VarModule::is_flag(fighter.battle_object, yu::status::flag::IS_EX) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("lucina_dolphin_shadow"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        }
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE_OFF(fighter, 4);
    }
}

#[acmd_script( agent = "lucina", script = "sound_specialairhi", category = ACMD_SOUND, low_priority )]
unsafe fn lucina_specialairhi_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucina_special_h01"));
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_lucina_rnd_special_h"));
    }
}

#[acmd_script( agent = "lucina", script = "expression_specialairhi", category = ACMD_EXPRESSION, low_priority )]
unsafe fn lucina_specialairhi_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_slashm"), 0);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, yu::status::flag::IS_EX) {
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_slashl"), 0);
        }
        else {
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_slashm"), 0);
        }
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, yu::status::flag::IS_EX) {
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_slashl"), 0);
        }
        else {
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_slashm"), 0);
        }
    }
}

#[acmd_script( agent = "lucina", scripts = [ "game_speciallw", "game_specialairlw" ], category = ACMD_GAME, low_priority )]
unsafe fn lucina_speciallw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        let spent = VarModule::get_float(fighter.battle_object, yu::instance::float::SPENT_SP);
        let meter_max = VarModule::get_float(fighter.battle_object, yu::instance::float::SP_GAUGE_MAX);
        FGCModule::update_meter(fighter.battle_object, -spent, meter_max, yu::instance::float::SP_GAUGE);
        sp_diff_checker(fighter.module_accessor);
        JostleModule::set_status(fighter.module_accessor, false);
        KineticModule::unable_energy_all(fighter.module_accessor);
        if VarModule::is_flag(fighter.battle_object, yu::instance::flag::ROMAN_ON_HIT) {
            VarModule::off_flag(fighter.battle_object, yu::instance::flag::DISABLE_SPECIAL_N_S);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 20.0, 0.0, 10.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 1, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            full_invuln(fighter.module_accessor, true);
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, yu::status::flag::SPECIAL_LW_DECIDE_ROMAN_DIREC);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, yu::status::flag::SPECIAL_LW_ROMAN_MOVE);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 35.0, 0.0, 10.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 1, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    macros::FT_MOTION_RATE(fighter, 5.0);
    frame(fighter.lua_state_agent, 21.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, yu::status::flag::SPECIAL_LW_ROMAN_MOVE);
        if VarModule::is_flag(fighter.battle_object, yu::instance::flag::ROMAN_ON_HIT) {
            full_invuln(fighter.module_accessor, false);
        }
    }
}

#[acmd_script( agent = "lucina", scripts = [ "effect_speciallw", "effect_specialairlw" ], category = ACMD_EFFECT, low_priority )]
unsafe fn lucina_speciallw_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("lucina_counter_flash"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.3);
    }
}

#[acmd_script( agent = "lucina", scripts = [ "sound_speciallw", "sound_specialairlw" ], category = ACMD_SOUND, low_priority )]
unsafe fn lucina_speciallw_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucina_special_l01"));
    }
}

#[acmd_script( agent = "lucina", script = "game_speciallwhit", category = ACMD_GAME, low_priority )]
unsafe fn lucina_speciallw_hit(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::FT_START_CUTIN(fighter);
        macros::SLOW_OPPONENT(fighter, 100.0, 12.0);
    }
    macros::FT_MOTION_RATE(fighter, 2.0);
    frame(fighter.lua_state_agent, 4.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 80, 60, 80, 30, 15.0, 0.0, 15.0, 0.0, None, None, None, 2.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 30.0, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        VarModule::on_flag(fighter.battle_object, yu::instance::flag::SHADOW_FRENZY);
        VarModule::off_flag(fighter.battle_object, yu::instance::flag::DISABLE_SPECIAL_N_S);
        sp_diff_checker(fighter.module_accessor);
    }
    macros::FT_MOTION_RATE(fighter, 2.0);
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 20.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "lucina", script = "game_specialairlwhit", category = ACMD_GAME, low_priority )]
unsafe fn lucina_specialairlw_hit(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::FT_START_CUTIN(fighter);
        macros::SLOW_OPPONENT(fighter, 100.0, 12.0);
    }
    macros::FT_MOTION_RATE(fighter, 2.0);
    frame(fighter.lua_state_agent, 4.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 85, 60, 80, 35, 15.0, 0.0, 15.0, 0.0, None, None, None, 2.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 30.0, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        VarModule::on_flag(fighter.battle_object, yu::instance::flag::SHADOW_FRENZY);
        VarModule::off_flag(fighter.battle_object, yu::instance::flag::DISABLE_SPECIAL_N_S);
        sp_diff_checker(fighter.module_accessor);
    }
    macros::FT_MOTION_RATE(fighter, 2.0);
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 20.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "lucina", scripts = [ "effect_speciallwhit", "effect_specialairlwhit" ], category = ACMD_EFFECT, low_priority )]
unsafe fn lucina_speciallw_hit_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("lucina_counter_flash"), Hash40::new("top"), 0, 15, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.3);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 1.0);
    }
}

#[acmd_script( agent = "lucina", scripts = [ "sound_speciallwhit", "sound_specialairlwhit" ], category = ACMD_SOUND, low_priority )]
unsafe fn lucina_speciallw_hit_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_lucina_special_l01"));
    }
}

#[acmd_script( agent = "lucina", scripts = [ "expression_speciallwhit", "expression_specialairlwhit" ], category = ACMD_EXPRESSION, low_priority )]
unsafe fn lucina_speciallw_hit_exp(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_nohitl"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_slashl"), 0);
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
    }
}

#[acmd_script( agent = "lucina", script = "game_specials1", category = ACMD_GAME, low_priority )]
unsafe fn lucina_lightningflash(fighter: &mut L2CAgentBase) {
    let mut dmg : f32;
    let kbg : i32;
    if VarModule::is_flag(fighter.battle_object, yu::instance::flag::COMMAND)
    && spent_meter(fighter.battle_object, false) {
        let spent = VarModule::get_float(fighter.battle_object, yu::instance::float::SPENT_SP);
        let meter_max = VarModule::get_float(fighter.battle_object, yu::instance::float::SP_GAUGE_MAX);
        FGCModule::update_meter(fighter.battle_object, -spent, meter_max, yu::instance::float::SP_GAUGE);
        VarModule::set_int(fighter.battle_object, yu::instance::int::SP_FLASH_TIMER, 40);
        VarModule::on_flag(fighter.battle_object, yu::status::flag::IS_EX);
        sp_diff_checker(fighter.module_accessor);
    }
    else {
        VarModule::off_flag(fighter.battle_object, yu::status::flag::IS_EX);
    }
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 2.0/3.0);
    frame(fighter.lua_state_agent, 15.0);
    if VarModule::is_flag(fighter.battle_object, yu::status::flag::IS_EX) {
        macros::FT_MOTION_RATE(fighter, 0.2);
    }
    else {
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
    frame(fighter.lua_state_agent, 20.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.5, 367, 140, 50, 0, 5.0, 0.0, 7.0, 7.0, Some(0.0), Some(7.0), Some(1.0), 0.4, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.5, 367, 140, 50, 0, 5.0, 0.0, 7.0, 7.0, Some(0.0), Some(7.0), Some(-5.0), 0.4, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 44.0);
    if macros::is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, yu::status::flag::IS_EX) {
            dmg = 13.0;
            kbg = 60;
        }
        else {
            dmg = 9.0;
            kbg = 66;
        }
        if shadow_id(fighter.module_accessor) {
            dmg *= 0.8;
        }
        AttackModule::clear_all(fighter.module_accessor);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), dmg, 45, kbg, 0, 60, 2.5, 0.0, 8.5, -5.0, Some(0.0), Some(8.5), Some(20.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), dmg, 45, kbg, 0, 60, 2.5, 0.0, 8.5, 25.0, Some(0.0), Some(8.5), Some(27.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 47.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "lucina", script = "effect_specials1", category = ACMD_EFFECT, low_priority )]
unsafe fn lucina_lightningflash_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("lucina_sword_purple"), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 42.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("lucina_breaker_sting"), Hash40::new("top"), -0.0, 11.0, 12, 0, 0, 0, 1.2, true);
        macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 9.5, 6, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.264, 0.47, 1.3);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 44.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("sword1"), -0.0, -0.0, 10, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 46.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 18, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 51.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("lucina_sword_purple"), false, true);
    }
}

#[acmd_script( agent = "lucina", script = "sound_specials1", category = ACMD_SOUND, low_priority )]
unsafe fn lucina_lightningflash_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucina_jump02"));
    }
    frame(fighter.lua_state_agent, 44.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_lucina_special_n01"));
        macros::PLAY_SE(fighter, Hash40::new("se_lucina_special_n04"));
    }
}

#[acmd_script( agent = "lucina", script = "expression_specials1", category = ACMD_EXPRESSION, low_priority )]
unsafe fn lucina_lightningflash_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("sword1"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 42.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohit11"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 44.0);
    if macros::is_excute(fighter) {
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_piercel"), 0);
    }
}

pub fn install() {
    install_acmd_scripts!(
        lucina_specialnloop,
        lucina_specialnend,
        lucina_specialnendmax,
        lucina_specialairs1, lucina_specialairs1_eff,
        lucina_specialairs2hi, lucina_specialairs2hi_eff, lucina_specialairs2hi_snd,
        lucina_specials2hi, lucina_specials2hi_eff, lucina_specials2hi_snd, lucina_specials2hi_exp,
        lucina_specialhi, lucina_specialhi_eff, lucina_specialhi_snd, lucina_specialhi_exp,
        lucina_specialairhi, lucina_specialairhi_eff, lucina_specialairhi_snd, lucina_specialairhi_exp,
        lucina_speciallw, lucina_speciallw_eff, lucina_speciallw_snd,
        lucina_speciallw_hit, lucina_specialairlw_hit, lucina_speciallw_hit_eff, lucina_speciallw_hit_snd, lucina_speciallw_hit_exp,
        lucina_lightningflash, lucina_lightningflash_eff, lucina_lightningflash_snd, lucina_lightningflash_exp
    );
}