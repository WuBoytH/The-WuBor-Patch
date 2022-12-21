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
    wubor_utils::vars::*,
    crate::fighter::ike::vl
};

#[acmd_script( agent = "kirby", script = "effect_ikespecialnend", category = ACMD_EFFECT )]
unsafe fn kirby_ikespecialnend_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 14.5, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 11.0);
    if !VarModule::is_flag(fighter.battle_object, ike::status::flag::SPECIAL_N_AIR) {
        if macros::is_excute(fighter) {
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
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("ike_volcano_hold"), false, false);
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

#[acmd_script( agent = "kirby", script = "sound_ikespecialnend", category = ACMD_SOUND )]
unsafe fn kirby_ikespecialnend_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_ike_special_n01"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        let count = VarModule::get_int(fighter.battle_object, ike::status::int::ERUPTION_COUNT);
        if count > vl::special_n::eruption_count_for_critical {
            macros::PLAY_SE(fighter, Hash40::new("vc_kirby_copy_ike_02"));
        }
        else {
            macros::PLAY_SE(fighter, Hash40::new("vc_kirby_copy_ike_01"));
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

#[acmd_script( agent = "kirby", script = "expression_ikespecialnend", category = ACMD_EXPRESSION )]
unsafe fn kirby_ikespecialnend_exp(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "kirby", script = "effect_ikespecialairnend", category = ACMD_EFFECT )]
unsafe fn kirby_ikespecialairnend_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 14.5, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 11.0);
    wait(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("ike_volcano_hold"), false, false);
    }
}

#[acmd_script( agent = "kirby", script = "sound_ikespecialairnend", category = ACMD_SOUND )]
unsafe fn kirby_ikespecialairnend_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_ike_special_n01"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_kirby_copy_ike_01"));
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

#[acmd_script( agent = "kirby", script = "expression_ikespecialairnend", category = ACMD_EXPRESSION )]
unsafe fn kirby_ikespecialairnend_exp(fighter: &mut L2CAgentBase) {
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
        kirby_ikespecialnend_eff, kirby_ikespecialnend_snd, kirby_ikespecialnend_exp,
        kirby_ikespecialairnend_eff, kirby_ikespecialairnend_snd, kirby_ikespecialairnend_exp
    );
}
