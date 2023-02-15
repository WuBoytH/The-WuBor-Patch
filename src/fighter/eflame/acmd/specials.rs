use {
    smash::{
        lua2cpp::L2CAgentBase,
        hash40,
        phx::*,
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::lua_const::*
    },
    custom_var::*,
    smash_script::*,
    smashline::*,
    wubor_utils::vars::*
};

#[acmd_script( agent = "eflame", scripts = [ "game_specials", "game_specialairs", "game_specialsflick", "game_specialairsflick" ], category = ACMD_GAME, low_priority )]
unsafe fn eflame_specials(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) {
        if macros::is_excute(fighter) {
            ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 10.0, 10.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(fighter.module_accessor) {
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    if macros::is_excute(fighter) {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_s") as i64, *FIGHTER_EFLAME_INSTANCE_WORK_ID_INT_ESWORD_INHERIT_OPEN_MOTION_KIND);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        // No hitbox
    }
    // frame 16 clears the old hitbox
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
}

#[acmd_script( agent = "eflame_esword", scripts = [ "game_flyl", "game_flyr" ], category = ACMD_GAME, low_priority )]
unsafe fn eflame_esword_fly(weapon: &mut L2CAgentBase) {
    frame(weapon.lua_state_agent, 0.0);
    if macros::is_excute(weapon) {
        MotionModule::set_rate(weapon.module_accessor, 0.75);
        VarModule::on_flag(weapon.battle_object, eflame_esword::status::flag::ENABLE_EARLY_SPIN);
    }
    frame(weapon.lua_state_agent, 2.0);
    if macros::is_excute(weapon) {
        // Exists so it can be reflected
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 2.5, 0.0, -1.5, -3.0, Some(0.0), Some(-1.5), Some(2.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, 0, -1.0, 0, true, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_SWORD);
        macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, 0, -1.0, 0, true, false, true, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_SWORD);
    }
    frame(weapon.lua_state_agent, 20.0);
    if macros::is_excute(weapon) {
        VarModule::off_flag(weapon.battle_object, eflame_esword::status::flag::ENABLE_EARLY_SPIN);
    }
}

#[acmd_script( agent = "eflame_esword", scripts = [ "game_flyflickl", "game_flyflickr" ], category = ACMD_GAME, low_priority )]
unsafe fn eflame_esword_flyflick(weapon: &mut L2CAgentBase) {
    frame(weapon.lua_state_agent, 0.0);
    if macros::is_excute(weapon) {
        MotionModule::set_rate(weapon.module_accessor, 0.75);
        VarModule::on_flag(weapon.battle_object, eflame_esword::status::flag::ENABLE_EARLY_SPIN);
    }
    frame(weapon.lua_state_agent, 2.0);
    if macros::is_excute(weapon) {
        // Exists so it can be reflected
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 2.5, 0.0, -1.5, -3.0, Some(0.0), Some(-1.5), Some(2.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, 0, -1.0, 0, true, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_SWORD);
        macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, 0, -1.0, 0, true, false, true, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_SWORD);
    }
    frame(weapon.lua_state_agent, 20.0);
    if macros::is_excute(weapon) {
        VarModule::off_flag(weapon.battle_object, eflame_esword::status::flag::ENABLE_EARLY_SPIN);
    }
}

#[acmd_script( agent = "eflame_esword", script = "game_rotate", category = ACMD_GAME, low_priority )]
unsafe fn eflame_esword_rotate(weapon: &mut L2CAgentBase) {
    if macros::is_excute(weapon) {
        AttackModule::disable_tip(weapon.module_accessor);
    }
    frame(weapon.lua_state_agent, 0.0);
    if macros::is_excute(weapon) {
        MotionModule::set_rate(weapon.module_accessor, 0.8);
        ControlModule::set_rumble(
            weapon.module_accessor,
            Hash40::new("rbkind_nohitm"),
            7,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
        macros::ATTACK(weapon, 1, 0, Hash40::new("sword1"), 0.8, 0, 60, 20, 10, 2.3, 2.5, -2.0, 0.0, Some(2.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -3, -1.0, 8, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(weapon, 2, 0, Hash40::new("sword1"), 0.8, 90, 70, 20, 10, 2.3, 2.5, -2.0, 0.0, Some(2.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -3, -1.0, 8, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(weapon, 3, 0, Hash40::new("sword1"), 0.8, 190, 60, 20, 5, 1.8, 6.0, -2.0, 0.0, Some(6.0), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -3, -1.0, 8, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(weapon, 4, 0, Hash40::new("sword1"), 0.8, 120, 70, 20, 5, 1.8, 6.0, -2.0, 0.0, Some(6.0), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -3, -1.0, 8, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(weapon, 5, 0, Hash40::new("sword1"), 0.8, 190, 60, 20, 10, 1.8, 9.5, -2.0, 0.0, Some(9.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -3, -1.0, 8, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(weapon, 6, 0, Hash40::new("sword1"), 0.8, 120, 70, 30, 10, 1.8, 9.5, -2.0, 0.0, Some(9.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -3, -1.0, 8, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(weapon, 0, 1, Hash40::new("top"), 1.5, 90, 100, 10, 20, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -3, -1.0, 3, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        for x in 0..7 {
            AttackModule::set_add_reaction_frame_revised(weapon.module_accessor, x, 25.0, false);
            macros::ATK_SET_SHIELD_SETOFF_MUL(weapon, x as u64, 5.0);
        }
    }
    frame(weapon.lua_state_agent, 3.0);
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 1, Hash40::new("top"), 0.0, 367, 100, 100, 50, 2.5, 0.0, 2.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -3, -1.0, 1, false, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(weapon.lua_state_agent, 9.0);
    if macros::is_excute(weapon) {
        ControlModule::set_rumble(
            weapon.module_accessor,
            Hash40::new("rbkind_nohitm"),
            7,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(weapon.lua_state_agent, 17.0);
    if macros::is_excute(weapon) {
        ControlModule::set_rumble(
            weapon.module_accessor,
            Hash40::new("rbkind_nohitm"),
            7,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(weapon.lua_state_agent, 25.0);
    if macros::is_excute(weapon) {
        ControlModule::set_rumble(
            weapon.module_accessor,
            Hash40::new("rbkind_nohitm"),
            7,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(weapon.lua_state_agent, 33.0);
    if macros::is_excute(weapon) {
        ControlModule::set_rumble(
            weapon.module_accessor,
            Hash40::new("rbkind_nohitm"),
            7,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
        MotionModule::set_rate(weapon.module_accessor, 0.5);
    }
    frame(weapon.lua_state_agent, 40.0);
    if macros::is_excute(weapon) {
        AttackModule::clear_all(weapon.module_accessor);
    }
    frame(weapon.lua_state_agent, 41.0);
    if macros::is_excute(weapon) {
        MotionModule::set_rate(weapon.module_accessor, 0.8);
        ControlModule::set_rumble(
            weapon.module_accessor,
            Hash40::new("rbkind_nohitm"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
        macros::ATTACK(weapon, 1, 0, Hash40::new("sword1"), 10.0, 65, 50, 0, 35, 3.5, 5.0, 0.0, 0.0, None, None, None, 1.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(weapon, 2, 0, Hash40::new("sword1"), 10.0, 65, 50, 0, 35, 3.5, 9.0, 0.0, 0.0, None, None, None, 1.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(weapon, 3, 0, Hash40::new("sword1"), 10.0, 65, 50, 0, 35, 3.5, 11.0, 0.0, 0.0, None, None, None, 1.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATK_SET_SHIELD_SETOFF_MUL(weapon, 1, 2.0);
        macros::ATK_SET_SHIELD_SETOFF_MUL(weapon, 2, 2.0);
        macros::ATK_SET_SHIELD_SETOFF_MUL(weapon, 3, 2.0);
        WorkModule::on_flag(weapon.module_accessor, *WEAPON_EFLAME_ESWORD_STATUS_SPECIAL_S_FLAG_FINISH);
    }
    frame(weapon.lua_state_agent, 55.0);
    if macros::is_excute(weapon) {
        AttackModule::clear_all(weapon.module_accessor);
    }
    frame(weapon.lua_state_agent, 55.0);
    if macros::is_excute(weapon) {
        MotionModule::set_rate(weapon.module_accessor, 0.5);
    }
}

#[acmd_script( agent = "eflame_esword", script = "effect_rotate", category = ACMD_EFFECT, low_priority )]
unsafe fn eflame_esword_rotate_eff(weapon: &mut L2CAgentBase) {
    if macros::is_excute(weapon) {
        macros::EFFECT_FOLLOW(weapon, Hash40::new("eflame_blazeend_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(weapon, 0.8);
        macros::EFFECT_FOLLOW_NO_STOP(weapon, Hash40::new("eflame_blazeend_trail"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(weapon, 0.8);
    }
    frame(weapon.lua_state_agent, 34.0);
    if macros::is_excute(weapon) {
        macros::EFFECT_OFF_KIND(weapon, Hash40::new("eflame_blazeend_trail"), false, true);
    }
    frame(weapon.lua_state_agent, 42.0);
    if macros::is_excute(weapon) {
        macros::EFFECT_FOLLOW(weapon, Hash40::new("eflame_blazeend_trail_last_back"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        macros::EFFECT_FOLLOW(weapon, Hash40::new("eflame_blazeend_trail_last"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        macros::EFFECT_FOLLOW_NO_STOP(weapon, Hash40::new("eflame_blazeend_trail_last_spin"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
    }
    frame(weapon.lua_state_agent, 48.0);
    if macros::is_excute(weapon) {
        macros::EFFECT_OFF_KIND(weapon, Hash40::new("eflame_blazeend_trail_last_back"), false, true);
        macros::EFFECT_OFF_KIND(weapon, Hash40::new("eflame_blazeend_trail_last_spin"), false, true);
    }
    frame(weapon.lua_state_agent, 64.0);
    if macros::is_excute(weapon) {
        macros::EFFECT_OFF_KIND(weapon, Hash40::new("eflame_blazeend_sword"), false, true);
        macros::EFFECT_FOLLOW(weapon, Hash40::new("eflame_sword_close"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        macros::EFFECT_DETACH_KIND(weapon, Hash40::new("eflame_blazeend_trail_last"), -1);
    }
}

#[acmd_script( agent = "eflame_esword", scripts = [ "game_reflectedl", "game_reflectedr" ], category = ACMD_GAME, low_priority )]
unsafe fn eflame_esword_reflected(weapon: &mut L2CAgentBase) {
    frame(weapon.lua_state_agent, 0.0);
    if macros::is_excute(weapon) {
        MotionModule::set_rate(weapon.module_accessor, 1.5);
    }
    frame(weapon.lua_state_agent, 3.0);
    if macros::is_excute(weapon) {
        // Exists so it can be reflected
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 2.5, 0.0, -1.5, -3.0, Some(0.0), Some(-1.5), Some(2.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, 0, -1.0, 0, true, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_SWORD);
        macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, 0, -1.0, 0, true, false, true, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_SWORD);
    }
}

#[acmd_script( agent = "eflame", scripts = [ "game_speciallwattack", "game_specialairlwattack" ], category = ACMD_GAME, low_priority )]
unsafe fn eflame_speciallwattack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) {
        if macros::is_excute(fighter) {
            ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(fighter.module_accessor) {
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.5, 60, 78, 10, 40, 3.5, 0.0, 4.0, 17.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.5, 50, 78, 10, 40, 4.5, 0.0, 8.0, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.5, 40, 78, 10, 40, 5.5, 0.0, 12.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.5);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.5);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.5);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 36.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 40, 78, 0, 60, 3.5, 0.0, 4.0, 20.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 40, 78, 0, 60, 4.5, 0.0, 7.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 12.0, 40, 78, 0, 60, 5.5, 0.0, 10.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    macros::FT_MOTION_RATE(fighter, 0.6);
    frame(fighter.lua_state_agent, 60.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, element::status::flag::SPECIAL_LW_OUT_ATTACK_FALL);
    }
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) {
        if macros::is_excute(fighter) {
            ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 1.0, 1.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(fighter.module_accessor) {
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
}

#[acmd_script( agent = "eflame", scripts = [ "effect_speciallwattack", "effect_specialairlwattack" ], category = ACMD_EFFECT, low_priority )]
unsafe fn eflame_speciallwattack_eff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("eflame_change_end"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1.3, true);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("eflame_sword_core_start"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("eflame_sword_open"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("eflame_sword_open"), true, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("eflame_sword_beam_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        macros::LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, -3);
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_eflame_sword1"), Hash40::new("tex_eflame_sword2"), 5, Hash40::new("sword1"), 0.3, 0.0, 0.0, Hash40::new("sword1"), 18.5, 0.0, -0.25, true, Hash40::new("null"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("eflame_sword_fire"), Hash40::new("sword1"), -0.5, 0, 0, 0, 0, 0, 0.9, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("eflame_sword_fire2"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("eflame_sword_firetrail"), Hash40::new("sword1"), 1.5, 0, 0, -5, 0, 0, 0.5, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("eflame_sword_firetrail"), true, true);
        macros::AFTER_IMAGE_OFF(fighter, 5);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("eflame_sword_fire2"), false, true);
        macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("eflame_sword_firetrail_end"), Hash40::new("sword1"), 2, 0, 0, 0, 0, 0, 0.4, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 4);
    }
    frame(fighter.lua_state_agent, 33.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("eflame_sword_open"), true, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("eflame_sword_beam_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        macros::LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, -3);
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_eflame_sword1"), Hash40::new("tex_eflame_sword2"), 5, Hash40::new("sword1"), 0.3, 0.0, 0.0, Hash40::new("sword1"), 18.5, 0.0, -0.25, true, Hash40::new("null"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("eflame_sword_fire"), Hash40::new("sword1"), -0.5, 0, 0, 0, 0, 0, 0.9, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("eflame_sword_fire2"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("eflame_sword_firetrail"), Hash40::new("sword1"), 1.5, 0, 0, -5, 0, 0, 0.5, true);
    }
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("eflame_sword_firetrail"), true, true);
        macros::AFTER_IMAGE_OFF(fighter, 7);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("eflame_sword_fire2"), false, true);
        macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("eflame_sword_firetrail_end"), Hash40::new("sword1"), 2, 0, 0, 0, 0, 0, 0.4, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 4);
    }
    frame(fighter.lua_state_agent, 60.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("eflame_sword_fire"), false, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("eflame_sword_beam_m"), true, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("eflame_sword_close_s"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 62.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("eflame_sword_core_start"), false, true);
    }
}

#[acmd_script( agent = "eflame", script = "sound_speciallwattack", category = ACMD_SOUND, low_priority )]
unsafe fn eflame_speciallwattack_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_eflame_attack04"));
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_eflame_swing_m01"));
    }
    frame(fighter.lua_state_agent, 33.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_eflame_rnd_special_l02"));
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_eflame_swing_l01"));
    }
    frame(fighter.lua_state_agent, 107.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_eflame_step_right_m"));
    }
}

#[acmd_script( agent = "eflame", script = "sound_specialairlwattack", category = ACMD_SOUND, low_priority )]
unsafe fn eflame_specialairlwattack_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_eflame_attack04"));
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_eflame_swing_m01"));
    }
    frame(fighter.lua_state_agent, 33.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_eflame_rnd_special_l02"));
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_eflame_swing_l01"));
    }
}

#[acmd_script( agent = "eflame", scripts = [ "expression_speciallwattack", "expression_specialairlwattack" ], category = ACMD_EXPRESSION, low_priority )]
unsafe fn eflame_speciallwattack_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_79_changef"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_79_flameslashl"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
    }
    frame(fighter.lua_state_agent, 33.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_79_flamesmash"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(fighter.lua_state_agent, 36.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_explosion"), 0);
    }
}

pub fn install() {
    install_acmd_scripts!(
        eflame_specials,
        eflame_esword_fly,
        eflame_esword_flyflick,
        eflame_esword_rotate, eflame_esword_rotate_eff,
        eflame_esword_reflected,
        eflame_speciallwattack, eflame_speciallwattack_eff, eflame_speciallwattack_snd, eflame_specialairlwattack_snd, eflame_speciallwattack_exp
    );
}
