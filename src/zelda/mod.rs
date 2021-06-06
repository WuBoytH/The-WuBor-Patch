use smash::phx::Hash40;
use smash::lua2cpp::L2CAgentBase;
use smash::app::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smashline::*;

// ---------------------------------------------------------
// Zelda is a mess of very powerful and very weak tools. So here’s a better Zelda.
// If she’s really that good, we’ll nerf Nayru’s Love and Forward Smash for the haters,
// but don’t count on it.
// ---------------------------------------------------------

// Down Tilt's angle was changed (68 -> 80).

#[acmd_script( agent = "zelda", script = "game_attacklw3", category = ACMD_GAME, low_priority )]
unsafe fn zelda_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    sv_animcmd::frame(lua_state, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 80, 125, 0, 15, 2.6, 0.0, 2.0, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 80, 125, 0, 15, 2.6, 0.0, 1.7, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 5.5, 80, 125, 0, 15, 2.6, 0.0, 1.5, 7.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    sv_animcmd::wait(lua_state, 7.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

// Dash Attack no longer has sourspots, and the last hit's range is increased.

#[acmd_script( agent = "zelda", script = "game_attackdash", category = ACMD_GAME, low_priority )]
unsafe fn zelda_dashattack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    sv_animcmd::frame(lua_state, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 50, 70, 0, 85, 5.2, 0.0, 9.0, 13.6, Some(0.0), Some(9.0), Some(7.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 50, 50, 0, 80, 5.0, 0.0, 9.0, 16.0, Some(0.0), Some(9.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
    }
    sv_animcmd::wait(lua_state, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

// Down Smash always deals 11 damage, launches at a lower angle, and can be acted out of earlier (38 -> 31).

#[acmd_script( agent = "zelda", script = "game_attacklw4", category = ACMD_GAME, low_priority )]
unsafe fn zelda_dsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    sv_animcmd::frame(lua_state, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(lua_state, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 20, 71, 0, 40, 4.2, 0.0, 3.0, 12.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 20, 71, 0, 40, 3.0, 0.0, 3.0, 11.0, Some(0.0), Some(5.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 20, 71, 0, 40, 4.2, 0.0, 3.0, -11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 20, 71, 0, 40, 3.0, 0.0, 3.0, -9.0, Some(0.0), Some(7.0), Some(-4.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

// Neutral Air now always launches forward, and can be acted out of earlier (51 -> 41).

#[acmd_script( agent = "zelda", script = "game_attackairn", category = ACMD_GAME, low_priority )]
unsafe fn zelda_nair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    sv_animcmd::frame(lua_state, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(lua_state, 6.0);
    for _ in 0..6 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 70, 80, 0, 50, 5.0, 0.0, 10.0, 0.0, None, None, None, 0.6, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 132, 10, 0, 30, 1.5, 0.0, 6.5, 5.0, Some(0.0), Some(6.5), Some(10.0), 0.6, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 230, 10, 0, 15, 1.5, 0.0, 13.0, 5.0, Some(0.0), Some(13.0), Some(10.0), 0.6, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.5, 70, 80, 0, 50, 5.0, 0.0, 13.0, -5.0, None, None, None, 0.6, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 1.5, 152, 10, 0, 30, 1.5, 0.0, 9.5, -3.0, Some(0.0), Some(9.5), Some(-8.0), 0.6, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            macros::ATTACK(fighter, 5, 0, Hash40::new("top"), 1.5, 210, 10, 0, 15, 1.5, 0.0, 16.0, -3.0, Some(0.0), Some(16.0), Some(-8.0), 0.6, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        }
        sv_animcmd::wait(lua_state, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        sv_animcmd::wait(lua_state, 2.0);
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 130, 0, 35, 5.0, 0.0, 13.5, -7.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 361, 130, 0, 35, 5.0, 0.0, 9.0, 8.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 361, 110, 0, 35, 6.0, 0.0, 7.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(fighter, 3, 0, Hash40::new("head"), 5.0, 361, 110, 0, 35, 6.0, 0.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(lua_state, 38.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

// Forward Air and Back Air's sourspots deal more damage (4 -> 5.5) and Base Knockback (5 -> 15).
// Their landing lag was also reduced (15/16 -> 12).

#[acmd_script( agent = "zelda", script = "game_attackairf", category = ACMD_GAME, low_priority )]
unsafe fn zelda_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    sv_animcmd::frame(lua_state, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.625);
    sv_animcmd::frame(lua_state, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(lua_state, 9.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 361, 95, 0, 24, 2.2, -1.0, 4.4, 11.6, Some(1.0), Some(4.4), Some(11.6), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 4.0, 361, 80, 0, 5, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("hip"), 4.0, 361, 80, 0, 5, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 3, 0, Hash40::new("toel"), 4.0, 361, 96, 0, 5, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
        AttackModule::set_optional_hit_effect(fighter.module_accessor, 0, Hash40::new("0x148397b09f"));
    }
    sv_animcmd::wait(lua_state, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
    }
    sv_animcmd::wait(lua_state, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(lua_state, 46.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "zelda", script = "game_attackairb", category = ACMD_GAME, low_priority )]
unsafe fn zelda_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    sv_animcmd::frame(lua_state, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(lua_state, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 361, 96, 0, 30, 2.2, -1.0, 4.4, -11.1, Some(1.0), Some(4.4), Some(-11.1), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 5.5, 361, 80, 0, 15, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("hip"), 5.5, 361, 80, 0, 15, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 3, 0, Hash40::new("toer"), 5.5, 361, 96, 0, 15, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
        AttackModule::set_optional_hit_effect(fighter.module_accessor, 0, Hash40::new("0x1484fa7486"));
    }
    sv_animcmd::wait(lua_state, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
    }
    sv_animcmd::wait(lua_state, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    macros::FT_MOTION_RATE(fighter, 0.91);
    sv_animcmd::frame(lua_state, 48.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

// Down Air's sourspot has increased damage (5/4 -> 5.5),
// deals more Base Knockback (5/0 -> 15), and always deals 70 Knockback Growth.

#[acmd_script( agent = "zelda", script = "game_attackairlw", category = ACMD_GAME, low_priority )]
unsafe fn zelda_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    sv_animcmd::frame(lua_state, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(lua_state, 14.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("footl"), 16.0, 270, 65, 0, 30, 3.2, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(fighter, 1, 0, Hash40::new("footl"), 16.0, 270, 65, 0, 30, 3.0, -1.0, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
    }
    sv_animcmd::frame(lua_state, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneel"), 5.5, 275, 70, 0, 15, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("footl"), 5.5, 275, 70, 0, 15, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::frame(lua_state, 25.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(lua_state, 40.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

// Din's Fire starts up faster (13 -> 8).

#[acmd_script( agent = "zelda", scripts = [ "game_specialsstart", "game_specialairsstart" ], category = ACMD_GAME, low_priority )]
unsafe fn zelda_sspecialstart(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    sv_animcmd::frame(lua_state, 2.0);
    macros::FT_MOTION_RATE(fighter, 0.5);
    sv_animcmd::frame(lua_state, 12.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    sv_animcmd::frame(lua_state, 13.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_S_FLAG_1);
    }
}

// Releasing Special to activate Din's Fire is also faster (14 -> 8).

#[acmd_script( agent = "zelda", scripts = [ "game_specialsend", "game_specialairsend" ], category = ACMD_GAME, low_priority )]
unsafe fn zelda_sspecialend(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    sv_animcmd::frame(lua_state, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.5);
    sv_animcmd::frame(lua_state, 13.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_S_FLAG_2);
    }
    sv_animcmd::frame(lua_state, 15.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
}

pub fn install() {
    smashline::install_acmd_scripts!(
        zelda_dtilt,
        zelda_dashattack,
        zelda_dsmash,
        zelda_nair,
        zelda_fair,
        zelda_bair,
        zelda_dair,
        zelda_sspecialstart,
        zelda_sspecialend
    );
}