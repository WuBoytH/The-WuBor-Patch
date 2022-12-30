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

#[acmd_script( agent = "simon", script = "game_attackairf", category = ACMD_GAME )]
unsafe fn simon_attackairf(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.8);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 10.0);
    macros::FT_MOTION_RATE(fighter, 1);
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
        macros::SEARCH(fighter, 0, 0, Hash40::new("top"), 2.3, 0.0, 10.5, 7.0, Some(0.0), Some(10.5), Some(43.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_IG, *COLLISION_PART_MASK_ALL, false);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
        fighter.clear_lua_stack();
        let object = sv_system::battle_object(fighter.lua_state_agent) as *mut BattleObject;
        if !object.is_null() {
            FighterSpecializer_Simon::set_whip_reflect_attack_off_id(
                object as *mut Fighter,
                0,
                1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1
            );
        }
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 361, 55, 0, 80, 2.3, 0.0, 10.5, 43.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 361, 55, 0, 80, 2.3, 0.0, 10.5, 10.0, Some(0.0), Some(10.5), Some(43.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 361, 55, 0, 80, 5.5, 0.0, 10.5, 8.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        search!(fighter, MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "simon", script = "game_attackairfhi", category = ACMD_GAME )]
unsafe fn simon_attackairfhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.8);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 10.0);
    macros::FT_MOTION_RATE(fighter, 1);
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
        macros::SEARCH(fighter, 0, 0, Hash40::new("top"), 2.3, 0.0, 16.0, 9.0, Some(0.0), Some(30.0), Some(35.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_IG, *COLLISION_PART_MASK_ALL, false);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
        fighter.clear_lua_stack();
        let object = sv_system::battle_object(fighter.lua_state_agent) as *mut BattleObject;
        if !object.is_null() {
            FighterSpecializer_Simon::set_whip_reflect_attack_off_id(
                object as *mut Fighter,
                0,
                1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1
            );
        }
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 361, 55, 0, 80, 2.3, 0.0, 30.0, 35.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 361, 55, 0, 80, 2.3, 0.0, 16.0, 9.0, Some(0.0), Some(30.0), Some(35.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 361, 55, 0, 80, 5.5, 0.0, 16.0, 7.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        search!(fighter, MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "simon", script = "game_attackairflw", category = ACMD_GAME )]
unsafe fn simon_attackairflw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.8);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 10.0);
    macros::FT_MOTION_RATE(fighter, 1);
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
        macros::SEARCH(fighter, 0, 0, Hash40::new("top"), 2.3, 0.0, 9.0, 11.0, Some(0.0), Some(-2.0), Some(41.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_IG, *COLLISION_PART_MASK_ALL, false);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
        fighter.clear_lua_stack();
        let object = sv_system::battle_object(fighter.lua_state_agent) as *mut BattleObject;
        if !object.is_null() {
            FighterSpecializer_Simon::set_whip_reflect_attack_off_id(
                object as *mut Fighter,
                0,
                1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1
            );
        }
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 361, 55, 0, 80, 2.3, 0.0, -2.0, 41.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 361, 55, 0, 80, 2.3, 0.0, 9.0, 11.0, Some(0.0), Some(-2.0), Some(41.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 361, 55, 0, 80, 5.5, 0.0, 9.0, 9.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        search!(fighter, MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "simon", scripts = [ "sound_attackairf", "sound_attackairfhi", "sound_attackairflw" ], category = ACMD_SOUND )]
unsafe fn simon_attackairf_snd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_simon_whip_holding"));
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_simon_attackair_h01"));
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_simon_attackair_h01"));
        macros::PLAY_SE(fighter, Hash40::new("se_simon_attackair_h02"));
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_simon_rnd_attack"));
    }
}

#[acmd_script( agent = "simon", script = "game_attackairhi", category = ACMD_GAME )]
unsafe fn simon_attackairhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.5);
    frame(fighter.lua_state_agent, 8.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
        macros::SEARCH(fighter, 0, 0, Hash40::new("top"), 2.5, 0.0, 15.0, 2.5, Some(0.0), Some(53.0), Some(2.5), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_IG, *COLLISION_PART_MASK_ALL, false);
        macros::SEARCH(fighter, 1, 0, Hash40::new("top"), 1.0, 0.0, 15.0, 2.5, Some(0.0), Some(48.0), Some(2.5), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
        fighter.clear_lua_stack();
        let object = sv_system::battle_object(fighter.lua_state_agent) as *mut BattleObject;
        if !object.is_null() {
            FighterSpecializer_Simon::set_whip_reflect_attack_off_id(
                object as *mut Fighter,
                0,
                1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1
            );
        }
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 90, 75, 0, 60, 2.5, 0.0, 53.0, 2.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 90, 75, 0, 60, 2.5, 0.0, 17.0, 2.5, Some(0.0), Some(53.0), Some(2.5), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 90, 75, 0, 60, 5.5, 0.0, 19.0, 2.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        search!(fighter, MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "simon", script = "game_attackairlw", category = ACMD_GAME )]
unsafe fn simon_attackairlw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::SEARCH(fighter, 0, 0, Hash40::new("top"), 2.5, 0.0, 0.0, 4.5, Some(0.0), Some(-27.0), Some(4.5), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_IG, *COLLISION_PART_MASK_ALL, false);
        macros::SEARCH(fighter, 1, 0, Hash40::new("top"), 1.0, 0.0, 0.0, 4.5, Some(0.0), Some(-24.0), Some(4.5), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
        fighter.clear_lua_stack();
        let object = sv_system::battle_object(fighter.lua_state_agent) as *mut BattleObject;
        if !object.is_null() {
            FighterSpecializer_Simon::set_whip_reflect_attack_off_id(
                object as *mut Fighter,
                0,
                1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1
            );
        }
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 270, 98, 0, 14, 2.5, 0.0, -27.0, 4.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 270, 64, 0, 16, 2.5, 0.0, 8.0, 4.5, Some(0.0), Some(-27.0), Some(4.5), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.5, 270, 32, 0, 18, 5.5, 0.0, 6.0, 4.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        search!(fighter, MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "simon", script = "effect_attackairlw", category = ACMD_EFFECT )]
unsafe fn simon_attackairlw_eff(_fighter: &mut L2CAgentBase) {
    
}

#[acmd_script( agent = "simon", script = "sound_attackairlw", category = ACMD_SOUND )]
unsafe fn simon_attackairlw_snd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_simon_whip_holding"));
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_simon_attackair_h01"));
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_simon_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_simon_attackair_h01"));
        macros::PLAY_SE(fighter, Hash40::new("se_simon_attackair_h02"));
    }
}

#[acmd_script( agent = "simon", script = "expression_attackairlw", category = ACMD_EXPRESSION )]
unsafe fn simon_attackairlw_exp(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_nohitm"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}

#[acmd_script( agent = "simon_whip", script = "game_attackairlw", category = ACMD_GAME )]
unsafe fn simon_whip_attackairlw(weapon: &mut L2CAgentBase) {
    frame(weapon.lua_state_agent, 1.0);
    if macros::is_excute(weapon) {
        PhysicsModule::set_2nd_status(weapon.module_accessor, *PH2NDARY_CRAW_NONE);
        weapon.clear_lua_stack();
        let object = sv_system::battle_object(weapon.lua_state_agent) as *mut BattleObject;
        if !object.is_null() {
            WeaponSpecializer_SimonWhip::reset_node_fix_flag_list(
                object as *mut smash::app::Weapon
            );
        }
    }
    frame(weapon.lua_state_agent, 11.0);
    if macros::is_excute(weapon) {
        PhysicsModule::set_2nd_status(weapon.module_accessor, *PH2NDARY_CRAW_COLLIDE);
    }
}

#[acmd_script( agent = "simon_whip", script = "effect_attackairlw", category = ACMD_EFFECT )]
unsafe fn simon_whip_attackairlw_eff(weapon: &mut L2CAgentBase) {
    frame(weapon.lua_state_agent, 13.0);
    if macros::is_excute(weapon) {
        weapon.clear_lua_stack();
        let object = sv_system::battle_object(weapon.lua_state_agent) as *mut BattleObject;
        if !object.is_null() {
            WeaponSpecializer_SimonWhip::set_chain_2_visibility(
                object as *mut smash::app::Weapon,
                true
            );
        }
        macros::EFFECT_FOLLOW(weapon, Hash40::new("simon_whip_straight"), Hash40::new("hookshot3"), -6, 0, 0, 180, 50, -90, 1.1, true);
        macros::EFFECT_FOLLOW_ALPHA(weapon, Hash40::new("simon_whip_light"), Hash40::new("hookshot10"), 0, 0, 0, 0, 0, 0, 1, true, 0.65);
        macros::EFFECT_FOLLOW(weapon, Hash40::new("simon_whip_light_s"), Hash40::new("hookshot3"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(weapon, Hash40::new("simon_whip_flash_top"), Hash40::new("hookshot27"), 1, 0, 0, 0, 0, 0, 1, true);
    }
    frame(weapon.lua_state_agent, 18.0);
    if macros::is_excute(weapon) {
        weapon.clear_lua_stack();
        let object = sv_system::battle_object(weapon.lua_state_agent) as *mut BattleObject;
        if !object.is_null() {
            WeaponSpecializer_SimonWhip::set_chain_2_visibility(
                object as *mut smash::app::Weapon,
                false
            );
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        simon_attackairf,
        simon_attackairfhi,
        simon_attackairflw,
        simon_attackairf_snd,
        simon_attackairhi,
        simon_attackairlw, simon_attackairlw_eff, simon_attackairlw_snd, simon_attackairlw_exp,
        simon_whip_attackairlw, simon_whip_attackairlw_eff
    );
}