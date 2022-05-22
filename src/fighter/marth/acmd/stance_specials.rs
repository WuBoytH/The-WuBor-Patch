use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::Hash40,
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    super::super::vars::*
};

#[acmd_script( agent = "marth", script = "game_speciallwspecials", category = ACMD_GAME, low_priority )]
unsafe fn marth_speciallwspecials(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S_FLAG_FLASHING_BLADE);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S_FLAG_DASH);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
    }
    wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
    }
    frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S_FLAG_END);
    }
}

#[acmd_script( agent = "marth", script = "effect_speciallwspecials", category = ACMD_EFFECT, low_priority )]
unsafe fn marth_speciallwspecials_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 9, -8, 0, 180, 0, 1.5, true);
        macros::LAST_PARTICLE_SET_COLOR(fighter, 0.4, 0.6, 1);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 9, -8, 0, 180, 0, 1.5, true);
        macros::LAST_PARTICLE_SET_COLOR(fighter, 0.4, 0.6, 1);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 9, -8, 0, 180, 0, 1.5, true);
        macros::LAST_PARTICLE_SET_COLOR(fighter, 0.4, 0.6, 1);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 9, -8, 0, 180, 0, 1.5, true);
        macros::LAST_PARTICLE_SET_COLOR(fighter, 0.4, 0.6, 1);
    }
    frame(fighter.lua_state_agent, 30.0);
    for _ in 0..3 {
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 8, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 3.0);
    }
}

#[acmd_script( agent = "marth", script = "sound_speciallwspecials", category = ACMD_SOUND, low_priority )]
unsafe fn marth_speciallwspecials_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_marth_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_marth_escape"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_marth_dash_stop"));
        macros::SET_PLAY_INHIVIT(fighter, Hash40::new("se_marth_dash_stop"), 20);
    }
}

#[acmd_script( agent = "marth", script = "expression_speciallwspecials", category = ACMD_EXPRESSION, low_priority )]
unsafe fn marth_speciallwspecials_exp(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_dash"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        )
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_dash"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        )
    }
}

#[acmd_script( agent = "marth", script = "game_speciallwspecialairs", category = ACMD_GAME, low_priority )]
unsafe fn marth_speciallwspecialairs(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S_FLAG_FLASHING_BLADE);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S_FLAG_DASH);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
    }
    wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
    }
    frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S_FLAG_END);
    }
}

#[acmd_script( agent = "marth", script = "effect_speciallwspecialairs", category = ACMD_EFFECT, low_priority )]
unsafe fn marth_speciallwspecialairs_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        let angle = WorkModule::get_float(fighter.module_accessor, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S_FLOAT_ANGLE);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 9, -8, angle, 180, 0, 1.5, true);
        macros::LAST_PARTICLE_SET_COLOR(fighter, 0.4, 0.6, 1);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        let angle = WorkModule::get_float(fighter.module_accessor, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S_FLOAT_ANGLE);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 9, -8, angle, 180, 0, 1.5, true);
        macros::LAST_PARTICLE_SET_COLOR(fighter, 0.4, 0.6, 1);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        let angle = WorkModule::get_float(fighter.module_accessor, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S_FLOAT_ANGLE);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 9, -8, angle, 180, 0, 1.5, true);
        macros::LAST_PARTICLE_SET_COLOR(fighter, 0.4, 0.6, 1);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        let angle = WorkModule::get_float(fighter.module_accessor, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S_FLOAT_ANGLE);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 9, -8, angle, 180, 0, 1.5, true);
        macros::LAST_PARTICLE_SET_COLOR(fighter, 0.4, 0.6, 1);
    }
}

#[acmd_script( agent = "marth", script = "sound_speciallwspecialairs", category = ACMD_SOUND, low_priority )]
unsafe fn marth_speciallwspecialairs_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_marth_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_marth_escape"));
    }
}

#[acmd_script( agent = "marth", script = "expression_speciallwspecialairs", category = ACMD_EXPRESSION, low_priority )]
unsafe fn marth_speciallwspecialairs_exp(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_dash"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        )
    }
}

#[acmd_script( agent = "marth", script = "effect_speciallwspecials2start", category = ACMD_EFFECT, low_priority )]
unsafe fn marth_speciallwspecials2start_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_marth_sword1"), Hash40::new("tex_marth_sword2"), 10, Hash40::new("sword1"), 0.0, 0.0, 0.5, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new("marth_sword_blue"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("sword1"), -0.0, -0.0, 10, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "marth", script = "sound_speciallwspecials2start", category = ACMD_SOUND, low_priority )]
unsafe fn marth_speciallwspecials2start_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_marth_swing_l"));
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_marth_special_n03"));
    }
}

#[acmd_script( agent = "marth", script = "expression_speciallwspecials2start", category = ACMD_EXPRESSION, low_priority )]
unsafe fn marth_speciallwspecials2start_exp(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_nohitm"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
}

#[acmd_script( agent = "marth", script = "game_speciallwspecials2loop", category = ACMD_GAME, low_priority )]
unsafe fn marth_speciallwspecials2loop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 0, 12, 60, 20, 2.0, 0.0, 8.5, 20.0, Some(0.0), Some(8.5), Some(15.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 0, 12, 100, 60, 2.0, 0.0, 8.5, 20.0, Some(0.0), Some(8.5), Some(6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 30, 12, 100, 60, 2.0, 0.0, 4.0, 6.0, Some(0.0), Some(4.0), Some(20.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.0, 350, 12, 100, 60, 2.0, 0.0, 13.0, 6.0, Some(0.0), Some(13.0), Some(20.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 3.6);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 3.6);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 3.6);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 3, 3.6);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 0, 12, 60, 20, 2.0, 0.0, 8.5, 20.0, Some(0.0), Some(8.5), Some(15.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 0, 12, 100, 60, 2.0, 0.0, 8.5, 20.0, Some(0.0), Some(8.5), Some(6.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 30, 12, 100, 60, 2.0, 0.0, 4.0, 6.0, Some(0.0), Some(4.0), Some(20.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.0, 350, 12, 100, 60, 2.0, 0.0, 13.0, 6.0, Some(0.0), Some(13.0), Some(20.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 3.6);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 3.6);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 3.6);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 3, 3.6);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 0, 12, 60, 20, 2.0, 0.0, 8.5, 20.0, Some(0.0), Some(8.5), Some(15.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 0, 12, 100, 60, 2.0, 0.0, 8.5, 20.0, Some(0.0), Some(8.5), Some(6.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 30, 12, 100, 60, 2.0, 0.0, 4.0, 6.0, Some(0.0), Some(4.0), Some(20.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.0, 350, 12, 100, 60, 2.0, 0.0, 13.0, 6.0, Some(0.0), Some(13.0), Some(20.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        let loop_count = WorkModule::get_int(fighter.module_accessor, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S2_LOOP_COUNT) - 1;
        let setoff_mul = 10.0 - (loop_count as f32 * 2.5);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, setoff_mul);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, setoff_mul);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, setoff_mul);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 3, setoff_mul);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "marth", script = "effect_speciallwspecials2loop", category = ACMD_EFFECT, low_priority )]
unsafe fn marth_speciallwspecials2loop_eff(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S2_LOOP_COUNT) == 1 {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("marth_sword_blue"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("marth_breaker_sting"), Hash40::new("top"), -0.0, 8.0, 10, 0, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("marth_breaker_sting"), Hash40::new("top"), -0.0, 3.0, 5, 0, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("marth_breaker_sting"), Hash40::new("top"), -0.0, 14.0, 7, 0, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
}

#[acmd_script( agent = "marth", script = "sound_speciallwspecials2loop", category = ACMD_SOUND, low_priority )]
unsafe fn marth_speciallwspecials2loop_snd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_marth_swing_s"));
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_marth_rnd_attack"));
        macros::PLAY_SE(fighter, Hash40::new("se_marth_swing_s"));
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_marth_swing_s"));
    }
}

#[acmd_script( agent = "marth", script = "expression_speciallwspecials2loop", category = ACMD_EXPRESSION, low_priority )]
unsafe fn marth_speciallwspecials2loop_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_slashs"), 0);
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_nohits"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_nohits"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_nohits"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
}

#[acmd_script( agent = "marth", script = "game_speciallwspecials2end2", category = ACMD_GAME, low_priority )]
unsafe fn marth_speciallwspecials2end2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 361, 50, 0, 90, 3.0, 0.0, 7.5, 20.0, Some(0.0), Some(7.5), Some(6.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_FINAL, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "marth", script = "effect_speciallwspecials2end2", category = ACMD_EFFECT, low_priority )]
unsafe fn marth_speciallwspecials2end2_eff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -0.0, 9.0, -10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_marth_sword1"), Hash40::new("tex_marth_sword2"), 10, Hash40::new("sword1"), 0.0, 0.0, 0.5, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new("marth_sword_blue"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("marth_breaker_sting"), Hash40::new("top"), -0.0, 8.0, 10, 0, 0, 0, 1.0, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE_OFF(fighter, 4);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "marth", script = "sound_speciallwspecials2end2", category = ACMD_SOUND, low_priority )]
unsafe fn marth_speciallwspecials2end2_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_marth_rnd_special_l"));
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_marth_swing_l"));
    }
}

#[acmd_script( agent = "marth", script = "expression_speciallwspecials2end2", category = ACMD_EXPRESSION, low_priority )]
unsafe fn marth_speciallwspecials2end2_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_nohitl"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_slash_critical"), 0);
    }
}

#[acmd_script( agent = "marth", script = "effect_speciallwspecialairs2start", category = ACMD_EFFECT, low_priority )]
unsafe fn marth_speciallwspecialairs2start_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_marth_sword1"), Hash40::new("tex_marth_sword2"), 10, Hash40::new("sword1"), 0.0, 0.0, 0.5, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new("marth_sword_blue"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("sword1"), -0.0, -0.0, 10, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "marth", script = "sound_speciallwspecialairs2start", category = ACMD_SOUND, low_priority )]
unsafe fn marth_speciallwspecialairs2start_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_marth_swing_l"));
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_marth_special_n03"));
    }
}

#[acmd_script( agent = "marth", script = "expression_speciallwspecialairs2start", category = ACMD_EXPRESSION, low_priority )]
unsafe fn marth_speciallwspecialairs2start_exp(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_nohitm"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
}

#[acmd_script( agent = "marth", script = "game_speciallwspecialairs2loop", category = ACMD_GAME, low_priority )]
unsafe fn marth_speciallwspecialairs2loop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 367, 30, 20, 60, 4.5, 0.0, 0.0, 12.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 305, 30, 60, 60, 4.5, 0.0, 0.0, 12.0, Some(0.0), Some(8.5), Some(6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 3.6);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 3.6);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 367, 30, 20, 60, 4.5, 0.0, 0.0, 12.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 305, 30, 60, 60, 4.5, 0.0, 0.0, 12.0, Some(0.0), Some(8.5), Some(6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 3.6);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 3.6);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 367, 30, 20, 60, 4.5, 0.0, 0.0, 12.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 305, 30, 60, 60, 4.5, 0.0, 0.0, 12.0, Some(0.0), Some(8.5), Some(6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 3.6);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 3.6);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "marth", script = "effect_speciallwspecialairs2loop", category = ACMD_EFFECT, low_priority )]
unsafe fn marth_speciallwspecialairs2loop_eff(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S2_LOOP_COUNT) == 1 {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("marth_sword_blue"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("marth_breaker_sting"), Hash40::new("top"), -0.0, 8.0, 10, 50, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("marth_breaker_sting"), Hash40::new("top"), -0.0, 3.0, 5, 50, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("marth_breaker_sting"), Hash40::new("top"), -0.0, 14.0, 7, 50, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
}

#[acmd_script( agent = "marth", script = "sound_speciallwspecialairs2loop", category = ACMD_SOUND, low_priority )]
unsafe fn marth_speciallwspecialairs2loop_snd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_marth_swing_s"));
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_marth_rnd_attack"));
        macros::PLAY_SE(fighter, Hash40::new("se_marth_swing_s"));
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_marth_swing_s"));
    }
}

#[acmd_script( agent = "marth", script = "expression_speciallwspecialairs2loop", category = ACMD_EXPRESSION, low_priority )]
unsafe fn marth_speciallwspecialairs2loop_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_slashs"), 0);
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_nohits"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_nohits"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_nohits"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
}

#[acmd_script( agent = "marth", script = "game_speciallwspecialairs2end", category = ACMD_GAME, low_priority )]
unsafe fn marth_speciallwspecialairs2end(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 30, 12, 0, 65, 4.0, 0.0, 0.0, 12.0, Some(0.0), Some(8.5), Some(6.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "marth", script = "effect_speciallwspecialairs2end", category = ACMD_EFFECT, low_priority )]
unsafe fn marth_speciallwspecialairs2end_eff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("marth_sword_blue"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("marth_breaker_sting"), Hash40::new("top"), -0.0, 8.0, 10, 50, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
}

#[acmd_script( agent = "marth", script = "sound_speciallwspecialairs2end", category = ACMD_SOUND, low_priority )]
unsafe fn marth_speciallwspecialairs2end_snd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_marth_swing_m"));
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_marth_rnd_attack"));
    }
}

#[acmd_script( agent = "marth", script = "expression_speciallwspecialairs2end", category = ACMD_EXPRESSION, low_priority )]
unsafe fn marth_speciallwspecialairs2end_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_slashm"), 0);
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_nohitm"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
}

pub fn install() {
    install_acmd_scripts!(
        marth_speciallwspecials, marth_speciallwspecials_eff, marth_speciallwspecials_snd, marth_speciallwspecials_exp,
        marth_speciallwspecialairs, marth_speciallwspecialairs_eff, marth_speciallwspecialairs_snd, marth_speciallwspecialairs_exp,
        marth_speciallwspecials2start_eff, marth_speciallwspecials2start_snd, marth_speciallwspecials2start_exp,
        marth_speciallwspecials2loop, marth_speciallwspecials2loop_eff, marth_speciallwspecials2loop_snd, marth_speciallwspecials2loop_exp,
        marth_speciallwspecials2end2, marth_speciallwspecials2end2_eff, marth_speciallwspecials2end2_snd, marth_speciallwspecials2end2_exp,
        marth_speciallwspecialairs2start_eff, marth_speciallwspecialairs2start_snd, marth_speciallwspecialairs2start_exp,
        marth_speciallwspecialairs2loop, marth_speciallwspecialairs2loop_eff, marth_speciallwspecialairs2loop_snd, marth_speciallwspecialairs2loop_exp,
        marth_speciallwspecialairs2end, marth_speciallwspecialairs2end_eff, marth_speciallwspecialairs2end_snd, marth_speciallwspecialairs2end_exp
    );
}