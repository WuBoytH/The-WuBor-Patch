use crate::imports::acmd_imports::*;

#[acmd_script( agent = "toonlink", script = "game_attackairn", category = ACMD_GAME, low_priority )]
unsafe extern "C" fn toonlink_attackairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword2"), 8.5, 361, 80, 0, 45, 4.0, 4.3, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword2"), 8.5, 361, 80, 0, 45, 4.5, 0.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("arml"), 8.5, 361, 80, 0, 45, 4.5, -0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("colonellm"), 8.5, 361, 80, 0, 45, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword2"), 7.0, 361, 80, 0, 45, 4.0, 4.3, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword2"), 7.0, 361, 80, 0, 45, 4.5, 0.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("arml"), 7.0, 361, 80, 0, 45, 4.5, -0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("colonellm"), 7.0, 361, 80, 0, 45, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "toonlink", script = "game_attackairhi", category = ACMD_GAME, low_priority )]
unsafe extern "C" fn toonlink_attackairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 11.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, fighter::status::flag::JUMP_CANCEL);
        macros::ATTACK(agent, 0, 0, Hash40::new("sword2"), 14.0, 80, 95, 0, 25, 4.6, 5.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword2"), 14.0, 80, 95, 0, 25, 5.2, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("arml"), 14.0, 80, 95, 0, 25, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATK_POWER(agent, 0, 11.0);
        macros::ATK_POWER(agent, 1, 11.0);
        macros::ATK_POWER(agent, 2, 11.0);
    }
    frame(agent.lua_state_agent, 15.0);
    macros::FT_MOTION_RATE(agent, 0.2);
    frame(agent.lua_state_agent, 35.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        VarModule::off_flag(agent.module_accessor, fighter::status::flag::JUMP_CANCEL);
    }
}

#[acmd_script( agent = "toonlink", script = "game_attackairlw", category = ACMD_GAME, low_priority )]
unsafe extern "C" fn toonlink_attackairlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(agent, 0, 1.6, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::suspend_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        if StatusModule::prev_status_kind(agent.module_accessor, 0) == *FIGHTER_STATUS_KIND_DAMAGE_FLY {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
        }
        JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(agent, 0, -3.8, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LW_SET_ATTACK);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 268, 60, 0, 30, 5.5, 1.0, -1.1, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 50, 98, 0, 40, 5.0, 1.0, -1.1, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        VarModule::on_flag(agent.module_accessor, fighter::status::flag::NORMAL_CANCEL);
    }
    frame(agent.lua_state_agent, 65.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LW_SET_ATTACK);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 71.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
        KineticModule::resume_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
}

#[acmd_script( agent = "toonlink", script = "effect_attackairlw", category = ACMD_EFFECT, low_priority )]
unsafe extern "C" fn toonlink_attackairlw_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("havel"), 0, -8, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.7);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("toonlink_sword"), Hash40::new("sword1"), -0.03, 0.05, 0, 0, 0, -0.87, 1.01, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("toonlink_sword_flare"), Hash40::new("sword1"), -0.03, 0.05, 0, 0, 0, -0.87, 1.01, true);
    }
    frame(agent.lua_state_agent, 15.0);
    for _ in 0..17 {
        if !AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT) {
            if macros::is_excute(agent) {
                macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 3, 0, -90, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, true);
            }
        }
        wait(agent.lua_state_agent, 3.0);
    }
}

pub fn install(agent : &mut smashline::Agent) {
    agent.game_acmd("game_attackairn", toonlink_attackairn);

    agent.game_acmd("game_attackairn", toonlink_attackairhi);

    agent.game_acmd("game_attackairlw", toonlink_attackairlw);
    agent.effect_acmd_acmd("effect_attackairlw", toonlink_attackairlw_eff);
}