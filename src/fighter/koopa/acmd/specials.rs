use crate::imports::acmd_imports::*;

#[acmd_script( agent = "koopa_breath", script = "game_move", category = ACMD_GAME, low_priority )]
unsafe fn koopa_breath_move(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 50, 60, 0, 72, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
}

#[acmd_script( agent = "koopa_breath", script = "effect_move", category = ACMD_EFFECT, low_priority )]
unsafe fn koopa_breath_move_eff(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire_fly"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("koopa_breath_m_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, true);
        }
        wait(agent.lua_state_agent, 15.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire_fly"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, true);
        }
        wait(agent.lua_state_agent, 15.0);
    }
}

#[acmd_script( agent = "koopa_breath", script = "sound_move", category = ACMD_SOUND, low_priority )]
unsafe fn koopa_breath_move_snd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_koopa_special_n02"));
    }
}

#[acmd_script( agent = "koopa", script = "game_specialscatch", category = ACMD_GAME, low_priority )]
unsafe fn koopa_specialscatch(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
    }
    macros::FT_MOTION_RATE(agent, 10.0);
    frame(agent.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 8.5, 19.0, None, None, None, *FIGHTER_STATUS_KIND_KOOPA_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 5.0, 0.0, 8.0, 11.0, None, None, None, *FIGHTER_STATUS_KIND_KOOPA_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 2, Hash40::new("top"), 2.0, 0.0, 8.5, 19.0, Some(0.0), Some(8.5), Some(16.0), *FIGHTER_STATUS_KIND_KOOPA_DIVED, *COLLISION_SITUATION_MASK_A);
        macros::CATCH(agent, 3, Hash40::new("top"), 3.0, 0.0, 8.0, 11.0, None, None, None, *FIGHTER_STATUS_KIND_KOOPA_DIVED, *COLLISION_SITUATION_MASK_A);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
}

#[acmd_script( agent = "koopa", script = "game_specialsaircatch", category = ACMD_GAME, low_priority )]
unsafe fn koopa_specialsaircatch(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
    }
    macros::FT_MOTION_RATE(agent, 10.0);
    frame(agent.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 4.5, 0.0, 6.0, 21.0, None, None, None, *FIGHTER_STATUS_KIND_KOOPA_DIVED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(agent, 1, Hash40::new("top"), 5.5, 0.0, 7.0, 14.0, None, None, None, *FIGHTER_STATUS_KIND_KOOPA_DIVED, *COLLISION_SITUATION_MASK_GA);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
}

#[acmd_script( agent = "koopa", script = "game_specialairhi", category = ACMD_GAME, low_priority )]
unsafe fn koopa_specialairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        GroundModule::select_cliff_hangdata(agent.module_accessor, 1);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG4);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 367, 0, 0, 50, 7.0, 0.0, 11.0, 3.5, Some(0.0), Some(11.0), Some(-3.5), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        // notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    for _ in 0..10 {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 367, 20, 0, 50, 4.3, 0.0, 11.0, 3.5, Some(0.0), Some(11.0), Some(-3.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        }
        wait(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 60, 180, 0, 50, 5.0, 0.0, 11.0, 3.5, Some(0.0), Some(11.0), Some(-3.5), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 51.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG4);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG1);
    }
}

pub fn install() {
    install_acmd_scripts!(
        koopa_breath_move,
        koopa_breath_move_eff,
        koopa_breath_move_snd,

        koopa_specialscatch,

        koopa_specialsaircatch,

        koopa_specialairhi
    );
}