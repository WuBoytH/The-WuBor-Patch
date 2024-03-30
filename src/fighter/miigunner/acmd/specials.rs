use crate::imports::*;

unsafe extern "C" fn miigunner_specialhi1(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 10.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MIIGUNNER_STATUS_BOTTOM_SHOOT_FLAG_JUMP);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_BOTTOMSHOOT, false, -1);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP); // Was ALWAYS
    }
}

unsafe extern "C" fn miigunner_specialhi2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 80, 100, 0, 83, 5.0, 0.0, 18.0, 1.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        // notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        macros::ATK_POWER(agent, 0, 6);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_specialhi1", miigunner_specialhi1);

    agent.acmd("game_specialairhi1", miigunner_specialhi1);

    agent.acmd("game_specialhi2", miigunner_specialhi2);
}