use crate::imports::*;

unsafe extern "C" fn game_attackdash(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_PICK, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
    }
    frame(agent.lua_state_agent, 8.0);
    let material = WorkModule::get_int(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    if [
        *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND, *FIGHTER_PICKEL_MATERIAL_KIND_GOLD, *FIGHTER_PICKEL_MATERIAL_KIND_IRON,
        *FIGHTER_PICKEL_MATERIAL_KIND_STONE, *FIGHTER_PICKEL_MATERIAL_KIND_WOOD
    ].contains(&material) {
        let damage = if material == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD || material == *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
            9.5
        }
        else if material == *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
            10.5
        }
        else if material == *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
            11.5
        }
        else {
            12.5
        };
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 50, 66, 0, 68, 5.8, 0.0, 8.8, 4.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 50, 66, 0, 68, 5.8, 0.0, 8.8, 9.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            WorkModule::set_float(agent.module_accessor, 4.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage * 0.8, 50, 58, 0, 66, 5.2, 0.0, 8.8, 3.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage * 0.8, 50, 58, 0, 66, 5.2, 0.0, 8.8, 9.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.5, 50, 66, 0, 68, 5.8, 0.0, 8.8, 4.2, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.75, 50, 58, 0, 66, 5.2, 0.0, 8.8, 3.8, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 1.2);
            MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.2);
        }
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.0);
        MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }
}

unsafe extern "C" fn game_attacks3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
    }
    frame(agent.lua_state_agent, 2.0);
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
        if macros::is_excute(agent) {
            MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 2.0);
        }
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }
    let material = WorkModule::get_int(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    if [
        *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND, *FIGHTER_PICKEL_MATERIAL_KIND_GOLD, *FIGHTER_PICKEL_MATERIAL_KIND_IRON,
        *FIGHTER_PICKEL_MATERIAL_KIND_STONE, *FIGHTER_PICKEL_MATERIAL_KIND_WOOD
    ].contains(&material) {
        if macros::is_excute(agent) {
            let damage = if material == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD || material == *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
                2.5
            }
            else if material == *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
                3.0
            }
            else if material == *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
                3.5
            }
            else {
                4.0
            };
            macros::ATTACK(agent, 0, 0, Hash40::new("haver"), damage, 45, 134, 0, 27, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("haver"), damage, 45, 134, 0, 27, 2.3, 0.0, 3.6, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 2, 0, Hash40::new("haver"), damage, 45, 134, 0, 27, 2.3, 0.0, 7.2, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), damage, 45, 134, 0, 27, 2.3, 0.0, 6.8, 5.4, Some(0.0), Some(6.8), Some(10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 4, 0, Hash40::new("haver"), damage, 57, 134, 0, 20, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 5, 0, Hash40::new("haver"), damage, 57, 134, 0, 20, 2.3, 0.0, 3.6, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 6, 0, Hash40::new("haver"), damage, 57, 134, 0, 20, 2.3, 0.0, 7.2, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 7, 0, Hash40::new("top"), damage, 57, 134, 0, 20, 2.3, 0.0, 6.8, 5.4, Some(0.0), Some(6.8), Some(10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(agent, 0, 1, 2, 3, 0.25);
            macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(agent, 4, 5, 6, 7, 0.25);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, -5.0, false);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, -5.0, false);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, -5.0, false);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 3, -5.0, false);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 4, -3.0, false);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 5, -3.0, false);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 6, -3.0, false);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 7, -3.0, false);
            WorkModule::set_float(agent.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 2.0, 60, 50, 0, 72, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 4, 0, Hash40::new("haver"), 2.0, 60, 50, 0, 72, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(agent, 0, 1, 2, 3, 0.25);
            macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(agent, 4, 5, 6, 7, 0.25);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, -10.0, false);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 4, -10.0, false);
        }
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 1.0);
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 2.25);
            MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.25);
        }
    }
    else {
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 2.0);
            MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
        }
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }
}

unsafe extern "C" fn game_attackhi3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_AXE, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
    }
    frame(agent.lua_state_agent, 2.0);
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
        if macros::is_excute(agent) {
            MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 2.0);
        }
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 2.0);
    }
    let material = WorkModule::get_int(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    if [
        *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND, *FIGHTER_PICKEL_MATERIAL_KIND_GOLD, *FIGHTER_PICKEL_MATERIAL_KIND_IRON,
        *FIGHTER_PICKEL_MATERIAL_KIND_STONE, *FIGHTER_PICKEL_MATERIAL_KIND_WOOD
    ].contains(&material) {
        let damage = if material == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD || material == *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
            4.5
        }
        else if material == *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
            5.15
        }
        else if material == *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
            5.8
        }
        else {
            6.775
        };
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("armr"), damage, 86, 78, 0, 45, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.6, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("haver"), damage, 86, 78, 0, 45, 5.4, 0.0, 4.2, 0.0, None, None, None, 1.0, 1.6, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            WorkModule::set_float(agent.module_accessor, 5.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
            macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.85);
            macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 0.85);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("armr"), damage, 98, 74, 0, 41, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.6, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("haver"), damage, 98, 74, 0, 41, 5.4, 0.0, 4.2, 0.0, None, None, None, 1.0, 1.6, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.85);
            macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 0.85);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 3.2, 86, 78, 0, 45, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 3.2, 98, 74, 0, 41, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) != *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
        if macros::is_excute(agent) {
            MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 0.5);
        }
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attackdash", game_attackdash, Priority::Low);

    agent.acmd("game_attacks3", game_attacks3, Priority::Low);

    agent.acmd("game_attackhi3", game_attackhi3, Priority::Low);
}