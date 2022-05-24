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

#[acmd_script( agent = "demon", scripts = [ "game_attackragedrive", "game_attackairragedrive" ], category = ACMD_GAME, low_priority )]
unsafe fn demon_attackragedrive(fighter: &mut L2CAgentBase) {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ATTACK_RAGE_CAPTURE) {
        fighter.clear_lua_stack();
        let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        FighterSpecializer_Demon::set_devil(&mut *boma, true, 10.0);
        if macros::is_excute(fighter) {
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
        }
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            damage!(fighter, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        }
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            damage!(fighter, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0);
            macros::CATCH(fighter, 0, Hash40::new("top"), 5.0, 0.0, 9.5, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(fighter, 1, Hash40::new("top"), 4.5, 0.0, 8.5, 12.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(fighter, 2, Hash40::new("top"), 4.0, 0.0, 9.5, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            macros::CATCH(fighter, 3, Hash40::new("top"), 3.5, 0.0, 8.5, 12.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(fighter.module_accessor, 0, true);
            GrabModule::set_constraint(fighter.module_accessor, 1, true);
            GrabModule::set_constraint(fighter.module_accessor, 2, true);
            GrabModule::set_constraint(fighter.module_accessor, 3, true);
        }
        frame(fighter.lua_state_agent, 14.0);
        if macros::is_excute(fighter) {
            macros::CATCH(fighter, 0, Hash40::new("top"), 5.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(fighter, 1, Hash40::new("top"), 4.5, 0.0, 19.0, 9.0, Some(0.0), Some(8.5), Some(12.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(fighter, 2, Hash40::new("top"), 4.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            macros::CATCH(fighter, 3, Hash40::new("top"), 3.5, 0.0, 21.0, 8.5, Some(0.0), Some(8.5), Some(12.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(fighter.module_accessor, 0, true);
            GrabModule::set_constraint(fighter.module_accessor, 1, true);
            GrabModule::set_constraint(fighter.module_accessor, 2, true);
            GrabModule::set_constraint(fighter.module_accessor, 3, true);
        }
        frame(fighter.lua_state_agent, 15.0);
        if macros::is_excute(fighter) {
            grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        }
    }
    else {
        fighter.clear_lua_stack();
        let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        FighterSpecializer_Demon::set_devil(&mut *boma, true, 10.0);
        macros::FT_MOTION_RATE(fighter, 0.7);
        if macros::is_excute(fighter) {
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
        }
        frame(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            damage!(fighter, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        }
        frame(fighter.lua_state_agent, 10.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        frame(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            damage!(fighter, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0);
            macros::CATCH(fighter, 0, Hash40::new("top"), 5.0, 0.0, 8.0, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(fighter, 1, Hash40::new("top"), 4.5, 0.0, 6.0, 8.5, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(fighter, 2, Hash40::new("top"), 4.0, 0.0, 8.0, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            macros::CATCH(fighter, 3, Hash40::new("top"), 3.5, 0.0, 6.0, 8.5, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(fighter.module_accessor, 0, true);
            GrabModule::set_constraint(fighter.module_accessor, 1, true);
            GrabModule::set_constraint(fighter.module_accessor, 2, true);
            GrabModule::set_constraint(fighter.module_accessor, 3, true);
        }
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            damage!(fighter, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0);
            macros::CATCH(fighter, 0, Hash40::new("top"), 5.0, 0.0, 9.5, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(fighter, 1, Hash40::new("top"), 4.5, 0.0, 8.5, 12.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(fighter, 2, Hash40::new("top"), 4.0, 0.0, 9.5, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            macros::CATCH(fighter, 3, Hash40::new("top"), 3.5, 0.0, 8.5, 12.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(fighter.module_accessor, 0, true);
            GrabModule::set_constraint(fighter.module_accessor, 1, true);
            GrabModule::set_constraint(fighter.module_accessor, 2, true);
            GrabModule::set_constraint(fighter.module_accessor, 3, true);
        }
        frame(fighter.lua_state_agent, 14.0);
        if macros::is_excute(fighter) {
            macros::CATCH(fighter, 0, Hash40::new("top"), 5.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(fighter, 1, Hash40::new("top"), 4.5, 0.0, 19.0, 9.0, Some(0.0), Some(8.5), Some(12.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(fighter, 2, Hash40::new("top"), 4.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            macros::CATCH(fighter, 3, Hash40::new("top"), 3.5, 0.0, 21.0, 8.5, Some(0.0), Some(8.5), Some(12.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(fighter.module_accessor, 0, true);
            GrabModule::set_constraint(fighter.module_accessor, 1, true);
            GrabModule::set_constraint(fighter.module_accessor, 2, true);
            GrabModule::set_constraint(fighter.module_accessor, 3, true);
        }
        frame(fighter.lua_state_agent, 15.0);
        if macros::is_excute(fighter) {
            macros::CATCH(fighter, 0, Hash40::new("top"), 5.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(fighter, 1, Hash40::new("top"), 4.5, 0.0, 22.0, 7.0, Some(0.0), Some(8.5), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(fighter, 2, Hash40::new("top"), 4.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            macros::CATCH(fighter, 3, Hash40::new("top"), 3.5, 0.0, 24.0, 6.5, Some(0.0), Some(8.5), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(fighter.module_accessor, 0, true);
            GrabModule::set_constraint(fighter.module_accessor, 1, true);
            GrabModule::set_constraint(fighter.module_accessor, 2, true);
            GrabModule::set_constraint(fighter.module_accessor, 3, true);
        }
        frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        }
    }
    frame(fighter.lua_state_agent, 38.0);
    macros::FT_MOTION_RATE(fighter, 1.3);
    frame(fighter.lua_state_agent, 52.0);
    fighter.clear_lua_stack();
    let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    FighterSpecializer_Demon::set_devil(&mut *boma, false, 0.0);
}

#[acmd_script( agent = "demon", script = "0x15cb9d3406", category = ACMD_GAME, low_priority )]
unsafe fn demon_15cb9d3406(fighter: &mut L2CAgentBase) {
    fighter.clear_lua_stack();
    let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    FighterSpecializer_Demon::set_devil(&mut *boma, true, 10.0);
    macros::FT_MOTION_RATE(fighter, 0.7);
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        damage!(fighter, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(fighter.lua_state_agent, 10.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        damage!(fighter, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        macros::CATCH(fighter, 0, Hash40::new("top"), 5.0, 0.0, 8.0, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(fighter, 1, Hash40::new("top"), 4.5, 0.0, 6.0, 8.5, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(fighter, 2, Hash40::new("top"), 4.0, 0.0, 8.0, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        macros::CATCH(fighter, 3, Hash40::new("top"), 3.5, 0.0, 6.0, 8.5, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        GrabModule::set_constraint(fighter.module_accessor, 0, true);
        GrabModule::set_constraint(fighter.module_accessor, 1, true);
        GrabModule::set_constraint(fighter.module_accessor, 2, true);
        GrabModule::set_constraint(fighter.module_accessor, 3, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        damage!(fighter, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        macros::CATCH(fighter, 0, Hash40::new("top"), 5.0, 0.0, 9.5, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(fighter, 1, Hash40::new("top"), 4.5, 0.0, 8.5, 12.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(fighter, 2, Hash40::new("top"), 4.0, 0.0, 9.5, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        macros::CATCH(fighter, 3, Hash40::new("top"), 3.5, 0.0, 8.5, 12.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        GrabModule::set_constraint(fighter.module_accessor, 0, true);
        GrabModule::set_constraint(fighter.module_accessor, 1, true);
        GrabModule::set_constraint(fighter.module_accessor, 2, true);
        GrabModule::set_constraint(fighter.module_accessor, 3, true);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 5.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(fighter, 1, Hash40::new("top"), 4.5, 0.0, 19.0, 9.0, Some(0.0), Some(8.5), Some(12.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(fighter, 2, Hash40::new("top"), 4.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        macros::CATCH(fighter, 3, Hash40::new("top"), 3.5, 0.0, 21.0, 8.5, Some(0.0), Some(8.5), Some(12.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        GrabModule::set_constraint(fighter.module_accessor, 0, true);
        GrabModule::set_constraint(fighter.module_accessor, 1, true);
        GrabModule::set_constraint(fighter.module_accessor, 2, true);
        GrabModule::set_constraint(fighter.module_accessor, 3, true);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 5.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(fighter, 1, Hash40::new("top"), 4.5, 0.0, 22.0, 7.0, Some(0.0), Some(8.5), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(fighter, 2, Hash40::new("top"), 4.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        macros::CATCH(fighter, 3, Hash40::new("top"), 3.5, 0.0, 24.0, 6.5, Some(0.0), Some(8.5), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        GrabModule::set_constraint(fighter.module_accessor, 0, true);
        GrabModule::set_constraint(fighter.module_accessor, 1, true);
        GrabModule::set_constraint(fighter.module_accessor, 2, true);
        GrabModule::set_constraint(fighter.module_accessor, 3, true);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
    frame(fighter.lua_state_agent, 38.0);
    macros::FT_MOTION_RATE(fighter, 1.3);
    frame(fighter.lua_state_agent, 52.0);
    fighter.clear_lua_stack();
    let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    FighterSpecializer_Demon::set_devil(&mut *boma, false, 0.0);
}

#[acmd_script( agent = "demon", script = "0x15b52c48bb", category = ACMD_GAME, low_priority )]
unsafe fn demon_15b52c48bb(fighter: &mut L2CAgentBase) {
    fighter.clear_lua_stack();
    let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    FighterSpecializer_Demon::set_devil(&mut *boma, true, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        damage!(fighter, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        damage!(fighter, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        macros::CATCH(fighter, 0, Hash40::new("top"), 5.0, 0.0, 9.5, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(fighter, 1, Hash40::new("top"), 4.5, 0.0, 8.5, 12.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(fighter, 2, Hash40::new("top"), 4.0, 0.0, 9.5, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        macros::CATCH(fighter, 3, Hash40::new("top"), 3.5, 0.0, 8.5, 12.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        GrabModule::set_constraint(fighter.module_accessor, 0, true);
        GrabModule::set_constraint(fighter.module_accessor, 1, true);
        GrabModule::set_constraint(fighter.module_accessor, 2, true);
        GrabModule::set_constraint(fighter.module_accessor, 3, true);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 5.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(fighter, 1, Hash40::new("top"), 4.5, 0.0, 19.0, 9.0, Some(0.0), Some(8.5), Some(12.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(fighter, 2, Hash40::new("top"), 4.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        macros::CATCH(fighter, 3, Hash40::new("top"), 3.5, 0.0, 21.0, 8.5, Some(0.0), Some(8.5), Some(12.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        GrabModule::set_constraint(fighter.module_accessor, 0, true);
        GrabModule::set_constraint(fighter.module_accessor, 1, true);
        GrabModule::set_constraint(fighter.module_accessor, 2, true);
        GrabModule::set_constraint(fighter.module_accessor, 3, true);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
    frame(fighter.lua_state_agent, 38.0);
    macros::FT_MOTION_RATE(fighter, 1.3);
    frame(fighter.lua_state_agent, 52.0);
    fighter.clear_lua_stack();
    let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    FighterSpecializer_Demon::set_devil(&mut *boma, false, 0.0);
}

#[acmd_script( agent = "demon", script = "game_speciallw", category = ACMD_GAME, low_priority )]
unsafe fn demon_speciallw(fighter: &mut L2CAgentBase) {
    fighter.clear_lua_stack();
    let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    FighterSpecializer_Demon::set_devil(&mut *boma, true, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        damage!(fighter, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 10);
    }
    frame(fighter.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(fighter, 0.6);
    frame(fighter.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        damage!(fighter, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        macros::CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, 8.75, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(fighter, 1, Hash40::new("top"), 3.5, 0.0, 7.75, 12.0, Some(0.0), Some(7.75), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(fighter, 2, Hash40::new("top"), 3.0, 0.0, 8.75, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        macros::CATCH(fighter, 3, Hash40::new("top"), 2.0, 0.0, 7.75, 12.5, Some(0.0), Some(7.75), Some(9.5), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        GrabModule::set_constraint(fighter.module_accessor, 0, true);
        GrabModule::set_constraint(fighter.module_accessor, 1, true);
        GrabModule::set_constraint(fighter.module_accessor, 2, true);
        GrabModule::set_constraint(fighter.module_accessor, 3, true);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, 14.0, 5.0, Some(0.0), Some(8.75), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(fighter, 1, Hash40::new("top"), 3.5, 0.0, 19.0, 9.0, Some(0.0), Some(7.75), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(fighter, 2, Hash40::new("top"), 3.0, 0.0, 14.0, 5.0, Some(0.0), Some(8.75), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        macros::CATCH(fighter, 3, Hash40::new("top"), 2.0, 0.0, 19.0, 9.0, Some(0.0), Some(7.75), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        GrabModule::set_constraint(fighter.module_accessor, 0, true);
        GrabModule::set_constraint(fighter.module_accessor, 1, true);
        GrabModule::set_constraint(fighter.module_accessor, 2, true);
        GrabModule::set_constraint(fighter.module_accessor, 3, true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::SET_SPEED_EX(fighter, 0.1, 1.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    macros::FT_MOTION_RATE(fighter, 0.9);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 70, 60, 0, 80, 3.0, 0.0, 19.0, 4.5, Some(0.0), Some(23.0), Some(4.5), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 60, 60, 0, 80, 3.0, 0.0, 12.0, 6.0, Some(0.0), Some(16.0), Some(6.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 64, 50, 0, 75, 3.0, 0.0, 3.0, 4.0, Some(0.0), Some(3.0), Some(5.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::set_down_only(fighter.module_accessor, 2, true);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 2, false);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 70, 60, 0, 80, 3.0, 0.0, 19.5, 4.5, Some(0.0), Some(23.5), Some(4.5), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 60, 60, 0, 80, 3.0, 0.0, 12.0, 6.0, Some(0.0), Some(16.0), Some(6.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 70, 60, 0, 80, 3.0, 0.0, 20.0, 4.5, Some(0.0), Some(24.0), Some(4.5), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::clear(fighter.module_accessor, 1, false);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 25.0);
    fighter.clear_lua_stack();
    let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    FighterSpecializer_Demon::set_devil(&mut *boma, false, 0.0);
}

#[acmd_script( agent = "demon", script = "game_specialairlw", category = ACMD_GAME, low_priority )]
unsafe fn demon_specialairlw(fighter: &mut L2CAgentBase) {
    fighter.clear_lua_stack();
    let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    FighterSpecializer_Demon::set_devil(&mut *boma, true, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        damage!(fighter, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 10);
    }
    frame(fighter.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(fighter, 0.6);
    frame(fighter.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        damage!(fighter, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        macros::CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, 8.75, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(fighter, 1, Hash40::new("top"), 3.5, 0.0, 7.75, 12.0, Some(0.0), Some(7.75), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(fighter, 2, Hash40::new("top"), 3.0, 0.0, 8.75, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        macros::CATCH(fighter, 3, Hash40::new("top"), 2.0, 0.0, 7.75, 12.5, Some(0.0), Some(7.75), Some(9.5), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        GrabModule::set_constraint(fighter.module_accessor, 0, true);
        GrabModule::set_constraint(fighter.module_accessor, 1, true);
        GrabModule::set_constraint(fighter.module_accessor, 2, true);
        GrabModule::set_constraint(fighter.module_accessor, 3, true);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, 15.0, 5.0, Some(0.0), Some(8.75), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(fighter, 1, Hash40::new("top"), 3.5, 0.0, 20.0, 9.0, Some(0.0), Some(7.75), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(fighter, 2, Hash40::new("top"), 3.0, 0.0, 15.0, 5.0, Some(0.0), Some(8.75), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        macros::CATCH(fighter, 3, Hash40::new("top"), 2.0, 0.0, 20.0, 9.0, Some(0.0), Some(7.75), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        GrabModule::set_constraint(fighter.module_accessor, 0, true);
        GrabModule::set_constraint(fighter.module_accessor, 1, true);
        GrabModule::set_constraint(fighter.module_accessor, 2, true);
        GrabModule::set_constraint(fighter.module_accessor, 3, true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::SET_SPEED_EX(fighter, 0.1, 1.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    macros::FT_MOTION_RATE(fighter, 0.9);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 70, 60, 0, 80, 3.0, 0.0, 21.0, 4.5, Some(0.0), Some(25.0), Some(4.5), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 60, 60, 0, 80, 3.0, 0.0, 14.0, 6.0, Some(0.0), Some(18.0), Some(6.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 64, 50, 0, 75, 3.0, 0.0, 3.0, 4.0, Some(0.0), Some(3.0), Some(5.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::set_down_only(fighter.module_accessor, 2, true);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 2, false);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 70, 60, 0, 80, 3.0, 0.0, 21.5, 4.5, Some(0.0), Some(25.5), Some(4.5), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 60, 60, 0, 80, 3.0, 0.0, 14.0, 6.0, Some(0.0), Some(18.0), Some(6.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 70, 60, 0, 80, 3.0, 0.0, 22.0, 4.5, Some(0.0), Some(26.0), Some(4.5), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::clear(fighter.module_accessor, 1, false);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 25.0);
    fighter.clear_lua_stack();
    let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    FighterSpecializer_Demon::set_devil(&mut *boma, false, 0.0);
}

pub fn install() {
    install_acmd_scripts!(
        demon_attackragedrive,
        demon_15cb9d3406,
        demon_15b52c48bb,
        demon_speciallw,
        demon_specialairlw
    );
}