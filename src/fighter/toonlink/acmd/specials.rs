use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::Hash40,
        app::{lua_bind::*, sv_animcmd::*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};

#[acmd_script( agent = "toonlink", scripts = ["game_specialnstart", "game_specialairnstart"], category = ACMD_GAME, low_priority )]
unsafe fn toonlink_specialnstart(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW, false, 0);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOWARROW, false, 0);
    }
    macros::FT_MOTION_RATE(fighter, 21.0/18.0);
    frame(fighter.lua_state_agent, 18.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE);
    }
}

#[acmd_script( agent = "toonlink", scripts = ["game_speciallw", "game_specialairlw"], category = ACMD_GAME, low_priority )]
unsafe fn toonlink_speciallw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_WORK_ID_FLAG_BOMB_GENERATE_LINKBOMB);
    }
    macros::FT_MOTION_RATE(fighter, 2.5);
    frame(fighter.lua_state_agent, 21.0);
    macros::FT_MOTION_RATE(fighter, 1.5);
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        ItemModule::throw_item(fighter.module_accessor, 80.0, 3.0, 1.0, 0, true, 0.0);
    }
}

#[acmd_script( agent = "toonlink_bowarrow", script = "game_fly", category = ACMD_GAME, low_priority )]
unsafe fn toonlink_bowarrow_fly(weapon: &mut L2CAgentBase) {
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 4.0, 361, 35, 0, 20, 1.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(weapon.module_accessor);
    }
}

#[acmd_script( agent = "toonlink_boomerang", script = "game_fly", category = ACMD_GAME, low_priority )]
unsafe fn toonlink_boomerang_fly(weapon: &mut L2CAgentBase) {
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(weapon.module_accessor);
    }
}

#[acmd_script( agent = "toonlink_boomerang", script = "game_turn", category = ACMD_GAME, low_priority )]
unsafe fn toonlink_boomerang_turn(weapon: &mut L2CAgentBase) {
    frame(weapon.lua_state_agent, 10.0);
    if macros::is_excute(weapon) {
        AttackModule::clear_all(weapon.module_accessor);
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 6.0, 30, 100, 70, 0, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, -1.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
}

pub fn install() {
    install_acmd_scripts!(
        toonlink_specialnstart,
        toonlink_speciallw,
        toonlink_bowarrow_fly,
        toonlink_boomerang_fly,
        toonlink_boomerang_turn
    );
}