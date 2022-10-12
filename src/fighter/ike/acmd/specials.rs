use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::*,
        app::{lua_bind::*, sv_animcmd::*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    custom_var::*,
    wubor_utils::{wua_bind::*, vars::*}
};

#[acmd_script( agent = "ike", script = "game_specialnend", category = ACMD_GAME, low_priority )]
unsafe fn ike_specialnend(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        KineticModule::set_consider_ground_friction(fighter.module_accessor, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        let count = VarModule::get_int(fighter.battle_object, ike::status::int::ERUPTION_COUNT);
        let eruptions = ike_special_n_end_ray_check(fighter, count);
        if eruptions > 0 {
            VarModule::on_flag(fighter.battle_object, ike::status::flag::SPECIAL_N_RANGED_ERUPTION);
            let eruption_pos = 10.0 + (eruptions as f32 * 12.0);
            VarModule::set_float(fighter.battle_object, ike::status::float::SPECIAL_N_ERUPT_LOCATION, eruption_pos);
        }
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
            let damage_add = count as f32 * 2.0;
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0 + damage_add, 80, 70, 0, 60, 10.0, 0.0, 10.0, 8.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0 + damage_add, 80, 70, 0, 60, 8.0, 0.0, 20.0, 8.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0 + damage_add, 80, 70, 0, 60, 4.0, 0.0, 28.0, 8.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
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
                let damage_add = count as f32 * 2.0;
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0 + damage_add, 80, 70, 0, 60, 10.0, 0.0, 10.0, eruption_pos - 1.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0 + damage_add, 80, 70, 0, 60, 8.0, 0.0, 20.0, eruption_pos - 1.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
                macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0 + damage_add, 80, 70, 0, 60, 4.0, 0.0, 28.0, eruption_pos - 1.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
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
        macros::PLAY_SE(fighter, Hash40::new("se_ike_special_n02"));
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

unsafe extern "C" fn ike_special_n_end_ray_check(fighter: &mut L2CAgentBase, max_count: i32) -> i32 {
    let mut counter = 0;
    let mut x_distance = 10.0;
    for x in 0..max_count {
        let pos = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};
        ModelModule::joint_global_position_with_offset(
            fighter.module_accessor,
            Hash40::new("top"),
            &Vector3f{x: 0.0, y: 1.0, z: x_distance},
            pos,
            true
        );
        if GroundModule::ray_check(
            fighter.module_accessor,
            &Vector2f{x: pos.x, y: pos.y},
            &Vector2f{x: 0.0, y: -2.0},
            true
        ) != 1 {
            counter = x - 1;
            break;
        }
        else {
            counter = x;
            x_distance += 12.0;
        }
    }
    counter
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
        let damage_add = count as f32 * 2.0;
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
    // if VarModule::is_flag(fighter.battle_object, ike::status::flag::SPECIAL_N_AIR) {
    //     if macros::is_excute(fighter) {
    //         macros::EFFECT(fighter, Hash40::new("ike_volcano_ground"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    //         if !VarModule::is_flag(fighter.battle_object, ike::status::flag::SPECIAL_N_RANGED_ERUPTION) {
    //             macros::EFFECT(fighter, Hash40::new("ike_volcano"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    //             macros::EFFECT(fighter, Hash40::new("ike_volcano_add4"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    //             macros::EFFECT(fighter, Hash40::new("ike_volcano_flash3_g"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    //             macros::EFFECT(fighter, Hash40::new("ike_volcano_add2"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    //         } 
    //         else {
    //             macros::EFFECT(fighter, Hash40::new("ike_volcano_max"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    //             macros::EFFECT(fighter, Hash40::new("ike_volcano_flash_g"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    //         }
    //     }
    // }
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
        macros::PLAY_SE(fighter, Hash40::new("se_ike_special_n02"));
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

pub fn install() {
    install_acmd_scripts!(
        ike_specialnend, ike_specialnend_eff, ike_specialnend_snd, ike_specialnend_exp,
        ike_specialairnend, ike_specialairnend_eff, ike_specialairnend_snd, ike_specialairnend_exp
    );
}
