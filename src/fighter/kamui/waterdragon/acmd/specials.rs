use crate::imports::acmd_imports::*;

unsafe extern "C" fn kamui_waterdragon_speciallwhit(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 30, 0, 60, 12.0, 0.0, 6.0, 11.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 361, 30, 0, 60, 12.0, 0.0, 6.0, -11.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 14.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 14.0, false);
        AttackModule::set_optional_hit_sound(agent.module_accessor, 0, Hash40::new("se_kamui_criticalhit"));
        AttackModule::set_optional_hit_sound(agent.module_accessor, 1, Hash40::new("se_kamui_criticalhit"));
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 361, 30, 0, 60, 8.0, 0.0, 21.0, 11.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 8.0, 361, 30, 0, 60, 8.0, 0.0, 21.0, -11.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 2, 14.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 3, 14.0, false);
        AttackModule::set_optional_hit_sound(agent.module_accessor, 2, Hash40::new("se_kamui_criticalhit"));
        AttackModule::set_optional_hit_sound(agent.module_accessor, 3, Hash40::new("se_kamui_criticalhit"));
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        if AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT) {
            let otarget_id = WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
            let owner_module_accessor = sv_battle_object::module_accessor(otarget_id);
            VarModule::set_float(owner_module_accessor, kamui::instance::float::DRAGON_INSTALL, 600.0);
            VarModule::set_float(owner_module_accessor, kamui::instance::float::DRAGON_INSTALL_TIMER, 24.0);
        }
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_speciallwhit", kamui_waterdragon_speciallwhit);

    agent.acmd("game_specialairlwhit", kamui_waterdragon_speciallwhit);
}