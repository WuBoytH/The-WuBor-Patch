use crate::imports::*;
use super::super::vl;

// Special N

unsafe extern "C" fn ike_specialnend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        KineticModule::set_consider_ground_friction(agent.module_accessor, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        ike_special_n_end_ray_check(agent);
    }
    if VarModule::is_flag(agent.module_accessor, ike::status::flag::SPECIAL_N_RANGED_ERUPTION) {
        if macros::is_excute(agent) {
            let eruption_pos = VarModule::get_float(agent.module_accessor, ike::status::float::SPECIAL_N_ERUPT_LOCATION);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 368, 90, 10, 50, 10.0, 0.0, 10.0, 8.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 368, 90, 10, 50, 8.0, 0.0, 20.0, 8.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 368, 90, 10, 50, 4.0, 0.0, 28.0, 8.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            for x in 0..3 {
                AttackModule::set_vec_target_pos(
                    agent.module_accessor,
                    x,
                    Hash40::new("top"),
                    &Vector2f{x: eruption_pos - 1.4, y: 10.0},
                    12,
                    false
                );
            }
        }
    }
    else {
        if macros::is_excute(agent) {
            let count = VarModule::get_int(agent.module_accessor, ike::status::int::ERUPTION_COUNT);
            if count > vl::special_n::eruption_count_for_critical {
                VarModule::on_flag(agent.module_accessor, ike::status::flag::SPECIAL_N_ENABLE_CRITICAL);
            }
            let damage_add = count as f32 * vl::special_n::eruption_count_damage_add;
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0 + damage_add, 80, 70, 0, 60, 12.0, 0.0, 10.0, 8.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0 + damage_add, 80, 70, 0, 60, 10.0, 0.0, 28.0, 8.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 8.0 + damage_add, 80, 70, 0, 60, 4.0, 0.0, 38.0, 8.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        }
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 7.0);
    if !VarModule::is_flag(agent.module_accessor, ike::status::flag::SPECIAL_N_AIR) {
        if VarModule::is_flag(agent.module_accessor, ike::status::flag::SPECIAL_N_RANGED_ERUPTION) {
            if macros::is_excute(agent) {
                let eruption_pos = VarModule::get_float(agent.module_accessor, ike::status::float::SPECIAL_N_ERUPT_LOCATION);
                let count = VarModule::get_int(agent.module_accessor, ike::status::int::ERUPTION_COUNT);
                if count > vl::special_n::eruption_count_for_critical {
                    VarModule::on_flag(agent.module_accessor, ike::status::flag::SPECIAL_N_ENABLE_CRITICAL);
                }
                let damage_add = count as f32 * vl::special_n::eruption_count_damage_add;
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0 + damage_add, 80, 70, 0, 60, 12.0, 0.0, 10.0, eruption_pos - 1.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
                macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0 + damage_add, 80, 70, 0, 60, 10.0, 0.0, 28.0, eruption_pos - 1.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
                macros::ATTACK(agent, 2, 0, Hash40::new("top"), 8.0 + damage_add, 80, 70, 0, 60, 4.0, 0.0, 38.0, eruption_pos - 1.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            }
        }
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    MiscModule::calc_motion_rate_from_cancel_frame(agent, 24.0, -15.0);
}

unsafe extern "C" fn effect_specialnend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("ike_sword2"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 14.5, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 11.0);
    if !VarModule::is_flag(agent.module_accessor, ike::status::flag::SPECIAL_N_AIR) {
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("ike_sword2"), false, false);
            macros::EFFECT(agent, Hash40::new("ike_volcano_ground"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
            if !VarModule::is_flag(agent.module_accessor, ike::status::flag::SPECIAL_N_RANGED_ERUPTION) {
                macros::EFFECT(agent, Hash40::new("ike_volcano_max"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
                macros::EFFECT(agent, Hash40::new("ike_volcano_add4"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
                macros::EFFECT(agent, Hash40::new("ike_volcano_flash3_g"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
                macros::EFFECT(agent, Hash40::new("ike_volcano_add2"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
            } 
            else {
                macros::EFFECT(agent, Hash40::new("ike_volcano"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
                macros::EFFECT(agent, Hash40::new("ike_volcano_flash_g"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
            }
        }
    }
    wait(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ike_sword2"), false, false);
    }
    if !VarModule::is_flag(agent.module_accessor, ike::status::flag::SPECIAL_N_AIR) {
        if VarModule::is_flag(agent.module_accessor, ike::status::flag::SPECIAL_N_RANGED_ERUPTION) {
            if macros::is_excute(agent) {
                let eruption_pos = VarModule::get_float(agent.module_accessor, ike::status::float::SPECIAL_N_ERUPT_LOCATION);
                macros::EFFECT(agent, Hash40::new("ike_volcano_max"), Hash40::new("top"), 0, 0, eruption_pos, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
                macros::EFFECT(agent, Hash40::new("ike_volcano_add4"), Hash40::new("top"), 0, 0, eruption_pos, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
                macros::EFFECT(agent, Hash40::new("ike_volcano_flash3_g"), Hash40::new("top"), 0, 0, eruption_pos, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
                macros::EFFECT(agent, Hash40::new("ike_volcano_add2"), Hash40::new("top"), 0, 0, eruption_pos, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
            } 
        }
    }
}

unsafe extern "C" fn sound_specialnend(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_ike_special_n01"));
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        let count = VarModule::get_int(agent.module_accessor, ike::status::int::ERUPTION_COUNT);
        if count > vl::special_n::eruption_count_for_critical {
            macros::PLAY_SE(agent, Hash40::new("vc_ike_special_n02"));
        }
        else {
            macros::PLAY_SE(agent, Hash40::new("vc_ike_special_n01"));
        }
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ike_special_n07"));
    }
    frame(agent.lua_state_agent, 11.0);
    if !VarModule::is_flag(agent.module_accessor, ike::status::flag::SPECIAL_N_AIR) {
        if VarModule::is_flag(agent.module_accessor, ike::status::flag::SPECIAL_N_RANGED_ERUPTION) {
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("se_ike_special_n08"));
            }
        }
        else {
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("se_ike_special_n09"));
            }
        }
    }
    wait(agent.lua_state_agent, 10.0);
    if !VarModule::is_flag(agent.module_accessor, ike::status::flag::SPECIAL_N_AIR) {
        if VarModule::is_flag(agent.module_accessor, ike::status::flag::SPECIAL_N_RANGED_ERUPTION) {
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("se_ike_special_n10"));
            } 
        }
    }
    frame(agent.lua_state_agent, 48.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ike_appeal_pullout"));
    }
}

unsafe extern "C" fn expression_specialnend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
        macros::AREA_WIND_2ND_arg10(agent, 0, 3, 110, 300, 1, 0, 12, 30, 30, 40);
    }
    frame(agent.lua_state_agent, 11.0);
    if !VarModule::is_flag(agent.module_accessor, ike::status::flag::SPECIAL_N_AIR) {
        if VarModule::is_flag(agent.module_accessor, ike::status::flag::SPECIAL_N_RANGED_ERUPTION) {
            if macros::is_excute(agent) {
                macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
                macros::RUMBLE_HIT(agent, Hash40::new("rbkind_explosion"), 0);
                ControlModule::set_rumble(
                    agent.module_accessor,
                    Hash40::new("rbkind_explosion"),
                    0,
                    false,
                    *BATTLE_OBJECT_ID_INVALID as u32
                );
            }
        }
        else {
            if macros::is_excute(agent) {
                macros::QUAKE(agent, *CAMERA_QUAKE_KIND_L);
                macros::RUMBLE_HIT(agent, Hash40::new("rbkind_explosionl"), 0);
                ControlModule::set_rumble(
                    agent.module_accessor,
                    Hash40::new("rbkind_explosionl"),
                    0,
                    false,
                    *BATTLE_OBJECT_ID_INVALID as u32
                );
            }
        }
    }
    wait(agent.lua_state_agent, 10.0);
    if !VarModule::is_flag(agent.module_accessor, ike::status::flag::SPECIAL_N_AIR) {
        if VarModule::is_flag(agent.module_accessor, ike::status::flag::SPECIAL_N_RANGED_ERUPTION) {
            if macros::is_excute(agent) {
                macros::QUAKE(agent, *CAMERA_QUAKE_KIND_L);
                macros::RUMBLE_HIT(agent, Hash40::new("rbkind_explosionl"), 0);
                ControlModule::set_rumble(
                    agent.module_accessor,
                    Hash40::new("rbkind_explosionl"),
                    0,
                    false,
                    *BATTLE_OBJECT_ID_INVALID as u32
                );
            }
        }
    }
}

unsafe extern "C" fn ike_special_n_end_ray_check(agent: &mut L2CAgentBase) {
    let count = VarModule::get_int(agent.module_accessor, ike::status::int::ERUPTION_COUNT);
    let mut counter = 0;
    let mut x_distance = vl::special_n::ray_check_x_offset;
    for x in 1..=count {
        counter = x;
        x_distance += vl::special_n::eruption_distance_add;
        let pos = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};
        ModelModule::joint_global_position_with_offset(
            agent.module_accessor,
            Hash40::new("top"),
            &Vector3f{x: 0.0, y: vl::special_n::ray_check_y_offset, z: x_distance},
            pos,
            true
        );
        if GroundModule::ray_check(
            agent.module_accessor,
            &Vector2f{x: pos.x, y: pos.y},
            &Vector2f{x: 0.0, y: vl::special_n::ray_check_y_check},
            true
        ) != 1 {
            counter = x - 1;
            break;
        }
    }
    if counter > 0 {
        VarModule::on_flag(agent.module_accessor, ike::status::flag::SPECIAL_N_RANGED_ERUPTION);
        let eruption_pos = vl::special_n::ray_check_x_offset + (counter as f32 * vl::special_n::eruption_distance_add);
        VarModule::set_float(agent.module_accessor, ike::status::float::SPECIAL_N_ERUPT_LOCATION, eruption_pos);
    }
}

unsafe extern "C" fn ike_specialairnend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        KineticModule::set_consider_ground_friction(agent.module_accessor, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, ike::status::flag::SPECIAL_N_AIR);
        let count = VarModule::get_int(agent.module_accessor, ike::status::int::ERUPTION_COUNT);
        let damage_add = count as f32 * vl::special_n::eruption_count_damage_add;
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0 + damage_add, 275, 70, 0, 45, 5.0, 0.0, 7.0, 8.6, Some(0.0), Some(-5.0), Some(8.6), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 7.0);
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    MiscModule::calc_motion_rate_from_cancel_frame(agent, 24.0, -15.0);
}

unsafe extern "C" fn effect_specialairnend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("ike_sword2"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 14.5, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 11.0);
    wait(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ike_sword2"), false, false);
    }
}

unsafe extern "C" fn sound_specialairnend(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_ike_special_n01"));
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ike_special_n01"));
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ike_special_n07"));
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_05"));
    }
}

unsafe extern "C" fn expression_specialairnend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
        macros::AREA_WIND_2ND_arg10(agent, 0, 3, 110, 300, 1, 0, 12, 30, 30, 40);
    }
    frame(agent.lua_state_agent, 11.0);
    if VarModule::is_flag(agent.module_accessor, ike::status::flag::SPECIAL_N_AIR) {
        if macros::is_excute(agent) {
            macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
            ControlModule::set_rumble(
                agent.module_accessor,
                Hash40::new("rbkind_slashl"),
                0,
                false,
                *BATTLE_OBJECT_ID_INVALID as u32
            );
        }
    }
}

// Special S

unsafe extern "C" fn sound_specialsdash(agent: &mut L2CAgentBase) {
    // if macros::is_excute(agent) {
    //     macros::PLAY_SE(agent, Hash40::new("vc_ike_special_s01"));
    // }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ike_special_s02"));
    }
}

unsafe extern "C" fn ike_specialsend(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    MiscModule::calc_motion_rate_from_cancel_frame(agent, 1.0, -8.0);
    let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(agent.module_accessor, Hash40::new("special_s_end"), true);
    frame(agent.lua_state_agent, cancel_frame);
    macros::FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn ike_specialsattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 60, 88, 0, 70, 6.5, 0.0, 8.4, 14.8, Some(0.0), Some(8.4), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_ATTACK_END);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        JostleModule::set_status(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn effect_specialsattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.5);
        macros::EFFECT_FOLLOW(agent, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        let eff = if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
            Hash40::new("ike_iaigiri_attack")
        }
        else {
            Hash40::new("ike_iaigiri_attack_r")
        };
        macros::EFFECT_FOLLOW(agent, eff, Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        let eff = if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
            Hash40::new("ike_iaigiri_attack")
        }
        else {
            Hash40::new("ike_iaigiri_attack_r")
        };
        macros::EFFECT_DETACH_KIND(agent, eff, -1);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ike_sword"), true, true);
    }
}

unsafe extern "C" fn sound_specialsattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ike_special_s01"));
        macros::PLAY_SE(agent, Hash40::new("se_ike_special_s02"));
        macros::PLAY_SE(agent, Hash40::new("se_ike_special_s03"));
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_specialnend", ike_specialnend);
    agent.acmd("effect_specialnend", effect_specialnend);
    agent.acmd("sound_specialnend", sound_specialnend);
    agent.acmd("expression_specialnend", expression_specialnend);

    agent.acmd("game_specialairnend", ike_specialairnend);
    agent.acmd("effect_specialairnend", effect_specialairnend);
    agent.acmd("sound_specialairnend", sound_specialairnend);
    agent.acmd("expression_specialairnend", expression_specialairnend);

    agent.acmd("sound_specialsdash", sound_specialsdash);

    agent.acmd("game_specialsend", ike_specialsend);

    agent.acmd("game_specialsattack", ike_specialsattack);
    agent.acmd("effect_specialsattack", effect_specialsattack);
    agent.acmd("sound_specialsattack", sound_specialsattack);

    agent.acmd("game_specialairsattack", ike_specialsattack);
    agent.acmd("effect_specialairsattack", effect_specialsattack);
    agent.acmd("sound_specialairsattack", sound_specialsattack);
}