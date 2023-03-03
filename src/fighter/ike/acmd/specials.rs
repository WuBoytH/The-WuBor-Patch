use crate::imports::acmd_imports::*;
use super::super::vl;

// Special N

#[acmd_script( agent = "ike", script = "game_specialnend", category = ACMD_GAME, low_priority )]
unsafe fn ike_specialnend(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        KineticModule::set_consider_ground_friction(fighter.module_accessor, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        ike_special_n_end_ray_check(fighter);
    }
    if VarModule::is_flag(fighter.battle_object, ike::status::flag::SPECIAL_N_RANGED_ERUPTION) {
        if macros::is_excute(fighter) {
            let eruption_pos = VarModule::get_float(fighter.battle_object, ike::status::float::SPECIAL_N_ERUPT_LOCATION);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 368, 90, 10, 50, 10.0, 0.0, 10.0, 8.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 368, 90, 10, 50, 8.0, 0.0, 20.0, 8.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 368, 90, 10, 50, 4.0, 0.0, 28.0, 8.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            for x in 0..3 {
                AttackModule::set_vec_target_pos(
                    fighter.module_accessor,
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
        if macros::is_excute(fighter) {
            let count = VarModule::get_int(fighter.battle_object, ike::status::int::ERUPTION_COUNT);
            if count > vl::special_n::eruption_count_for_critical {
                VarModule::on_flag(fighter.battle_object, ike::status::flag::SPECIAL_N_ENABLE_CRITICAL);
            }
            let damage_add = count as f32 * vl::special_n::eruption_count_damage_add;
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0 + damage_add, 80, 70, 0, 60, 12.0, 0.0, 10.0, 8.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0 + damage_add, 80, 70, 0, 60, 10.0, 0.0, 28.0, 8.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0 + damage_add, 80, 70, 0, 60, 4.0, 0.0, 38.0, 8.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        }
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    wait(fighter.lua_state_agent, 7.0);
    if !VarModule::is_flag(fighter.battle_object, ike::status::flag::SPECIAL_N_AIR) {
        if VarModule::is_flag(fighter.battle_object, ike::status::flag::SPECIAL_N_RANGED_ERUPTION) {
            if macros::is_excute(fighter) {
                let eruption_pos = VarModule::get_float(fighter.battle_object, ike::status::float::SPECIAL_N_ERUPT_LOCATION);
                let count = VarModule::get_int(fighter.battle_object, ike::status::int::ERUPTION_COUNT);
                if count > vl::special_n::eruption_count_for_critical {
                    VarModule::on_flag(fighter.battle_object, ike::status::flag::SPECIAL_N_ENABLE_CRITICAL);
                }
                let damage_add = count as f32 * vl::special_n::eruption_count_damage_add;
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0 + damage_add, 80, 70, 0, 60, 12.0, 0.0, 10.0, eruption_pos - 1.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0 + damage_add, 80, 70, 0, 60, 10.0, 0.0, 28.0, eruption_pos - 1.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
                macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0 + damage_add, 80, 70, 0, 60, 4.0, 0.0, 38.0, eruption_pos - 1.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            }
        }
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    MiscModule::calc_motion_rate_from_cancel_frame(fighter, 24.0, -15.0);
}

#[acmd_script( agent = "ike", script = "effect_specialnend", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_specialnend_eff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_sword2"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 14.5, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 11.0);
    if !VarModule::is_flag(fighter.battle_object, ike::status::flag::SPECIAL_N_AIR) {
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("ike_sword2"), false, false);
            macros::EFFECT(fighter, Hash40::new("ike_volcano_ground"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
            if !VarModule::is_flag(fighter.battle_object, ike::status::flag::SPECIAL_N_RANGED_ERUPTION) {
                macros::EFFECT(fighter, Hash40::new("ike_volcano_max"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
                macros::EFFECT(fighter, Hash40::new("ike_volcano_add4"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
                macros::EFFECT(fighter, Hash40::new("ike_volcano_flash3_g"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
                macros::EFFECT(fighter, Hash40::new("ike_volcano_add2"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
            } 
            else {
                macros::EFFECT(fighter, Hash40::new("ike_volcano"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
                macros::EFFECT(fighter, Hash40::new("ike_volcano_flash_g"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
            }
        }
    }
    wait(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("ike_sword2"), false, false);
    }
    if !VarModule::is_flag(fighter.battle_object, ike::status::flag::SPECIAL_N_AIR) {
        if VarModule::is_flag(fighter.battle_object, ike::status::flag::SPECIAL_N_RANGED_ERUPTION) {
            if macros::is_excute(fighter) {
                let eruption_pos = VarModule::get_float(fighter.battle_object, ike::status::float::SPECIAL_N_ERUPT_LOCATION);
                macros::EFFECT(fighter, Hash40::new("ike_volcano_max"), Hash40::new("top"), 0, 0, eruption_pos, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
                macros::EFFECT(fighter, Hash40::new("ike_volcano_add4"), Hash40::new("top"), 0, 0, eruption_pos, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
                macros::EFFECT(fighter, Hash40::new("ike_volcano_flash3_g"), Hash40::new("top"), 0, 0, eruption_pos, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
                macros::EFFECT(fighter, Hash40::new("ike_volcano_add2"), Hash40::new("top"), 0, 0, eruption_pos, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
            } 
        }
    }
}

#[acmd_script( agent = "ike", script = "sound_specialnend", category = ACMD_SOUND, low_priority )]
unsafe fn ike_specialnend_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_ike_special_n01"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        let count = VarModule::get_int(fighter.battle_object, ike::status::int::ERUPTION_COUNT);
        if count > vl::special_n::eruption_count_for_critical {
            macros::PLAY_SE(fighter, Hash40::new("vc_ike_special_n02"));
        }
        else {
            macros::PLAY_SE(fighter, Hash40::new("vc_ike_special_n01"));
        }
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ike_special_n07"));
    }
    frame(fighter.lua_state_agent, 11.0);
    if !VarModule::is_flag(fighter.battle_object, ike::status::flag::SPECIAL_N_AIR) {
        if VarModule::is_flag(fighter.battle_object, ike::status::flag::SPECIAL_N_RANGED_ERUPTION) {
            if macros::is_excute(fighter) {
                macros::PLAY_SE(fighter, Hash40::new("se_ike_special_n08"));
            }
        }
        else {
            if macros::is_excute(fighter) {
                macros::PLAY_SE(fighter, Hash40::new("se_ike_special_n09"));
            }
        }
    }
    wait(fighter.lua_state_agent, 10.0);
    if !VarModule::is_flag(fighter.battle_object, ike::status::flag::SPECIAL_N_AIR) {
        if VarModule::is_flag(fighter.battle_object, ike::status::flag::SPECIAL_N_RANGED_ERUPTION) {
            if macros::is_excute(fighter) {
                macros::PLAY_SE(fighter, Hash40::new("se_ike_special_n10"));
            } 
        }
    }
    frame(fighter.lua_state_agent, 48.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ike_appeal_pullout"));
    }
}

#[acmd_script( agent = "ike", script = "expression_specialnend", category = ACMD_EXPRESSION, low_priority )]
unsafe fn ike_specialnend_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
        macros::AREA_WIND_2ND_arg10(fighter, 0, 3, 110, 300, 1, 0, 12, 30, 30, 40);
    }
    frame(fighter.lua_state_agent, 11.0);
    if !VarModule::is_flag(fighter.battle_object, ike::status::flag::SPECIAL_N_AIR) {
        if VarModule::is_flag(fighter.battle_object, ike::status::flag::SPECIAL_N_RANGED_ERUPTION) {
            if macros::is_excute(fighter) {
                macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
                macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_explosion"), 0);
                ControlModule::set_rumble(
                    fighter.module_accessor,
                    Hash40::new("rbkind_explosion"),
                    0,
                    false,
                    *BATTLE_OBJECT_ID_INVALID as u32
                );
            }
        }
        else {
            if macros::is_excute(fighter) {
                macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
                macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_explosionl"), 0);
                ControlModule::set_rumble(
                    fighter.module_accessor,
                    Hash40::new("rbkind_explosionl"),
                    0,
                    false,
                    *BATTLE_OBJECT_ID_INVALID as u32
                );
            }
        }
    }
    wait(fighter.lua_state_agent, 10.0);
    if !VarModule::is_flag(fighter.battle_object, ike::status::flag::SPECIAL_N_AIR) {
        if VarModule::is_flag(fighter.battle_object, ike::status::flag::SPECIAL_N_RANGED_ERUPTION) {
            if macros::is_excute(fighter) {
                macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
                macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_explosionl"), 0);
                ControlModule::set_rumble(
                    fighter.module_accessor,
                    Hash40::new("rbkind_explosionl"),
                    0,
                    false,
                    *BATTLE_OBJECT_ID_INVALID as u32
                );
            }
        }
    }
}

unsafe extern "C" fn ike_special_n_end_ray_check(fighter: &mut L2CAgentBase) {
    let count = VarModule::get_int(fighter.battle_object, ike::status::int::ERUPTION_COUNT);
    let mut counter = 0;
    let mut x_distance = vl::special_n::ray_check_x_offset;
    for x in 1..=count {
        counter = x;
        x_distance += vl::special_n::eruption_distance_add;
        let pos = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};
        ModelModule::joint_global_position_with_offset(
            fighter.module_accessor,
            Hash40::new("top"),
            &Vector3f{x: 0.0, y: vl::special_n::ray_check_y_offset, z: x_distance},
            pos,
            true
        );
        if GroundModule::ray_check(
            fighter.module_accessor,
            &Vector2f{x: pos.x, y: pos.y},
            &Vector2f{x: 0.0, y: vl::special_n::ray_check_y_check},
            true
        ) != 1 {
            counter = x - 1;
            break;
        }
    }
    if counter > 0 {
        VarModule::on_flag(fighter.battle_object, ike::status::flag::SPECIAL_N_RANGED_ERUPTION);
        let eruption_pos = vl::special_n::ray_check_x_offset + (counter as f32 * vl::special_n::eruption_distance_add);
        VarModule::set_float(fighter.battle_object, ike::status::float::SPECIAL_N_ERUPT_LOCATION, eruption_pos);
    }
}

#[acmd_script( agent = "ike", script = "game_specialairnend", category = ACMD_GAME, low_priority )]
unsafe fn ike_specialairnend(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        KineticModule::set_consider_ground_friction(fighter.module_accessor, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, ike::status::flag::SPECIAL_N_AIR);
        let count = VarModule::get_int(fighter.battle_object, ike::status::int::ERUPTION_COUNT);
        let damage_add = count as f32 * vl::special_n::eruption_count_damage_add;
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0 + damage_add, 275, 70, 0, 45, 5.0, 0.0, 7.0, 8.6, Some(0.0), Some(-5.0), Some(8.6), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    wait(fighter.lua_state_agent, 7.0);
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    MiscModule::calc_motion_rate_from_cancel_frame(fighter, 24.0, -15.0);
}

#[acmd_script( agent = "ike", script = "effect_specialairnend", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_specialairnend_eff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_sword2"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 14.5, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 11.0);
    wait(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("ike_sword2"), false, false);
    }
}

#[acmd_script( agent = "ike", script = "sound_specialairnend", category = ACMD_SOUND, low_priority )]
unsafe fn ike_specialairnend_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_ike_special_n01"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_ike_special_n01"));
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ike_special_n07"));
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_swing_05"));
    }
}

#[acmd_script( agent = "ike", script = "expression_specialairnend", category = ACMD_EXPRESSION, low_priority )]
unsafe fn ike_specialairnend_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
        macros::AREA_WIND_2ND_arg10(fighter, 0, 3, 110, 300, 1, 0, 12, 30, 30, 40);
    }
    frame(fighter.lua_state_agent, 11.0);
    if VarModule::is_flag(fighter.battle_object, ike::status::flag::SPECIAL_N_AIR) {
        if macros::is_excute(fighter) {
            macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_slashl"), 0);
            ControlModule::set_rumble(
                fighter.module_accessor,
                Hash40::new("rbkind_slashl"),
                0,
                false,
                *BATTLE_OBJECT_ID_INVALID as u32
            );
        }
    }
}

// Special S

#[acmd_script( agent = "ike", scripts = [ "sound_specialsdash", "sound_specialairsdash" ], category = ACMD_SOUND, low_priority )]
unsafe fn ike_specialsdash_snd(fighter: &mut L2CAgentBase) {
    // if macros::is_excute(fighter) {
    //     macros::PLAY_SE(fighter, Hash40::new("vc_ike_special_s01"));
    // }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ike_special_s02"));
    }
}

#[acmd_script( agent = "ike", script = "game_specialsend", category = ACMD_GAME, low_priority )]
unsafe fn ike_specialsend(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    MiscModule::calc_motion_rate_from_cancel_frame(fighter, 1.0, -8.0);
    let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new("special_s_end"), true);
    frame(fighter.lua_state_agent, cancel_frame);
    macros::FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "ike", scripts = [ "game_specialsattack", "game_specialairsattack" ], category = ACMD_GAME, low_priority )]
unsafe fn ike_specialsattack(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 60, 88, 0, 70, 6.5, 0.0, 8.4, 14.8, Some(0.0), Some(8.4), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_ATTACK_END);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        JostleModule::set_status(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "ike", scripts = [ "effect_specialsattack", "effect_specialairsattack" ], category = ACMD_EFFECT, low_priority )]
unsafe fn ike_specialsattack_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.5);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        let eff = if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
            Hash40::new("ike_iaigiri_attack")
        }
        else {
            Hash40::new("ike_iaigiri_attack_r")
        };
        macros::EFFECT_FOLLOW(fighter, eff, Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        let eff = if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
            Hash40::new("ike_iaigiri_attack")
        }
        else {
            Hash40::new("ike_iaigiri_attack_r")
        };
        macros::EFFECT_DETACH_KIND(fighter, eff, -1);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("ike_sword"), true, true);
    }
}

#[acmd_script( agent = "ike", scripts = [ "sound_specialsattack", "sound_specialairsattack" ], category = ACMD_SOUND, low_priority )]
unsafe fn ike_specialsattack_snd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_ike_special_s01"));
        macros::PLAY_SE(fighter, Hash40::new("se_ike_special_s02"));
        macros::PLAY_SE(fighter, Hash40::new("se_ike_special_s03"));
    }
}

pub fn install() {
    install_acmd_scripts!(
        ike_specialnend, ike_specialnend_eff, ike_specialnend_snd, ike_specialnend_exp,
        ike_specialairnend, ike_specialairnend_eff, ike_specialairnend_snd, ike_specialairnend_exp,

        ike_specialsdash_snd,
        ike_specialsend,
        ike_specialsattack, ike_specialsattack_eff, ike_specialsattack_snd
    );
}
