use super::*;

unsafe extern "C" fn game_fly(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let mash_count = VarModule::get_int(agent.module_accessor, vars::krool_backpack::instance::int::SPECIAL_HI_MASHED_COUNT);
        let rehit = 16 - (mash_count * 2);
        macros::ATTACK(agent, 0, 0, Hash40::new("wingl1"), 2.0, 80, 30, 0, 90, 6.0, 2.0, 0.0, 0.0, Some(-2.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, rehit, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_flywind", game_fly, Priority::Low);

    agent.acmd("game_fly", game_fly, Priority::Low);

    agent.acmd("game_flymash", game_fly, Priority::Low);
}