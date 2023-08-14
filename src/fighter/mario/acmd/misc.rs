use crate::imports::acmd_imports::*;
use super::super::helper::*;

#[acmd("mario_pump", "effect_start")]
unsafe fn mario_pump_start_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        bonker_vis(agent.module_accessor);
        LinkModule::set_model_constraint_pos_ort(agent.module_accessor, *LINK_NO_ARTICLE, Hash40::new("have"), Hash40::new("haver"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32, false);
    }
}

#[acmd("mario_pump", "effect_light")]
unsafe fn mario_pump_light_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        bonker_vis(agent.module_accessor);
        LinkModule::set_model_constraint_pos_ort(agent.module_accessor, *LINK_NO_ARTICLE, Hash40::new("have"), Hash40::new("havel"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32, false);
    }
}

#[acmd("mario_pump", "game_attackairf")]
unsafe fn mario_pump_attackairf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        bonker_vis(agent.module_accessor);
        LinkModule::set_model_constraint_pos_ort(agent.module_accessor, *LINK_NO_ARTICLE, Hash40::new("have"), Hash40::new("haver"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32, false);
    }
}

pub fn install() {
    mario_pump_start_eff::install();
    mario_pump_light_eff::install();
    mario_pump_attackairf::install();
}