use crate::imports::acmd_imports::*;

#[acmd_script( agent = "captain", scripts = [ "game_specialhi", "game_specialairhi" ], category = ACMD_GAME, low_priority )]
unsafe fn captain_specialhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
        macros::CATCH(fighter, 0, Hash40::new("top"), 5.0, 0.0, 13.0, 7.0, *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("top"), 5.5, 0.0, 8.8, 13.7, *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN, *COLLISION_SITUATION_MASK_G);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 0, 50, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 5.0, 0.0, 15.0, 6.0, *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("top"), 5.0, 0.0, 11.0, 6.0, *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN, *COLLISION_SITUATION_MASK_GA);
    }
    // frame(fighter.lua_state_agent, 19.0);
    // if macros::is_excute(fighter) {
    //     notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    // }
    frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 36.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(fighter.lua_state_agent, 50.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_SPECIAL_HI_FLAG_IS_CHECK_DIVE);
    }
}

pub fn install() {
    install_acmd_scripts!(
        captain_specialhi
    );
}