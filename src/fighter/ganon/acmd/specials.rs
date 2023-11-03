use crate::imports::acmd_imports::*;

#[acmd_script( agent = "ganon", scripts = ["game_specialn", "game_specialairn"], category = ACMD_GAME, low_priority )]
unsafe fn ganon_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        VarModule::set_int(agent.module_accessor, ganon::status::int::TELEPORT_STEP, ganon::TELEPORT_STEP_INIT);
    }
    macros::FT_MOTION_RATE(agent, 0.2);
    frame(agent.lua_state_agent, 25.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_GROUND {
            VarModule::on_flag(agent.module_accessor, ganon::status::flag::TELEPORT_START_GROUND);
        }
        macros::SA_SET(agent, *SITUATION_KIND_AIR);
        let og_x = PostureModule::pos_x(agent.module_accessor);
        let og_y = PostureModule::pos_y(agent.module_accessor);
        VarModule::set_vec2(agent.module_accessor, ganon::status::float::TELEPORT_START_POS, Vector2f{x: og_x, y: og_y});
        VarModule::on_flag(agent.module_accessor, fighter::instance::flag::DISABLE_SPECIAL_N);
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_RESET);
        VarModule::on_flag(agent.module_accessor, ganon::status::flag::TELEPORT_STOP);
        HitModule::set_whole(agent.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
        JostleModule::set_status(agent.module_accessor, false);
        GroundModule::set_correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        VarModule::set_int(agent.module_accessor, ganon::status::int::TELEPORT_STEP, ganon::TELEPORT_STEP_MOVE);
    }
    frame(agent.lua_state_agent, 55.0);
    if macros::is_excute(agent) {
        VarModule::set_int(agent.module_accessor, ganon::status::int::TELEPORT_STEP, ganon::TELEPORT_STEP_CHECK_FEINT);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        HitModule::set_whole(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    macros::FT_MOTION_RATE(agent, 1.5);
    frame(agent.lua_state_agent, 64.0);
    macros::FT_MOTION_RATE(agent, 5.0);
    if macros::is_excute(agent) {
        VarModule::off_flag(agent.module_accessor, ganon::status::flag::TELEPORT_STOP);
        JostleModule::set_status(agent.module_accessor, true);
        CancelModule::enable_cancel(agent.module_accessor);
    }
}

#[acmd_script( agent = "ganon", scripts = ["effect_specialn", "effect_specialairn"], category = ACMD_EFFECT, low_priority )]
unsafe fn ganon_specialn_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_start"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
        for _ in 0..5 {
            macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        WorkModule::set_flag(agent.module_accessor, false, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("ganon_entry"), Hash40::new("top"), 0, 12, -2, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.7);
        macros::FLASH(agent, 1, 0, 1, 1.0);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, false);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        ItemModule::set_attach_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("ganon_entry"), Hash40::new("top"), 0, 12.0, -2.0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.7);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        WorkModule::set_flag(agent.module_accessor, true, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
        VisibilityModule::set_whole(agent.module_accessor, true);
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
        ItemModule::set_attach_item_visibility(agent.module_accessor, true, 0);
        for _ in 0..5 {
            macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    frame(agent.lua_state_agent, 64.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
}

#[acmd_script( agent = "ganon", scripts = ["sound_specialn", "sound_specialairn"], category = ACMD_SOUND, low_priority )]
unsafe fn ganon_specialn_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ganon_appeal_h01"));
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ganon_appear01"));
    }
}

#[acmd_script( agent = "ganon", script = "game_specialsstart", category = ACMD_GAME, low_priority )]
unsafe fn ganon_specialsstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 6.8, 6.7, 7.5, 3.8);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 8.0, 7.5, Some(0.0), Some(8.0), Some(11.0), *FIGHTER_STATUS_KIND_CATCHED_GANON, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 1.0, 0.0, 8.0, 7.2, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_GANON, *COLLISION_SITUATION_MASK_GA);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 7.0, 7.0, 7.5, 7.5);
    }
}

#[acmd_script( agent = "ganon", script = "game_specialairsstart", category = ACMD_GAME, low_priority )]
unsafe fn ganon_specialairsstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        GroundModule::select_cliff_hangdata(agent.module_accessor, 1);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.8);
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 6.0, 7.5, Some(0.0), Some(6.0), Some(11.0), *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 5.0, 0.0, 11.0, 7.5, Some(0.0), Some(11.0), Some(11.0), *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON, *COLLISION_SITUATION_MASK_GA);
    }
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_EXPLOSION_GRAVITY_ONOFF);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd_script( agent = "ganon", script = "game_specialairscatch", category = ACMD_GAME, low_priority )]
unsafe fn ganon_specialairscatch(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    macros::FT_MOTION_RATE(agent, 0.8);
}

#[acmd_script( agent = "ganon", script = "game_specialairs", category = ACMD_GAME, low_priority )]
unsafe fn ganon_specialairs(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 12.0, 292, 82, 0, 40, 1.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::REVERSE_LR(agent);
    }
}

#[acmd_script( agent = "ganon", scripts = ["game_specialhi", "game_specialairhi"], category = ACMD_GAME, low_priority )]
unsafe fn ganon_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 4.4, 0.0, 16.0, 6.5, None, None, None, *FIGHTER_STATUS_KIND_CLUNG_GANON, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(agent, 1, Hash40::new("top"), 6.5, 0.0, 8.8, 13.7, Some(0.0), Some(1.0), Some(13.7), *FIGHTER_STATUS_KIND_CLUNG_GANON, *COLLISION_SITUATION_MASK_GA);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 0, 50, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR, 1);
        macros::CATCH(agent, 0, Hash40::new("top"), 4.4, 0.0, 16.0, 6.5, Some(0.0), Some(18.0), Some(3.0), *FIGHTER_STATUS_KIND_CLUNG_GANON, *COLLISION_SITUATION_MASK_GA);
    }
    // frame(agent.lua_state_agent, 18.0);
    // if macros::is_excute(agent) {
    //     notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
    // }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 7.0, 70, 90, 0, 70, 9.0, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 7.0, 70, 90, 0, 70, 6.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GANON_STATUS_SPECIAL_HI_FLAG_IS_CHECK_DIVE);
    }
}

#[acmd_script( agent = "ganon", script = "game_specialhicatch", category = ACMD_GAME, low_priority )]
unsafe fn ganon_specialhicatch(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 8.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
    }
    for _ in 0..4 {
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 361, 100, 0, 50, 7.0, 0.0, 9.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
            AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
}

#[acmd_script( agent = "ganon", script = "game_specialhithrow", category = ACMD_GAME, low_priority )]
unsafe fn ganon_specialhithrow(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 361, 108, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 8.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(agent.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(agent, 0.75);
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GANON_STATUS_SPECIAL_HI_THROW_FLAG_FALL);
    }
}

#[acmd_script( agent = "ganon", script = "game_speciallw", category = ACMD_GAME, low_priority )]
unsafe fn ganon_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 3.0, 6.0, 8.5, 9.5);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 2.0, 6.0, 8.5, 10.0);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 14.0, 45, 65, 0, 65, 3.0, 2.7, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 14.0, 45, 65, 0, 65, 4.0, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        JostleModule::set_status(agent.module_accessor, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_WALL_CHECK);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 8.0, 8.0, 8.0, 4.0);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        JostleModule::set_status(agent.module_accessor, true);
    }
}

#[acmd_script( agent = "ganon", script = "game_specialairlw", category = ACMD_GAME, low_priority )]
unsafe fn ganon_specialairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_WALL_CHECK);
        macros::ATTACK(agent, 0, 0, Hash40::new("legl"), 14.0, 40, 75, 0, 50, 5.0, 12.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("legl"), 14.0, 40, 75, 0, 50, 3.5, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        JostleModule::set_status(agent.module_accessor, false);
        WorkModule::set_int(agent.module_accessor, 1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    }
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        JostleModule::set_status(agent.module_accessor, true);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ganon_specialn,
        ganon_specialn_eff,
        ganon_specialn_snd,

        ganon_specialsstart,

        ganon_specialairsstart,

        ganon_specialairscatch,

        ganon_specialairs,

        ganon_specialhi,

        ganon_specialhicatch,

        ganon_specialhithrow,

        ganon_speciallw,

        ganon_specialairlw
    );
}