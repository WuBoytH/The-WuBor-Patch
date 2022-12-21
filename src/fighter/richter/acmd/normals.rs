use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::Hash40,
        app::{lua_bind::*, sv_animcmd::*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    custom_var::*,
    wubor_utils::vars::*
};

#[acmd_script( agent = "richter", script = "game_attack100end", category = ACMD_GAME )]
unsafe fn richter_attack100end(fighter: &mut L2CAgentBase) {
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter){
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 55, 115, 0, 80, 7.5, 0.0, 7.5, 12.0, Some(0.0), Some(7.5), Some(13.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        VarModule::on_flag(fighter.battle_object, commons::status::flag::NORMAL_CANCEL);
        VarModule::on_flag(fighter.battle_object, commons::status::flag::SPECIAL_CANCEL);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    wait(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, commons::status::flag::NORMAL_CANCEL);
        VarModule::off_flag(fighter.battle_object, commons::status::flag::SPECIAL_CANCEL);
    }
}

pub fn install() {
    install_acmd_scripts!(
        richter_attack100end
    );
}