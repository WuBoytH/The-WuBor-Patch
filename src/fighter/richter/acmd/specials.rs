use crate::imports::acmd_imports::*;

#[acmd_script( agent = "richter", scripts = ["game_specialn", "game_specialairn"], category = ACMD_GAME, low_priority )]
unsafe fn richter_specialn(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, 0.7);
    frame(fighter.lua_state_agent, 30.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, richter::status::flag::SPECIAL_N_SHOOT);
    }
}

#[acmd_script( agent = "richter", scripts = ["expression_specialn", "expression_specialairn"], category = ACMD_EXPRESSION, low_priority )]
unsafe fn richter_specialn_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_lightthrow4item"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
}

#[acmd_script( agent = "richter", scripts = ["game_specialnblank", "game_specialairnblank"], category = ACMD_GAME, low_priority )]
unsafe fn richter_specialnblank(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, 0.7);
    frame(fighter.lua_state_agent, 30.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "richter_axe", script = "game_fly", category = ACMD_GAME, low_priority )]
unsafe fn richter_axe_fly(weapon: &mut L2CAgentBase) {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    if sv_battle_object::is_active(owner_id) {
        let object = MiscModule::get_battle_object_from_id(owner_id);
        VarModule::set_int(object, richter::instance::int::AXE_ID, weapon.battle_object_id as i32);
    }
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("axe"), 9.0, 130, 30, 0, 80, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        AttackModule::set_add_reaction_frame_revised(weapon.module_accessor, 0, 5.0, false);
        macros::ATK_SET_SHIELD_SETOFF_MUL(weapon, 0, 1.1);
    }
}

#[acmd_script( agent = "richter", scripts = ["game_specials1", "game_specialairs1"], category = ACMD_GAME, low_priority )]
unsafe fn richter_specials1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 1.2);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SIMON_GENERATE_ARTICLE_CROSS, false, 0);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_S_FLAG_FALL);
    }
    frame(fighter.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_SIMON_GENERATE_ARTICLE_CROSS, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_S_FLAG_FALL);
    }
}

#[acmd_script( agent = "richter_cross", script = "game_fly", category = ACMD_GAME, low_priority )]
unsafe fn richter_cross_fly(weapon: &mut L2CAgentBase) {
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("rot"), 6.0, 90, 20, 0, 75, 1.2, 0.0, 3.7, 0.0, Some(0.0), Some(-3.7), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -6, -1.0, 24, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RICHTER_CROSS, *ATTACK_REGION_OBJECT);
        macros::ATTACK(weapon, 1, 0, Hash40::new("rot"), 6.0, 90, 20, 0, 75, 1.2, 0.0, 0.0, 3.7, Some(0.0), Some(0.0), Some(-3.7), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -6, -1.0, 24, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RICHTER_CROSS, *ATTACK_REGION_OBJECT);
    }
}

#[acmd_script( agent = "richter_cross", script = "game_turn", category = ACMD_GAME, low_priority )]
unsafe fn richter_cross_turn(weapon: &mut L2CAgentBase) {
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("rot"), 6.0, 90, 20, 0, 75, 1.2, 0.0, 3.7, 0.0, Some(0.0), Some(-3.7), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -6, -1.0, 24, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RICHTER_CROSS, *ATTACK_REGION_OBJECT);
        macros::ATTACK(weapon, 1, 0, Hash40::new("rot"), 6.0, 90, 20, 0, 75, 1.2, 0.0, 0.0, 3.7, Some(0.0), Some(0.0), Some(-3.7), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -6, -1.0, 24, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RICHTER_CROSS, *ATTACK_REGION_OBJECT);
    }
}

#[acmd_script( agent = "richter", script = "game_specialhi", category = ACMD_GAME, low_priority )]
unsafe fn richter_specialhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 95, 100, 155, 0, 8.5, 0.0, 9.5, 10.5, None, None, None, 1.4, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 367, 100, 80, 0, 6.0, 0.0, 26.0, 9.5, Some(0.0), Some(6.0), Some(7.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
        macros::ATTACK(fighter, 1, 1, Hash40::new("top"), 6.0, 45, 30, 0, 85, 6.0, 0.0, 26.0, 9.5, Some(0.0), Some(6.0), Some(7.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 3.0, false);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        VarModule::on_flag(fighter.battle_object, fighter::instance::flag::DISABLE_SPECIAL_HI);
    }
    frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_HI_FLAG_MOVE);
    }
}

#[acmd_script( agent = "richter", script = "game_specialairhi", category = ACMD_GAME, low_priority )]
unsafe fn richter_specialairhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 7.0 / 4.0);
    frame(fighter.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 95, 100, 155, 0, 8.5, 0.0, 9.5, 10.5, None, None, None, 1.4, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 367, 100, 80, 0, 6.0, 0.0, 26.0, 9.5, Some(0.0), Some(6.0), Some(7.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
        macros::ATTACK(fighter, 1, 1, Hash40::new("top"), 6.0, 45, 30, 0, 85, 6.0, 0.0, 26.0, 9.5, Some(0.0), Some(6.0), Some(7.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 3.0, false);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        VarModule::on_flag(fighter.battle_object, fighter::instance::flag::DISABLE_SPECIAL_HI);
    }
    frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_HI_FLAG_MOVE);
    }
}

#[acmd_script( agent = "richter", scripts = ["game_speciallw", "game_specialairlw"], category = ACMD_GAME, low_priority )]
unsafe fn richter_speciallw(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, 1.3);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_LW_FLAG_GENERATE_HOLYWATER);
    }
    frame(fighter.lua_state_agent, 18.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_LW_FLAG_SHOOT_HOLYWATER);
    }
}

pub fn install() {
    install_acmd_scripts!(
        richter_specialn,
        richter_specialn_exp,
        richter_specialnblank,
        richter_axe_fly,
        richter_specials1,
        richter_cross_fly,
        richter_cross_turn,
        richter_specialhi,
        richter_specialairhi,
        richter_speciallw
    );
}