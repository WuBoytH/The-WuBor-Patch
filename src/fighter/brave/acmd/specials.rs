use crate::imports::acmd_imports::*;

#[acmd_script( agent = "brave", scripts = [ "game_specialhi1", "game_specialairhi1" ], category = ACMD_GAME, low_priority )]
unsafe fn brave_specialhi1(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_BRAVE_GENERATE_ARTICLE_TORNADO, false, -1);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_JUMP_START);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_REVERT_ANGLE);
    }
}

#[acmd_script( agent = "brave", scripts = [ "game_specialhi2", "game_specialairhi2" ], category = ACMD_GAME, low_priority )]
unsafe fn brave_specialhi2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_BRAVE_GENERATE_ARTICLE_TORNADO, false, -1);
    }
    wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_JUMP_START);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_REVERT_ANGLE);
    }
}

#[acmd_script( agent = "brave", scripts = [ "game_specialhi3", "game_specialairhi3" ], category = ACMD_GAME, low_priority )]
unsafe fn brave_specialhi3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_BRAVE_GENERATE_ARTICLE_TORNADO, false, -1);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_JUMP_START);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_REVERT_ANGLE);
    }
}

#[acmd_script( agent = "brave", scripts = [ "sound_speciallw9", "sound_specialairlw9" ], category = ACMD_SOUND, low_priority )]
unsafe fn brave_speciallw9_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_brave_special_l03"));
        macros::PLAY_SE(agent, Hash40::new("se_brave_special_l14"));
    }
}

#[acmd_script( agent = "brave", scripts = [ "game_speciallw9end", "game_specialairlw9end" ], category = ACMD_GAME, low_priority )]
unsafe fn brave_speciallw9end(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FT_ADD_DAMAGE(agent, 999.0);
    }
    MiscModule::calc_motion_rate_from_end_frame(agent, 1.0, 15.0);
}

#[acmd_script( agent = "brave", scripts = [ "effect_speciallw9end", "effect_specialairlw9end" ], category = ACMD_EFFECT, low_priority )]
unsafe fn brave_speciallw9end_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_dead2"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1.0, true);
    }
}

#[acmd_script( agent = "brave", scripts = [ "sound_speciallw9end", "sound_specialairlw9end" ], category = ACMD_SOUND, low_priority )]
unsafe fn brave_speciallw9end_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_brave_final02"));
        macros::PLAY_SE(agent, Hash40::new("se_common_stage_fall"));
    }
}

#[acmd_script( agent = "brave", scripts = [ "expression_speciallw9end", "expression_specialairlw9end" ], category = ACMD_EXPRESSION, low_priority )]
unsafe fn brave_speciallw9end_exp(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_dead"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
}

pub fn install() {
    install_acmd_scripts!(
        brave_specialhi1,
        brave_specialhi2,
        brave_specialhi3,
        brave_speciallw9_snd,
        brave_speciallw9end, brave_speciallw9end_eff, brave_speciallw9end_snd, brave_speciallw9end_exp
    );
}