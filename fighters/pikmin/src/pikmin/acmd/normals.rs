use super::*;

unsafe extern "C" fn game_attackdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        KineticModule::change_kinetic(agent.module_accessor, *WEAPON_KINETIC_TYPE_PIKMIN_PIKMIN_MOTION);
    }
    frame(agent.lua_state_agent, 6.0);
    for _ in 0..3 {
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            let params = pikmin_game_helper(agent, 0.5, 2.5);
            macros::ATTACK(agent, 0, 0, Hash40::new("trans"), params.damage, 365, 25, 25, 0, params.hitbox_size, 0.0, 3.5, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, params.hit_effect, *ATTACK_SOUND_LEVEL_S, params.sound_attr, *ATTACK_REGION_BODY);
        }
        wait(agent.lua_state_agent, 2.0);
    }
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        let params = pikmin_game_helper(agent, 4.0, 3.5);
        macros::ATTACK(agent, 0, 0, Hash40::new("trans"), params.damage, 60, 100, 0, 60, params.hitbox_size, 0.0, 3.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, params.hit_effect, *ATTACK_SOUND_LEVEL_M, params.sound_attr, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install(agent: &mut Agent) {
    pikmin_acmd(agent, "game_attackdash", game_attackdash);
}