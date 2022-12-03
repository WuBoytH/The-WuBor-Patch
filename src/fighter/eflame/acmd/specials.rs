use {
    smash::{
        lua2cpp::L2CAgentBase,
        hash40,
        phx::*,
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
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
        MotionModule::set_rate(weapon.module_accessor, 0.5);
    }
    frame(weapon.lua_state_agent, 2.0);
    if macros::is_excute(weapon) {
        // Exists so it can be reflected
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 2.5, 0.0, -1.5, -3.0, Some(0.0), Some(-1.5), Some(2.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, 0, -1.0, 0, true, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_SWORD);
        macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, 0, -1.0, 0, true, false, true, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_SWORD);
    }
}

#[acmd_script( agent = "eflame_esword", scripts = [ "game_flyflickl", "game_flyflickr" ], category = ACMD_GAME, low_priority )]
unsafe fn eflame_esword_flyflick(weapon: &mut L2CAgentBase) {
    frame(weapon.lua_state_agent, 0.0);
    if macros::is_excute(weapon) {
        MotionModule::set_rate(weapon.module_accessor, 0.5);
    }
    frame(weapon.lua_state_agent, 2.0);
    if macros::is_excute(weapon) {
        // Exists so it can be reflected
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 2.5, 0.0, -1.5, -3.0, Some(0.0), Some(-1.5), Some(2.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, 0, -1.0, 0, true, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_SWORD);
        macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, 0, -1.0, 0, true, false, true, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_SWORD);
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
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 0.8, 90, 100, 10, 20, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -3, -1.0, 3, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        for x in 0..7 {
            AttackModule::set_add_reaction_frame_revised(weapon.module_accessor, x, 25.0, false);
        }
    }
    frame(weapon.lua_state_agent, 3.0);
    if macros::is_excute(weapon) {
        for x in 0..7 {
            macros::ATK_SET_SHIELD_SETOFF_MUL(weapon, x, 0.5);
        }
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
        macros::ATTACK(weapon, 1, 0, Hash40::new("sword1"), 10.0, 65, 100, 0, 35, 3.5, 5.0, 0.0, 0.0, None, None, None, 1.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(weapon, 2, 0, Hash40::new("sword1"), 10.0, 65, 100, 0, 35, 3.5, 9.0, 0.0, 0.0, None, None, None, 1.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(weapon, 3, 0, Hash40::new("sword1"), 10.0, 65, 100, 0, 35, 3.5, 11.0, 0.0, 0.0, None, None, None, 1.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
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

pub fn install() {
    install_acmd_scripts!(
        eflame_specials,
        eflame_esword_fly,
        eflame_esword_flyflick,
        eflame_esword_rotate, eflame_esword_rotate_eff,
        eflame_esword_reflected
    );
}
