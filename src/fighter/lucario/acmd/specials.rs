use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::Hash40,
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    custom_var::*,
    wubor_utils::vars::*
};

#[acmd_script( agent = "lucario", scripts = ["game_specialnshoot", "game_specialairnshoot"], category = ACMD_GAME, low_priority )]
unsafe fn lucario_specialnshoot(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
    }
}

#[acmd_script( agent = "lucario", script = "game_specials", category = ACMD_GAME, low_priority )]
unsafe fn lucario_specials(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 2.0, 5.0);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 3.0, 0.0, 6.0, 6.3, Some(0.0), Some(6.0), Some(2.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("top"), 3.0, 0.0, 6.0, 8.4, Some(0.0), Some(6.0), Some(2.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        if macros::IS_GENERATABLE_ARTICLE(fighter, *FIGHTER_LUCARIO_GENERATE_ARTICLE_QIGONG) == true {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_QIGONG, false, 0);
        }
    }
}

#[acmd_script( agent = "lucario", script = "game_specialhi", category = ACMD_GAME, low_priority )]
unsafe fn lucario_specialhi(fighter: &mut L2CAgentBase) {
    if VarModule::is_flag(fighter.battle_object, commons::instance::flag::IS_FGC) {
        VarModule::off_flag(fighter.battle_object, lucario::instance::flag::IS_SUPER_DASH_CANCEL);
        macros::FT_MOTION_RATE(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_RUSH_DIR);
    }
}

#[acmd_script( agent = "lucario", script = "game_specialairhi", category = ACMD_GAME, low_priority )]
unsafe fn lucario_specialairhi(fighter: &mut L2CAgentBase) {
    if VarModule::is_flag(fighter.battle_object, commons::instance::flag::IS_FGC) {
        VarModule::on_flag(fighter.battle_object, commons::instance::flag::DISABLE_SPECIAL_HI);
        VarModule::off_flag(fighter.battle_object, lucario::instance::flag::IS_SUPER_DASH_CANCEL);
        macros::FT_MOTION_RATE(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_GRAVITY_ONOFF);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_RUSH_DIR);
    }
}

#[acmd_script( agent = "lucario", script = "game_specialhimove", category = ACMD_GAME, low_priority )]
unsafe fn lucario_specialhimove(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, commons::instance::flag::IS_FGC) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 5.0, 55, 100, 30, 60, 12.0, 0.0, 0.0, 0.0, None, None, None, 1.4, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_NONE);
        }
    }
}

#[acmd_script( agent = "lucario", script = "game_specialairhimove", category = ACMD_GAME, low_priority )]
unsafe fn lucario_specialairhimove(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, commons::instance::flag::IS_FGC) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 5.0, 55, 100, 30, 60, 12.0, 0.0, 0.0, 0.0, None, None, None, 1.4, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_NONE);
        }
    }
}

#[acmd_script( agent = "lucario", script = "game_specialhiend", category = ACMD_GAME, low_priority )]
unsafe fn lucario_specialhiend(fighter: &mut L2CAgentBase) {
    wait(fighter.lua_state_agent, 1.0);
    if !VarModule::is_flag(fighter.battle_object, commons::instance::flag::IS_FGC) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 6.0, 38, 100, 0, 70, 12.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_NONE);
        }
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    wait(fighter.lua_state_agent, 10.0);
    if VarModule::is_flag(fighter.battle_object, commons::instance::flag::IS_FGC) {
        if macros::is_excute(fighter) {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
}

#[acmd_script( agent = "lucario", script = "game_specialairhiend", category = ACMD_GAME, low_priority )]
unsafe fn lucario_specialairhiend(fighter: &mut L2CAgentBase) {
    wait(fighter.lua_state_agent, 1.0);
    if !VarModule::is_flag(fighter.battle_object, commons::instance::flag::IS_FGC) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 6.0, 38, 100, 0, 70, 12.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_NONE);
        }
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        if VarModule::is_flag(fighter.battle_object, commons::instance::flag::IS_FGC) {
            if VarModule::is_flag(fighter.battle_object, lucario::instance::flag::IS_SUPER_DASH_CANCEL) {
                macros::SET_SPEED_EX(fighter, 1.1, 1.8, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
        }
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_AIR_END_CONTROL_X);
    }
}

#[acmd_script( agent = "lucario_qigong", script = "game_shoot", category = ACMD_GAME, low_priority )]
unsafe fn lucario_qigong_shoot(weapon: &mut L2CAgentBase) {
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 11.0, 43, 58, 0, 48, 5.6, 0.0, 0.0, 4.0, Some(0.0), Some(0.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.2, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_NONE);
        macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 5.0, 43, 58, 0, 48, 3.0, 0.0, 0.0, 2.0, Some(0.0), Some(0.0), Some(22.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.2, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_NONE);
        macros::ATTACK(weapon, 3, 0, Hash40::new("top"), 11.0, 43, 58, 0, 48, 2.0, 0.0, -1.0, -4.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.2, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_NONE);
    }
    wait(weapon.lua_state_agent, 2.0);
    if macros::is_excute(weapon) {
        AttackModule::clear_all(weapon.module_accessor);
    }
    wait(weapon.lua_state_agent, 8.0);
    if macros::is_excute(weapon) {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
}

pub fn install() {
    install_acmd_scripts!(
        lucario_specialnshoot,
        lucario_specials,
        lucario_specialhi,
        lucario_specialairhi,
        lucario_specialhimove,
        lucario_specialairhimove,
        lucario_specialhiend,
        lucario_specialairhiend,
        lucario_qigong_shoot
    );
}