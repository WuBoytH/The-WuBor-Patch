use crate::imports::acmd_imports::*;
use super::super::helper::*;

#[acmd_script( agent = "mario_pump", script = "effect_start", category = ACMD_EFFECT, low_priority )]
unsafe fn mario_pump_start_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        bonker_vis(agent.module_accessor);
        LinkModule::set_model_constraint_pos_ort(agent.module_accessor, *LINK_NO_ARTICLE, Hash40::new("have"), Hash40::new("haver"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32, false);
    }
}

#[acmd_script( agent = "mario_pump", script = "effect_light", category = ACMD_EFFECT, low_priority )]
unsafe fn mario_pump_light_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        bonker_vis(agent.module_accessor);
        LinkModule::set_model_constraint_pos_ort(agent.module_accessor, *LINK_NO_ARTICLE, Hash40::new("have"), Hash40::new("havel"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32, false);
    }
}

pub fn install() {
    install_acmd_scripts!(
        mario_pump_start_eff,
        mario_pump_light_eff
    );
}