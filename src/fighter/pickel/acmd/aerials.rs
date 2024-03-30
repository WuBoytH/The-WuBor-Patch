use crate::imports::*;

unsafe extern "C" fn pickel_attackairn(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn pickel_attackairf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    if 0 < WorkModule::get_int(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_JUMP_MINI_FRAME) {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            WorkModule::set_int(agent.module_accessor, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
        }
        wait(agent.lua_state_agent, 1.0);
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
            if macros::is_excute(agent) {
                MotionModule::set_rate(agent.module_accessor, 6.0);
                MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 6.0);
            }
        }
        else {
            if macros::is_excute(agent) {
                MotionModule::set_rate(agent.module_accessor, 3.0);
                MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 3.0);
            }
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 1.5);
            MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.5);
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
        frame(agent.lua_state_agent, 12.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
            if macros::is_excute(agent) {
                MotionModule::set_rate(agent.module_accessor, 2.25);
                MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 2.25);
            }
        }
        else {
            if macros::is_excute(agent) {
                MotionModule::set_rate(agent.module_accessor, 2.0);
                MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 2.0);
            }
        }
        frame(agent.lua_state_agent, 20.0);
        if macros::is_excute(agent) {
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(agent.lua_state_agent, 32.0);
        if macros::is_excute(agent) {
            MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
        }
    }
    else {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            WorkModule::set_int(agent.module_accessor, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_PICK, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
        }
        frame(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(agent.lua_state_agent, 8.0);
        let material = WorkModule::get_int(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
        if [
            *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND, *FIGHTER_PICKEL_MATERIAL_KIND_GOLD, *FIGHTER_PICKEL_MATERIAL_KIND_IRON,
            *FIGHTER_PICKEL_MATERIAL_KIND_STONE, *FIGHTER_PICKEL_MATERIAL_KIND_WOOD
        ].contains(&material) {
            let damage = if material == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD || material == *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
                8.0
            }
            else if material == *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
                9.0
            }
            else if material == *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
                10.5
            }
            else {
                12.0
            };
            if macros::is_excute(agent) {
                macros::ATTACK(agent, 0, 0, Hash40::new("armr"), damage, 361, 63, 0, 56, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(agent, 1, 0, Hash40::new("armr"), damage, 361, 63, 0, 56, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(agent, 3, 0, Hash40::new("haver"), damage, 361, 69, 0, 56, 4.2, 0.0, 4.8, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(agent, 4, 0, Hash40::new("haver"), damage, 361, 69, 0, 56, 4.2, 0.0, 4.8, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                WorkModule::set_float(agent.module_accessor, 6.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
            }
            wait(agent.lua_state_agent, 1.0);
            if macros::is_excute(agent) {
                macros::ATTACK(agent, 2, 0, Hash40::new("haver"), damage, 275, 67, 0, 25, 4.2, 0.0, 4.8, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
            wait(agent.lua_state_agent, 1.0);
            if macros::is_excute(agent) {
                AttackModule::clear(agent.module_accessor, 2, false);
                macros::ATTACK(agent, 3, 0, Hash40::new("haver"), damage, 275, 58, 0, 25, 4.2, 0.0, 4.8, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
        }
        else {
            if macros::is_excute(agent) {
                macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 6.0, 361, 63, 0, 56, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 6.0, 361, 63, 0, 56, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            }
            wait(agent.lua_state_agent, 2.0);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
            macros::FT_MOTION_RATE(agent, 0.8);
        }
        frame(agent.lua_state_agent, 20.0);
        if macros::is_excute(agent) {
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }
}

unsafe extern "C" fn pickel_attackairb(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    if 0 < WorkModule::get_int(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_JUMP_MINI_FRAME) {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            WorkModule::set_int(agent.module_accessor, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
        }
        wait(agent.lua_state_agent, 1.0);
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
            if macros::is_excute(agent) {
                MotionModule::set_rate(agent.module_accessor, 10.0);
                MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 10.0);
            }
        }
        else {
            if macros::is_excute(agent) {
                MotionModule::set_rate(agent.module_accessor, 5.0);
                MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 5.0);
            }
        }
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(agent.lua_state_agent, 12.0);
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 1.75);
            MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.75);
        }
        let material = WorkModule::get_int(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
        if [
            *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND, *FIGHTER_PICKEL_MATERIAL_KIND_GOLD, *FIGHTER_PICKEL_MATERIAL_KIND_IRON,
            *FIGHTER_PICKEL_MATERIAL_KIND_STONE, *FIGHTER_PICKEL_MATERIAL_KIND_WOOD
        ].contains(&material) {
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
            if macros::is_excute(agent) {
                macros::ATTACK(agent, 0, 0, Hash40::new("haver"), damage, 45, 134, 0, 27, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(agent, 1, 0, Hash40::new("haver"), damage, 45, 134, 0, 27, 2.3, 0.0, 3.6, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(agent, 2, 0, Hash40::new("haver"), damage, 45, 134, 0, 27, 2.3, 0.0, 7.2, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(agent, 3, 0, Hash40::new("top"), damage, 45, 134, 0, 27, 2.3, 0.0, 6.8, -5.4, Some(0.0), Some(6.8), Some(-10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(agent, 4, 0, Hash40::new("haver"), damage, 57, 134, 0, 20, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(agent, 5, 0, Hash40::new("haver"), damage, 57, 134, 0, 20, 2.3, 0.0, 3.6, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(agent, 6, 0, Hash40::new("haver"), damage, 57, 134, 0, 20, 2.3, 0.0, 7.2, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(agent, 7, 0, Hash40::new("top"), damage, 57, 134, 0, 20, 2.3, 0.0, 6.8, -5.4, Some(0.0), Some(6.8), Some(-10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
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
                macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 2.0, 60, 50, 0, 72, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                macros::ATTACK(agent, 4, 0, Hash40::new("haver"), 2.0, 60, 50, 0, 72, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(agent, 0, 1, 2, 3, 0.25);
                macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(agent, 4, 5, 6, 7, 0.25);
                AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, -10.0, false);
                AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 4, -10.0, false);
            }
        }
        wait(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 19.0);
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
            if macros::is_excute(agent) {
                MotionModule::set_rate(agent.module_accessor, 3.625);
                MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 3.625);
            }
        }
        else {
            if macros::is_excute(agent) {
                MotionModule::set_rate(agent.module_accessor, 3.23);
                MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 3.23);
            }
        }
        frame(agent.lua_state_agent, 22.0);
        if macros::is_excute(agent) {
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(agent.lua_state_agent, 47.0);
        if macros::is_excute(agent) {
            MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
        }
    }
    else {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            WorkModule::set_int(agent.module_accessor, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_PICK, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
        }
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(agent.lua_state_agent, 12.0);
        let material = WorkModule::get_int(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
        if [
            *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND, *FIGHTER_PICKEL_MATERIAL_KIND_GOLD, *FIGHTER_PICKEL_MATERIAL_KIND_IRON,
            *FIGHTER_PICKEL_MATERIAL_KIND_STONE, *FIGHTER_PICKEL_MATERIAL_KIND_WOOD
        ].contains(&material) {
            if macros::is_excute(agent) {
                let damage = if material == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD || material == *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
                    8.5
                }
                else if material == *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
                    9.75
                }
                else if material == *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
                    11.0
                }
                else {
                    12.25
                };
                macros::ATTACK(agent, 0, 0, Hash40::new("armr"), damage, 50, 85, 0, 56, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(agent, 1, 0, Hash40::new("armr"), damage, 50, 85, 0, 56, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(agent, 2, 0, Hash40::new("haver"), damage, 361, 85, 0, 51, 5.2, 0.0, 4.4, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(agent, 3, 0, Hash40::new("haver"), damage, 361, 85, 0, 51, 5.2, 0.0, 4.4, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                WorkModule::set_float(agent.module_accessor, 6.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
            }
        }
        else {
            if macros::is_excute(agent) {
                macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 6.0, 50, 85, 0, 56, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 6.0, 50, 85, 0, 56, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
        wait(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
            macros::FT_MOTION_RATE(agent, 0.8);
        }
        frame(agent.lua_state_agent, 22.0);
        if macros::is_excute(agent) {
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }
}

unsafe extern "C" fn pickel_attackairhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_AXE, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_HI_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 4.0);
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
            macros::ATTACK(agent, 0, 0, Hash40::new("armr"), damage, 71, 76, 0, 50, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.6, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("haver"), damage, 71, 76, 0, 50, 5.4, 0.0, 4.2, 0.0, None, None, None, 1.0, 1.6, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            WorkModule::set_float(agent.module_accessor, 5.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("armr"), damage, 71, 76, 0, 50, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.6, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("haver"), damage, 71, 76, 0, 50, 5.4, 0.0, 4.2, 0.0, None, None, None, 1.0, 1.6, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 3.2, 71, 76, 0, 50, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 3.2, 71, 76, 0, 50, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
        macros::FT_MOTION_RATE(agent, 0.5);
    }
    frame(agent.lua_state_agent, 13.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_HI_ENABLE_LANDING);
    }
}

unsafe extern "C" fn pickel_attackairlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(agent, 0, 0.4, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::suspend_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    macros::FT_MOTION_RATE(agent, 4.0 / 3.0);
    frame(agent.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PICKEL_STATUS_ATTACK_FLAG_FORGE_GENERATE_ENABLE);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        KineticModule::resume_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
}

unsafe extern "C" fn pickel_attackairlw2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 60, 50, 0, 72, 4.0, 0.0, 1.0, 4.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn pickel_attackairlw2_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitm"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 8);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_attackairn", pickel_attackairn);

    agent.acmd("game_attackairf", pickel_attackairf);

    agent.acmd("game_attackairb", pickel_attackairb);

    agent.acmd("game_attackairhi", pickel_attackairhi);

    agent.acmd("game_attackairlw", pickel_attackairlw);

    agent.acmd("game_attackairlw2", pickel_attackairlw2);
    agent.acmd("expression_attackairlw2", pickel_attackairlw2_exp);
}