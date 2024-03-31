use crate::imports::*;

unsafe extern "C" fn game_normalw(agent: &mut L2CAgentBase) {
    let otarget_id = WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let oboma = sv_battle_object::module_accessor(otarget_id);
    let owner_module_accessor = sv_battle_object::module_accessor(otarget_id);
    if utility::get_category(&mut *oboma) == *BATTLE_OBJECT_CATEGORY_FIGHTER
    && utility::get_kind(&mut *oboma) == *FIGHTER_KIND_DOLLY
    && VarModule::is_flag(owner_module_accessor, dolly::status::flag::IS_SPECIAL_CANCEL) {
        VarModule::on_flag(agent.module_accessor, dolly_wave::instance::flag::FROM_CANCEL);
    }
    if macros::is_excute(agent) {
        let mut damage = 6.0;
        if VarModule::is_flag(agent.module_accessor, dolly_wave::instance::flag::FROM_CANCEL)  {
            let special_cancel_damage_mul = WorkModule::get_param_float(agent.module_accessor, hash40("param_misc"), hash40("special_cancel_damage_mul"));
            damage *= special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 55, 20, 0, 50, 2.4, 0.0, 12.0, -5.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 55, 20, 0, 50, 1.8, 0.0, 2.0, -10.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        let mut damage = 6.0;
        if VarModule::is_flag(agent.module_accessor, dolly_wave::instance::flag::FROM_CANCEL)  {
            let special_cancel_damage_mul = WorkModule::get_param_float(agent.module_accessor, hash40("param_misc"), hash40("special_cancel_damage_mul"));
            damage *= special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 55, 20, 0, 50, 2.4, 0.0, 9.0, -4.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 55, 20, 0, 50, 1.8, 0.0, 2.0, -8.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        let mut damage = 6.0;
        if VarModule::is_flag(agent.module_accessor, dolly_wave::instance::flag::FROM_CANCEL)  {
            let special_cancel_damage_mul = WorkModule::get_param_float(agent.module_accessor, hash40("param_misc"), hash40("special_cancel_damage_mul"));
            damage *= special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 55, 20, 0, 50, 2.4, 0.0, 6.5, -2.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 55, 20, 0, 50, 1.8, 0.0, 2.0, -6.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
}

unsafe extern "C" fn game_normal(agent: &mut L2CAgentBase) {
    let otarget_id = WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let oboma = sv_battle_object::module_accessor(otarget_id);
    let owner_module_accessor = sv_battle_object::module_accessor(otarget_id);
    if utility::get_category(&mut *oboma) == *BATTLE_OBJECT_CATEGORY_FIGHTER
    && utility::get_kind(&mut *oboma) == *FIGHTER_KIND_DOLLY
    && VarModule::is_flag(owner_module_accessor, dolly::status::flag::IS_SPECIAL_CANCEL) {
        VarModule::on_flag(agent.module_accessor, dolly_wave::instance::flag::FROM_CANCEL);
    }
    if macros::is_excute(agent) {
        let mut damage = 8.0;
        if VarModule::is_flag(agent.module_accessor, dolly_wave::instance::flag::FROM_CANCEL)  {
            let special_cancel_damage_mul = WorkModule::get_param_float(agent.module_accessor, hash40("param_misc"), hash40("special_cancel_damage_mul"));
            damage *= special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 55, 20, 0, 50, 2.4, 0.0, 12.0, -5.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 55, 20, 0, 50, 1.8, 0.0, 2.0, -10.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        let mut damage = 8.0;
        if VarModule::is_flag(agent.module_accessor, dolly_wave::instance::flag::FROM_CANCEL)  {
            let special_cancel_damage_mul = WorkModule::get_param_float(agent.module_accessor, hash40("param_misc"), hash40("special_cancel_damage_mul"));
            damage *= special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 55, 20, 0, 50, 2.4, 0.0, 9.0, -4.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 55, 20, 0, 50, 1.8, 0.0, 2.0, -8.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        let mut damage = 8.0;
        if VarModule::is_flag(agent.module_accessor, dolly_wave::instance::flag::FROM_CANCEL)  {
            let special_cancel_damage_mul = WorkModule::get_param_float(agent.module_accessor, hash40("param_misc"), hash40("special_cancel_damage_mul"));
            damage *= special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 55, 20, 0, 50, 2.4, 0.0, 6.5, -2.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 55, 20, 0, 50, 1.8, 0.0, 2.0, -6.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
}

unsafe extern "C" fn game_normalairw(agent: &mut L2CAgentBase) {
    let otarget_id = WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let oboma = sv_battle_object::module_accessor(otarget_id);
    let owner_module_accessor = sv_battle_object::module_accessor(otarget_id);
    if utility::get_category(&mut *oboma) == *BATTLE_OBJECT_CATEGORY_FIGHTER
    && utility::get_kind(&mut *oboma) == *FIGHTER_KIND_DOLLY
    && VarModule::is_flag(owner_module_accessor, dolly::status::flag::IS_SPECIAL_CANCEL) {
        VarModule::on_flag(agent.module_accessor, dolly_wave::instance::flag::FROM_CANCEL);
    }
    if macros::is_excute(agent) {
        let mut damage = 10.0;
        if VarModule::is_flag(agent.module_accessor, dolly_wave::instance::flag::FROM_CANCEL)  {
            let special_cancel_damage_mul = WorkModule::get_param_float(agent.module_accessor, hash40("param_misc"), hash40("special_cancel_damage_mul"));
            damage *= special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 10.0, -2.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 0.0, -3.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let mut damage = 10.0;
        if VarModule::is_flag(agent.module_accessor, dolly_wave::instance::flag::FROM_CANCEL)  {
            let special_cancel_damage_mul = WorkModule::get_param_float(agent.module_accessor, hash40("param_misc"), hash40("special_cancel_damage_mul"));
            damage *= special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 9.4, -0.8, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 0.0, -4.6, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let mut damage = 10.0;
        if VarModule::is_flag(agent.module_accessor, dolly_wave::instance::flag::FROM_CANCEL)  {
            let special_cancel_damage_mul = WorkModule::get_param_float(agent.module_accessor, hash40("param_misc"), hash40("special_cancel_damage_mul"));
            damage *= special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 8.8, 0.4, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 0.0, -6.2, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let mut damage = 10.0;
        if VarModule::is_flag(agent.module_accessor, dolly_wave::instance::flag::FROM_CANCEL)  {
            let special_cancel_damage_mul = WorkModule::get_param_float(agent.module_accessor, hash40("param_misc"), hash40("special_cancel_damage_mul"));
            damage *= special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 8.2, 1.6, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 0.0, -7.8, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let mut damage = 10.0;
        if VarModule::is_flag(agent.module_accessor, dolly_wave::instance::flag::FROM_CANCEL)  {
            let special_cancel_damage_mul = WorkModule::get_param_float(agent.module_accessor, hash40("param_misc"), hash40("special_cancel_damage_mul"));
            damage *= special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 7.6, 2.8, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 0.0, -9.4, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let mut damage = 10.0;
        if VarModule::is_flag(agent.module_accessor, dolly_wave::instance::flag::FROM_CANCEL)  {
            let special_cancel_damage_mul = WorkModule::get_param_float(agent.module_accessor, hash40("param_misc"), hash40("special_cancel_damage_mul"));
            damage *= special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 7.0, 4.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 0.0, -11.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let mut damage = 10.0;
        if VarModule::is_flag(agent.module_accessor, dolly_wave::instance::flag::FROM_CANCEL)  {
            let special_cancel_damage_mul = WorkModule::get_param_float(agent.module_accessor, hash40("param_misc"), hash40("special_cancel_damage_mul"));
            damage *= special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 5.8, 6.4, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 0.0, -14.2, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let mut damage = 10.0;
        if VarModule::is_flag(agent.module_accessor, dolly_wave::instance::flag::FROM_CANCEL)  {
            let special_cancel_damage_mul = WorkModule::get_param_float(agent.module_accessor, hash40("param_misc"), hash40("special_cancel_damage_mul"));
            damage *= special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 5.2, 7.6, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 0.0, -15.8, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let mut damage = 10.0;
        if VarModule::is_flag(agent.module_accessor, dolly_wave::instance::flag::FROM_CANCEL)  {
            let special_cancel_damage_mul = WorkModule::get_param_float(agent.module_accessor, hash40("param_misc"), hash40("special_cancel_damage_mul"));
            damage *= special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 4.6, 8.8, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 0.0, -17.4, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
}

unsafe extern "C" fn game_normalair(agent: &mut L2CAgentBase) {
    let otarget_id = WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let oboma = sv_battle_object::module_accessor(otarget_id);
    let owner_module_accessor = sv_battle_object::module_accessor(otarget_id);
    if utility::get_category(&mut *oboma) == *BATTLE_OBJECT_CATEGORY_FIGHTER
    && utility::get_kind(&mut *oboma) == *FIGHTER_KIND_DOLLY
    && VarModule::is_flag(owner_module_accessor, dolly::status::flag::IS_SPECIAL_CANCEL) {
        VarModule::on_flag(agent.module_accessor, dolly_wave::instance::flag::FROM_CANCEL);
    }
    if macros::is_excute(agent) {
        let mut damage = 11.0;
        if VarModule::is_flag(agent.module_accessor, dolly_wave::instance::flag::FROM_CANCEL)  {
            let special_cancel_damage_mul = WorkModule::get_param_float(agent.module_accessor, hash40("param_misc"), hash40("special_cancel_damage_mul"));
            damage *= special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 10.0, -2.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 0.0, -3.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let mut damage = 11.0;
        if VarModule::is_flag(agent.module_accessor, dolly_wave::instance::flag::FROM_CANCEL)  {
            let special_cancel_damage_mul = WorkModule::get_param_float(agent.module_accessor, hash40("param_misc"), hash40("special_cancel_damage_mul"));
            damage *= special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 9.2, -0.2, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 0.0, -5.4, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let mut damage = 11.0;
        if VarModule::is_flag(agent.module_accessor, dolly_wave::instance::flag::FROM_CANCEL)  {
            let special_cancel_damage_mul = WorkModule::get_param_float(agent.module_accessor, hash40("param_misc"), hash40("special_cancel_damage_mul"));
            damage *= special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 8.4, 1.6, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 0.0, -7.8, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let mut damage = 11.0;
        if VarModule::is_flag(agent.module_accessor, dolly_wave::instance::flag::FROM_CANCEL)  {
            let special_cancel_damage_mul = WorkModule::get_param_float(agent.module_accessor, hash40("param_misc"), hash40("special_cancel_damage_mul"));
            damage *= special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 7.6, 3.4, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 0.0, -10.2, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let mut damage = 11.0;
        if VarModule::is_flag(agent.module_accessor, dolly_wave::instance::flag::FROM_CANCEL)  {
            let special_cancel_damage_mul = WorkModule::get_param_float(agent.module_accessor, hash40("param_misc"), hash40("special_cancel_damage_mul"));
            damage *= special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 6.8, 5.2, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 0.0, -12.6, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let mut damage = 11.0;
        if VarModule::is_flag(agent.module_accessor, dolly_wave::instance::flag::FROM_CANCEL)  {
            let special_cancel_damage_mul = WorkModule::get_param_float(agent.module_accessor, hash40("param_misc"), hash40("special_cancel_damage_mul"));
            damage *= special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 6.0, 7.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 0.0, -15.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let mut damage = 11.0;
        if VarModule::is_flag(agent.module_accessor, dolly_wave::instance::flag::FROM_CANCEL)  {
            let special_cancel_damage_mul = WorkModule::get_param_float(agent.module_accessor, hash40("param_misc"), hash40("special_cancel_damage_mul"));
            damage *= special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 5.2, 8.8, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 0.0, -17.4, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_normalw", game_normalw);

    agent.acmd("game_normal", game_normal);

    agent.acmd("game_normalairw", game_normalairw);

    agent.acmd("game_normalair", game_normalair);
}