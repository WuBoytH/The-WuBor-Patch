use crate::imports::acmd_imports::*;

unsafe extern "C" fn ken_hadoken_move(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 65, 10, 0, 65, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, -1.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
        AttackModule::enable_safe_pos(agent.module_accessor);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 2.2);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_movew", ken_hadoken_move);

    agent.game_acmd("game_movem", ken_hadoken_move);

    agent.game_acmd("game_moves", ken_hadoken_move);
}