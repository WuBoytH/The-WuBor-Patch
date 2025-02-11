use super::*;

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::shoot_exist(
            agent.module_accessor,
            *FIGHTER_BUDDY_GENERATE_ARTICLE_PAD,
            ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL),
            false
        );
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_HI_FLAG_RESET_CONTROL);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
}