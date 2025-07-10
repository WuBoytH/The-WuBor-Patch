use super::*;
use super::super::vl;

// Special N

unsafe extern "C" fn game_specialnend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        KineticModule::set_consider_ground_friction(agent.module_accessor, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        game_special_n_end_ray_check(agent);
    }
    if VarModule::is_flag(agent.module_accessor, vars::ike::status::flag::SPECIAL_N_RANGED_ERUPTION) {
        if macros::is_excute(agent) {
            let eruption_pos = VarModule::get_float(agent.module_accessor, vars::ike::status::float::SPECIAL_N_ERUPT_LOCATION);
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
            let count = VarModule::get_int(agent.module_accessor, vars::ike::status::int::ERUPTION_COUNT);
            if count > vl::special_n::eruption_count_for_critical {
                VarModule::on_flag(agent.module_accessor, vars::ike::status::flag::SPECIAL_N_ENABLE_CRITICAL);
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
    if !VarModule::is_flag(agent.module_accessor, vars::ike::status::flag::SPECIAL_N_AIR) {
        if VarModule::is_flag(agent.module_accessor, vars::ike::status::flag::SPECIAL_N_RANGED_ERUPTION) {
            if macros::is_excute(agent) {
                let eruption_pos = VarModule::get_float(agent.module_accessor, vars::ike::status::float::SPECIAL_N_ERUPT_LOCATION);
                let count = VarModule::get_int(agent.module_accessor, vars::ike::status::int::ERUPTION_COUNT);
                if count > vl::special_n::eruption_count_for_critical {
                    VarModule::on_flag(agent.module_accessor, vars::ike::status::flag::SPECIAL_N_ENABLE_CRITICAL);
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
    if !VarModule::is_flag(agent.module_accessor, vars::ike::status::flag::SPECIAL_N_AIR) {
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("ike_sword2"), false, false);
            macros::EFFECT(agent, Hash40::new("ike_volcano_ground"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
            if !VarModule::is_flag(agent.module_accessor, vars::ike::status::flag::SPECIAL_N_RANGED_ERUPTION) {
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
    if !VarModule::is_flag(agent.module_accessor, vars::ike::status::flag::SPECIAL_N_AIR) {
        if VarModule::is_flag(agent.module_accessor, vars::ike::status::flag::SPECIAL_N_RANGED_ERUPTION) {
            if macros::is_excute(agent) {
                let eruption_pos = VarModule::get_float(agent.module_accessor, vars::ike::status::float::SPECIAL_N_ERUPT_LOCATION);
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
        let count = VarModule::get_int(agent.module_accessor, vars::ike::status::int::ERUPTION_COUNT);
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
    if !VarModule::is_flag(agent.module_accessor, vars::ike::status::flag::SPECIAL_N_AIR) {
        if VarModule::is_flag(agent.module_accessor, vars::ike::status::flag::SPECIAL_N_RANGED_ERUPTION) {
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
    if !VarModule::is_flag(agent.module_accessor, vars::ike::status::flag::SPECIAL_N_AIR) {
        if VarModule::is_flag(agent.module_accessor, vars::ike::status::flag::SPECIAL_N_RANGED_ERUPTION) {
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
    if !VarModule::is_flag(agent.module_accessor, vars::ike::status::flag::SPECIAL_N_AIR) {
        if VarModule::is_flag(agent.module_accessor, vars::ike::status::flag::SPECIAL_N_RANGED_ERUPTION) {
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
    if !VarModule::is_flag(agent.module_accessor, vars::ike::status::flag::SPECIAL_N_AIR) {
        if VarModule::is_flag(agent.module_accessor, vars::ike::status::flag::SPECIAL_N_RANGED_ERUPTION) {
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

unsafe extern "C" fn game_special_n_end_ray_check(agent: &mut L2CAgentBase) {
    let count = VarModule::get_int(agent.module_accessor, vars::ike::status::int::ERUPTION_COUNT);
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
        VarModule::on_flag(agent.module_accessor, vars::ike::status::flag::SPECIAL_N_RANGED_ERUPTION);
        let eruption_pos = vl::special_n::ray_check_x_offset + (counter as f32 * vl::special_n::eruption_distance_add);
        VarModule::set_float(agent.module_accessor, vars::ike::status::float::SPECIAL_N_ERUPT_LOCATION, eruption_pos);
    }
}

unsafe extern "C" fn game_specialairnend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        KineticModule::set_consider_ground_friction(agent.module_accessor, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, vars::ike::status::flag::SPECIAL_N_AIR);
        let count = VarModule::get_int(agent.module_accessor, vars::ike::status::int::ERUPTION_COUNT);
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
    if VarModule::is_flag(agent.module_accessor, vars::ike::status::flag::SPECIAL_N_AIR) {
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

unsafe extern "C" fn game_specialsstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 10.0 / 15.0);
    frame(agent.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_CHARGE);
    }
}

unsafe extern "C" fn sound_specialsdash(agent: &mut L2CAgentBase) {
    // if macros::is_excute(agent) {
    //     macros::PLAY_SE(agent, Hash40::new("vc_ike_special_s01"));
    // }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ike_special_s02"));
    }
}

unsafe extern "C" fn game_specialsend(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    MiscModule::calc_motion_rate_from_cancel_frame(agent, 1.0, -8.0);
    let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(agent.module_accessor, Hash40::new("special_s_end"), true);
    frame(agent.lua_state_agent, cancel_frame);
    macros::FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialsattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
    }
    macros::FT_MOTION_RATE(agent, 1.0 / 4.0);
    frame(agent.lua_state_agent, 4.0);
    macros::FT_MOTION_RATE(agent, 1.0);
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

unsafe extern "C" fn game_specialhi2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_IKE_INSTANCE_WORK_ID_FLAG_SWORD_FINAL);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_IKE_GENERATE_ARTICLE_SWORD, false, -1);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 88, 100, 150, 0, 3.5, 0.0, 5.0, 7.0, Some(0.0), Some(5.0), Some(8.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 94, 100, 150, 0, 1.5, 0.0, 3.0, 15.0, Some(0.0), Some(7.0), Some(15.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 6.0, 96, 100, 150, 0, 3.5, 0.0, 5.0, 19.0, Some(0.0), Some(5.0), Some(7.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 6.0, 86, 100, 145, 0, 3.5, 0.0, 13.5, 7.0, Some(0.0), Some(13.5), Some(8.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 6.0, 94, 100, 145, 0, 1.5, 0.0, 11.5, 15.0, Some(0.0), Some(15.5), Some(15.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 5, 0, Hash40::new("top"), 6.0, 96, 100, 140, 0, 3.5, 0.0, 13.5, 19.0, Some(0.0), Some(13.5), Some(7.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_HI_FLAG_CONTROL);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_HI_FLAG_TRANS_JUMP);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_HI_FLAG_CONTROL);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        WorkModule::set_float(agent.module_accessor, 4.0, *FIGHTER_IKE_STATUS_SPECIAL_HI_WORK_FLOAT_SLIDEGAP_RECOVER_FRAME_INIT);
        WorkModule::set_float(agent.module_accessor, 4.0, *FIGHTER_IKE_STATUS_SPECIAL_HI_WORK_FLOAT_SLIDEGAP_RECOVER_FRAME);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 367, 100, 50, 0, 9.0, 0.0, 16.0, 14.0, Some(0.0), Some(8.0), Some(14.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 44.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 47.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_IKE_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn game_specialhi3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        camera!(agent, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, -20);
        KineticModule::clear_speed_all(agent.module_accessor);
        macros::ADD_SPEED_NO_LIMIT(agent, 0, -6);
        macros::ATTACK(agent, 0, 0, Hash40::new("sword"), 3.0, 70, 30, 0, 120, 6.5, 0.0, 6.8, -1.0, Some(0.0), Some(6.8), Some(-12.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword"), 3.0, 274, 100, 165, 0, 4.8, 0.0, 6.8, -1.0, Some(0.0), Some(6.8), Some(-13.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_finish_camera(agent.module_accessor, 1, true, false);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword"), 3.0, 70, 50, 0, 0, 4.8, 0.0, 6.8, -1.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword"), 3.0, 274, 100, 150, 0, 4.8, 0.0, 6.8, -1.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialnend", game_specialnend, Priority::Low);
    agent.acmd("effect_specialnend", effect_specialnend, Priority::Low);
    agent.acmd("sound_specialnend", sound_specialnend, Priority::Low);
    agent.acmd("expression_specialnend", expression_specialnend, Priority::Low);

    agent.acmd("game_specialairnend", game_specialairnend, Priority::Low);
    agent.acmd("effect_specialairnend", effect_specialairnend, Priority::Low);
    agent.acmd("sound_specialairnend", sound_specialairnend, Priority::Low);
    agent.acmd("expression_specialairnend", expression_specialairnend, Priority::Low);

    agent.acmd("game_specialsstart", game_specialsstart, Priority::Low);

    agent.acmd("game_specialairsstart", game_specialsstart, Priority::Low);

    agent.acmd("sound_specialsdash", sound_specialsdash, Priority::Low);

    agent.acmd("game_specialsend", game_specialsend, Priority::Low);

    agent.acmd("game_specialsattack", game_specialsattack, Priority::Low);
    agent.acmd("effect_specialsattack", effect_specialsattack, Priority::Low);
    agent.acmd("sound_specialsattack", sound_specialsattack, Priority::Low);

    agent.acmd("game_specialairsattack", game_specialsattack, Priority::Low);
    agent.acmd("effect_specialairsattack", effect_specialsattack, Priority::Low);
    agent.acmd("sound_specialairsattack", sound_specialsattack, Priority::Low);

    agent.acmd("game_specialhi2", game_specialhi2, Priority::Low);

    agent.acmd("game_specialairhi2", game_specialhi2, Priority::Low);

    agent.acmd("game_specialhi3", game_specialhi3, Priority::Low);
}