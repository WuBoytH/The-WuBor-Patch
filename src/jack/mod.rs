use smash::{
    lua2cpp::{L2CFighterCommon, L2CAgentBase},
    hash40,
    phx::Hash40,
    app::{lua_bind::*, sv_animcmd::*, *},
    lib::{lua_const::*, L2CValue}
};
use smash_script::*;
use smashline::*;
use crate::{
    table_const::*
};


#[fighter_frame( agent = FIGHTER_KIND_JACK )]
fn jack_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if [
            hash40("special_s1"),
            hash40("special_air_s1")
        ].contains(&MotionModule::motion_kind(fighter.module_accessor)) {
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_s1")
            && fighter.global_table[PREV_SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND
            && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
            && MotionModule::frame(fighter.module_accessor) >= 34.0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
            }
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RESET);
            }
        }

        // if IS_FGC[entry_id(fighter.module_accessor)] {
        //     jack_fgc(fighter);
        // }
    }
}

// #[status_script(agent = "jack", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
// unsafe fn jack_specials_main(fighter: &mut L2CFighterCommon) -> L2CValue {
//     jack_special_mot_helper(fighter, true.into(), hash40("special_s1").into(), hash40("special_air_s1").into());
//     notify_event_msc_cmd!(fighter, 0x20cbc92683u64, 1, *FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_09 - 1);
//     notify_event_msc_cmd!(fighter, 0x3a40337e2cu64, 1, *FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_09 - 1);
//     jack_special_s_main_helper(fighter)
// }

// unsafe extern "C" fn jack_special_mot_helper(fighter: &mut L2CFighterCommon, da_bool: L2CValue, ground_mot: L2CValue, air_mot: L2CValue) {
//     let mot;
//     if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
//         mot = air_mot.get_u64();
//         KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
//         GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
//         if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_S_FLAG_FALL_NORMAL) == false {
//             let speed_max_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("speed_max_y"));
//             let accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("accel_y"));
//             fighter.clear_lua_stack();
//             lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -accel_y);
//             sv_kinetic_energy::set_accel(fighter.lua_state_agent);
//             fighter.clear_lua_stack();
//             lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_max_y);
//             sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
//             fighter.clear_lua_stack();
//             lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_max_y);
//             sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
//         }
//         if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_S_FLAG_CONTROL_ENERGY) {
//             let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
//             fighter.clear_lua_stack();
//             lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, *ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, sum_speed_x, 0.0, 0.0, 0.0, 0.0);
//             sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
//             KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
//             KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
//         }
//         if !(da_bool.get_bool() == false) {
//             MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot), 0.0, 1.0, false, 0.0, false, false);
//             return;
//         }
//     }
//     else {
//         mot = ground_mot.get_u64();
//         KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
//         GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
//         if !(da_bool.get_bool() == false) {
//             MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot), 0.0, 1.0, false, 0.0, false, false);
//             return;
//         }
//     }
//     MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot), -1.0, 1.0, 0.0, false, false);
//     return
// }

// unsafe extern "C" fn jack_special_s_main_helper(fighter: &mut L2CFighterCommon) -> L2CValue {
//     if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
//         fighter.clear_lua_stack();
//         lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP);
//         let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
//         let air_start_speed_mul_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("air_start_speed_mul_x"));
//         fighter.clear_lua_stack();
//         lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x * air_start_speed_mul_x, 0.0);
//         sv_kinetic_energy::set_speed(fighter.lua_state_agent);
//         fighter.clear_lua_stack();
//         lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
//         let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
//         let air_start_speed_mul_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("air_start_speed_mul_y"));
//         fighter.clear_lua_stack();
//         lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed_y * air_start_speed_mul_y);
//         sv_kinetic_energy::set_speed(fighter.lua_state_agent);
//     }
//     fighter.sub_shift_status_main(L2CValue::Ptr(jack_special_s_main_loop as *const () as _))
// }

// unsafe extern "C" fn jack_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
//     if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_s1") {
//         fighter.sub_transition_group_check_air_cliff();
//     }
//     if !CancelModule::is_enable_cancel(fighter.module_accessor) {
//         if !MotionModule::is_end(fighter.module_accessor) {
//             if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_S_FLAG_SET_FALL_NORMAL) {
//                 let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
//                 fighter.clear_lua_stack();
//                 lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, sum_speed_y, 0.0, 0.0, 0.0);
//                 sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
//                 WorkModule::off_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_S_FLAG_SET_FALL_NORMAL);
//                 WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_S_FLAG_FALL_NORMAL);
//             }
//             if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_S_FLAG_ENABLE_CONTROL_ENERGY) {
//                 let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
//                 fighter.clear_lua_stack();
//                 lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, *ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, sum_speed_x, 0.0, 0.0, 0.0, 0.0);
//                 sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
//                 KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
//                 KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
//                 WorkModule::off_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_S_FLAG_ENABLE_CONTROL_ENERGY);
//                 WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_S_FLAG_CONTROL_ENERGY);
//             }
//             if StatusModule::is_changing(fighter.module_accessor) {
//                 return 0.into();
//             }
//             if StatusModule::is_situation_changed(fighter.module_accessor) {
//                 if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
//                     if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
//                     && MotionModule::frame(fighter.module_accessor) >= 34.0 {
//                         fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
//                     }
//                     else {
//                         jack_special_mot_helper(fighter, false.into(), 0xae47766c9u64.into(), 0xe14250c5du64.into());
//                     }
//                 }
//                 else {
//                     jack_special_mot_helper(fighter, false.into(), 0xa7d7e3773u64.into(), 0xe8d2c5de7u64.into());
//                 }
//             }
//         }
//         else {
//             if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
//                 fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
//             }
//             else {
//                 fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
//             }
//         }
//     }
//     else {
//         if fighter.sub_wait_ground_check_common(false.into()).get_bool() == false {
//             if fighter.sub_air_check_fall_common().get_bool() == false {
//             }
//         }
//     }
//     0.into()
// }

#[status_script(agent = "jack", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn jack_specials_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
    ItemModule::set_attach_item_visibility(fighter.module_accessor, true, 0);
    VisibilityModule::set_whole(fighter.module_accessor, true);
    macros::AFTER_IMAGE_OFF(fighter, 0);
    if [
        *FIGHTER_STATUS_KIND_FALL_SPECIAL,
        *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL
    ].contains(&fighter.global_table[STATUS_KIND].get_i32()) {
        WorkModule::set_float(fighter.module_accessor,  20.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    }
    0.into()
}

#[acmd_script( agent = "jack", script = "game_specials1", category = ACMD_GAME, low_priority )]
unsafe fn jack_sspecial1(fighter: &mut L2CAgentBase) {
    let mut attack = true;
    macros::FT_MOTION_RATE(fighter, 14.0 / 9.0);
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        // KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    }
    frame(fighter.lua_state_agent, 9.0);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        if macros::is_excute(fighter) {
            attack = false;
        }
        macros::FT_MOTION_RATE(fighter, 5.0);
    }
    else {
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
    frame(fighter.lua_state_agent, 13.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if attack {
        if macros::is_excute(fighter) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 80, 20, 0, 100, 3.0, 0.0, 6.5, 5.0, Some(0.0), Some(6.5), Some(0.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    else {
        if macros::is_excute(fighter) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
            HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
            JostleModule::set_status(fighter.module_accessor, false);
        }
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        if attack {
            AttackModule::clear_all(fighter.module_accessor);
        }
        else {
            HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
            JostleModule::set_status(fighter.module_accessor, true);
        }
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
}

#[acmd_script( agent = "jack", script = "effect_specials1", category = ACMD_EFFECT, low_priority )]
unsafe fn jack_sspecial1eff(fighter: &mut L2CAgentBase) {
    let mut attack = true;
    frame(fighter.lua_state_agent, 9.0);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        if macros::is_excute(fighter) {
            attack = false;
        }
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.6, 0, 0, 0, 0, 0, 0, false);
        if attack {
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_jack_sword1"), Hash40::new("tex_jack_sword2"), 14, Hash40::new("knife"), 0.0, 0.25, 0.15, Hash40::new("knife"), 0.0, 5.8, 0.0, false, Hash40::new("none"), Hash40::new("none"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        if attack {
            macros::AFTER_IMAGE_OFF(fighter, 0);
        }
    }
    frame(fighter.lua_state_agent, 20.0);
    for _ in 0..2 {
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), 7, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 3.0);
    }
}

#[acmd_script( agent = "jack", script = "sound_specials1", category = ACMD_SOUND, low_priority )]
unsafe fn jack_sspecial1snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_jack_attackhard_l01"));
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_jack_rnd_attack_l"));
    }
}

#[acmd_script( agent = "jack", script = "expression_specials1", category = ACMD_EXPRESSION, low_priority )]
unsafe fn jack_sspecial1exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        ItemModule::set_attach_item_visibility(fighter.module_accessor, false, 0);
        VisibilityModule::set_whole(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
        ItemModule::set_attach_item_visibility(fighter.module_accessor, true, 0);
        VisibilityModule::set_whole(fighter.module_accessor, true);
    }
}

#[acmd_script( agent = "jack", script = "game_specialairs1", category = ACMD_GAME, low_priority )]
unsafe fn jack_sspecial1air(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, 14.0 / 9.0);
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        // KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    }
    frame(fighter.lua_state_agent, 9.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 80, 20, 0, 70, 3.0, 0.0, 6.5, 5.0, Some(0.0), Some(6.5), Some(0.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_S_FLAG_ENABLE_CONTROL_ENERGY);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        macros::SET_SPEED_EX(fighter, 0.4, 0.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_S_FLAG_SET_FALL_NORMAL);
    }
    frame(fighter.lua_state_agent, 42.0);
    if macros::is_excute(fighter) {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
    }
}

#[acmd_script( agent = "jack", script = "effect_specialairs1", category = ACMD_EFFECT, low_priority )]
unsafe fn jack_sspecial1aireff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_jack_sword1"), Hash40::new("tex_jack_sword2"), 14, Hash40::new("knife"), 0.0, 0.25, 0.15, Hash40::new("knife"), 0.0, 5.8, 0.0, false, Hash40::new("none"), Hash40::new("none"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE_OFF(fighter, 0);
    }
}

#[acmd_script( agent = "jack", script = "sound_specialairs1", category = ACMD_SOUND, low_priority )]
unsafe fn jack_sspecial1airsnd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_jack_attackhard_l01"));
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_jack_rnd_attack_l"));
    }
}

#[acmd_script( agent = "jack", script = "expression_specialairs1", category = ACMD_EXPRESSION, low_priority )]
unsafe fn jack_sspecial1airexp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        ItemModule::set_attach_item_visibility(fighter.module_accessor, false, 0);
        VisibilityModule::set_whole(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
        ItemModule::set_attach_item_visibility(fighter.module_accessor, true, 0);
        VisibilityModule::set_whole(fighter.module_accessor, true);
    }
}

#[acmd_script( agent = "jack", scripts = [ "game_speciallw", "game_specialairlw" ], category = ACMD_GAME, low_priority )]
unsafe fn jack_dspecial(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 2.0);
    frame(fighter.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        smash_script::shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_SHIELD, 0, *FIGHTER_JACK_SHIELD_GROUP_KIND_SPECIAL_LW);
        smash_script::shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_JACK_REFLECTOR_KIND_SPECIAL_LW, *FIGHTER_REFLECTOR_GROUP_EXTEND);
    }
    frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        smash_script::shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_SHIELD, 0, *FIGHTER_JACK_SHIELD_GROUP_KIND_SPECIAL_LW);
        smash_script::shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_JACK_REFLECTOR_KIND_SPECIAL_LW, *FIGHTER_REFLECTOR_GROUP_EXTEND);
    }
}

#[acmd_script( agent = "jack", scripts = [ "game_speciallwcounter", "game_specialairlwcounter" ], category = ACMD_GAME, low_priority )]
unsafe fn jack_dspecialcounter(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 361, 51, 0, 80, 11.0, 0.0, 15.0, 4.0, Some(0.0), Some(15.0), Some(19.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
    }
    frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    install_agent_frames!(
        jack_frame
    );
    install_status_scripts!(
        jack_specials_end
    );
    install_acmd_scripts!(
        jack_sspecial1,
        jack_sspecial1eff,
        jack_sspecial1snd,
        jack_sspecial1exp,
        jack_sspecial1air,
        jack_sspecial1aireff,
        jack_sspecial1airsnd,
        jack_sspecial1airexp,
        jack_dspecial,
        jack_dspecialcounter
    );
}