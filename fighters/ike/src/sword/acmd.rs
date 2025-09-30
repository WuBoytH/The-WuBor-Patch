use super::*;

unsafe extern "C" fn game_bladebeam(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 16.0, 361, 40, 0, 50, 3.0, 0.0, 4.0, -6.0, Some(0.0), Some(2.0), Some(6.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, vars::ike_sword::status::flag::BLADE_BEAM_KINETIC_SHIFT);
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 12.0, 361, 40, 0, 50, 3.0, 0.0, 4.0, -6.0, Some(0.0), Some(2.0), Some(6.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
    }
}

unsafe extern "C" fn effect_bladebeam(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 8, Hash40::new("haver"), 0, 2, -9, Hash40::new("haver"), 0, 4, 9, true, Hash40::new("null"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_bladebeam", game_bladebeam, Priority::Low);
    agent.acmd("effect_bladebeam", effect_bladebeam, Priority::Low);
}