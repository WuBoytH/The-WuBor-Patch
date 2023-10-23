use crate::imports::acmd_imports::*;
use crate::fighter::ike::vl;

#[acmd_script( agent = "kirby", script = "effect_ikespecialnend", category = ACMD_EFFECT, low_priority )]
unsafe fn kirby_ikespecialnend_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 14.5, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 11.0);
    if !VarModule::is_flag(agent.battle_object, ike::status::flag::SPECIAL_N_AIR) {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("ike_volcano_ground"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
            if !VarModule::is_flag(agent.battle_object, ike::status::flag::SPECIAL_N_RANGED_ERUPTION) {
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
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ike_volcano_hold"), false, false);
    }
    if !VarModule::is_flag(agent.battle_object, ike::status::flag::SPECIAL_N_AIR) {
        if VarModule::is_flag(agent.battle_object, ike::status::flag::SPECIAL_N_RANGED_ERUPTION) {
            if macros::is_excute(agent) {
                let eruption_pos = VarModule::get_float(agent.battle_object, ike::status::float::SPECIAL_N_ERUPT_LOCATION);
                macros::EFFECT(agent, Hash40::new("ike_volcano_max"), Hash40::new("top"), 0, 0, eruption_pos, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
                macros::EFFECT(agent, Hash40::new("ike_volcano_add4"), Hash40::new("top"), 0, 0, eruption_pos, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
                macros::EFFECT(agent, Hash40::new("ike_volcano_flash3_g"), Hash40::new("top"), 0, 0, eruption_pos, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
                macros::EFFECT(agent, Hash40::new("ike_volcano_add2"), Hash40::new("top"), 0, 0, eruption_pos, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
            } 
        }
    }
}

#[acmd_script( agent = "kirby", script = "sound_ikespecialnend", category = ACMD_SOUND, low_priority )]
unsafe fn kirby_ikespecialnend_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_ike_special_n01"));
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        let count = VarModule::get_int(agent.battle_object, ike::status::int::ERUPTION_COUNT);
        if count > vl::special_n::eruption_count_for_critical {
            macros::PLAY_SE(agent, Hash40::new("vc_kirby_copy_ike_02"));
        }
        else {
            macros::PLAY_SE(agent, Hash40::new("vc_kirby_copy_ike_01"));
        }
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ike_special_n07"));
    }
    frame(agent.lua_state_agent, 11.0);
    if !VarModule::is_flag(agent.battle_object, ike::status::flag::SPECIAL_N_AIR) {
        if VarModule::is_flag(agent.battle_object, ike::status::flag::SPECIAL_N_RANGED_ERUPTION) {
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
    if !VarModule::is_flag(agent.battle_object, ike::status::flag::SPECIAL_N_AIR) {
        if VarModule::is_flag(agent.battle_object, ike::status::flag::SPECIAL_N_RANGED_ERUPTION) {
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

#[acmd_script( agent = "kirby", script = "expression_ikespecialnend", category = ACMD_EXPRESSION, low_priority )]
unsafe fn kirby_ikespecialnend_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
        macros::AREA_WIND_2ND_arg10(agent, 0, 3, 110, 300, 1, 0, 12, 30, 30, 40);
    }
    frame(agent.lua_state_agent, 11.0);
    if !VarModule::is_flag(agent.battle_object, ike::status::flag::SPECIAL_N_AIR) {
        if VarModule::is_flag(agent.battle_object, ike::status::flag::SPECIAL_N_RANGED_ERUPTION) {
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
    if !VarModule::is_flag(agent.battle_object, ike::status::flag::SPECIAL_N_AIR) {
        if VarModule::is_flag(agent.battle_object, ike::status::flag::SPECIAL_N_RANGED_ERUPTION) {
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

#[acmd_script( agent = "kirby", script = "effect_ikespecialairnend", category = ACMD_EFFECT, low_priority )]
unsafe fn kirby_ikespecialairnend_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 14.5, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 11.0);
    wait(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ike_volcano_hold"), false, false);
    }
}

#[acmd_script( agent = "kirby", script = "sound_ikespecialairnend", category = ACMD_SOUND, low_priority )]
unsafe fn kirby_ikespecialairnend_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_ike_special_n01"));
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_kirby_copy_ike_01"));
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

#[acmd_script( agent = "kirby", script = "expression_ikespecialairnend", category = ACMD_EXPRESSION, low_priority )]
unsafe fn kirby_ikespecialairnend_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
        macros::AREA_WIND_2ND_arg10(agent, 0, 3, 110, 300, 1, 0, 12, 30, 30, 40);
    }
    frame(agent.lua_state_agent, 11.0);
    if VarModule::is_flag(agent.battle_object, ike::status::flag::SPECIAL_N_AIR) {
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

pub fn install() {
    install_acmd_scripts!(
        kirby_ikespecialnend_eff,
        kirby_ikespecialnend_snd,
        kirby_ikespecialnend_exp,

        kirby_ikespecialairnend_eff,
        kirby_ikespecialairnend_snd,
        kirby_ikespecialairnend_exp
    );
}
