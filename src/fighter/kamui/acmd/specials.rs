use crate::imports::acmd_imports::*;

#[acmd_script( agent = "kamui", script = "game_specialsjump", category = ACMD_GAME, low_priority )]
unsafe fn kamui_specialsjump(agent: &mut L2CAgentBase) {
    let mut di = false;
    if VarModule::get_float(agent.battle_object, kamui::instance::float::DRAGON_INSTALL) > 0.0 {
        di = true;
    }
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, false, 0);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, Hash40::new("special_s_jump"), false, -1.0);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_S_FLAG_ATTACK);
    }
    frame(agent.lua_state_agent, 8.0);
    if di
    && ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        if macros::is_excute(agent) {
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_S_FLAG_ATTACK);
            CancelModule::enable_cancel(agent.module_accessor);
        }
    }
}

#[acmd_script( agent = "kamui", scripts = [ "game_specialhi", "game_specialairhi" ], category = ACMD_GAME, low_priority )]
unsafe fn kamui_specialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        GroundModule::select_cliff_hangdata(agent.module_accessor, 1);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::SA_SET(agent, *SITUATION_KIND_AIR);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 1, Hash40::new("top"), 4.5, 85, 100, 160, 0, 5.0, 0.0, 4.0, -3.0, Some(0.0), Some(4.0), Some(6.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 0, 70, 32, 0, 7.0, 0.0, 3.5, -14.0, Some(0.0), Some(3.5), Some(15.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_HI_FLAG_TILT_BODY_ON);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 1, Hash40::new("rot"), 1.2, 80, 100, 150, 0, 2.0, 0.0, 5.5, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 1, Hash40::new("rot"), 1.2, 105, 100, 150, 0, 5.5, 0.0, 5.5, 6.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 2, 1, Hash40::new("rot"), 1.2, 68, 100, 150, 0, 5.5, 0.0, 5.5, -6.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 3, 1, Hash40::new("rot"), 1.2, 100, 100, 165, 0, 5.0, 0.0, -1.0, 5.5, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 4, 1, Hash40::new("rot"), 1.2, 73, 100, 165, 0, 5.0, 0.0, -1.0, -5.5, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 2, Hash40::new("rot"), 3.0, 65, 170, 0, 70, 8.5, 0.0, 5.5, -7.0, None, None, None, 2.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 2, Hash40::new("rot"), 3.0, 65, 170, 0, 70, 8.5, 0.0, 5.5, 7.0, None, None, None, 2.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_HI_FLAG_TILT_BODY_ON);
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_HI_FLAG_AIR_CONTROL);
    }
    frame(agent.lua_state_agent, 49.0);
    macros::FT_MOTION_RATE(agent, 0.8);
}

#[acmd_script( agent = "kamui", scripts = ["game_speciallwhit", "game_specialairlwhit"], category = ACMD_GAME, low_priority )]
unsafe fn kamui_speciallwhit(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, false, 0);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, false);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        VisibilityModule::set_whole(agent.module_accessor, true);
    }
}

#[acmd_script( agent = "kamui_waterdragon", scripts = ["game_speciallwhit", "game_specialairlwhit"], category = ACMD_GAME, low_priority )]
unsafe fn kamui_waterdragon_speciallwhit(agent: &mut L2CAgentBase) {
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
            let object = MiscModule::get_battle_object_from_id(otarget_id);
            VarModule::set_float(object, kamui::instance::float::DRAGON_INSTALL, 600.0);
            VarModule::set_float(object, kamui::instance::float::DRAGON_INSTALL_TIMER, 24.0);
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        kamui_specialsjump,
        kamui_specialhi,
        kamui_speciallwhit,
        kamui_waterdragon_speciallwhit
    );
}