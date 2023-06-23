use crate::imports::acmd_imports::*;

#[acmd_script( agent = "trail", scripts = [ "game_speciallw", "game_specialairlw" ], category = ACMD_GAME, low_priority )]
unsafe fn trail_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::set_float(agent.module_accessor, 0.0, *FIGHTER_TRAIL_STATUS_SPECIAL_LW_FLOAT_ATTACK_POWER);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 5.0, 86, 20, 0, 100, 3.2, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 5.0, 86, 20, 0, 100, 3.2, 0.0, 4.6, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 5.0, 86, 20, 0, 100, 3.2, 0.0, 9.2, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 5.0, 86, 20, 0, 100, 9.0, 9.0, 4.0, 11.5, Some(0.0), Some(9.0), Some(11.5), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 10.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 2, 10.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 3, 10.0, false);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 1, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 2, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 3, true, false);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_LW_FLAG_ATTACK_END);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 5.0, 86, 20, 0, 80, 9.0, 9.0, 4.0, 11.5, Some(0.0), Some(13.0), Some(11.5), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 5.0, 86, 20, 0, 80, 9.0, 9.0, 4.0, 11.5, Some(0.0), Some(16.5), Some(11.5), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    install_acmd_scripts!(
        trail_speciallw
    );
}