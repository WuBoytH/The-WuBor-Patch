use crate::imports::acmd_imports::*;

#[acmd_script( agent = "koopa", script = "game_specialscatch", category = ACMD_GAME, low_priority )]
unsafe fn koopa_specialscatch(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
    }
    macros::FT_MOTION_RATE(fighter, 10.0);
    frame(fighter.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, 8.5, 19.0, None, None, None, *FIGHTER_STATUS_KIND_KOOPA_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(fighter, 1, Hash40::new("top"), 5.0, 0.0, 8.0, 11.0, None, None, None, *FIGHTER_STATUS_KIND_KOOPA_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(fighter, 2, Hash40::new("top"), 2.0, 0.0, 8.5, 19.0, Some(0.0), Some(8.5), Some(16.0), *FIGHTER_STATUS_KIND_KOOPA_DIVED, *COLLISION_SITUATION_MASK_A);
        macros::CATCH(fighter, 3, Hash40::new("top"), 3.0, 0.0, 8.0, 11.0, None, None, None, *FIGHTER_STATUS_KIND_KOOPA_DIVED, *COLLISION_SITUATION_MASK_A);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
}

#[acmd_script( agent = "koopa", script = "game_specialsaircatch", category = ACMD_GAME, low_priority )]
unsafe fn koopa_specialsaircatch(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
    }
    macros::FT_MOTION_RATE(fighter, 10.0);
    frame(fighter.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 4.5, 0.0, 6.0, 21.0, None, None, None, *FIGHTER_STATUS_KIND_KOOPA_DIVED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("top"), 5.5, 0.0, 7.0, 14.0, None, None, None, *FIGHTER_STATUS_KIND_KOOPA_DIVED, *COLLISION_SITUATION_MASK_GA);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
}

#[acmd_script( agent = "koopa", script = "game_specialairhi", category = ACMD_GAME, low_priority )]
unsafe fn koopa_specialairhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        GroundModule::select_cliff_hangdata(fighter.module_accessor, 1);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG4);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 367, 0, 0, 50, 7.0, 0.0, 11.0, 3.5, Some(0.0), Some(11.0), Some(-3.5), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    for _ in 0..10 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 367, 20, 0, 50, 4.3, 0.0, 11.0, 3.5, Some(0.0), Some(11.0), Some(-3.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        }
        wait(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 60, 180, 0, 50, 5.0, 0.0, 11.0, 3.5, Some(0.0), Some(11.0), Some(-3.5), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 51.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG4);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG1);
    }
}

pub fn install() {
    install_acmd_scripts!(
        koopa_specialscatch,
        koopa_specialsaircatch,
        koopa_specialairhi
    );
}