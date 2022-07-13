use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::Hash40,
        app::{lua_bind::*, sv_animcmd::*/*, **/},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    // super::vl::*,
    // wubor_utils::vars::*
};

// #[acmd_script( agent = "samus", scripts = [ "game_attacks3", "game_attacks3hi", "game_attacks3lw" ], category = ACMD_GAME, low_priority )]
// unsafe fn samus_attacks3(fighter: &mut L2CAgentBase) {
//     frame(fighter.lua_state_agent, 10.0);
//     if macros::is_excute(fighter) {
//         ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, false, -1);
//         ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
//     }
//     frame(fighter.lua_state_agent, 13.0);
//     if macros::is_excute(fighter) {
//         WorkModule::on_flag(fighter.module_accessor, FIGHTER_SAMUS_INSTANCE_WORK_ID_FLAG_BEAM_RAPID);
//     }
//     frame(fighter.lua_state_agent, 37.0);
//     if macros::is_excute(fighter) {
//         WorkModule::off_flag(fighter.module_accessor, FIGHTER_SAMUS_INSTANCE_WORK_ID_FLAG_BEAM_RAPID);
//     }
// }

// #[acmd_script( agent = "samus", script = "effect_attacks3", category = ACMD_EFFECT, low_priority )]
// unsafe fn samus_attacks3_eff(fighter: &mut L2CAgentBase) {
//     frame(fighter.lua_state_agent, 10.0);
//     if macros::is_excute(fighter) {
//         macros::EFFECT(fighter, Hash40::new("samus_cshot_shot"), Hash40::new("top"), 6, 6, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
//     }
//     frame(fighter.lua_state_agent, 11.0);
//     if macros::is_excute(fighter) {
//         macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
//         macros::FLASH(fighter, 1, 0.753, 1, 0.706);
//     }
//     frame(fighter.lua_state_agent, 12.0);
//     if macros::is_excute(fighter) {
//         fighter.clear_lua_stack();
//         lua_args!(fighter, 10, 0.314, 0.314, 0.314, 0);
//         sv_animcmd::FLASH_FRM(fighter.lua_state_agent);
//     }
//     frame(fighter.lua_state_agent, 20.0);
//     if macros::is_excute(fighter) {
//         macros::COL_NORMAL(fighter);
//     }
// }

// #[acmd_script( agent = "samus", script = "effect_attacks3hi", category = ACMD_EFFECT, low_priority )]
// unsafe fn samus_attacks3hi_eff(fighter: &mut L2CAgentBase) {
//     frame(fighter.lua_state_agent, 10.0);
//     if macros::is_excute(fighter) {
//         macros::EFFECT(fighter, Hash40::new("samus_cshot_shot"), Hash40::new("top"), 6, 10, 0, -15, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
//     }
//     frame(fighter.lua_state_agent, 11.0);
//     if macros::is_excute(fighter) {
//         macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
//         macros::FLASH(fighter, 1, 0.753, 1, 0.706);
//     }
//     frame(fighter.lua_state_agent, 12.0);
//     if macros::is_excute(fighter) {
//         fighter.clear_lua_stack();
//         lua_args!(fighter, 10, 0.314, 0.314, 0.314, 0);
//         sv_animcmd::FLASH_FRM(fighter.lua_state_agent);
//     }
//     frame(fighter.lua_state_agent, 20.0);
//     if macros::is_excute(fighter) {
//         macros::COL_NORMAL(fighter);
//     }
// }

// #[acmd_script( agent = "samus", script = "effect_attacks3lw", category = ACMD_EFFECT, low_priority )]
// unsafe fn samus_attacks3lw_eff(fighter: &mut L2CAgentBase) {
//     frame(fighter.lua_state_agent, 10.0);
//     if macros::is_excute(fighter) {
//         macros::EFFECT(fighter, Hash40::new("samus_cshot_shot"), Hash40::new("top"), 6, 5, 0, 15, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
//     }
//     frame(fighter.lua_state_agent, 11.0);
//     if macros::is_excute(fighter) {
//         macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
//         macros::FLASH(fighter, 1, 0.753, 1, 0.706);
//     }
//     frame(fighter.lua_state_agent, 12.0);
//     if macros::is_excute(fighter) {
//         fighter.clear_lua_stack();
//         lua_args!(fighter, 10, 0.314, 0.314, 0.314, 0);
//         sv_animcmd::FLASH_FRM(fighter.lua_state_agent);
//     }
//     frame(fighter.lua_state_agent, 20.0);
//     if macros::is_excute(fighter) {
//         macros::COL_NORMAL(fighter);
//     }
// }

// #[acmd_script( agent = "samus", scripts = [ "sound_attacks3", "sound_attacks3hi", "sound_attacks3lw" ], category = ACMD_SOUND, low_priority )]
// unsafe fn samus_attacks3_snd(_fighter: &mut L2CAgentBase) {
    
// }

// #[acmd_script( agent = "samus", scripts = [ "expression_attacks3", "expression_attacks3hi", "expression_attacks3lw" ], category = ACMD_EXPRESSION, low_priority )]
// unsafe fn samus_attacks3_exp(fighter: &mut L2CAgentBase) {
//     if macros::is_excute(fighter) {
//         slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
//     }
//     frame(fighter.lua_state_agent, 9.0);
//     if macros::is_excute(fighter) {
//         ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_beams"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
//     }
// }

#[acmd_script( agent = "samus", script = "game_attackhi3" , category = ACMD_GAME, low_priority )]
unsafe fn samus_attackhi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 366, 0, 70, 100, 6.0, 0.0, 20.0, 9.0, None, None, None, 0.8, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    macros::FT_MOTION_RATE(fighter, 0.75);
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 366, 0, 70, 60, 6.0, 0.0, 21.0, 8.0, None, None, None, 0.8, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 55, 92, 0, 50, 6.0, 0.0, 22.0, 6.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 34.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "samus", script = "effect_attackhi3" , category = ACMD_EFFECT, low_priority )]
unsafe fn samus_attackhi3_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("samus_atk_bomb"), Hash40::new("top"), 0.0, 21.0, 10.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_v"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, -3, -40, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("samus_atk_bomb"), Hash40::new("top"), 0.0, 22.0, 9.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, -3, -44, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("samus_atk_bomb"), Hash40::new("top"), 0.0, 23.0, 7.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, -3, -48, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "samus", script = "sound_attackhi3" , category = ACMD_SOUND, low_priority )]
unsafe fn samus_attackhi3_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samus_smash_h01"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samus_smash_h01"));
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samus_smash_h02"));
    }
}

#[acmd_script( agent = "samus", script = "expression_attackhi3" , category = ACMD_EXPRESSION, low_priority )]
unsafe fn samus_attackhi3_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_explosion"), 8);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohit_explosion"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_explosion"), 8);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohit_explosion"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_explosion"), 8);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohit_explosion"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    install_acmd_scripts!(
        // samus_attacks3, samus_attacks3_eff, samus_attacks3hi_eff, samus_attacks3lw_eff, samus_attacks3_snd, samus_attacks3_exp,
        samus_attackhi3, samus_attackhi3_eff, samus_attackhi3_snd, samus_attackhi3_exp
    );
}