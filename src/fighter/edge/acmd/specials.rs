use crate::imports::acmd_imports::*;

unsafe extern "C" fn edge_specialhistart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLAG_DECIDED_RUSH);
    }
    macros::FT_MOTION_RATE(agent, 22.0 / 17.0);
}

unsafe extern "C" fn edge_specialhi1(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
        let rush_angle = WorkModule::get_float(agent.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLOAT_RUSH_DEGREE).abs();
        let angle_ratio = 1.0 - (rush_angle / 90.0).clamp(0.0, 1.0);
        let angle = 125.0 - (35.0 * angle_ratio).clamp(0.0, 35.0);
        macros::ATTACK(agent, 0, 0, Hash40::new("rot"), 7.0, angle as u64, 60, 0, 70, 3.0, 0.0, -2.0, 9.0, Some(0.0), Some(-2.0), Some(-4.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.5);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_NONE);
        AttackModule::clear_all(agent.module_accessor);
        JostleModule::set_status(agent.module_accessor, true);
    }
}

unsafe extern "C" fn edge_specialairhi2end(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 4.0, 3.0);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_NONE);
        let rush_angle = WorkModule::get_float(agent.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLOAT_RUSH_DEGREE) * -1.0;
        let base_angle = if rush_angle < 0.0 { 360.0 } else { 0.0 } as f32;
        let angle_ratio = rush_angle / 90.0;
        let angle_diff = 60.0 * angle_ratio;
        let mut angle = base_angle + angle_diff;
        if !(15.0..=345.0).contains(&angle) { angle = 361.0 }
        let bkb;
        let kbg;
        if angle < 360.0 && angle > 60.0 {
            bkb = 55;
            kbg = 30;
        }
        else {
            bkb = 64;
            kbg = 104;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("rot"), 7.0, angle as u64, kbg, 0, bkb, 10.0, 0.0, 0.0, 6.0, None, None, None, 2.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("rot"), 7.0, angle as u64, kbg, 0, bkb, 10.0, 0.0, 0.0, 18.0, None, None, None, 2.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("rot"), 2.3, 60, 60, 0, 90, 8.5, 0.0, 0.0, -2.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_specialhistart", edge_specialhistart);
    agent.game_acmd("game_specialairhistart", edge_specialhistart);

    agent.game_acmd("game_specialhi1", edge_specialhi1);

    agent.game_acmd("game_specialairhi2end", edge_specialairhi2end);
}