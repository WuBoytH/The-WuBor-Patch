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

// #[acmd("samus", [ "game_attacks3", "game_attacks3hi", "game_attacks3lw" ])]
// unsafe fn samus_attacks3(agent: &mut L2CAgentBase) {
//     frame(agent.lua_state_agent, 10.0);
//     if macros::is_excute(agent) {
//         ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, false, -1);
//         ArticleModule::shoot_exist(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
//     }
//     frame(agent.lua_state_agent, 13.0);
//     if macros::is_excute(agent) {
//         WorkModule::on_flag(agent.module_accessor, FIGHTER_SAMUS_INSTANCE_WORK_ID_FLAG_BEAM_RAPID);
//     }
//     frame(agent.lua_state_agent, 37.0);
//     if macros::is_excute(agent) {
//         WorkModule::off_flag(agent.module_accessor, FIGHTER_SAMUS_INSTANCE_WORK_ID_FLAG_BEAM_RAPID);
//     }
// }

// #[acmd("samus", "effect_attacks3")]
// unsafe fn samus_attacks3_eff(agent: &mut L2CAgentBase) {
//     frame(agent.lua_state_agent, 10.0);
//     if macros::is_excute(agent) {
//         macros::EFFECT(agent, Hash40::new("samus_cshot_shot"), Hash40::new("top"), 6, 6, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
//     }
//     frame(agent.lua_state_agent, 11.0);
//     if macros::is_excute(agent) {
//         macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
//         macros::FLASH(agent, 1, 0.753, 1, 0.706);
//     }
//     frame(agent.lua_state_agent, 12.0);
//     if macros::is_excute(agent) {
//         agent.clear_lua_stack();
//         lua_args!(agent, 10, 0.314, 0.314, 0.314, 0);
//         sv_animcmd::FLASH_FRM(agent.lua_state_agent);
//     }
//     frame(agent.lua_state_agent, 20.0);
//     if macros::is_excute(agent) {
//         macros::COL_NORMAL(agent);
//     }
// }

// #[acmd("samus", "effect_attacks3hi")]
// unsafe fn samus_attacks3hi_eff(agent: &mut L2CAgentBase) {
//     frame(agent.lua_state_agent, 10.0);
//     if macros::is_excute(agent) {
//         macros::EFFECT(agent, Hash40::new("samus_cshot_shot"), Hash40::new("top"), 6, 10, 0, -15, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
//     }
//     frame(agent.lua_state_agent, 11.0);
//     if macros::is_excute(agent) {
//         macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
//         macros::FLASH(agent, 1, 0.753, 1, 0.706);
//     }
//     frame(agent.lua_state_agent, 12.0);
//     if macros::is_excute(agent) {
//         agent.clear_lua_stack();
//         lua_args!(agent, 10, 0.314, 0.314, 0.314, 0);
//         sv_animcmd::FLASH_FRM(agent.lua_state_agent);
//     }
//     frame(agent.lua_state_agent, 20.0);
//     if macros::is_excute(agent) {
//         macros::COL_NORMAL(agent);
//     }
// }

// #[acmd("samus", "effect_attacks3lw")]
// unsafe fn samus_attacks3lw_eff(agent: &mut L2CAgentBase) {
//     frame(agent.lua_state_agent, 10.0);
//     if macros::is_excute(agent) {
//         macros::EFFECT(agent, Hash40::new("samus_cshot_shot"), Hash40::new("top"), 6, 5, 0, 15, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
//     }
//     frame(agent.lua_state_agent, 11.0);
//     if macros::is_excute(agent) {
//         macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
//         macros::FLASH(agent, 1, 0.753, 1, 0.706);
//     }
//     frame(agent.lua_state_agent, 12.0);
//     if macros::is_excute(agent) {
//         agent.clear_lua_stack();
//         lua_args!(agent, 10, 0.314, 0.314, 0.314, 0);
//         sv_animcmd::FLASH_FRM(agent.lua_state_agent);
//     }
//     frame(agent.lua_state_agent, 20.0);
//     if macros::is_excute(agent) {
//         macros::COL_NORMAL(agent);
//     }
// }

// #[acmd("samus", [ "sound_attacks3", "sound_attacks3hi", "sound_attacks3lw" ])]
// unsafe fn samus_attacks3_snd(_agent: &mut L2CAgentBase) {
    
// }

// #[acmd("samus", [ "expression_attacks3", "expression_attacks3hi", "expression_attacks3lw" ])]
// unsafe fn samus_attacks3_exp(agent: &mut L2CAgentBase) {
//     if macros::is_excute(agent) {
//         slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
//     }
//     frame(agent.lua_state_agent, 9.0);
//     if macros::is_excute(agent) {
//         ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_beams"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
//     }
// }

#[acmd("samus", "game_attackhi3")]
unsafe fn samus_attackhi3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 366, 0, 70, 100, 6.0, 0.0, 20.0, 9.0, None, None, None, 0.8, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    macros::FT_MOTION_RATE(agent, 0.75);
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 366, 0, 70, 60, 6.0, 0.0, 21.0, 8.0, None, None, None, 0.8, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 55, 92, 0, 50, 6.0, 0.0, 22.0, 6.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 34.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

#[acmd("samus", "effect_attackhi3")]
unsafe fn samus_attackhi3_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("samus_atk_bomb"), Hash40::new("top"), 0.0, 21.0, 10.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_v"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, -3, -40, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("samus_atk_bomb"), Hash40::new("top"), 0.0, 22.0, 9.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, -3, -44, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("samus_atk_bomb"), Hash40::new("top"), 0.0, 23.0, 7.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, -3, -48, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd("samus", "sound_attackhi3")]
unsafe fn samus_attackhi3_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_samus_smash_h01"));
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_samus_smash_h01"));
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_samus_smash_h02"));
    }
}

#[acmd("samus", "expression_attackhi3")]
unsafe fn samus_attackhi3_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_explosion"), 8);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohit_explosion"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_explosion"), 8);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohit_explosion"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_explosion"), 8);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohit_explosion"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    // samus_attacks3::install();
    // samus_attacks3_eff::install();
    // samus_attacks3hi_eff::install();
    // samus_attacks3lw_eff::install();
    // samus_attacks3_snd::install();
    // samus_attacks3_exp::install();
    samus_attackhi3::install();
    samus_attackhi3_eff::install();
    samus_attackhi3_snd::install();
    samus_attackhi3_exp::install();
}