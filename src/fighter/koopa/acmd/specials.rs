use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::Hash40,
        app::sv_animcmd::*,
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};

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

pub fn install() {
    install_acmd_scripts!(
        koopa_specialscatch,
        koopa_specialsaircatch
    );
}