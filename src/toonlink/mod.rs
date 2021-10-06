use smash::{
    lua2cpp::{L2CFighterCommon, L2CAgentBase/*, L2CFighterBase*/},
    hash40,
    phx::{Hash40, Vector3f},
    app::{lua_bind::*, sv_animcmd::*, *},
    lib::{lua_const::*/*, L2CValue*/}
};
use smash_script::*;
use smashline::*;
use crate::{
    commonfuncs::*,
    vars::*,
    // globals::*
};

#[fighter_frame( agent = FIGHTER_KIND_TOONLINK )]
fn toonlink_frame(fighter: &mut L2CFighterCommon) {
    unsafe {

        // Toon Link can now move during his grounded Spin Attack.

        if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("special_hi")
        && WorkModule::get_float(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_WORK_HOLD_FRAME) >= WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("rslash_hold_frame")) as f32 {
            if MotionModule::frame(fighter.module_accessor) > 46.0 {
                macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
            else if MotionModule::frame(fighter.module_accessor) > 6.0 {
                let stickx = ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor);
                SPIN_SPEED[entry_id(fighter.module_accessor)] += 0.1 * stickx;
                if IS_FUNNY[entry_id(fighter.module_accessor)]
                && SPIN_SPEED[entry_id(fighter.module_accessor)] > 3.0 {
                    SPIN_SPEED[entry_id(fighter.module_accessor)] = 3.0;
                }
                else if SPIN_SPEED[entry_id(fighter.module_accessor)] > 2.0 {
                    SPIN_SPEED[entry_id(fighter.module_accessor)] = 2.0;
                }
                let speed = SPIN_SPEED[entry_id(fighter.module_accessor)];
                macros::SET_SPEED_EX(fighter, speed, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
            else {
                if IS_FUNNY[entry_id(fighter.module_accessor)] {
                    SPIN_SPEED[entry_id(fighter.module_accessor)] = 3.0;
                }
                else {
                    SPIN_SPEED[entry_id(fighter.module_accessor)] = 2.0;
                }
            }
        }

        // Down Air Bounce

        if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("attack_air_lw"){
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                BOUNCE[entry_id(fighter.module_accessor)] = true;
            }
            if BOUNCE[entry_id(fighter.module_accessor)] {
                macros::SET_SPEED_EX(fighter, 0.0, 0.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                KineticModule::suspend_energy_all(fighter.module_accessor);
                if MotionModule::frame(fighter.module_accessor) > HIT_FRAME[entry_id(fighter.module_accessor)] + 1.0
                && MotionModule::frame(fighter.module_accessor) < 65.0 {
                    MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, 65.0, true, false, false);
                }
                else if MotionModule::frame(fighter.module_accessor) > 66.0 {
                    KineticModule::resume_energy_all(fighter.module_accessor);
                    BOUNCE[entry_id(fighter.module_accessor)] = false;
                    MotionModule::set_rate(fighter.module_accessor, 0.4);
                }
            }
        }
    }
}

// #[status_script(agent = "toonlink_boomerang", status = WN_LINK_BOOMERANG_STATUS_KIND_FLY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
// unsafe fn toonlink_boomerang_fly_main(weapon: &mut L2CFighterBase) -> L2CValue {
//     WorkModule::off_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_INFLICTION);
//     WorkModule::off_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_TO_HOP);
//     if !StopModule::is_stop(weapon.module_accessor) {
//         boomerang_ground_check(weapon, false.into());
//     }
//     weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(boomerang_ground_check_wrap as *const () as _));
//     MotionModule::change_motion(weapon.module_accessor, Hash40::new("fly"), 0.0, 1.0, false, 0.0, false, false);
//     weapon.fastshift(L2CValue::Ptr(toonlink_boomerang_fly_main_loop as *const () as _))
// }

// unsafe extern "C" fn boomerang_ground_check_wrap(weapon: &mut L2CFighterBase, unk1: L2CValue) -> L2CValue {
//     boomerang_ground_check(weapon, unk1)
// }

// unsafe extern "C" fn boomerang_ground_check(weapon: &mut L2CFighterBase, somethin : L2CValue) -> L2CValue {
//     if somethin.get_bool() {
//         if !GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
//             notify_event_msc_cmd!(weapon, 0x18b78d41a0u64);
//         }
//         boomerang_dec_life(weapon);
//     }
//     0.into()
// }

// unsafe extern "C" fn boomerang_dec_life(weapon: &mut L2CFighterBase) {
//     WorkModule::dec_int(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_LIFE);
//     let life = WorkModule::get_int(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_LIFE);
//     if life <= 0 {
//         notify_event_msc_cmd!(weapon, 0x199c462b5du64);
//         weapon.pop_lua_stack(1);
//     }
// }

// unsafe extern "C" fn toonlink_boomerang_fly_main_loop(weapon: &mut L2CFighterBase) -> L2CValue {
//     let sum_speed_length = KineticModule::get_sum_speed_length(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
//     let min_speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), hash40("speed_min"));
//     let speed_mul = WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), hash40("speed_mul"));
//     if !WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_TO_HOP) {
//         println!("sum_speed_length: {}", sum_speed_length);
//         println!("min speed * mul: {}", min_speed * speed_mul);
//         if sum_speed_length as f32 <= min_speed * speed_mul {
//             println!("speed check pass");
//             if WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_APPLY_FLY_SPEED) {
//                 println!("fly speed check pass");
//                 weapon.change_status(WN_LINK_BOOMERANG_STATUS_KIND_TURN.into(), false.into());
//                 return 1.into()
//             }
//         }
//     }
//     else {
//         weapon.change_status(WN_LINK_BOOMERANG_STATUS_KIND_HOP.into(), false.into());
//     }
//     0.into()
// }

#[acmd_script( agent = "toonlink", script = "game_attackdash", category = ACMD_GAME, low_priority )]
unsafe fn toonlink_dashattack(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, 0.7);
    frame(fighter.lua_state_agent, 8.0);
    macros::FT_MOTION_RATE(fighter, 1.1);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 3, 0, Hash40::new("sword2"), 8.0, 82, 70, 0, 55, 4.2, 5.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword2"), 8.0, 82, 70, 0, 55, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 8.0, 82, 70, 0, 55, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(fighter, 0.6186);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 43.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "toonlink", script = "game_throwlw", category = ACMD_GAME, low_priority )]
unsafe fn toonlink_dthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 4.0, 150, 70, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 150, 0, 60, 5.0, 0.0, 2.4, 7.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ELBOW);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    frame(fighter.lua_state_agent, 23.0);
    macros::FT_MOTION_RATE(fighter, 0.6186);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, -6, 4);
        let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut smash::app::FighterCutInManager);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.5);
        lua_bind::FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{x: 0.0, y: 0.0, z: 0.0});
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

#[acmd_script( agent = "toonlink", script = "game_attackairn", category = ACMD_GAME, low_priority )]
unsafe fn toonlink_nair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword2"), 8.5, 361, 80, 0, 45, 4.0, 4.3, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword2"), 8.5, 361, 80, 0, 45, 4.5, 0.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("arml"), 8.5, 361, 80, 0, 45, 4.5, -0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("colonellm"), 8.5, 361, 80, 0, 45, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword2"), 7.0, 361, 80, 0, 45, 4.0, 4.3, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword2"), 7.0, 361, 80, 0, 45, 4.5, 0.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("arml"), 7.0, 361, 80, 0, 45, 4.5, -0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("colonellm"), 7.0, 361, 80, 0, 45, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 39.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "toonlink", script = "game_attackairhi", category = ACMD_GAME, low_priority )]
unsafe fn toonlink_uair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    macros::FT_MOTION_RATE(fighter, 0.5);
    frame(fighter.lua_state_agent, 11.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword2"), 14.0, 80, 95, 0, 25, 4.6, 5.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword2"), 14.0, 80, 95, 0, 25, 5.2, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("arml"), 14.0, 80, 95, 0, 25, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::ATK_POWER(fighter, 0, 11.0);
        macros::ATK_POWER(fighter, 1, 11.0);
        macros::ATK_POWER(fighter, 2, 11.0);
    }
    frame(fighter.lua_state_agent, 15.0);
    macros::FT_MOTION_RATE(fighter, 0.2);
    frame(fighter.lua_state_agent, 35.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 41.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "toonlink", script = "game_attackairlw", category = ACMD_GAME, low_priority )]
unsafe fn toonlink_dair(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(fighter, 0, 1.6, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        if StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_STATUS_KIND_DAMAGE_FLY {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
        }
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(fighter, 0, -3.8, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LW_SET_ATTACK);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 268, 60, 0, 30, 5.5, 1.0, -1.1, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 50, 98, 0, 40, 5.0, 1.0, -1.1, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 65.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LW_SET_ATTACK);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 71.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
        KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
}

#[acmd_script( agent = "toonlink", script = "effect_attackairlw", category = ACMD_EFFECT, low_priority )]
unsafe fn toonlink_daireff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("havel"), 0, -8, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
	    macros::LAST_EFFECT_SET_RATE(fighter, 1.7);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("toonlink_sword"), Hash40::new("sword1"), -0.03, 0.05, 0, 0, 0, -0.87, 1.01, true);
	    macros::EFFECT_FOLLOW(fighter, Hash40::new("toonlink_sword_flare"), Hash40::new("sword1"), -0.03, 0.05, 0, 0, 0, -0.87, 1.01, true);
    }
    frame(fighter.lua_state_agent, 15.0);
    for _ in 0..17 {
        if !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            if macros::is_excute(fighter) {
                macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 3, 0, -90, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, true);
            }
        }
        wait(fighter.lua_state_agent, 3.0);
    }
}

#[acmd_script( agent = "toonlink", scripts = ["game_specialnstart", "game_specialairnstart"], category = ACMD_GAME, low_priority )]
unsafe fn toonlink_nspecialstart(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW, false, 0);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOWARROW, false, 0);
    }
    macros::FT_MOTION_RATE(fighter, 21.0/18.0);
    frame(fighter.lua_state_agent, 18.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE);
    }
}

#[acmd_script( agent = "toonlink", scripts = ["game_speciallw", "game_specialairlw"], category = ACMD_GAME, low_priority )]
unsafe fn toonlink_dspecial(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_WORK_ID_FLAG_BOMB_GENERATE_LINKBOMB);
    }
    macros::FT_MOTION_RATE(fighter, 2.5);
    frame(fighter.lua_state_agent, 21.0);
    macros::FT_MOTION_RATE(fighter, 1.5);
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        ItemModule::throw_item(fighter.module_accessor, 80.0, 3.0, 1.0, 0, true, 0.0);
    }
}

#[acmd_script( agent = "toonlink_bowarrow", script = "game_fly", category = ACMD_GAME, low_priority )]
unsafe fn toonlink_bowarrow_fly(weapon: &mut L2CAgentBase) {
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 4.0, 361, 35, 0, 20, 1.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(weapon.module_accessor);
    }
}

#[acmd_script( agent = "toonlink_boomerang", script = "game_fly", category = ACMD_GAME, low_priority )]
unsafe fn toonlink_boomerang_fly(weapon: &mut L2CAgentBase) {
    if macros::is_excute(weapon) {
        // macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(weapon.module_accessor);
    }
}

#[acmd_script( agent = "toonlink_boomerang", script = "game_turn", category = ACMD_GAME, low_priority )]
unsafe fn toonlink_boomerang_turn(weapon: &mut L2CAgentBase) {
    frame(weapon.lua_state_agent, 10.0);
    if macros::is_excute(weapon) {
        AttackModule::clear_all(weapon.module_accessor);
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 6.0, 30, 100, 70, 0, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, -1.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
}

pub fn install() {
    install_agent_frames!(
        toonlink_frame
    );
    // install_status_scripts!(
    //     toonlink_boomerang_fly_main
    // );
    install_acmd_scripts!(
        toonlink_dashattack,
        toonlink_dthrow,
        toonlink_nair,
        toonlink_uair,
        toonlink_dair,
        toonlink_daireff,
        toonlink_nspecialstart,
        toonlink_dspecial,
        toonlink_bowarrow_fly,
        toonlink_boomerang_fly,
        toonlink_boomerang_turn
    );
}