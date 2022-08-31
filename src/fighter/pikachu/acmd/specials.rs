use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::Hash40,
        app::{lua_bind::*, sv_animcmd::*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    custom_var::*,
    wubor_utils::vars::*
};

#[acmd_script( agent = "pikachu", scripts = [ "game_specialn", "game_specialairn" ], category = ACMD_GAME, low_priority )]
unsafe fn pikachu_specialn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 4.0 / 3.0);
    frame(fighter.lua_state_agent, 13.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 38, 100, 70, 0, 3.0, 0.0, 5.0, 18.0, Some(0.0), Some(5.0), Some(6.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_TAIL);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 8.0, false);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_DENGEKIDAMA, false, -1);
    }
}

#[acmd_script( agent = "pikachu", script = "effect_specialn" , category = ACMD_EFFECT, low_priority )]
unsafe fn pikachu_specialn_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pikachu_cheek"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
        let lr = PostureModule::lr(fighter.module_accessor);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("pikachu_dengeki"), Hash40::new("tail3"), 0, -3.0 * lr, 0, 0, 0, 0, 0.8, true);
    }
    for _ in 0..3 {
        if macros::is_excute(fighter) {
            macros::FLASH(fighter, 0, 0, 1, 0.196);
        }
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::FLASH(fighter, 0, 0, 1, 0.353);
        }
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::COL_NORMAL(fighter);
        }
        wait(fighter.lua_state_agent, 1.0);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pikachu_dengeki"), true, true);
    }
}

#[acmd_script( agent = "pikachu", script = "effect_specialairn" , category = ACMD_EFFECT, low_priority )]
unsafe fn pikachu_specialairn_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pikachu_cheek"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
        let lr = PostureModule::lr(fighter.module_accessor);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("pikachu_dengeki"), Hash40::new("tail3"), 0, -3.0 * lr, 0, 0, 0, 0, 0.8, true);
    }
    for _ in 0..3 {
        if macros::is_excute(fighter) {
            macros::FLASH(fighter, 0, 0, 1, 0.196);
        }
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::FLASH(fighter, 0, 0, 1, 0.353);
        }
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::COL_NORMAL(fighter);
        }
        wait(fighter.lua_state_agent, 1.0);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pikachu_dengeki"), true, true);
    }
}

#[acmd_script( agent = "pikachu", scripts = [ "sound_specialn", "sound_specialairn" ], category = ACMD_SOUND, low_priority )]
unsafe fn pikachu_specialn_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_pikachu_002"));
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pikachu_special_n02"));
    }
}

#[acmd_script( agent = "pikachu", scripts = [ "expression_specialn", "expression_specialairn" ], category = ACMD_SOUND, low_priority )]
unsafe fn pikachu_specialn_exp(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE_INTP, SLOPE_STATUS_TOP, 3);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_attackm"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
    }
}

#[acmd_script( agent = "pikachu_dengekidama", script = "game_regular", category = ACMD_GAME, low_priority )]
unsafe fn pikachu_dengekidama_regular(weapon: &mut L2CAgentBase) {
    frame(weapon.lua_state_agent, 6.0);
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 3.0, 60, 30, 0, 35, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -1.9, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
    }
    frame(weapon.lua_state_agent, 10.0);
    if macros::is_excute(weapon) {
        VarModule::on_flag(weapon.battle_object, pikachu_dengekidama::status::flag::SPEED_UP);
    }
}

#[acmd_script( agent = "pikachu", script = "game_speciallwstrike", category = ACMD_GAME, low_priority )]
unsafe fn pikachu_speciallwstrike(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, pikachu::status::flag::SPECIAL_LW_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 277, 30, 0, 75, 4.0, 0.0, -3.0, 5.0, Some(0.0), Some(7.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 15.0, 277, 30, 0, 75, 3.0, 0.0, 2.0, 1.0, Some(0.0), Some(7.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 15.0, 30, 60, 0, 40, 4.0, 0.0, -3.0, 5.0, Some(0.0), Some(7.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 15.0, 30, 60, 0, 40, 3.0, 0.0, 2.0, 1.0, Some(0.0), Some(7.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 11.0, 30, 60, 0, 40, 4.0, 0.0, 17.0, 8.0, Some(0.0), Some(7.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 5, 0, Hash40::new("top"), 11.0, 30, 60, 0, 40, 3.0, 0.0, 12.0, 3.0, Some(0.0), Some(7.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_TAIL);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, pikachu::status::flag::SPECIAL_LW_ENABLE_GRAVITY);
    }
}

#[acmd_script( agent = "pikachu", script = "effect_speciallwstrike", category = ACMD_EFFECT, low_priority )]
unsafe fn pikachu_speciallwstrike_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 6.0, 0, 0, 0, -90, 0.6, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(fighter, 2.0);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 6.0, 0, 0, 0, -90, 0.8, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(fighter, 2.0);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 6.0, 0, 0, 0, -90, 1.0, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(fighter, 2.0);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("pikachu_tail_arc2"), Hash40::new("pikachu_tail_arc2"), Hash40::new("top"), 1, 9, 2, 0, 0, -90, 0.95, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.75);
    }
}

#[acmd_script( agent = "pikachu", script = "sound_speciallwstrike", category = ACMD_SOUND, low_priority )]
unsafe fn pikachu_speciallwstrike_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_swing_02"));
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_swing_02"));
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pikachu_swing_s"));
    }
}

#[acmd_script( agent = "pikachu", script = "expression_speciallwstrike", category = ACMD_EXPRESSION, low_priority )]
unsafe fn pikachu_speciallwstrike_exp(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_nohits"),
            7,
            true,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_nohits"),
            7,
            true,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_nohits"),
            7,
            true,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"),0);
    }
}

pub fn install() {
    install_acmd_scripts!(
        pikachu_specialn, pikachu_specialn_eff, pikachu_specialairn_eff, pikachu_specialn_snd, pikachu_specialn_exp,
        pikachu_dengekidama_regular,
        pikachu_speciallwstrike, pikachu_speciallwstrike_eff, pikachu_speciallwstrike_snd, pikachu_speciallwstrike_exp
    );
}