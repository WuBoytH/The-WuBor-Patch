use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::Hash40,
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};

#[acmd_script( agent = "demon", scripts = [ "game_attacks4", "game_attacks4transform" ], category = ACMD_GAME, low_priority )]
unsafe fn demon_attacks4(fighter: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 10.0);
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_DEMON_STATUS_ATTACK_S4_WORK_INT_CRITICAL_HIT_NO);
    }
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 10.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 11.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 13.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 4.0);
    frame(fighter.lua_state_agent, 14.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    frame(fighter.lua_state_agent, 21.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_DEMON_STATUS_ATTACK_S4_WORK_INT_CRITICAL_HIT_NO);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 23.0, 361, 63, 0, 50, 3.0, 0.0, 8.5, 4.0, Some(0.0), Some(13.5), Some(4.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("handl"), 26.0, 361, 73, 0, 50, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.1);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.75);
    }
    wait(fighter.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_S4_FLAG_HIT) {
        if macros::is_excute(fighter) {
            macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        }
    }
    frame(fighter.lua_state_agent, 51.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 4.0);
    frame(fighter.lua_state_agent, 53.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 57.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 58.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 59.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 8.0);
    frame(fighter.lua_state_agent, 61.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}

#[acmd_script( agent = "demon", scripts = [ "game_attackhi4", "game_attackhi4transform" ], category = ACMD_GAME, low_priority )]
unsafe fn demon_attackhi4(fighter: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 10.0);
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 9.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 12.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 19.0, 90, 62, 0, 65, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 19.0, 90, 62, 0, 65, 4.75, 0.0, 9.0, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 19.0, 90, 62, 0, 65, 3.5, 0.0, 11.0, 3.5, Some(0.0), Some(8.0), Some(2.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_M, false);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 2, false);
        macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 19.0, 90, 62, 0, 65, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("handr"), 19.0, 90, 62, 0, 65, 4.75, 0.0, 9.0, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_M, false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 15.0, 90, 62, 0, 65, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("handr"), 15.0, 90, 62, 0, 65, 4.75, 0.0, 9.0, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_S, false);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    macros::FT_MOTION_RATE(fighter, 0.8);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 51.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 53.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 8.0);
    frame(fighter.lua_state_agent, 54.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 9.0);
    frame(fighter.lua_state_agent, 56.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}

#[acmd_script( agent = "demon", scripts = [ "game_attacklw4", "game_attacklw4transform" ], category = ACMD_GAME, low_priority )]
unsafe fn demon_attacklw4(fighter: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 10.0);
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        damage!(fighter, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    macros::FT_MOTION_RATE(fighter, 1.5);
    frame(fighter.lua_state_agent, 12.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 8.0);
    frame(fighter.lua_state_agent, 13.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 14.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 15.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 17.0, 277, 10, 0, 100, 4.0, 0.0, 8.2, 11.5, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 15.0, 277, 10, 0, 100, 3.0, 0.0, 8.0, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 17.0, 277, 80, 0, 20, 4.0, 0.0, 8.2, 11.5, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 15.0, 277, 80, 0, 20, 3.0, 0.0, 8.0, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 5.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 5.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 5.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 5.0, false);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 1.2);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 2, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 3, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 17.0, 277, 10, 0, 100, 4.0, 0.0, 6.3, 11.5, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 15.0, 277, 10, 0, 100, 3.0, 0.0, 6.3, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 17.0, 277, 80, 0, 20, 4.0, 0.0, 6.3, 11.5, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 15.0, 277, 80, 0, 20, 3.0, 0.0, 6.3, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 5.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 5.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 5.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 5.0, false);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 1.2);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 2, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 3, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 17.0, 277, 10, 0, 100, 4.0, 0.0, 2.5, 11.5, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 15.0, 277, 10, 0, 100, 3.0, 0.0, 3.0, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 17.0, 277, 80, 0, 20, 4.0, -5.0, 2.5, 11.5, Some(5.0), Some(2.5), Some(11.5), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 15.0, 277, 80, 0, 20, 3.0, 0.0, 3.0, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 5.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 5.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 5.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 5.0, false);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 1.2);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 2, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 3, *CAMERA_QUAKE_KIND_L, false);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        damage!(fighter, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 46.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 47.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 48.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 50.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 8.0);
    frame(fighter.lua_state_agent, 52.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 9.0);
    frame(fighter.lua_state_agent, 53.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}

pub fn install() {
    install_acmd_scripts!(
        demon_attacks4,
        demon_attackhi4,
        demon_attacklw4
    );
}