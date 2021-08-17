use smash::phx::Hash40;
// use smash::hash40;
use smash::lua2cpp::{/*L2CFighterBase, */L2CAgentBase, L2CFighterCommon};
// use smash::app::*;
use smash::app::sv_animcmd::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
// use smash::lib::L2CValue;
use smash_script::*;
use smashline::*;
//use smash::phx::Vector3f;
use crate::commonfuncs::*;
use crate::vars::*;
// use crate::globals::*;

#[fighter_frame( agent = FIGHTER_KIND_TOONLINK )]
fn toonlink_frame(fighter: &mut L2CFighterCommon) {
    unsafe {

        // Toon Link can now move during his grounded Spin Attack.

        if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("special_hi") {
            if MotionModule::frame(fighter.module_accessor) > 6.0 && MotionModule::frame(fighter.module_accessor) < 46.0 {
                let stickx = ControlModule::get_stick_x(fighter.module_accessor);
                let lr = PostureModule::lr(fighter.module_accessor);
                let speed : f32;
                if IS_FUNNY[entry_id(fighter.module_accessor)] {
                    speed = 3.0;
                }
                else {
                    speed = 1.56;
                }
                macros::SET_SPEED_EX(fighter, lr * speed * stickx, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
        }
    }
}

// #[status_script(agent = "toonlink_boomerang", status = WN_LINK_BOOMERANG_STATUS_KIND_FLY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
// unsafe fn toonlink_boomerang_fly_main(weapon: &mut L2CFighterBase) -> L2CValue {
//     WorkModule::off_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_INFLICTION);
//     WorkModule::off_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_TO_HOP);
//     if !StopModule::is_stop(weapon.module_accessor) {
//         boomerang_ground_check(weapon, true, false);
//     }
//     weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(boomerang_ground_check_wrap as *const () as _));
//     MotionModule::change_motion(weapon.module_accessor, Hash40::new("fly"), 0.0, 1.0, false, 0.0, false, false);
//     weapon.fastshift(L2CValue::Ptr(toonlink_boomerang_fly_main_loop as *const () as _))
// }

// unsafe extern "C" fn boomerang_ground_check_wrap(weapon: &mut L2CFighterBase) -> L2CValue {
//     boomerang_ground_check(weapon, false, true)
// }

// unsafe extern "C" fn boomerang_ground_check(weapon: &mut L2CFighterBase, is_stop : bool, somethin : bool) -> L2CValue {
//     if somethin {
//         if !GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
//             notify_event_msc_cmd!(weapon, 0x18b78d41a0u64);
//         }
//         boomerang_dec_life(weapon);
//     }
//     // L2CValue::I32(0)
//     return is_stop.into()
// }

// unsafe extern "C" fn boomerang_dec_life(weapon: &mut L2CFighterBase) {
//     WorkModule::dec_int(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_LIFE);
//     let life = WorkModule::get_int(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_LIFE);
//     if life <= 0 {
//         notify_event_msc_cmd!(weapon, 0x199c462b5du64);
//     }
// }

// unsafe extern "C" fn toonlink_boomerang_fly_main_loop(weapon: &mut L2CFighterBase) -> L2CValue {
//     let mut val = 0;
//     let sum_speed_length = KineticModule::get_sum_speed_length(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
//     let min_speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), hash40("speed_min"));
//     let speed_mul = WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), hash40("speed_mul"));
//     if !WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_TO_HOP) {
//         if sum_speed_length as f32 <= min_speed * speed_mul {
//             if !WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_APPLY_FLY_SPEED) {
//                 weapon.change_status(WN_LINK_BOOMERANG_STATUS_KIND_TURN.into(), false.into());
//                 val = 1;
//             }
//         }
//     }
//     else {
//         weapon.change_status(WN_LINK_BOOMERANG_STATUS_KIND_HOP.into(), false.into());
//     }
//     L2CValue::I32(val)
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
    smashline::install_agent_frames!(
        toonlink_frame
    );
    // smashline::install_status_scripts!(
    //     toonlink_boomerang_fly_main
    // );
    smashline::install_acmd_scripts!(
        toonlink_dashattack,
        toonlink_nspecialstart,
        toonlink_bowarrow_fly,
        toonlink_boomerang_fly,
        toonlink_boomerang_turn
    );
}