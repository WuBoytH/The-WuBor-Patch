use crate::imports::acmd_imports::*;

#[acmd_script( agent = "purin", script = "game_specialairs", category = ACMD_GAME, low_priority )]
unsafe fn purin_specialairs(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 75, 75, 0, 52, 4.5, 0.0, 4.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 20, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 75, 75, 0, 52, 4.5, 0.0, 4.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 20, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 2.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 2.0, false);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 1.5);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PURIN_STATUS_SPECIAL_S_FLAG_INPUT);
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_PURIN_SPECIAL_S_ENERGY_MODE_BRAKE, *FIGHTER_PURIN_STATUS_SPECIAL_S_WORK_INT_ENERGY_MODE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PURIN_STATUS_SPECIAL_S_FLAG_CHANGE_ENERGY);
        GroundModule::select_cliff_hangdata(fighter.module_accessor, 1);
    }
    frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_PURIN_SPECIAL_S_ENERGY_MODE_FALL, *FIGHTER_PURIN_STATUS_SPECIAL_S_WORK_INT_ENERGY_MODE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PURIN_STATUS_SPECIAL_S_FLAG_CHANGE_ENERGY);
    }
    frame(fighter.lua_state_agent, 41.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

pub fn install() {
    install_acmd_scripts!(
        purin_specialairs
    );
}