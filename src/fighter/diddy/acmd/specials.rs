use crate::imports::acmd_imports::*;

#[acmd_script( agent = "diddy", script = "game_specialairsjump", category = ACMD_GAME, low_priority )]
unsafe fn diddy_specialairsjump(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        GroundModule::select_cliff_hangdata(fighter.module_accessor, 2);
        macros::CATCH(fighter, 0, Hash40::new("top"), 3.0, 0.0, 2.5, 4.5, None, None, None, *FIGHTER_STATUS_KIND_CLUNG_DIDDY, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(fighter, 1, Hash40::new("top"), 4.0, 0.0, 5.5, 4.0, None, None, None, *FIGHTER_STATUS_KIND_CLUNG_DIDDY, *COLLISION_SITUATION_MASK_GA);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 1);
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 0);
    }
}

pub fn install() {
    install_acmd_scripts!(
        diddy_specialairsjump
    );
}