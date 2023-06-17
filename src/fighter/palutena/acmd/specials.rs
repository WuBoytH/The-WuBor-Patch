use crate::imports::acmd_imports::*;

#[acmd_script( agent = "palutena", scripts = [ "game_specialhistart", "game_specialairhistart" ], category = ACMD_GAME, low_priority )]
unsafe fn palutena_specialhistart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
        GroundModule::select_cliff_hangdata(fighter.module_accessor, 1);
    }
}

#[acmd_script( agent = "palutena", script = "game_specialhi", category = ACMD_GAME, low_priority )]
unsafe fn palutena_specialhi(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        GroundModule::select_cliff_hangdata(fighter.module_accessor, 1);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "palutena", script = "game_specialairhi", category = ACMD_GAME, low_priority )]
unsafe fn palutena_specialairhi(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        GroundModule::select_cliff_hangdata(fighter.module_accessor, 1);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_HI_DIVE);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_HI_CONTROL_ON);
    }
}

pub fn install() {
    install_acmd_scripts!(
        palutena_specialhistart,
        palutena_specialhi,
        palutena_specialairhi
    );
}