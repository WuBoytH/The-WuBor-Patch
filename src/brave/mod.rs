use smash::phx::Hash40;
use smash::lua2cpp::L2CAgentBase;
use smash::app::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smashline::*;
use crate::IS_FUNNY;
use crate::commonfuncs;

#[acmd_script( agent = "brave", scripts = ["game_speciallw9", "game_specialairlw9"], category = ACMD_GAME )]
unsafe fn brave_kamikazee(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 12.0);
    macros::FT_MOTION_RATE(fighter, 4.0);
    sv_animcmd::frame(lua_state, 19.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    sv_animcmd::frame(lua_state, 20.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_START_FLAG_STRENGTH);
        ArticleModule::generate_article(boma, *FIGHTER_BRAVE_GENERATE_ARTICLE_CRASH, false, 0);
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_START_FLAG_GENERATED_ARTICLE);
        SlowModule::set_whole(boma, 20 as u8, 30);
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_START_FLAG_SLOW_WHOLE);
    }
    if macros::is_excute(fighter) {
        macros::FT_SET_FINAL_FEAR_FACE(fighter, 30 as u64);
    }
    sv_animcmd::frame(lua_state, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 10.0, 361, 78, 0, 50, 5.5, 10.0, 0.0, -0.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 10.0, 361, 78, 0, 50, 5.0, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("arml"), 10.0, 361, 78, 0, 50, 3.5, -0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(lua_state, 22.0);
    if macros::is_excute(fighter) {
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
        if IS_FUNNY[commonfuncs::get_player_number(boma)] {
            DamageModule::add_damage(boma, 999.9, 0);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
        }
        else {
            WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_START_FLAG_DEATH_RESERVE);
        }
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        brave_kamikazee
    );
}