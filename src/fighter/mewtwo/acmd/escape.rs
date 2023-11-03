use crate::imports::acmd_imports::*;

#[acmd_script( agent = "mewtwo", script = "game_escapeair", category = ACMD_GAME, low_priority )]
unsafe fn mewtwo_escapeair(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_MEWTWO_GENERATE_ARTICLE_ESCAPEAIRDUMMY, false, -1);
        VisibilityModule::set_whole(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, true);
        ArticleModule::remove(agent.module_accessor, *FIGHTER_MEWTWO_GENERATE_ARTICLE_ESCAPEAIRDUMMY, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

#[acmd_script( agent = "mewtwo", script = "effect_escapeairslide", category = ACMD_EFFECT, low_priority )]
unsafe fn mewtwo_escapeair_eff(_agent: &mut L2CAgentBase) {
    // if macros::is_excute(agent) {
    //     macros::EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 12, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
    // }
}

#[acmd_script( agent = "mewtwo", script = "game_escapeairslide", category = ACMD_GAME, low_priority )]
unsafe fn mewtwo_escapeairslide(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_GRAVITY);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

pub fn install() {
    install_acmd_scripts!(
        mewtwo_escapeair,
        mewtwo_escapeair_eff,

        mewtwo_escapeairslide
    );
}