use crate::imports::*;

unsafe extern "C" fn game_charge(agent: &mut L2CAgentBase) {
    if VarModule::is_flag(agent.module_accessor, lucario_auraball::instance::flag::SPIRIT_BOMB) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 366, 49, 20, 60, 2.2, 0.0, 0.0, 0.0, None, None, None, 0.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2.3, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
    }
}

unsafe extern "C" fn sound_charge(agent: &mut L2CAgentBase) {
    if VarModule::is_flag(agent.module_accessor, lucario_auraball::instance::flag::SPIRIT_BOMB) {
        if macros::is_excute(agent) {
            macros::PLAY_STATUS(agent, Hash40::new("se_lucario_special_n01_l"));
        }
    }
}

unsafe extern "C" fn game_shoot(agent: &mut L2CAgentBase) {
    if !VarModule::is_flag(agent.module_accessor, lucario_auraball::instance::flag::SPIRIT_BOMB) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 361, 42, 0, 14, 2.2, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2.3, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 361, 49, 0, 35, 2.2, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2.3, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            attack!(agent, MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
            AttackModule::enable_safe_pos(agent.module_accessor);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 366, 49, 20, 60, 2.2, 0.0, 0.0, 0.0, None, None, None, 0.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2.3, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.5, 366, 49, 20, 60, 2.2, 0.0, 0.0, 0.0, None, None, None, 0.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2.3, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            attack!(agent, MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
            AttackModule::enable_safe_pos(agent.module_accessor);
        }
    }
}

unsafe extern "C" fn sound_shoot(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_lucario_special_n01"));
        macros::STOP_SE(agent, Hash40::new_raw(0x16b0e86b15));
        if VarModule::is_flag(agent.module_accessor, lucario_auraball::instance::flag::SPIRIT_BOMB) {
            macros::PLAY_STATUS(agent, Hash40::new("se_lucario_special_n01_l"));
        }
    }
    if macros::is_excute(agent) {
        let charge_rate = WorkModule::get_float(agent.module_accessor, *WEAPON_LUCARIO_AURABALL_INSTANCE_WORK_ID_FLOAT_CHARGE_RATE);
        let aurapower = WorkModule::get_float(agent.module_accessor, *WEAPON_LUCARIO_AURABALL_INSTANCE_WORK_ID_FLOAT_AURAPOWER);
        let aurapower_mid = WorkModule::get_float(agent.module_accessor, *WEAPON_LUCARIO_AURABALL_INSTANCE_WORK_ID_FLOAT_SE_AURAPOWER_MIDDLE);
        let aurapower_hi = WorkModule::get_float(agent.module_accessor, *WEAPON_LUCARIO_AURABALL_INSTANCE_WORK_ID_FLOAT_SE_AURAPOWER_HIGH);
        let se = if charge_rate <= 0.45 {
            if aurapower < aurapower_mid {
                Hash40::new("se_lucario_special_n05_s")
            }
            else if aurapower < aurapower_hi {
                Hash40::new("se_lucario_special_n06_s")
            }
            else {
                Hash40::new("se_lucario_special_n07_s")
            }
        }
        else if charge_rate <= 0.9 {
            if aurapower < aurapower_mid {
                Hash40::new("se_lucario_special_n05_m")
            }
            else if aurapower < aurapower_hi {
                Hash40::new("se_lucario_special_n06_m")
            }
            else {
                Hash40::new("se_lucario_special_n07_m")
            }
        }
        else {
            if aurapower < aurapower_mid {
                Hash40::new("se_lucario_special_n05_l")
            }
            else if aurapower < aurapower_hi {
                Hash40::new("se_lucario_special_n06_l")
            }
            else {
                Hash40::new("se_lucario_special_n07_l")
            }
        };
        macros::PLAY_SE(agent, se);
    }
}

unsafe extern "C" fn expression_auraballlosion(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 70, 70, 0, 80, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2.3, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}

unsafe extern "C" fn effect_explosion(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new_raw(0x15cff20136), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_charge", game_charge);
    agent.acmd("sound_charge", sound_charge);

    agent.acmd("game_shoot", game_shoot);
    agent.acmd("sound_shoot", sound_shoot);

    agent.acmd("game_explosion", expression_auraballlosion);
    agent.acmd("effect_explosion", effect_explosion);
}