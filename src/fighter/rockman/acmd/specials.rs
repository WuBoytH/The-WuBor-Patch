use crate::imports::acmd_imports::*;

#[acmd_script( agent = "rockman", script = "game_attacks4", category = ACMD_GAME, low_priority )]
unsafe fn rockman_specialn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 11.0);
    macros::FT_MOTION_RATE(fighter, 1.0 / 8.0);
    frame(fighter.lua_state_agent, 19.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_CHARGESHOT, false, -1);
        VarModule::off_flag(fighter.battle_object, rockman::status::flag::CHARGE_SHOT_KEEP_CHARGE);
    }
}

#[acmd_script( agent = "rockman", script = "effect_attacks4", category = ACMD_EFFECT, low_priority )]
unsafe fn rockman_specialn_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 8, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        // macros::EFFECT_FOLLOW(fighter, Hash40::new("rockman_chargeshot_hold"), Hash40::new("handl"), 4, 0, 0, 0, 0, 0, 1, true);
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

pub fn install() {
    install_acmd_scripts!(
        rockman_specialn, rockman_specialn_eff,
        rockman_chargeshot_regular,
        rockman_speciallw,
        rockman_specialairlw
    );
}