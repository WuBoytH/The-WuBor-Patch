use smash::phx::Hash40;
use smash::lua2cpp::L2CAgentBase;
use smash::app::sv_animcmd;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smash_script::macros;

#[script( agent = "koopajr", script = "game_attackdash", category = ACMD_GAME )]
unsafe fn koopajr_dashattack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 8.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.8, 0, 10, 0, 38, 4.0, 0.0, 8.0, 9.0, Some(0.0), Some(8.0), Some(8.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.8, 0, 10, 0, 38, 4.0, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(8.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.8, 0, 10, 0, 38, 4.0, 0.0, 8.0, 18.0, Some(0.0), Some(8.0), Some(8.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    sv_animcmd::wait(lua_state, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(boma, 0, false);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.8, 0, 10, 0, 38, 4.0, 0.0, 8.0, 9.0, Some(0.0), Some(8.0), Some(8.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.8, 0, 10, 0, 38, 4.0, 0.0, 8.0, 14.0, Some(0.0), Some(8.0), Some(8.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    sv_animcmd::wait(lua_state, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    sv_animcmd::frame(lua_state, 25.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 50, 70, 0, 100, 5.0, 0.0, 8.0, 9.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("clowntongue3"), 4.0, 50, 70, 0, 100, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[script( agent = "koopajr", script = "game_attackairn", category = ACMD_GAME )]
unsafe fn koopajr_nair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("clownarmr1"), 8.0, 361, 83, 0, 50, 4.0, -3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("clownarml1"), 8.0, 361, 83, 0, 50, 4.0, -3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("clownarmr1"), 8.0, 361, 83, 0, 50, 6.0, 3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 3, 0, Hash40::new("clownarml1"), 8.0, 361, 83, 0, 50, 6.0, 3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("clownarmr1"), 7.0, 361, 81, 0, 40, 4.0, -3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("clownarml1"), 7.0, 361, 81, 0, 40, 4.0, -3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("clownarmr1"), 7.0, 361, 81, 0, 40, 5.5, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 3, 0, Hash40::new("clownarml1"), 7.0, 361, 81, 0, 40, 5.5, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(lua_state, 5.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("clownarmr1"), 5.0, 361, 80, 0, 30, 3.0, -3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("clownarml1"), 5.0, 361, 80, 0, 30, 3.0, -3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("clownarmr1"), 5.0, 361, 80, 0, 30, 5.0, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 3, 0, Hash40::new("clownarml1"), 5.0, 361, 80, 0, 30, 5.0, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(lua_state, 25.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 50, 70, 0, 100, 5.0, 0.0, 8.0, 9.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("clowntongue3"), 4.0, 50, 70, 0, 100, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    sv_animcmd::wait(lua_state, 6.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    sv_animcmd::frame(lua_state, 42.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[script( agent = "koopajr", scripts = ["game_speciallw", "game_specialairlw"], category = ACMD_GAME )]
unsafe fn koopajr_dspecial(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 10.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article_enable(boma, *FIGHTER_KOOPAJR_GENERATE_ARTICLE_MECHAKOOPA, false, 0);
    }
}

pub fn install() {
    smash_script::replace_scripts!(
        koopajr_dashattack,
        koopajr_nair,
        koopajr_dspecial
    );
}