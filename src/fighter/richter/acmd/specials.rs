use crate::imports::acmd_imports::*;

#[acmd_script( agent = "richter", scripts = ["game_specialn", "game_specialairn"], category = ACMD_GAME, low_priority )]
unsafe fn richter_specialn(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.7);
    frame(agent.lua_state_agent, 30.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, richter::status::flag::SPECIAL_N_SHOOT);
    }
}

#[acmd_script( agent = "richter", scripts = ["expression_specialn", "expression_specialairn"], category = ACMD_EXPRESSION, low_priority )]
unsafe fn richter_specialn_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_lightthrow4item"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
}

#[acmd_script( agent = "richter", scripts = ["game_specialnblank", "game_specialairnblank"], category = ACMD_GAME, low_priority )]
unsafe fn richter_specialnblank(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.7);
    frame(agent.lua_state_agent, 30.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

#[acmd_script( agent = "richter_axe", script = "game_fly", category = ACMD_GAME, low_priority )]
unsafe fn richter_axe_fly(agent: &mut L2CAgentBase) {
    let owner_id = WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    if sv_battle_object::is_active(owner_id) {
        let owner_module_accessor = sv_battle_object::module_accessor(owner_id);
        VarModule::set_int(owner_module_accessor, richter::instance::int::AXE_ID, agent.battle_object_id as i32);
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("axe"), 9.0, 130, 30, 0, 80, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 5.0, false);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.1);
    }
}

#[acmd_script( agent = "richter", scripts = ["game_specials1", "game_specialairs1"], category = ACMD_GAME, low_priority )]
unsafe fn richter_specials1(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 1.2);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SIMON_GENERATE_ARTICLE_CROSS, false, 0);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_S_FLAG_FALL);
    }
    frame(agent.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        ArticleModule::shoot(agent.module_accessor, *FIGHTER_SIMON_GENERATE_ARTICLE_CROSS, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_S_FLAG_FALL);
    }
}

#[acmd_script( agent = "richter_cross", script = "game_fly", category = ACMD_GAME, low_priority )]
unsafe fn richter_cross_fly(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("rot"), 6.0, 90, 20, 0, 75, 1.2, 0.0, 3.7, 0.0, Some(0.0), Some(-3.7), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -6, -1.0, 24, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RICHTER_CROSS, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("rot"), 6.0, 90, 20, 0, 75, 1.2, 0.0, 0.0, 3.7, Some(0.0), Some(0.0), Some(-3.7), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -6, -1.0, 24, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RICHTER_CROSS, *ATTACK_REGION_OBJECT);
    }
}

#[acmd_script( agent = "richter_cross", script = "game_turn", category = ACMD_GAME, low_priority )]
unsafe fn richter_cross_turn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("rot"), 6.0, 90, 20, 0, 75, 1.2, 0.0, 3.7, 0.0, Some(0.0), Some(-3.7), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -6, -1.0, 24, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RICHTER_CROSS, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("rot"), 6.0, 90, 20, 0, 75, 1.2, 0.0, 0.0, 3.7, Some(0.0), Some(0.0), Some(-3.7), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -6, -1.0, 24, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RICHTER_CROSS, *ATTACK_REGION_OBJECT);
    }
}

#[acmd_script( agent = "richter", script = "game_specialhi", category = ACMD_GAME, low_priority )]
unsafe fn richter_specialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        GroundModule::select_cliff_hangdata(agent.module_accessor, 9);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 95, 100, 155, 0, 8.5, 0.0, 9.5, 10.5, None, None, None, 1.4, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        // notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 367, 100, 80, 0, 6.0, 0.0, 26.0, 9.5, Some(0.0), Some(6.0), Some(7.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    // frame(agent.lua_state_agent, 20.0);
    // if macros::is_excute(agent) {
    //     notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    // }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        macros::ATTACK(agent, 1, 1, Hash40::new("top"), 6.0, 45, 30, 0, 85, 6.0, 0.0, 26.0, 9.5, Some(0.0), Some(6.0), Some(7.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 3.0, false);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        VarModule::on_flag(agent.module_accessor, fighter::instance::flag::DISABLE_SPECIAL_HI);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_HI_FLAG_MOVE);
    }
}

#[acmd_script( agent = "richter", script = "game_specialairhi", category = ACMD_GAME, low_priority )]
unsafe fn richter_specialairhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        GroundModule::select_cliff_hangdata(agent.module_accessor, 9);
    }
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 7.0 / 4.0);
    frame(agent.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 95, 100, 155, 0, 8.5, 0.0, 9.5, 10.5, None, None, None, 1.4, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        // notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 367, 100, 80, 0, 6.0, 0.0, 26.0, 9.5, Some(0.0), Some(6.0), Some(7.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    // frame(agent.lua_state_agent, 20.0);
    // if macros::is_excute(agent) {
    //     notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    // }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        macros::ATTACK(agent, 1, 1, Hash40::new("top"), 6.0, 45, 30, 0, 85, 6.0, 0.0, 26.0, 9.5, Some(0.0), Some(6.0), Some(7.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 3.0, false);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        VarModule::on_flag(agent.module_accessor, fighter::instance::flag::DISABLE_SPECIAL_HI);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_HI_FLAG_MOVE);
    }
}

#[acmd_script( agent = "richter", scripts = ["game_speciallw", "game_specialairlw"], category = ACMD_GAME, low_priority )]
unsafe fn richter_speciallw(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 1.3);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_LW_FLAG_GENERATE_HOLYWATER);
    }
    frame(agent.lua_state_agent, 18.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_LW_FLAG_SHOOT_HOLYWATER);
    }
}

pub fn install() {
    install_acmd_scripts!(
        richter_specialn,
        richter_specialn_exp,

        richter_specialnblank,

        richter_axe_fly,

        richter_specials1,

        richter_cross_fly,

        richter_cross_turn,

        richter_specialhi,

        richter_specialairhi,

        richter_speciallw
    );
}