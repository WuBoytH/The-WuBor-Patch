use crate::imports::acmd_imports::*;

#[acmd_script( agent = "rockman", scripts = [ "game_busterchargeshot", "game_busterairchargeshot" ], category = ACMD_GAME, low_priority )]
unsafe fn rockman_specialn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    macros::FT_MOTION_RATE(fighter, 1.0 / 7.0);
    frame(fighter.lua_state_agent, 18.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_CHARGESHOT, false, -1);
        VarModule::off_flag(fighter.battle_object, rockman::status::flag::CHARGE_SHOT_KEEP_CHARGE);
    }
}

#[acmd_script( agent = "rockman", scripts = [ "effect_busterchargeshot", "effect_busterairchargeshot" ], category = ACMD_EFFECT, low_priority )]
unsafe fn rockman_specialn_eff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 8, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("rockman_chargeshot_elec"), Hash40::new("havel"), 0, 0, -1.5, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("rockman_chargeshot_shot"), Hash40::new("rockman_chargeshot_shot"), Hash40::new("top"), 0, 7.2, 9, 0, 0, 0, 1, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "rockman", scripts = [ "sound_busterchargeshot", "sound_busterairchargeshot" ], category = ACMD_SOUND, low_priority )]
unsafe fn rockman_specialn_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_rockman_smash_s02"));
    }
}

#[acmd_script( agent = "rockman", scripts = [ "expression_busterchargeshot", "expression_busterairchargeshot" ], category = ACMD_EXPRESSION, low_priority )]
unsafe fn rockman_specialn_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f5b14bb65), *FIGHTER_ROCKMAN_ARM_LEFT, *FIGHTER_ROCKMAN_ARMFORM_ROCKBUSTER, 5);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f5b14bb65), *FIGHTER_ROCKMAN_ARM_RIGHT, *FIGHTER_ROCKMAN_ARMFORM_HAND, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 18.0);
    if WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_HOLD_RATE) < 1.0 {
        if macros::is_excute(fighter) {
            macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_beams"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_beaml"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
}

#[acmd_script( agent = "rockman_chargeshot", script = "game_regular", category = ACMD_GAME, low_priority )]
unsafe fn rockman_chargeshot_regular(weapon: &mut L2CAgentBase) {
    if macros::is_excute(weapon) {
        let is_charge_max = 1.0 <= WorkModule::get_float(weapon.module_accessor, *WEAPON_ROCKMAN_CHARGESHOT_INSTANCE_WORK_ID_FLOAT_HOLD_RATE);
        let damage;
        let bkb;
        let kbg;
        if is_charge_max {
            damage = 15.0;
            bkb = 40;
            kbg = 90;
        }
        else {
            damage = 9.0;
            bkb = 50;
            kbg = 85;
        }
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), damage, 361, kbg, 0, bkb, 2.6, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
        macros::ATK_SET_SHIELD_SETOFF_MUL(weapon, 0, 0.32);
        AttackModule::enable_safe_pos(weapon.module_accessor);
    }
}

#[acmd_script( agent = "rockman", script = "game_speciallw", category = ACMD_GAME, low_priority )]
unsafe fn rockman_speciallw(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, 5.0);
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article_enable(fighter.module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_LEAFSHIELD, false, -1);
    }
    macros::FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "rockman", script = "game_specialairlw", category = ACMD_GAME, low_priority )]
unsafe fn rockman_specialairlw(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, 5.0);
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article_enable(fighter.module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_LEAFSHIELD, false, -1);
    }
    macros::FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "rockman_leafshield", scripts = [ "game_start", "game_startreverse" ], category = ACMD_GAME, low_priority )]
unsafe fn rockman_leafshield_start(weapon: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(weapon, 4.0 / 3.0);
    if macros::is_excute(weapon) {
        if !WorkModule::is_flag(weapon.module_accessor, 0x20000006) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_0
            macros::ATTACK(weapon, 0, 0, Hash40::new("leafshield1"), 1.5, 361, 20, 0, 35, 1.3, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 11, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(weapon.module_accessor, 0x20000007) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_1
            macros::ATTACK(weapon, 1, 0, Hash40::new("leafshield2"), 1.5, 361, 20, 0, 35, 1.3, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 11, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(weapon.module_accessor, 0x20000008) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_2
            macros::ATTACK(weapon, 2, 0, Hash40::new("leafshield3"), 1.5, 361, 20, 0, 35, 1.3, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 11, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(weapon.module_accessor, 0x20000009) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_3
            macros::ATTACK(weapon, 3, 0, Hash40::new("leafshield4"), 1.5, 361, 20, 0, 35, 1.3, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 11, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
    }
}

#[acmd_script( agent = "rockman_leafshield", scripts = [ "game_shield", "game_shieldreverse" ], category = ACMD_GAME, low_priority )]
unsafe fn rockman_leafshield_shield(weapon: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(weapon, 4.0 / 3.0);
    if macros::is_excute(weapon) {
        if !WorkModule::is_flag(weapon.module_accessor, 0x20000006) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_0
            macros::ATTACK(weapon, 0, 0, Hash40::new("leafshield1"), 1.5, 361, 20, 0, 35, 1.3, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 11, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(weapon.module_accessor, 0x20000007) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_1
            macros::ATTACK(weapon, 1, 0, Hash40::new("leafshield2"), 1.5, 361, 20, 0, 35, 1.3, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 11, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(weapon.module_accessor, 0x20000008) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_2
            macros::ATTACK(weapon, 2, 0, Hash40::new("leafshield3"), 1.5, 361, 20, 0, 35, 1.3, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 11, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(weapon.module_accessor, 0x20000009) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_3
            macros::ATTACK(weapon, 3, 0, Hash40::new("leafshield4"), 1.5, 361, 20, 0, 35, 1.3, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 11, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
    }
}

#[acmd_script( agent = "rockman_leafshield", scripts = [ "game_fly", "game_flyreverse" ], category = ACMD_GAME, low_priority )]
unsafe fn rockman_leafshield_fly(weapon: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(weapon, 4.0 / 3.0);
    if macros::is_excute(weapon) {
        if !WorkModule::is_flag(weapon.module_accessor, 0x20000006) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_0
            macros::ATTACK(weapon, 0, 0, Hash40::new("leafshield1"), 3.8, 65, 70, 0, 50, 1.8, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 11, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(weapon.module_accessor, 0x20000007) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_1
            macros::ATTACK(weapon, 1, 0, Hash40::new("leafshield2"), 3.8, 65, 70, 0, 50, 1.8, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 11, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(weapon.module_accessor, 0x20000008) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_2
            macros::ATTACK(weapon, 2, 0, Hash40::new("leafshield3"), 3.8, 65, 70, 0, 50, 1.8, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 11, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(weapon.module_accessor, 0x20000009) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_3
            macros::ATTACK(weapon, 3, 0, Hash40::new("leafshield4"), 3.8, 65, 70, 0, 50, 1.8, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 11, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        rockman_specialn, rockman_specialn_eff, rockman_specialn_snd, rockman_specialn_exp,
        rockman_chargeshot_regular,
        rockman_speciallw,
        rockman_specialairlw,
        rockman_leafshield_start, rockman_leafshield_shield, rockman_leafshield_fly
    );
}