use super::*;

unsafe extern "C" fn game_specialhi2(agent: &mut L2CAgentBase) {
    // frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 2.0, 90, 100, 90, 0, 5.0, 0.0, 4.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_PART, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 2.0, 90, 100, 90, 0, 5.0, 0.0, 10.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_PART, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 1.0, 90, 100, 40, 0, 5.0, 0.0, 7.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_PART, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 1.0, 90, 100, 40, 0, 5.0, 0.0, 7.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_PART, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *WEAPON_IKE_SWORD_STATUS_SPECIAL_HI_WORK_ID_FLAG_HAVE);
    }
}

unsafe extern "C" fn game_bladebeam(agent: &mut L2CAgentBase) {
    let damage = if StatusModule::status_kind(agent.module_accessor) == vars::ike_sword::status::BLADE_BEAM {
        (16.0, 12.0)
    }
    else {
        (10.0, 6.0)
    };
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), damage.0, 70, 40, 0, 50, 4.0, 0.0, -2.0, -3.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), damage.0, 70, 40, 0, 50, 5.0, 0.0, -2.0, 3.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, vars::ike_sword::status::flag::BLADE_BEAM_KINETIC_SHIFT);
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), damage.1, 70, 40, 0, 50, 4.0, 0.0, -2.0, -3.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), damage.1, 70, 40, 0, 50, 5.0, 0.0, -2.0, 3.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
    }
}

unsafe extern "C" fn effect_bladebeam(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 8, Hash40::new("haver"), 0, 2, -9, Hash40::new("haver"), 0, 4, 9, true, Hash40::new("null"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialhi2", game_specialhi2, Priority::Low);

    agent.acmd("game_specialairhi2", game_specialhi2, Priority::Low);

    agent.acmd("game_bladebeam", game_bladebeam, Priority::Low);
    agent.acmd("effect_bladebeam", effect_bladebeam, Priority::Low);
}