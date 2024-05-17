use super::*;

unsafe extern "C" fn game_fly(agent: &mut L2CAgentBase) {
    let owner_id = WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    if sv_battle_object::is_active(owner_id) {
        let owner_module_accessor = sv_battle_object::module_accessor(owner_id);
        VarModule::set_int(owner_module_accessor, vars::simon::instance::int::AXE_ID, agent.battle_object_id as i32);
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("axe"), 13.0, 69, 70, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 8, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.1);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_fly", game_fly, Priority::Low);
}