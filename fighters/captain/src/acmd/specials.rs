use super::*;

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        spend_boost_power(agent);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_TURN);
    }
    if VarModule::is_flag(agent.module_accessor, vars::captain::status::flag::USED_BOOST_POWER) {
        macros::FT_MOTION_RATE(agent, 20.0 / 38.0);
    }
    frame(agent.lua_state_agent, 53.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_GENERATE_BIRD);
        macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 25.0, 361, 59, 0, 93, 4.5, -1.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 25.0, 361, 59, 0, 93, 2.5, -2.5, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 25.0, 361, 59, 0, 93, 4.0, 4.2, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 25.0, 361, 59, 0, 93, 2.5, -3.0, 0.0, 2.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 25.0, 361, 59, 0, 93, 5.0, 1.0, 0.0, 2.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 25.0, 361, 59, 0, 93, 4.0, 5.0, 0.0, 2.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
    }
    if VarModule::is_flag(agent.module_accessor, vars::captain::status::flag::USED_BOOST_POWER) {
        if macros::is_excute(agent) {
            VarModule::on_flag(agent.module_accessor, vars::captain::status::flag::SPECIAL_N_BOOST_POWER_KINETIC_SHIFT);
            GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
        macros::FT_MOTION_RATE(agent, 20.0 / 5.0);
    }
    wait(agent.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        // KineticModule::resume_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }
    if VarModule::is_flag(agent.module_accessor, vars::captain::status::flag::USED_BOOST_POWER) {
        if macros::is_excute(agent) {
            VarModule::on_flag(agent.module_accessor, vars::captain::status::flag::SPECIAL_N_BOOST_POWER_KINETIC_SHIFT_END);
            GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        }
    }
}

unsafe extern "C" fn game_specialnturn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if VarModule::is_flag(agent.module_accessor, vars::captain::status::flag::USED_BOOST_POWER) {
        macros::FT_MOTION_RATE(agent, 25.0 / 43.0);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::REVERSE_LR(agent);
    }
    frame(agent.lua_state_agent, 48.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_GENERATE_BIRD);
        macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 28.0, 361, 65, 0, 85, 4.5, -1.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 28.0, 361, 65, 0, 85, 2.5, -2.5, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 28.0, 361, 65, 0, 85, 4.0, 4.2, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 28.0, 361, 65, 0, 85, 2.5, -3.0, 0.0, 2.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 28.0, 361, 65, 0, 85, 5.0, 1.0, 0.0, 2.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 28.0, 361, 65, 0, 85, 4.0, 5.0, 0.0, 2.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
    }
    if VarModule::is_flag(agent.module_accessor, vars::captain::status::flag::USED_BOOST_POWER) {
        if macros::is_excute(agent) {
            KineticModule::unable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            VarModule::on_flag(agent.module_accessor, vars::captain::status::flag::SPECIAL_N_BOOST_POWER_KINETIC_SHIFT);
            GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
        macros::FT_MOTION_RATE(agent, 20.0 / 5.0);
    }
    wait(agent.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        // KineticModule::resume_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }
    if VarModule::is_flag(agent.module_accessor, vars::captain::status::flag::USED_BOOST_POWER) {
        if macros::is_excute(agent) {
            VarModule::on_flag(agent.module_accessor, vars::captain::status::flag::SPECIAL_N_BOOST_POWER_KINETIC_SHIFT_END);
            GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        }
    }
}

unsafe extern "C" fn game_specialairn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        KineticModule::add_speed(agent.module_accessor, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
    }
    frame(agent.lua_state_agent, 8.0);
    if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        spend_boost_power(agent);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_TURN);
    }
    if VarModule::is_flag(agent.module_accessor, vars::captain::status::flag::USED_BOOST_POWER) {
        macros::FT_MOTION_RATE(agent, 20.0 / 38.0);
    }
    frame(agent.lua_state_agent, 51.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_DIR_DECIDE);
        WorkModule::set_int(agent.module_accessor, 1, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_PUNCH_AIR_PHASE);
    }
    frame(agent.lua_state_agent, 53.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_GENERATE_BIRD);
        macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 22.0, 361, 59, 0, 93, 5.175, -1.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 22.0, 361, 59, 0, 93, 2.875, -2.5, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 22.0, 361, 59, 0, 93, 4.6, 4.2, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 22.0, 361, 68, 0, 58, 2.5, -3.0, 0.0, 2.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 22.0, 361, 68, 0, 58, 5.0, 1.0, 0.0, 2.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 22.0, 361, 68, 0, 58, 4.0, 5.0, 0.0, 2.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
    }
    if VarModule::is_flag(agent.module_accessor, vars::captain::status::flag::USED_BOOST_POWER) {
        if macros::is_excute(agent) {
            KineticModule::unable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            VarModule::on_flag(agent.module_accessor, vars::captain::status::flag::SPECIAL_N_BOOST_POWER_KINETIC_SHIFT);
        }
        macros::FT_MOTION_RATE(agent, 20.0 / 5.0);
    }
    wait(agent.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    if VarModule::is_flag(agent.module_accessor, vars::captain::status::flag::USED_BOOST_POWER) {
        if macros::is_excute(agent) {
            VarModule::on_flag(agent.module_accessor, vars::captain::status::flag::SPECIAL_N_BOOST_POWER_KINETIC_SHIFT_END);
        }
    }
    wait(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, 2, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_PUNCH_AIR_PHASE);
    }
}

unsafe extern "C" fn game_specialairnturn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if VarModule::is_flag(agent.module_accessor, vars::captain::status::flag::USED_BOOST_POWER) {
        macros::FT_MOTION_RATE(agent, 25.0 / 43.0);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::REVERSE_LR(agent);
    }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_DIR_DECIDE);
        WorkModule::set_int(agent.module_accessor, 1, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_PUNCH_AIR_PHASE);
    }
    frame(agent.lua_state_agent, 48.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_GENERATE_BIRD);
        macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 25.0, 361, 59, 0, 93, 5.175, -1.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 25.0, 361, 59, 0, 93, 2.875, -2.5, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 25.0, 361, 59, 0, 93, 4.6, 4.2, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 22.0, 361, 68, 0, 58, 2.5, -3.0, 0.0, 2.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 22.0, 361, 68, 0, 58, 5.0, 1.0, 0.0, 2.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 22.0, 361, 68, 0, 58, 4.0, 5.0, 0.0, 2.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
    }
    if VarModule::is_flag(agent.module_accessor, vars::captain::status::flag::USED_BOOST_POWER) {
        if macros::is_excute(agent) {
            KineticModule::unable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            VarModule::on_flag(agent.module_accessor, vars::captain::status::flag::SPECIAL_N_BOOST_POWER_KINETIC_SHIFT);
        }
        macros::FT_MOTION_RATE(agent, 20.0 / 5.0);
    }
    wait(agent.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    if VarModule::is_flag(agent.module_accessor, vars::captain::status::flag::USED_BOOST_POWER) {
        if macros::is_excute(agent) {
            VarModule::on_flag(agent.module_accessor, vars::captain::status::flag::SPECIAL_N_BOOST_POWER_KINETIC_SHIFT_END);
        }
    }
    wait(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, 2, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_PUNCH_AIR_PHASE);
    }
}

unsafe extern "C" fn game_specialsstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    // macros::FT_MOTION_RATE(agent, 0.75);
    frame(agent.lua_state_agent, 8.0);
    if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        spend_boost_power(agent);
    }
    frame(agent.lua_state_agent, 9.0);
    // macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
    }
    if VarModule::is_flag(agent.module_accessor, vars::captain::status::flag::USED_BOOST_POWER) {
        if macros::is_excute(agent) {
            sv_kinetic_energy!(
                set_speed_mul,
                agent,
                FIGHTER_KINETIC_ENERGY_ID_MOTION,
                1.5
            );
        }
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 4.0, 0.0, 9.0, 8.8, Some(0.0), Some(5.0), Some(8.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 4.0, 0.0, 12.0, 8.8, Some(0.0), Some(5.0), Some(8.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KNUCKLE_HIT_CHECK_ONOFF);
        // VarModule::on_flag(agent.module_accessor, vars::captain::status::flag::SPECIAL_S_ENABLE_MANUAL_ATTACK);
    }
    if VarModule::is_flag(agent.module_accessor, vars::captain::status::flag::USED_BOOST_POWER) {
        macros::FT_MOTION_RATE(agent, 24.0 / 16.0);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KNUCKLE_HIT_CHECK_ONOFF);
        // VarModule::off_flag(agent.module_accessor, vars::captain::status::flag::SPECIAL_S_ENABLE_MANUAL_ATTACK);
        JostleModule::set_status(agent.module_accessor, true);
    }
}

unsafe extern "C" fn game_specialsend(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    // macros::FT_MOTION_RATE(agent, 0.8);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
    }
    if VarModule::is_flag(agent.module_accessor, vars::captain::status::flag::USED_BOOST_POWER) {
        frame(agent.lua_state_agent, 6.0);
        macros::FT_MOTION_RATE(agent, 3.0 / 1.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("handl"), 5.0, 368, 100, 0, 0, 6.0, 0.0, -2.0, -1.0, None, None, None, 3.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 368, 100, 0, 0, 5.5, 0.0, 9.0, 5.5, Some(0.0), Some(9.0), Some(12.5), 3.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 5.5, y: 11.0}, 4, false);
        }
        wait(agent.lua_state_agent, 1.0 / 3.0);
        if macros::is_excute(agent) {
            AttackModule::clear(agent.module_accessor, 1, false);
            damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
            macros::ATTACK(agent, 0, 0, Hash40::new("handl"), 2.0, 368, 100, 0, 0, 6.0, 0.0, -2.0, -1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 4, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 2.0, y: 12.0}, 2, false);
        }
        frame(agent.lua_state_agent, 7.0);
        macros::FT_MOTION_RATE(agent, 1.0);
        if macros::is_excute(agent) {
            VarModule::on_flag(agent.module_accessor, vars::captain::status::flag::ENABLE_BOOST_POWER_CRITICAL);
            macros::ATTACK(agent, 0, 0, Hash40::new("handl"), 9.0, 85, 75, 0, 100, 4.5, 0.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        }
        frame(agent.lua_state_agent, 9.0);
        macros::FT_MOTION_RATE(agent, 0.75);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            JostleModule::set_status(agent.module_accessor, true);
        }
    }
    else {
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("handl"), 10.0, 75, 35, 0, 82, 6.0, 0.0, -2.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 75, 35, 0, 82, 5.5, 0.0, 9.0, 5.5, Some(0.0), Some(9.0), Some(12.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        }
        wait(agent.lua_state_agent, 1.0);
        // macros::FT_MOTION_RATE(agent, 1.0);
        if macros::is_excute(agent) {
            AttackModule::clear(agent.module_accessor, 1, false);
            macros::ATTACK(agent, 0, 0, Hash40::new("handl"), 10.0, 75, 35, 0, 82, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        }
        wait(agent.lua_state_agent, 2.0);
        macros::FT_MOTION_RATE(agent, 0.8);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            JostleModule::set_status(agent.module_accessor, true);
        }
    }
}

unsafe extern "C" fn game_specialairsstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
    frame(agent.lua_state_agent, 8.0);
    if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        spend_boost_power(agent);
    }
    frame(agent.lua_state_agent, 9.0);
    if VarModule::is_flag(agent.module_accessor, vars::captain::status::flag::USED_BOOST_POWER) {
        if macros::is_excute(agent) {
            sv_kinetic_energy!(
                set_speed_mul,
                agent,
                FIGHTER_KINETIC_ENERGY_ID_MOTION,
                1.5
            );
        }
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BACK);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 4.1, 0.0, 12.0, 11.0, Some(0.0), Some(3.0), Some(11.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KNUCKLE_HIT_CHECK_ONOFF);
    }
    if VarModule::is_flag(agent.module_accessor, vars::captain::status::flag::USED_BOOST_POWER) {
        macros::FT_MOTION_RATE(agent, 24.0 / 17.0);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KNUCKLE_GRAVITY_ONOFF);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KNUCKLE_HIT_CHECK_ONOFF);
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_specialairsend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
    if VarModule::is_flag(agent.module_accessor, vars::captain::status::flag::USED_BOOST_POWER) {
        frame(agent.lua_state_agent, 5.0);
        macros::FT_MOTION_RATE(agent, 3.0 / 1.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("arml"), 5.0, 367, 100, 0, 0, 8.0, 8.0, 0.0, 0.4, None, None, None, 3.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 2, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("arml"), 5.0, 367, 100, 0, 0, 8.0, 8.0, 0.0, 0.4, None, None, None, 3.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 2, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 2, 0, Hash40::new("arml"), 5.0, 367, 100, 0, 0, 2.5, -2.0, 0.0, 0.0, Some(-5.0), Some(0.0), None, 3.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 2, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        }
        wait(agent.lua_state_agent, 1.0 / 3.0);
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &Vector3f{x: -1.5, y: 2.5, z: 0.0});
            macros::ATTACK(agent, 0, 0, Hash40::new("arml"), 2.0, 367, 100, 0, 0, 8.0, 8.0, 0.0, 0.4, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 2, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("arml"), 2.0, 367, 100, 0, 0, 8.0, 8.0, 0.0, 0.4, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 2, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 367, 100, 0, 0, 4.0, 0.0, 8.0, 5.5, Some(0.0), Some(1.0), Some(3.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 2, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        }
        frame(agent.lua_state_agent, 6.0);
        macros::FT_MOTION_RATE(agent, 1.0);
        if macros::is_excute(agent) {
            VarModule::on_flag(agent.module_accessor, vars::captain::status::flag::ENABLE_BOOST_POWER_CRITICAL);
            macros::ATTACK(agent, 0, 0, Hash40::new("arml"), 9.0, 270, 70, 0, 60, 8.0, 8.0, 0.0, 0.4, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("arml"), 9.0, 270, 70, 0, 60, 8.0, 8.0, 0.0, 0.4, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 270, 70, 0, 60, 4.0, 0.0, 8.0, 5.5, Some(0.0), Some(1.0), Some(3.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        }
        frame(agent.lua_state_agent, 7.0);
        if macros::is_excute(agent) {
            AttackModule::clear(agent.module_accessor, 2, false);
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
            WorkModule::enable_transition_term(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
            AttackModule::clear_all(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 30.0);
        if macros::is_excute(agent) {
            JostleModule::set_status(agent.module_accessor, true);
        }
    }
    else {
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("arml"), 10.0, 270, 70, 0, 60, 8.0, 8.0, 0.0, 0.4, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("arml"), 10.0, 45, 80, 0, 60, 8.0, 8.0, 0.0, 0.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 2, 0, Hash40::new("arml"), 10.0, 45, 80, 0, 60, 2.5, -2.0, 0.0, 0.0, Some(-5.0), Some(0.0), None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &Vector3f{x: -1.5, y: 2.5, z: 0.0});
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 45, 80, 0, 60, 4.0, 0.0, 8.0, 5.5, Some(0.0), Some(1.0), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        }
        frame(agent.lua_state_agent, 7.0);
        if macros::is_excute(agent) {
            AttackModule::clear(agent.module_accessor, 2, false);
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
            WorkModule::enable_transition_term(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
            AttackModule::clear_all(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 30.0);
        if macros::is_excute(agent) {
            JostleModule::set_status(agent.module_accessor, true);
        }
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        spend_boost_power(agent);
    }
    if VarModule::is_flag(agent.module_accessor, vars::captain::status::flag::USED_BOOST_POWER) {
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            WorkModule::set_float(agent.module_accessor, 1.1, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_KINETIC_MOTION_SPEED_MUL);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 60, 100, 160, 0, 2.5, 0.0, 6.5, 0.0, None, None, None, 2.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 86, 100, 150, 0, 4.0, 0.0, 6.5, 5.0, None, None, None, 2.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 100, 100, 150, 0, 4.0, 0.0, 6.3, 9.2, None, None, None, 2.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.6, 367, 100, 85, 0, 5.0, 0.0, 15.0, 4.0, None, None, None, 0.7, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.6, 367, 100, 85, 0, 4.0, 0.0, 11.0, 4.0, None, None, None, 0.7, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
            AttackModule::set_no_finish_camera(agent.module_accessor, 1, true, false);
            AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
        }
        frame(agent.lua_state_agent, 19.0);
        if macros::is_excute(agent) {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
        frame(agent.lua_state_agent, 31.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            macros::CATCH(agent, 0, Hash40::new("top"), 8.0, 0.0, 15.0, 6.0, None, None, None, *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN, *COLLISION_SITUATION_MASK_GA);
            macros::CATCH(agent, 1, Hash40::new("top"), 8.0, 0.0, 11.0, 6.0, None, None, None, *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN, *COLLISION_SITUATION_MASK_GA);
        }
        frame(agent.lua_state_agent, 36.0);
        if macros::is_excute(agent) {
            grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
        frame(agent.lua_state_agent, 50.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_SPECIAL_HI_FLAG_IS_CHECK_DIVE);
        }
    }
    else {
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
            macros::CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 13.0, 7.0, None, None, None, *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN, *COLLISION_SITUATION_MASK_GA);
            macros::CATCH(agent, 1, Hash40::new("top"), 5.5, 0.0, 8.8, 13.7, None, None, None, *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN, *COLLISION_SITUATION_MASK_G);
            macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 0, 50, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 15.0, 6.0, None, None, None, *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN, *COLLISION_SITUATION_MASK_GA);
            macros::CATCH(agent, 1, Hash40::new("top"), 5.0, 0.0, 11.0, 6.0, None, None, None, *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN, *COLLISION_SITUATION_MASK_GA);
        }
        frame(agent.lua_state_agent, 19.0);
        if macros::is_excute(agent) {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
        frame(agent.lua_state_agent, 31.0);
        if macros::is_excute(agent) {
            grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            AttackModule::clear_all(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 36.0);
        if macros::is_excute(agent) {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
        frame(agent.lua_state_agent, 50.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_SPECIAL_HI_FLAG_IS_CHECK_DIVE);
        }
    }
}

unsafe extern "C" fn game_specialhithrow(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 13.0, 50, 62, 0, 85, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 8.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
        macros::CHECK_FINISH_CAMERA(agent, 2, 1);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.3);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 5.0, y: 3.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        captain_update_boost_power(agent.module_accessor, 13.0);
    }
    macros::FT_MOTION_RATE(agent, 0.75);
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_SPECIAL_HI_THROW_FLAG_FALL);
    }
    frame(agent.lua_state_agent, 45.0);
    macros::FT_MOTION_RATE(agent, 0.7);
    frame(agent.lua_state_agent, 59.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        spend_boost_power(agent);
    }
    if VarModule::is_flag(agent.module_accessor, vars::captain::status::flag::USED_BOOST_POWER) {
        macros::FT_MOTION_RATE(agent, 4.0 / 7.0);
        frame(agent.lua_state_agent, 15.0);
        macros::FT_MOTION_RATE(agent, 1.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("legr"), 15.0, 95, 100, 60, 30, 3.8, 10.0, 0.0, 0.0, Some(2.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            JostleModule::set_status(agent.module_accessor, false);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KICK_WALL_CHECK);
        }
        frame(agent.lua_state_agent, 19.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("legr"), 11.0, 95, 100, 60, 30, 3.2, 10.0, 0.0, 0.0, Some(2.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
        frame(agent.lua_state_agent, 26.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("legr"), 7.0, 95, 100, 60, 30, 3.2, 10.0, 0.0, 0.0, Some(2.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
        frame(agent.lua_state_agent, 36.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            JostleModule::set_status(agent.module_accessor, true);
        }
    }
    else {
        // macros::FT_MOTION_RATE(agent, 4.0 / 7.0);
        frame(agent.lua_state_agent, 15.0);
        macros::FT_MOTION_RATE(agent, 1.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("legr"), 15.0, 52, 88, 0, 60, 3.8, 10.0, 0.0, 0.0, Some(2.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            JostleModule::set_status(agent.module_accessor, false);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KICK_WALL_CHECK);
        }
        frame(agent.lua_state_agent, 19.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("legr"), 11.0, 52, 60, 0, 90, 3.2, 10.0, 0.0, 0.0, Some(2.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
        frame(agent.lua_state_agent, 26.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("legr"), 7.0, 46, 65, 0, 80, 3.2, 10.0, 0.0, 0.0, Some(2.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
        frame(agent.lua_state_agent, 36.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            JostleModule::set_status(agent.module_accessor, true);
        }
    }
}

unsafe extern "C" fn game_specialairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        spend_boost_power(agent);
    }
    if VarModule::is_flag(agent.module_accessor, vars::captain::status::flag::USED_BOOST_POWER) {
        macros::FT_MOTION_RATE(agent, 5.0 / 8.0);
        frame(agent.lua_state_agent, 16.0);
        macros::FT_MOTION_RATE(agent, 1.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KICK_WALL_CHECK);
            macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 15.0, 95, 100, 60, 30, 5.76, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
        frame(agent.lua_state_agent, 21.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 12.0, 95, 100, 60, 30, 5.0, 4.0, 0.0, 0.0, Some(-2.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
        frame(agent.lua_state_agent, 27.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 9.0, 95, 100, 60, 30, 5.0, 4.0, 0.0, 0.0, Some(-2.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
        wait(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    else {
        // macros::FT_MOTION_RATE(agent, 0.867);
        frame(agent.lua_state_agent, 16.0);
        macros::FT_MOTION_RATE(agent, 1.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KICK_WALL_CHECK);
            macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 15.0, 361, 57, 0, 80, 5.76, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
        frame(agent.lua_state_agent, 21.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 12.0, 361, 46, 0, 80, 5.0, 4.0, 0.0, 0.0, Some(-2.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
        frame(agent.lua_state_agent, 27.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 9.0, 361, 46, 0, 80, 5.0, 4.0, 0.0, 0.0, Some(-2.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
        wait(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
}

unsafe extern "C" fn game_speciallwwallend(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if VarModule::is_flag(agent.module_accessor, vars::captain::status::flag::USED_BOOST_POWER) {
        if macros::is_excute(agent) {
            sv_kinetic_energy!(
                reset_energy,
                agent,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0
            );
            KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
    }
    frame(agent.lua_state_agent, 10.0);
    if VarModule::is_flag(agent.module_accessor, vars::captain::status::flag::USED_BOOST_POWER) {
        if macros::is_excute(agent) {
            CancelModule::enable_cancel(agent.module_accessor);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn", game_specialn, Priority::Default);

    agent.acmd("game_specialnturn", game_specialnturn, Priority::Default);

    agent.acmd("game_specialairn", game_specialairn, Priority::Default);

    agent.acmd("game_specialairnturn", game_specialairnturn, Priority::Default);

    agent.acmd("game_specialsstart", game_specialsstart, Priority::Low);

    agent.acmd("game_specialsend", game_specialsend, Priority::Low);

    agent.acmd("game_specialairsstart", game_specialairsstart, Priority::Low);

    agent.acmd("game_specialairsend", game_specialairsend, Priority::Low);

    agent.acmd("game_specialhi", game_specialhi, Priority::Low);

    agent.acmd("game_specialairhi", game_specialhi, Priority::Low);

    agent.acmd("game_specialhithrow", game_specialhithrow, Priority::Default);

    agent.acmd("game_speciallw", game_speciallw, Priority::Default);

    agent.acmd("game_specialairlw", game_specialairlw, Priority::Default);

    agent.acmd("game_speciallwwallend", game_speciallwwallend, Priority::Default);
}