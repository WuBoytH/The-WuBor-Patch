use super::*;

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_DEMON_STATUS_SPECIAL_HI_FLAG_AIR) {
        FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 2.0);
        if macros::is_excute(agent) {
            GroundModule::select_cliff_hangdata(agent.module_accessor, 1);
        }
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 76, 53, 0, 80, 6.0, 0.0, 6.0, 6.0, Some(0.0), Some(11.25), Some(6.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 16.0, 76, 53, 0, 80, 6.0, 0.0, 18.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 18.0, 76, 53, 0, 80, 6.0, 0.0, 6.0, -2.5, Some(0.0), Some(10.25), Some(-2.5), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
        }
        frame(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            AttackModule::clear(agent.module_accessor, 0, false);
            AttackModule::clear(agent.module_accessor, 2, false);
        }
        frame(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 13.0, 71, 65, 0, 75, 5.15, 0.0, 18.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
        }
        frame(agent.lua_state_agent, 7.0);
        if macros::is_excute(agent) {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
        frame(agent.lua_state_agent, 7.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 63, 85, 0, 60, 4.6, 0.0, 18.0, 0.0, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
        }
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 60, 85, 0, 60, 4.0, 0.0, 18.0, 0.0, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
        }
        frame(agent.lua_state_agent, 15.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 20.0);
        if macros::is_excute(agent) {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
        frame(agent.lua_state_agent, 46.0);
        FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 3.0);
        frame(agent.lua_state_agent, 52.0);
        FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 4.0);
        frame(agent.lua_state_agent, 53.0);
        FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 6.0);
        frame(agent.lua_state_agent, 54.0);
        FighterSpecializer_Demon::set_devil(agent.module_accessor, false, 0.0);
    }
    else {
        FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 2.0);
        if macros::is_excute(agent) {
            GroundModule::select_cliff_hangdata(agent.module_accessor, 1);
        }
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 76, 53, 0, 85, 6.0, 0.0, 6.0, 6.0, Some(0.0), Some(11.25), Some(6.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 16.0, 76, 53, 0, 85, 6.0, 0.0, 18.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 18.0, 76, 53, 0, 85, 6.0, 0.0, 6.0, -2.5, Some(0.0), Some(10.25), Some(-2.5), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
        }
        frame(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            AttackModule::clear(agent.module_accessor, 0, false);
            AttackModule::clear(agent.module_accessor, 2, false);
        }
        frame(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 13.0, 71, 65, 0, 75, 5.15, 0.0, 18.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
        }
        frame(agent.lua_state_agent, 7.0);
        if macros::is_excute(agent) {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
        frame(agent.lua_state_agent, 7.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 63, 85, 0, 60, 4.6, 0.0, 18.0, 0.0, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
        }
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 60, 85, 0, 60, 4.0, 0.0, 18.0, 0.0, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
        }
        frame(agent.lua_state_agent, 15.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 20.0);
        if macros::is_excute(agent) {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
        frame(agent.lua_state_agent, 46.0);
        FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 3.0);
        frame(agent.lua_state_agent, 52.0);
        FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 4.0);
        frame(agent.lua_state_agent, 53.0);
        FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 6.0);
        frame(agent.lua_state_agent, 54.0);
        FighterSpecializer_Demon::set_devil(agent.module_accessor, false, 0.0);
    }
}

unsafe extern "C" fn game_specialhigroundair(agent: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 2.0);
    if macros::is_excute(agent) {
        GroundModule::select_cliff_hangdata(agent.module_accessor, 1);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 76, 53, 0, 85, 6.0, 0.0, 6.0, 6.0, Some(0.0), Some(11.25), Some(6.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 16.0, 76, 53, 0, 85, 6.0, 0.0, 18.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 18.0, 76, 53, 0, 85, 6.0, 0.0, 6.0, -2.5, Some(0.0), Some(10.25), Some(-2.5), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        AttackModule::clear(agent.module_accessor, 2, false);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 13.0, 71, 65, 0, 75, 5.15, 0.0, 18.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
    }
    // frame(agent.lua_state_agent, 7.0);
    // if macros::is_excute(agent) {
    //     notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    // }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 63, 85, 0, 60, 4.6, 0.0, 18.0, 0.0, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 60, 85, 0, 60, 4.0, 0.0, 18.0, 0.0, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 46.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 3.0);
    frame(agent.lua_state_agent, 52.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 4.0);
    frame(agent.lua_state_agent, 53.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 6.0);
    frame(agent.lua_state_agent, 54.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, false, 0.0);
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        damage!(agent, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 10);
    }
    frame(agent.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(agent, 0.6);
    frame(agent.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        damage!(agent, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 8.75, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 3.5, 0.0, 7.75, 12.0, Some(0.0), Some(7.75), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 2, Hash40::new("top"), 3.0, 0.0, 8.75, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        macros::CATCH(agent, 3, Hash40::new("top"), 2.0, 0.0, 7.75, 12.5, Some(0.0), Some(7.75), Some(9.5), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        GrabModule::set_constraint(agent.module_accessor, 0, true);
        GrabModule::set_constraint(agent.module_accessor, 1, true);
        GrabModule::set_constraint(agent.module_accessor, 2, true);
        GrabModule::set_constraint(agent.module_accessor, 3, true);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 14.0, 5.0, Some(0.0), Some(8.75), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 3.5, 0.0, 19.0, 9.0, Some(0.0), Some(7.75), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 2, Hash40::new("top"), 3.0, 0.0, 14.0, 5.0, Some(0.0), Some(8.75), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        macros::CATCH(agent, 3, Hash40::new("top"), 2.0, 0.0, 19.0, 9.0, Some(0.0), Some(7.75), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        GrabModule::set_constraint(agent.module_accessor, 0, true);
        GrabModule::set_constraint(agent.module_accessor, 1, true);
        GrabModule::set_constraint(agent.module_accessor, 2, true);
        GrabModule::set_constraint(agent.module_accessor, 3, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::SET_SPEED_EX(agent, 0.1, 1.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    macros::FT_MOTION_RATE(agent, 0.9);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 70, 60, 0, 80, 3.0, 0.0, 19.0, 4.5, Some(0.0), Some(23.0), Some(4.5), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 60, 60, 0, 80, 3.0, 0.0, 12.0, 6.0, Some(0.0), Some(16.0), Some(6.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 64, 50, 0, 75, 3.0, 0.0, 3.0, 4.0, Some(0.0), Some(3.0), Some(5.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::set_down_only(agent.module_accessor, 2, true);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 2, false);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 70, 60, 0, 80, 3.0, 0.0, 19.5, 4.5, Some(0.0), Some(23.5), Some(4.5), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 60, 60, 0, 80, 3.0, 0.0, 12.0, 6.0, Some(0.0), Some(16.0), Some(6.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 70, 60, 0, 80, 3.0, 0.0, 20.0, 4.5, Some(0.0), Some(24.0), Some(4.5), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 25.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, false, 0.0);
}

unsafe extern "C" fn game_specialairlw(agent: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        damage!(agent, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 10);
    }
    frame(agent.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(agent, 0.6);
    frame(agent.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        damage!(agent, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 8.75, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 3.5, 0.0, 7.75, 12.0, Some(0.0), Some(7.75), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 2, Hash40::new("top"), 3.0, 0.0, 8.75, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        macros::CATCH(agent, 3, Hash40::new("top"), 2.0, 0.0, 7.75, 12.5, Some(0.0), Some(7.75), Some(9.5), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        GrabModule::set_constraint(agent.module_accessor, 0, true);
        GrabModule::set_constraint(agent.module_accessor, 1, true);
        GrabModule::set_constraint(agent.module_accessor, 2, true);
        GrabModule::set_constraint(agent.module_accessor, 3, true);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 15.0, 5.0, Some(0.0), Some(8.75), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 3.5, 0.0, 20.0, 9.0, Some(0.0), Some(7.75), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 2, Hash40::new("top"), 3.0, 0.0, 15.0, 5.0, Some(0.0), Some(8.75), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        macros::CATCH(agent, 3, Hash40::new("top"), 2.0, 0.0, 20.0, 9.0, Some(0.0), Some(7.75), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        GrabModule::set_constraint(agent.module_accessor, 0, true);
        GrabModule::set_constraint(agent.module_accessor, 1, true);
        GrabModule::set_constraint(agent.module_accessor, 2, true);
        GrabModule::set_constraint(agent.module_accessor, 3, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::SET_SPEED_EX(agent, 0.1, 1.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    macros::FT_MOTION_RATE(agent, 0.9);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 70, 60, 0, 80, 3.0, 0.0, 21.0, 4.5, Some(0.0), Some(25.0), Some(4.5), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 60, 60, 0, 80, 3.0, 0.0, 14.0, 6.0, Some(0.0), Some(18.0), Some(6.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 64, 50, 0, 75, 3.0, 0.0, 3.0, 4.0, Some(0.0), Some(3.0), Some(5.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::set_down_only(agent.module_accessor, 2, true);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 2, false);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 70, 60, 0, 80, 3.0, 0.0, 21.5, 4.5, Some(0.0), Some(25.5), Some(4.5), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 60, 60, 0, 80, 3.0, 0.0, 14.0, 6.0, Some(0.0), Some(18.0), Some(6.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 70, 60, 0, 80, 3.0, 0.0, 22.0, 4.5, Some(0.0), Some(26.0), Some(4.5), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 25.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, false, 0.0);
}

unsafe extern "C" fn game_speciallwground(agent: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 3.0);
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 14.0, 60, 60, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
        macros::ATTACK_IGNORE_THROW(agent, 0, 0, Hash40::new("top"), 10.0, 361, 80, 0, 80, 4.0, 0.0, 4.0, -7.0, Some(0.0), Some(4.0), Some(11.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::FT_CATCH_STOP(agent, 10, 1);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::CHECK_FINISH_CAMERA(agent, 15, 3);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.3);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: -2.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DEMON_STATUS_SPECIAL_LW_FLAG_HIT);
        macros::CAM_ZOOM_OUT(agent);
    }
    frame(agent.lua_state_agent, 35.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 3.0);
    frame(agent.lua_state_agent, 38.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 3.0);
    frame(agent.lua_state_agent, 39.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 5.0);
    frame(agent.lua_state_agent, 40.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 6.0);
    frame(agent.lua_state_agent, 41.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 7.0);
    frame(agent.lua_state_agent, 42.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, false, 0.0);
}

unsafe extern "C" fn game_attackragedrive(agent: &mut L2CAgentBase) {
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ATTACK_RAGE_CAPTURE) {
        FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 10.0);
        if macros::is_excute(agent) {
            macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
        }
        frame(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            damage!(agent, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        }
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            damage!(agent, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0);
            macros::CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 9.5, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(agent, 1, Hash40::new("top"), 4.5, 0.0, 8.5, 12.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(agent, 2, Hash40::new("top"), 4.0, 0.0, 9.5, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            macros::CATCH(agent, 3, Hash40::new("top"), 3.5, 0.0, 8.5, 12.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(agent.module_accessor, 0, true);
            GrabModule::set_constraint(agent.module_accessor, 1, true);
            GrabModule::set_constraint(agent.module_accessor, 2, true);
            GrabModule::set_constraint(agent.module_accessor, 3, true);
        }
        frame(agent.lua_state_agent, 14.0);
        if macros::is_excute(agent) {
            macros::CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(agent, 1, Hash40::new("top"), 4.5, 0.0, 19.0, 9.0, Some(0.0), Some(8.5), Some(12.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(agent, 2, Hash40::new("top"), 4.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            macros::CATCH(agent, 3, Hash40::new("top"), 3.5, 0.0, 21.0, 8.5, Some(0.0), Some(8.5), Some(12.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(agent.module_accessor, 0, true);
            GrabModule::set_constraint(agent.module_accessor, 1, true);
            GrabModule::set_constraint(agent.module_accessor, 2, true);
            GrabModule::set_constraint(agent.module_accessor, 3, true);
        }
        frame(agent.lua_state_agent, 15.0);
        if macros::is_excute(agent) {
            grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        }
    }
    else {
        FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 10.0);
        macros::FT_MOTION_RATE(agent, 0.7);
        if macros::is_excute(agent) {
            macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
        }
        frame(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            damage!(agent, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        }
        frame(agent.lua_state_agent, 10.0);
        macros::FT_MOTION_RATE(agent, 1.0);
        frame(agent.lua_state_agent, 12.0);
        if macros::is_excute(agent) {
            damage!(agent, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0);
            macros::CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 8.0, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(agent, 1, Hash40::new("top"), 4.5, 0.0, 6.0, 8.5, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(agent, 2, Hash40::new("top"), 4.0, 0.0, 8.0, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            macros::CATCH(agent, 3, Hash40::new("top"), 3.5, 0.0, 6.0, 8.5, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(agent.module_accessor, 0, true);
            GrabModule::set_constraint(agent.module_accessor, 1, true);
            GrabModule::set_constraint(agent.module_accessor, 2, true);
            GrabModule::set_constraint(agent.module_accessor, 3, true);
        }
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            damage!(agent, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0);
            macros::CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 9.5, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(agent, 1, Hash40::new("top"), 4.5, 0.0, 8.5, 12.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(agent, 2, Hash40::new("top"), 4.0, 0.0, 9.5, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            macros::CATCH(agent, 3, Hash40::new("top"), 3.5, 0.0, 8.5, 12.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(agent.module_accessor, 0, true);
            GrabModule::set_constraint(agent.module_accessor, 1, true);
            GrabModule::set_constraint(agent.module_accessor, 2, true);
            GrabModule::set_constraint(agent.module_accessor, 3, true);
        }
        frame(agent.lua_state_agent, 14.0);
        if macros::is_excute(agent) {
            macros::CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(agent, 1, Hash40::new("top"), 4.5, 0.0, 19.0, 9.0, Some(0.0), Some(8.5), Some(12.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(agent, 2, Hash40::new("top"), 4.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            macros::CATCH(agent, 3, Hash40::new("top"), 3.5, 0.0, 21.0, 8.5, Some(0.0), Some(8.5), Some(12.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(agent.module_accessor, 0, true);
            GrabModule::set_constraint(agent.module_accessor, 1, true);
            GrabModule::set_constraint(agent.module_accessor, 2, true);
            GrabModule::set_constraint(agent.module_accessor, 3, true);
        }
        frame(agent.lua_state_agent, 15.0);
        if macros::is_excute(agent) {
            macros::CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(agent, 1, Hash40::new("top"), 4.5, 0.0, 22.0, 7.0, Some(0.0), Some(8.5), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(agent, 2, Hash40::new("top"), 4.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            macros::CATCH(agent, 3, Hash40::new("top"), 3.5, 0.0, 24.0, 6.5, Some(0.0), Some(8.5), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(agent.module_accessor, 0, true);
            GrabModule::set_constraint(agent.module_accessor, 1, true);
            GrabModule::set_constraint(agent.module_accessor, 2, true);
            GrabModule::set_constraint(agent.module_accessor, 3, true);
        }
        frame(agent.lua_state_agent, 16.0);
        if macros::is_excute(agent) {
            grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        }
    }
    frame(agent.lua_state_agent, 38.0);
    macros::FT_MOTION_RATE(agent, 1.3);
    frame(agent.lua_state_agent, 52.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, false, 0.0);
}

unsafe extern "C" fn game_attackragedriveground(agent: &mut L2CAgentBase) {
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ATTACK_RAGE_CAPTURE) {
        FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 2.0);
        if macros::is_excute(agent) {
            macros::WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
        }
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            WorkModule::set_int(agent.module_accessor, *HIT_STATUS_NORMAL, *FIGHTER_DEMON_STATUS_ATTACK_RAGE_DRIVE_INT_TARGET_HIT_STATUS);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 100, 100, 100, 50, 6.5, 0.0, 1.0, 5.0, Some(10.0), Some(1.0), Some(5.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
            AttackModule::set_no_dead_all(agent.module_accessor, true, false);
            AttackModule::set_catch_only_all(agent.module_accessor, true, false);
            macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 11.0, 60, 95, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
            AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 0, *CAMERA_QUAKE_KIND_NONE, true);
            AttackModule::set_no_finish_camera(agent.module_accessor, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, true, true);
            macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
            macros::ATTACK_IGNORE_THROW(agent, 1, 0, Hash40::new("top"), 18.0, 361, 80, 0, 80, 7.5, 0.0, 7.5, -10.0, Some(0.0), Some(7.5), Some(14.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_PUNCH);
            AttackModule::set_no_finish_camera(agent.module_accessor, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, true, true);
            FighterSpecializer_Demon::clear_past_log_throw(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_DEMON_STATUS_SPECIAL_LW_FLAG_CHECK_DAMAGE);
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_DEMON_STATUS_SPECIAL_LW_FLAG_HIT);
            macros::CAM_ZOOM_OUT(agent);
        }
        frame(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
        }
        frame(agent.lua_state_agent, 35.0);
        FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 3.0);
        frame(agent.lua_state_agent, 38.0);
        FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 4.0);
        frame(agent.lua_state_agent, 40.0);
        FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 5.0);
        frame(agent.lua_state_agent, 41.0);
        FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 6.0);
        frame(agent.lua_state_agent, 42.0);
        FighterSpecializer_Demon::set_devil(agent.module_accessor, false, 0.0);
    }
    else {
        FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 2.0);
        if macros::is_excute(agent) {
            macros::WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
        }
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            WorkModule::set_int(agent.module_accessor, *HIT_STATUS_NORMAL, *FIGHTER_DEMON_STATUS_ATTACK_RAGE_DRIVE_INT_TARGET_HIT_STATUS);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 100, 100, 100, 50, 6.5, 0.0, 1.0, 5.0, Some(10.0), Some(1.0), Some(5.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
            AttackModule::set_no_dead_all(agent.module_accessor, true, false);
            AttackModule::set_catch_only_all(agent.module_accessor, true, false);
            macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 60, 95, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
            AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 0, *CAMERA_QUAKE_KIND_NONE, true);
            AttackModule::set_no_finish_camera(agent.module_accessor, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, true, true);
            macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
            macros::ATTACK_IGNORE_THROW(agent, 1, 0, Hash40::new("top"), 15.0, 361, 80, 0, 80, 5.0, 0.0, 5.0, -8.5, Some(0.0), Some(5.0), Some(12.5), 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_PUNCH);
            AttackModule::set_no_finish_camera(agent.module_accessor, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, true, true);
            FighterSpecializer_Demon::clear_past_log_throw(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_DEMON_STATUS_SPECIAL_LW_FLAG_CHECK_DAMAGE);
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_DEMON_STATUS_SPECIAL_LW_FLAG_HIT);
            macros::CAM_ZOOM_OUT(agent);
        }
        frame(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
        }
        frame(agent.lua_state_agent, 35.0);
        FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 3.0);
        frame(agent.lua_state_agent, 38.0);
        FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 4.0);
        frame(agent.lua_state_agent, 40.0);
        FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 5.0);
        frame(agent.lua_state_agent, 41.0);
        FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 6.0);
        frame(agent.lua_state_agent, 42.0);
        FighterSpecializer_Demon::set_devil(agent.module_accessor, false, 0.0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialhi", game_specialhi, Priority::Low);

    agent.acmd("game_specialhiground", game_specialhigroundair, Priority::Low);
    agent.acmd("game_specialhiair", game_specialhigroundair, Priority::Low);

    agent.acmd("game_speciallw", game_speciallw, Priority::Low);

    agent.acmd("game_specialairlw", game_specialairlw, Priority::Low);

    agent.acmd("game_speciallwground", game_speciallwground, Priority::Low);

    agent.acmd("game_attackragedrive", game_attackragedrive, Priority::Low);

    agent.acmd("game_attackairragedrive", game_attackragedrive, Priority::Low);

    agent.acmd("game_attackragedriveground", game_attackragedriveground, Priority::Low);
}