use crate::imports::acmd_imports::*;

#[acmd_script( agent = "miigunner", scripts = [ "game_specialhi1", "game_specialairhi1" ], category = ACMD_GAME, low_priority )]
unsafe fn miigunner_specialhi1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(fighter, 0.5);
    frame(fighter.lua_state_agent, 10.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIIGUNNER_STATUS_BOTTOM_SHOOT_FLAG_JUMP);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_BOTTOMSHOOT, false, -1);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP); // Was ALWAYS
    }
}

#[acmd_script( agent = "miigunner", script = "game_specialhi2", category = ACMD_GAME, low_priority )]
unsafe fn miigunner_specialhi2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 80, 100, 0, 83, 5.0, 0.0, 18.0, 1.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        // notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        macros::ATK_POWER(fighter, 0, 6);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

pub fn install() {
    install_acmd_scripts!(
        miigunner_specialhi1,
        miigunner_specialhi2
    );
}