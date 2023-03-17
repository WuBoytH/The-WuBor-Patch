use crate::imports::acmd_imports::*;

#[acmd_script( agent = "demon", script = "game_escapeairslide", category = ACMD_GAME, low_priority )]
unsafe fn demon_escapeairslide(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_GRAVITY);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

pub fn install() {
    install_acmd_scripts!(
        demon_escapeairslide
    );
}