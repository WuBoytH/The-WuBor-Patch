use super::*;

unsafe extern "C" fn game_normal(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 70, 40, 0, 50, 3.0, 0.0, 1.7, 0.0, Some(0.0), Some(11.0), Some(-2.1), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, vars::ike_wave::status::flag::BLADE_BEAM_KINETIC_SHIFT);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 70, 40, 0, 50, 3.0, 0.0, 1.7, 0.0, Some(0.0), Some(11.0), Some(-2.1), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
    }
}

unsafe extern "C" fn effect_normal(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 8, Hash40::new("top"), 0, -1, 0, Hash40::new("top"), 0, 14, -4, true, Hash40::new("null"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    loop {
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 0.9, 2, 0, 2, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 2.0);
    }
}

unsafe extern "C" fn effect_normalair(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 8, Hash40::new("top"), 0, -1, 0, Hash40::new("top"), 0, 14, -4, true, Hash40::new("null"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
}

unsafe extern "C" fn game_normalw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 70, 40, 0, 50, 3.0, 0.0, 1.7, 0.0, Some(0.0), Some(11.0), Some(-2.1), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, vars::ike_wave::status::flag::BLADE_BEAM_KINETIC_SHIFT);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 70, 40, 0, 50, 3.0, 0.0, 1.7, 0.0, Some(0.0), Some(11.0), Some(-2.1), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_normal", game_normal, Priority::Low);
    agent.acmd("effect_normal", effect_normal, Priority::Low);

    agent.acmd("game_normalair", game_normal, Priority::Low);
    agent.acmd("effect_normalair", effect_normalair, Priority::Low);

    agent.acmd("game_normalw", game_normalw, Priority::Low);
    agent.acmd("effect_normalw", effect_normal, Priority::Low);

    agent.acmd("game_normalairw", game_normalw, Priority::Low);
    agent.acmd("effect_normalairw", effect_normalair, Priority::Low);
}