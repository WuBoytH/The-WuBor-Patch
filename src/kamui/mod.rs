use smash::phx::Hash40;
// use smash::hash40;
use smash::phx::Vector3f;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smash::app::*;
use smash::app::sv_animcmd::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lib::L2CValue;
use smash_script::*;
use smashline::*;
// use crate::system::IS_FUNNY;
use crate::globals::*;
use crate::commonfuncs::*;
use crate::gameplay::*;
use crate::vars::*;

// pub unsafe fn mask_check(module_accessor : *mut BattleObjectModuleAccessor) -> bool {
//     if [
//         hash40("attack_air_b"),
//         hash40("attack_air_n"),
//         hash40("attack_air_lw"),
//         hash40("landing_air_n"),
//         hash40("landing_air_b"),
//         hash40("landing_air_lw"),
//         hash40("attack_100_start"),
//         hash40("attack_100"),
//         hash40("attack_100_end")
//     ].contains(&MotionModule::motion_kind(module_accessor))
//     || [
//         *FIGHTER_STATUS_KIND_ATTACK_100,
//         *FIGHTER_STATUS_KIND_ATTACK_S4,
//         *FIGHTER_STATUS_KIND_ATTACK_HI4,
//         *FIGHTER_STATUS_KIND_ATTACK_LW4,
//         *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD,
//         *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD,
//         *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD,
//         *FIGHTER_STATUS_KIND_ATTACK_S4_START,
//         *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
//         *FIGHTER_STATUS_KIND_ATTACK_LW4_START,
//         *FIGHTER_STATUS_KIND_SPECIAL_N,
//         *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_HOLD,
//         *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_SHOOT,
//         *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_BITE,
//         *FIGHTER_STATUS_KIND_SPECIAL_S,
//         *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_JUMP_END,
//         *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL,
//         *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK,
//         *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK_END,
//         *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_JUMP,
//         *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_END,
//         *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F,
//         *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B,
//         *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F_LANDING,
//         *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B_LANDING,
//         *FIGHTER_STATUS_KIND_SPECIAL_HI,
//         *FIGHTER_STATUS_KIND_SPECIAL_LW,
//         *FIGHTER_STATUS_KIND_FINAL,
//         *FIGHTER_KAMUI_STATUS_KIND_FINAL_READY,
//         *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ENTRY,
//         *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK,
//         *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK_WAIT
//     ].contains(&StatusModule::status_kind(module_accessor)) {
//         return true;
//     }
//     else {
//         return false;
//     }
// }

#[fighter_frame( agent = FIGHTER_KIND_KAMUI )]
fn kamui_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH || sv_information::is_ready_go() == false {
            DRAGON_INSTALL[entry_id(fighter.module_accessor)] = 0.0;
            _TIME_COUNTER[entry_id(fighter.module_accessor)] = 0;
            // CURR_MOT[entry_id(fighter.module_accessor)] = 0;
        }

        if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
            // ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("mask"), &Vector3f{x: 1.0, y: 1.0, z: 1.0});
            // ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_mask"), true);
            // ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_fronthair"), false);
            // ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_hair"), false);
            if !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
            && !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
                DRAGON_INSTALL[entry_id(fighter.module_accessor)] -= 1.0;
            }
            else if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
                DRAGON_INSTALL[entry_id(fighter.module_accessor)] += 2.0;
            }
            if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_HI {
                wall_jump_check(fighter.module_accessor);
            }
        }

        // if [
        //     *FIGHTER_STATUS_KIND_ATTACK_HI4,
        //     *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD,
        //     *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
        // ].contains(&StatusModule::prev_status_kind(fighter.module_accessor, 0))
        // && ![
        //     *FIGHTER_STATUS_KIND_ATTACK_HI4,
        //     *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD,
        //     *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
        // ].contains(&StatusModule::status_kind(fighter.module_accessor))
        // && SPECIAL_DRAGON[entry_id(fighter.module_accessor)] {
        //     ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_lspearhand"), false);
        //     ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_rspearhand"), false);
        //     ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_weapon"), true);
        //     if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
        //         ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_mask"), true);
        //         ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_fronthair"), false);
        //         ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_hair"), false);
        //     }
        //     SPECIAL_DRAGON[entry_id(fighter.module_accessor)] = false;
        // }
        // if [
        //     *FIGHTER_STATUS_KIND_ATTACK_LW4,
        //     *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD,
        //     *FIGHTER_STATUS_KIND_ATTACK_LW4_START,
        // ].contains(&StatusModule::prev_status_kind(fighter.module_accessor, 0))
        // && ![
        //     *FIGHTER_STATUS_KIND_ATTACK_LW4,
        //     *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD,
        //     *FIGHTER_STATUS_KIND_ATTACK_LW4_START,
        // ].contains(&StatusModule::status_kind(fighter.module_accessor))
        // && SPECIAL_DRAGON[entry_id(fighter.module_accessor)] {
        //     ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_lspearfoot"), false);
        //     ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_lfoot"), true);
        //     if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
        //         ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_mask"), true);
        //         ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_fronthair"), false);
        //         ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_hair"), false);
        //     }
        //     SPECIAL_DRAGON[entry_id(fighter.module_accessor)] = false;
        // }

        // if DRAGON_INSTALL[entry_id(fighter.module_accessor)] <= 0.0 {
        //     if !SET_DRAGON_OFF[entry_id(fighter.module_accessor)] {
        //         if !mask_check(fighter.module_accessor) {
        //             macros::EFFECT_FOLLOW(fighter, Hash40::new("kamui_transform_splash_end"), Hash40::new("neck"), 2, 0, 0, 0, 0, 0, 1, true);
        //             ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_mask"), false);
        //             ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_fronthair"), true);
        //             ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_hair"), true);
        //             SET_DRAGON_OFF[entry_id(fighter.module_accessor)] = true;
        //         }
        //         // VisibilityModule::set_int64(fighter.module_accessor, hash40("hair") as i64, hash40("hair_normal") as i64);
        //         // VisibilityModule::set_int64(fighter.module_accessor, hash40("front_hair") as i64, hash40("front_hair_normal") as i64);
        //         // VisibilityModule::set_int64(fighter.module_accessor, hash40("dragon") as i64, hash40("dragon_none") as i64);
        //     }
        // }

        if _TIME_COUNTER[entry_id(fighter.module_accessor)] > 0 {
            if _TIME_COUNTER[entry_id(fighter.module_accessor)] == 24 {
                macros::EFFECT(fighter, Hash40::new("kamui_water_sibuki_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
                macros::LAST_EFFECT_SET_RATE(fighter, 0.5);
                macros::LAST_EFFECT_SET_ALPHA(fighter, 0.75);
                macros::EFFECT(fighter, Hash40::new("kamui_water_sibuki_s"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
                macros::LAST_EFFECT_SET_RATE(fighter, 0.5);
                macros::LAST_EFFECT_SET_ALPHA(fighter, 0.75);
            }
            if _TIME_COUNTER[entry_id(fighter.module_accessor)] == 18 {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("kamui_transform_splash_start"), Hash40::new("handr"), 2, 0, 0, 0, 0, 0, 0.7, true);
                macros::LAST_EFFECT_SET_RATE(fighter, 0.5);
            }
            _TIME_COUNTER[entry_id(fighter.module_accessor)] -= 1;
            if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
                if _TIME_COUNTER[entry_id(fighter.module_accessor)] == 0 {
                    _TIME_COUNTER[entry_id(fighter.module_accessor)] = 24;
                }
            }
        }
    }
}

#[status_script(agent = "kamui", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn kamui_speciallwpre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, *FS_SUCCEEDS_KEEP_VISIBILITY);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    L2CValue::I32(0)
}

#[status_script(agent = "kamui", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn kamui_speciallw(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
    kamui_speciallwmotion(fighter);
    kamui_speciallwdragonmotion(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(kamui_speciallwmain as *const () as _))
}

unsafe extern "C" fn kamui_speciallwmotion(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_hit"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_hit"), -1.0, 1.0, 0.0, false, false);
        }
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_hit"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_hit"), -1.0, 1.0, 0.0, false, false);
        }
    }
    return
}

unsafe extern "C" fn kamui_speciallwdragonmotion(fighter: &mut L2CFighterCommon) {
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, Hash40::new("special_air_lw_hit"), false, -1.0);
            }
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, Hash40::new("special_air_lw_hit"), true, -1.0);
                let frame = MotionModule::frame(fighter.module_accessor);
                if frame >= 12.0 && frame < 20.0 {
                    ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, 0.5);
                }
            }
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, Hash40::new("special_lw_hit"), false, -1.0);
            }
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, Hash40::new("special_lw_hit"), true, -1.0);
                let frame = MotionModule::frame(fighter.module_accessor);
                if frame >= 12.0 && frame < 20.0 {
                    ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, 0.5);
                }
            }
        }
    }
    return
}

unsafe extern "C" fn kamui_speciallwmain(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut val = 0;
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(L2CValue::I32(0x80)).get_bool() == true
        && fighter.sub_air_check_fall_common().get_bool() == true {
            val = 1;
        }
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        kamui_speciallwmotion(fighter);
        kamui_speciallwdragonmotion(fighter);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    L2CValue::I32(val)
}

#[status_script(agent = "kamui", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn kamui_speciallwend(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_FINAL_VISUAL_ATTACK_OTHER {
        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON) {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            VisibilityModule::set_whole(fighter.module_accessor, true);
        }
    }
    L2CValue::I32(0)
}

// #[acmd_script( agent = "kamui", scripts = ["expression_appealhil", "expression_appealhir"], category = ACMD_EXPRESSION, low_priority )]
// unsafe fn kamui_utauntexp(fighter: &mut L2CAgentBase) {
//     let mut di = false;
//     if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
//         di = true;
//     }
//     if macros::is_excute(fighter) {
//         slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
//     }
//     frame(fighter.lua_state_agent, 19.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("dragon") as i64, hash40("dragon_horn") as i64);
//         }
//         ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, 0);
//     }
//     frame(fighter.lua_state_agent, 22.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("front_hair") as i64, hash40("front_hair_hide") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 23.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("hair") as i64, hash40("hair_hide") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 60.0);
//     di = false;
//     if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
//         di = true;
//     }
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("hair") as i64, hash40("hair_normal") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 61.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("front_hair") as i64, hash40("front_hair_normal") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 64.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("dragon") as i64, hash40("dragon_none") as i64);
//         }
//     }
// }

#[acmd_script( agent = "kamui", script = "game_attack12", category = ACMD_GAME, low_priority )]
unsafe fn kamui_jab2(fighter: &mut L2CAgentBase) {
    let mut rehit = 0;
    if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
        rehit = 1;
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 22, 0, 35, 4.0, 0.0, 10.0, 8.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, rehit, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 361, 22, 0, 35, 3.5, 0.0, 10.5, 13.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, rehit, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 2.0, 361, 22, 0, 35, 4.0, 0.0, 8.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, rehit, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 2.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 2.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 2.0, false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        if rehit == 0 {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
        }
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        if rehit == 0 {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
        }
    }
}

#[acmd_script( agent = "kamui", script = "game_attack13", category = ACMD_GAME, low_priority )]
unsafe fn kamui_jab3(fighter: &mut L2CAgentBase) {
    let mut di = false;
    if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
        di = true;
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        if di {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 180, 22, 0, 35, 5.0, 0.0, 9.0, 11.5, Some(0.0), Some(9.0), Some(11.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 180, 22, 0, 35, 4.2, 0.0, 9.0, 22.0, Some(0.0), Some(9.0), Some(10.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 2.0, false);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 2.0, false);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 50, 160, 0, 50, 5.0, 0.0, 9.0, 11.5, Some(0.0), Some(9.0), Some(11.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 50, 150, 0, 50, 4.2, 0.0, 9.0, 22.0, Some(0.0), Some(9.0), Some(10.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        if di {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
        }
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        if di {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
        }
    }
}

// #[acmd_script( agent = "kamui", script = "effect_attack100end", category = ACMD_EFFECT, low_priority )]
// unsafe fn kamui_rapidjabendeff(fighter: &mut L2CAgentBase) {
//     if macros::is_excute(fighter) {
// 	    macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_speedline"), true, true);
// 	    macros::EFFECT_OFF_KIND(fighter, Hash40::new("kamui_bite"), true, true);
//     }
//     frame(fighter.lua_state_agent, 2.0);
//     if macros::is_excute(fighter) {
//         macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 1, 8.5, 5, 2.5, -5, 0, 1.425, false, 0.75);
//     }
//     frame(fighter.lua_state_agent, 3.0);
//     if macros::is_excute(fighter) {
// 	    macros::EFFECT_FOLLOW(fighter, Hash40::new("kamui_bite"), Hash40::new("havel"), 0, 8, 0, 0, 0, 0, 1.6, true);
// 	    macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
//     }
//     frame(fighter.lua_state_agent, 18.0);
//     if macros::is_excute(fighter) {
//         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFEECT_TRANSFORM_WIND);
//     }
//     frame(fighter.lua_state_agent, 19.0);
//     if macros::is_excute(fighter) {
//         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_REQ_EFEECT_TRANSFORM_WIND);
//     }
//     frame(fighter.lua_state_agent, 35.0);
//     if macros::is_excute(fighter) {
//         if DRAGON_INSTALL[entry_id(fighter.module_accessor)] <= 0.0 {
//             macros::EFFECT_FOLLOW(fighter, Hash40::new("kamui_transform_splash_end"), Hash40::new("neck"), 2, -1, 0, 0, 0, 0, 1, true);
//         }
//     }
// }

// #[acmd_script( agent = "kamui", script = "expression_attack100end", category = ACMD_EXPRESSION, low_priority )]
// unsafe fn kamui_rapidjabendexp(fighter: &mut L2CAgentBase) {
//     let mut di = false;
//     if macros::is_excute(fighter) {
//         AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("arml"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
//         ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
//         slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
//     }
//     frame(fighter.lua_state_agent, 1.0);
//     if macros::is_excute(fighter) {
//         ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_erase"), 0, false, 0);
//     }
//     frame(fighter.lua_state_agent, 2.0);
//     if macros::is_excute(fighter) {
//         ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, 0);
//     }
//     frame(fighter.lua_state_agent, 3.0);
//     if macros::is_excute(fighter) {
//         macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
//     }
//     frame(fighter.lua_state_agent, 38.0);
//     if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
//         di = true;
//     }
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("hair") as i64, hash40("hair_normal") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 39.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("front_hair") as i64, hash40("front_hair_normal") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 42.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("dragon") as i64, hash40("dragon_none") as i64);
//         }
//     }
// }

#[acmd_script( agent = "kamui", script = "game_attacks3", category = ACMD_GAME, low_priority )]
unsafe fn kamui_ftilt(fighter: &mut L2CAgentBase) {
    let mut di = false;
    if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
        di = true;
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        if di {
            macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 5.5, 367, 22, 0, 35, 5.0, 0.0, 8.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 5.5, 367, 22, 0, 35, 4.0, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 5.5, 367, 22, 0, 35, 4.0, 0.0, 8.0, 9.0, Some(0.0), Some(8.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 10.5, 45, 100, 0, 30, 5.0, 0.0, 8.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 10.5, 45, 100, 0, 30, 4.0, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.5, 45, 100, 0, 30, 4.0, 0.0, 8.0, 9.0, Some(0.0), Some(8.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        if di {
            macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 6.0, 45, 100, 0, 30, 5.0, 0.0, 8.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 6.0, 45, 100, 0, 30, 4.0, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 45, 100, 0, 30, 4.0, 0.0, 8.0, 9.0, Some(0.0), Some(8.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    macros::FT_MOTION_RATE(fighter, 0.87);
}

#[acmd_script( agent = "kamui", script = "game_attackhi3", category = ACMD_GAME, low_priority )]
unsafe fn kamui_utilt(fighter: &mut L2CAgentBase) {
    let mut di = false;
    if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
        di = true;
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        if di {
            macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 2.0, 367, 53, 0, 65, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 2.0, 367, 53, 0, 65, 5.0, 0.0, 4.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 2.0, 367, 53, 0, 65, 5.0, 0.0, 8.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 2.0, 92, 53, 0, 65, 5.0, 0.0, 12.0, 4.0, Some(0.0), Some(12.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 9.0, 88, 53, 0, 65, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 9.0, 88, 53, 0, 65, 5.0, 0.0, 4.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 9.0, 88, 53, 0, 65, 5.0, 0.0, 8.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 9.0, 88, 53, 0, 65, 5.0, 0.0, 12.0, 4.0, Some(0.0), Some(12.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        if di {
            AttackModule::clear_all(fighter.module_accessor);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 367, 22, 0, 35, 5.0, 0.0, 22.0, -5.0, Some(0.0), Some(22.0), Some(5.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        else {
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 6.0, 88, 53, 0, 65, 5.0, 0.0, 22.0, -2.0, Some(0.0), Some(22.0), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    wait(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        if di {
            AttackModule::clear_all(fighter.module_accessor);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 88, 53, 0, 65, 5.0, 0.0, 22.0, -5.0, Some(0.0), Some(22.0), Some(5.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "kamui", script = "game_attacklw3", category = ACMD_GAME, low_priority )]
unsafe fn kamui_dtilt(fighter: &mut L2CAgentBase) {
    let mut di = false;
    if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
        di = true;
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        if di {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 367, 35, 0, 22, 4.0, 0.0, 2.0, 8.0, Some(0.0), Some(2.0), Some(19.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 7.5, 100, 80, 0, 50, 5.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 7.5, 100, 80, 0, 50, 4.0, -1.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 7.5, 100, 80, 0, 50, 4.0, -1.0, 8.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        if di {
            AttackModule::clear_all(fighter.module_accessor);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 100, 80, 0, 50, 4.0, 0.0, 2.0, 8.0, Some(0.0), Some(2.0), Some(19.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

// #[acmd_script( agent = "kamui", script = "effect_attacks4", category = ACMD_EFFECT, low_priority )]
// unsafe fn kamui_fsmasheff(fighter: &mut L2CAgentBase) {
//     if macros::is_excute(fighter) {
//         if DRAGON_INSTALL[entry_id(fighter.module_accessor)] <= 0.0 {
// 	        macros::EFFECT_FLW_POS(fighter, Hash40::new("kamui_transform_splash_start"), Hash40::new("neck"), 2, 0, 0, 0, 0, 0, 1, true);
//         }
//         macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 10, 10, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
//     }
//     frame(fighter.lua_state_agent, 15.0);
//     if macros::is_excute(fighter) {
//         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_REQ_EFEECT_TRANSFORM_WIND);
// 	    macros::EFFECT_FOLLOW(fighter, Hash40::new("kamui_attack_line"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
//     }
//     frame(fighter.lua_state_agent, 19.0);
//     if macros::is_excute(fighter) {
// 	    macros::EFFECT(fighter, Hash40::new("kamui_attack_flash"), Hash40::new("havel"), 0, 0, 21, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
// 	    macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
//     }
//     frame(fighter.lua_state_agent, 23.0);
//     if macros::is_excute(fighter) {
//         macros::EFFECT_OFF_KIND(fighter, Hash40::new("kamui_attack_line"), false, false);
//     }
//     frame(fighter.lua_state_agent, 28.0);
//     if macros::is_excute(fighter) {
//         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFEECT_TRANSFORM_WIND);
//     }
//     frame(fighter.lua_state_agent, 48.0);
//     if macros::is_excute(fighter) {
//         if DRAGON_INSTALL[entry_id(fighter.module_accessor)] <= 0.0 {
//             macros::EFFECT_FLW_POS(fighter, Hash40::new("kamui_transform_splash_end"), Hash40::new("neck"), 2, 0, 0, 0, 0, 0, 1, true);
//         }
//     }
// }

// #[acmd_script( agent = "kamui", script = "effect_attacks4hi", category = ACMD_EFFECT, low_priority )]
// unsafe fn kamui_fsmashhieff(fighter: &mut L2CAgentBase) {
//     if macros::is_excute(fighter) {
//         if DRAGON_INSTALL[entry_id(fighter.module_accessor)] <= 0.0 {
// 	        macros::EFFECT_FLW_POS(fighter, Hash40::new("kamui_transform_splash_start"), Hash40::new("neck"), 2, 0, 0, 0, 0, 0, 1, true);
//         }
//         macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 10, 10, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
//     }
//     frame(fighter.lua_state_agent, 15.0);
//     if macros::is_excute(fighter) {
//         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_REQ_EFEECT_TRANSFORM_WIND);
// 	    macros::EFFECT_FOLLOW(fighter, Hash40::new("kamui_attack_line"), Hash40::new("havel"), 0, 0, 0, -7, 0, 0, 1, true);
//     }
//     frame(fighter.lua_state_agent, 19.0);
//     if macros::is_excute(fighter) {
// 	    macros::EFFECT(fighter, Hash40::new("kamui_attack_flash"), Hash40::new("havel"), 0, 2.5, 22, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
// 	    macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
//     }
//     frame(fighter.lua_state_agent, 23.0);
//     if macros::is_excute(fighter) {
//         macros::EFFECT_OFF_KIND(fighter, Hash40::new("kamui_attack_line"), false, false);
//     }
//     frame(fighter.lua_state_agent, 28.0);
//     if macros::is_excute(fighter) {
//         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFEECT_TRANSFORM_WIND);
//     }
//     frame(fighter.lua_state_agent, 48.0);
//     if macros::is_excute(fighter) {
//         if DRAGON_INSTALL[entry_id(fighter.module_accessor)] <= 0.0 {
//             macros::EFFECT_FLW_POS(fighter, Hash40::new("kamui_transform_splash_end"), Hash40::new("neck"), 2, 0, 0, 0, 0, 0, 1, true);
//         }
//     }
// }

// #[acmd_script( agent = "kamui", script = "effect_attacks4lw", category = ACMD_EFFECT, low_priority )]
// unsafe fn kamui_fsmashlweff(fighter: &mut L2CAgentBase) {
//     if macros::is_excute(fighter) {
//         if DRAGON_INSTALL[entry_id(fighter.module_accessor)] <= 0.0 {
// 	        macros::EFFECT_FLW_POS(fighter, Hash40::new("kamui_transform_splash_start"), Hash40::new("neck"), 2, 0, 0, 0, 0, 0, 1, true);
//         }
//         macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 10, 10, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
//     }
//     frame(fighter.lua_state_agent, 15.0);
//     if macros::is_excute(fighter) {
//         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_REQ_EFEECT_TRANSFORM_WIND);
// 	    macros::EFFECT_FOLLOW(fighter, Hash40::new("kamui_attack_line"), Hash40::new("havel"), 0, 0, 0, 7, 0, 0, 1, true);
//     }
//     frame(fighter.lua_state_agent, 19.0);
//     if macros::is_excute(fighter) {
// 	    macros::EFFECT(fighter, Hash40::new("kamui_attack_flash"), Hash40::new("havel"), 0, -4, 20, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
// 	    macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
//     }
//     frame(fighter.lua_state_agent, 23.0);
//     if macros::is_excute(fighter) {
//         macros::EFFECT_OFF_KIND(fighter, Hash40::new("kamui_attack_line"), false, false);
//     }
//     frame(fighter.lua_state_agent, 28.0);
//     if macros::is_excute(fighter) {
//         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFEECT_TRANSFORM_WIND);
//     }
//     frame(fighter.lua_state_agent, 48.0);
//     if macros::is_excute(fighter) {
//         if DRAGON_INSTALL[entry_id(fighter.module_accessor)] <= 0.0 {
//             macros::EFFECT_FLW_POS(fighter, Hash40::new("kamui_transform_splash_end"), Hash40::new("neck"), 2, 0, 0, 0, 0, 0, 1, true);
//         }
//     }
// }

// #[acmd_script( agent = "kamui", scripts = ["expression_attacks4", "expression_attacks4lw", "expression_attacks4hi"], category = ACMD_EXPRESSION, low_priority )]
// unsafe fn kamui_fsmashexp(fighter: &mut L2CAgentBase) {
//     let mut di = false;
//     if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
//         di = true;
//     }
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("dragon") as i64, hash40("dragon_horn") as i64);
//         }
//         ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
//         slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
//     }
//     frame(fighter.lua_state_agent, 4.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("front_hair") as i64, hash40("front_hair_hide") as i64);
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("hair") as i64, hash40("hair_hide") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 9.0);
//     if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
//         if macros::is_excute(fighter) {
//             ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
//             slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
//         }
//     }
//     frame(fighter.lua_state_agent, 16.0);
//     if macros::is_excute(fighter) {
//         ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, 0);
//     }
//     frame(fighter.lua_state_agent, 50.0);
//     di = false;
//     if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
//         di = true;
//     }
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("hair") as i64, hash40("hair_normal") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 52.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("front_hair") as i64, hash40("front_hair_normal") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 55.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("dragon") as i64, hash40("dragon_none") as i64);
//         }
//     }
// }

// #[acmd_script( agent = "kamui", script = "effect_attackhi4", category = ACMD_EFFECT, low_priority )]
// unsafe fn kamui_usmasheff(fighter: &mut L2CAgentBase) {
//     if macros::is_excute(fighter) {
//         if DRAGON_INSTALL[entry_id(fighter.module_accessor)] <= 0.0 {
// 	        macros::EFFECT_FLW_POS(fighter, Hash40::new("kamui_transform_splash_start"), Hash40::new("neck"), 2, 0, 0, 0, 0, 0, 1, true);
//         }
//         macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 13, 9, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
//     }
//     frame(fighter.lua_state_agent, 11.0);
//     if macros::is_excute(fighter) {
//         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_REQ_EFEECT_TRANSFORM_WIND);
//     }
//     frame(fighter.lua_state_agent, 13.0);
//     if macros::is_excute(fighter) {
// 	    macros::EFFECT_FOLLOW(fighter, Hash40::new("kamui_attack_line_hi4"), Hash40::new("handl"), 6, 0, 0, 0, 90, 0, 0.5, true);
// 	    macros::EFFECT_FOLLOW(fighter, Hash40::new("kamui_attack_line_hi4"), Hash40::new("handr"), 6, 0, 0, 0, 90, 0, 0.5, true);
// 	    macros::EFFECT(fighter, Hash40::new("kamui_attack_flash"), Hash40::new("top"), 0, 26, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
// 	    macros::LAST_EFFECT_SET_RATE(fighter, 1.3);
//     }
//     frame(fighter.lua_state_agent, 14.0);
//     if macros::is_excute(fighter) {
//         macros::LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 45, 0, 1, 0, 0, 0, 0, 0, 0, true);
// 	    macros::LAST_EFFECT_SET_ALPHA(fighter, 0.7);
//     }
//     frame(fighter.lua_state_agent, 19.0);
//     if macros::is_excute(fighter) {
//         macros::EFFECT_OFF_KIND(fighter, Hash40::new("kamui_attack_line_hi4"), false, false);
//     }
//     frame(fighter.lua_state_agent, 33.0);
//     if macros::is_excute(fighter) {
//         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFEECT_TRANSFORM_WIND);
//         if DRAGON_INSTALL[entry_id(fighter.module_accessor)] <= 0.0 {
//             macros::EFFECT_FLW_POS(fighter, Hash40::new("kamui_transform_splash_end"), Hash40::new("neck"), 2, 0, 0, 0, 0, 0, 1, true);
//         }
//     }
// }

// #[acmd_script( agent = "kamui", script = "expression_attackhi4", category = ACMD_EXPRESSION, low_priority )]
// unsafe fn kamui_usmashexp(fighter: &mut L2CAgentBase) {
//     SPECIAL_DRAGON[entry_id(fighter.module_accessor)] = false;
//     if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
//         SPECIAL_DRAGON[entry_id(fighter.module_accessor)] = true;
//     }
//     if macros::is_excute(fighter) {
//         ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
//         slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
//         if !SPECIAL_DRAGON[entry_id(fighter.module_accessor)] {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("dragon") as i64, hash40("dragon_horn") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 4.0);
//     if macros::is_excute(fighter) {
//         if !SPECIAL_DRAGON[entry_id(fighter.module_accessor)] {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("front_hair") as i64, hash40("front_hair_hide") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 5.0);
//     if macros::is_excute(fighter) {
//         if !SPECIAL_DRAGON[entry_id(fighter.module_accessor)] {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("hair") as i64, hash40("hair_hide") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 9.0);
//     if macros::is_excute(fighter) {
//         if !SPECIAL_DRAGON[entry_id(fighter.module_accessor)] {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("dragon") as i64, 0x13a83cbf79 as i64);
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("weapon") as i64, hash40("weapon_hide") as i64);
//         }
//         else {
//             ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_lspearhand"), true);
//             ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_rspearhand"), true);
//             ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_weapon"), false);
//         }        
//         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFFECT_SWORD_AURA);
//     }
//     frame(fighter.lua_state_agent, 10.0);
//     if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
//         if macros::is_excute(fighter) {
//             ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
//             WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFFECT_SWORD_AURA);
//             slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
//         }
//     }
//     frame(fighter.lua_state_agent, 11.0);
//     if macros::is_excute(fighter) {
//         ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, 0);
//         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFFECT_SWORD_AURA);
//     }
//     frame(fighter.lua_state_agent, 13.0);
//     if macros::is_excute(fighter) {
//         macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_piercel"), 0);
//     }
//     frame(fighter.lua_state_agent, 34.0);
//     SPECIAL_DRAGON[entry_id(fighter.module_accessor)] = false;
//     if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
//         SPECIAL_DRAGON[entry_id(fighter.module_accessor)] = true;
//     }
//     if macros::is_excute(fighter) {
//         if !SPECIAL_DRAGON[entry_id(fighter.module_accessor)] {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("hair") as i64, hash40("hair_normal") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 35.0);
//     if macros::is_excute(fighter) {
//         if !SPECIAL_DRAGON[entry_id(fighter.module_accessor)] {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("front_hair") as i64, hash40("front_hair_normal") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 38.0);
//     if macros::is_excute(fighter) {
//         if !SPECIAL_DRAGON[entry_id(fighter.module_accessor)] {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("dragon") as i64, hash40("dragon_none") as i64);
//         }
//         else {
//             ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_lspearhand"), false);
//             ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_rspearhand"), false);
//             ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_weapon"), true);
//         }
//         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_REQ_EFFECT_SWORD_AURA);
//     }
// }

// #[acmd_script( agent = "kamui", script = "effect_attacklw4", category = ACMD_EFFECT, low_priority )]
// unsafe fn kamui_dsmasheff(fighter: &mut L2CAgentBase) {
//     if macros::is_excute(fighter) {
//         if DRAGON_INSTALL[entry_id(fighter.module_accessor)] <= 0.0 {
// 	        macros::EFFECT_FLW_POS(fighter, Hash40::new("kamui_transform_splash_start"), Hash40::new("neck"), 2, 0, 0, 0, 0, 0, 1, true);
//         }
//         macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 6, 11, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
//     }
//     frame(fighter.lua_state_agent, 10.0);
//     if macros::is_excute(fighter) {
//         macros::EFFECT_FOLLOW(fighter, Hash40::new("kamui_attack_line"), Hash40::new("footl"), -0.2, 0, 0, 0, 90, 0, 0.68, true);
//         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_REQ_EFEECT_TRANSFORM_WIND);
//     }
//     frame(fighter.lua_state_agent, 13.0);
//     if macros::is_excute(fighter) {
// 	    macros::EFFECT_FOLLOW(fighter, Hash40::new("kamui_sword_flare"), Hash40::new("haver"), 0, 0.1, 0, 0, 0, 0, 1, true);
// 	    macros::EFFECT(fighter, Hash40::new("kamui_attack_flash"), Hash40::new("footl"), 19, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
// 	    macros::LAST_EFFECT_SET_RATE(fighter, 1.3);
// 	    macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
//     }
//     frame(fighter.lua_state_agent, 14.0);
//     if macros::is_excute(fighter) {
//         macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("haver"), 0, 1, 0, -90, 0, 0, 0.75, true);
// 	    macros::LAST_PARTICLE_SET_COLOR(fighter, 1.1, 1, 0.3);
//     }
//     frame(fighter.lua_state_agent, 18.0);
//     if macros::is_excute(fighter) {
//         macros::EFFECT_OFF_KIND(fighter, Hash40::new("kamui_attack_line"), false, false);
// 	    macros::EFFECT_OFF_KIND(fighter, Hash40::new("kamui_sword_flare"), false, false);
//     }
//     frame(fighter.lua_state_agent, 33.0);
//     if macros::is_excute(fighter) {
//         if DRAGON_INSTALL[entry_id(fighter.module_accessor)] <= 0.0 {
//             macros::EFFECT_FLW_POS(fighter, Hash40::new("kamui_transform_splash_end"), Hash40::new("neck"), 2, 0, 0, 0, 0, 0, 1, true);
//         }
//     }
//     frame(fighter.lua_state_agent, 38.0);
//     if macros::is_excute(fighter) {
//         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFEECT_TRANSFORM_WIND);
//     }
// }

// #[acmd_script( agent = "kamui", script = "expression_attacklw4", category = ACMD_EXPRESSION, low_priority )]
// unsafe fn kamui_dsmashexp(fighter: &mut L2CAgentBase) {
//     SPECIAL_DRAGON[entry_id(fighter.module_accessor)] = false;
//     if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
//         SPECIAL_DRAGON[entry_id(fighter.module_accessor)] = true;
//     }
//     if macros::is_excute(fighter) {
//         ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
//         slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
//         if !SPECIAL_DRAGON[entry_id(fighter.module_accessor)] {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("dragon") as i64, hash40("dragon_horn") as i64);
//         } 
//     }
//     frame(fighter.lua_state_agent, 4.0);
//     if macros::is_excute(fighter) {
//         if !SPECIAL_DRAGON[entry_id(fighter.module_accessor)] {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("front_hair") as i64, hash40("front_hair_hide") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 5.0);
//     if macros::is_excute(fighter) {
//         if !SPECIAL_DRAGON[entry_id(fighter.module_accessor)] {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("hair") as i64, hash40("hair_hide") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 8.0);
//     if macros::is_excute(fighter) {
//         if !SPECIAL_DRAGON[entry_id(fighter.module_accessor)] {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("dragon") as i64, 0x126cc372b8 as i64);
//         }
//         else {
//             ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_lspearfoot"), true);
//             ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_lfoot"), false);
//         }
//     }
//     frame(fighter.lua_state_agent, 9.0);
//     if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
//         if macros::is_excute(fighter) {
//             slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
//         }
//     }
//     frame(fighter.lua_state_agent, 10.0);
//     if macros::is_excute(fighter) {
//         AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("rot"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
//         slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3);
//     }
//     frame(fighter.lua_state_agent, 11.0);
//     if macros::is_excute(fighter) {
//         ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, 0);
//     }
//     frame(fighter.lua_state_agent, 13.0);
//     if macros::is_excute(fighter) {
//         macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_piercel"), 0);
//     }
//     frame(fighter.lua_state_agent, 36.0);
//     SPECIAL_DRAGON[entry_id(fighter.module_accessor)] = false;
//     if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
//         SPECIAL_DRAGON[entry_id(fighter.module_accessor)] = true;
//     }
//     if macros::is_excute(fighter) {
//         if !SPECIAL_DRAGON[entry_id(fighter.module_accessor)] {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("hair") as i64, hash40("hair_normal") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 37.0);
//     if macros::is_excute(fighter) {
//         if !SPECIAL_DRAGON[entry_id(fighter.module_accessor)] {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("front_hair") as i64, hash40("front_hair_normal") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 40.0);
//     if macros::is_excute(fighter) {
//         if !SPECIAL_DRAGON[entry_id(fighter.module_accessor)] {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("dragon") as i64, hash40("dragon_none") as i64);
//         }
//         else {
//             ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_lspearfoot"), false);
//             ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_lfoot"), true);
//         }
//     }
//     frame(fighter.lua_state_agent, 43.0);
//     if macros::is_excute(fighter) {
//         slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 14);
//     }
// }

// #[acmd_script( agent = "kamui", script = "effect_throwf", category = ACMD_EFFECT, low_priority )]
// unsafe fn kamui_fthroweff(fighter: &mut L2CAgentBase) {
//     if macros::is_excute(fighter) {
// 	    if DRAGON_INSTALL[entry_id(fighter.module_accessor)] <= 0.0 {
//             macros::EFFECT_FLW_POS(fighter, Hash40::new("kamui_transform_splash_start"), Hash40::new("neck"), 2, -1, 0, 0, 0, 0, 1, true);
//         }
//     }
//     frame(fighter.lua_state_agent, 13.0);
//     if macros::is_excute(fighter) {
//         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_REQ_EFEECT_TRANSFORM_WIND);
//     }
//     frame(fighter.lua_state_agent, 14.0);
//     if macros::is_excute(fighter) {
// 	    macros::EFFECT_FOLLOW(fighter, Hash40::new("kamui_attack_line_hi4"), Hash40::new("havel"), 0, 0, 0, -4, 0, 0, 0.85, true);
// 	    macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
//     }
//     frame(fighter.lua_state_agent, 16.0);
//     if macros::is_excute(fighter) {
//         macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
//     }
//     frame(fighter.lua_state_agent, 18.0);
//     if macros::is_excute(fighter) {
//         macros::EFFECT_OFF_KIND(fighter, Hash40::new("kamui_attack_line_hi4"), false, true);
//     }
//     frame(fighter.lua_state_agent, 28.0);
//     if macros::is_excute(fighter) {
//         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFEECT_TRANSFORM_WIND);
//     }
//     frame(fighter.lua_state_agent, 43.0);
//     if macros::is_excute(fighter) {
//         if DRAGON_INSTALL[entry_id(fighter.module_accessor)] <= 0.0 {
//             macros::EFFECT_FOLLOW(fighter, Hash40::new("kamui_transform_splash_end"), Hash40::new("neck"), 2, -1, 0, 0, 0, 0, 1, true);
//         }
//     }
// }

// #[acmd_script( agent = "kamui", script = "expression_throwf", category = ACMD_EXPRESSION, low_priority )]
// unsafe fn kamui_fthrowexp(fighter: &mut L2CAgentBase) {
//     let mut di = false;
//     if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
//         di = true;
//     }
//     if macros::is_excute(fighter) {
//         AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("arml"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
//         fighter.clear_lua_stack();
//         lua_args!(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, *CAMERA_QUAKE_KIND_NONE);
//         FT_ATTACK_ABS_CAMERA_QUAKE(fighter.lua_state_agent);
//         slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("dragon") as i64, hash40("dragon_horn") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 4.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("front_hair") as i64, hash40("front_hair_hide") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 5.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("hair") as i64, hash40("hair_hide") as i64);
//         }
//         slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
//     }
//     frame(fighter.lua_state_agent, 14.0);
//     if macros::is_excute(fighter) {
//         macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_pierces"), 0);
//         macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
//     }
//     frame(fighter.lua_state_agent, 41.0);
//     di = false;
//     if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
//         di = true;
//     }
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("hair") as i64, hash40("hair_normal") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 42.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("front_hair") as i64, hash40("front_hair_normal") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 45.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("dragon") as i64, hash40("dragon_none") as i64);
//         }
//     }
// }

// #[acmd_script( agent = "kamui", script = "effect_throwb", category = ACMD_EFFECT, low_priority )]
// unsafe fn kamui_bthroweff(fighter: &mut L2CAgentBase) {
//     if macros::is_excute(fighter) {
//         if DRAGON_INSTALL[entry_id(fighter.module_accessor)] <= 0.0 {
//             macros::EFFECT_FLW_POS(fighter, Hash40::new("kamui_transform_splash_start"), Hash40::new("neck"), 2, -1, 0, 0, 0, 0, 1, true);
//         }
//     }
//     frame(fighter.lua_state_agent, 11.0);
//     if macros::is_excute(fighter) {
//         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_REQ_EFEECT_TRANSFORM_WIND);
//     }
//     frame(fighter.lua_state_agent, 12.0);
//     if macros::is_excute(fighter) {
// 	    macros::EFFECT_FOLLOW(fighter, Hash40::new("kamui_attack_line_hi4"), Hash40::new("havel"), 4, 0, 0, 0, 95, 0, 0.85, true);
// 	    macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
//     }
//     frame(fighter.lua_state_agent, 15.0);
//     if macros::is_excute(fighter) {
//         macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
//     }
//     frame(fighter.lua_state_agent, 15.0);
//     if macros::is_excute(fighter) {
//         macros::EFFECT_OFF_KIND(fighter, Hash40::new("kamui_attack_line_hi4"), false, true);
//     }
//     frame(fighter.lua_state_agent, 23.0);
//     if macros::is_excute(fighter) {
//         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFEECT_TRANSFORM_WIND);
//     }
//     frame(fighter.lua_state_agent, 26.0);
//     if macros::is_excute(fighter) {
//         if DRAGON_INSTALL[entry_id(fighter.module_accessor)] <= 0.0 {
//             macros::EFFECT_FOLLOW(fighter, Hash40::new("kamui_transform_splash_end"), Hash40::new("neck"), 2, -1, 0, 0, 0, 0, 1, true);
//         }
//     }
// }

// #[acmd_script( agent = "kamui", script = "expression_throwb", category = ACMD_EXPRESSION, low_priority )]
// unsafe fn kamui_bthrowexp(fighter: &mut L2CAgentBase) {
//     let mut di = false;
//     if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
//         di = true;
//     }
//     if macros::is_excute(fighter) {
//         AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("arml"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
//         fighter.clear_lua_stack();
//         lua_args!(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, *CAMERA_QUAKE_KIND_NONE);
//         FT_ATTACK_ABS_CAMERA_QUAKE(fighter.lua_state_agent);
//         slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
//     }
//     frame(fighter.lua_state_agent, 2.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("dragon") as i64, hash40("dragon_horn") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 5.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("front_hair") as i64, hash40("front_hair_hide") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 6.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("hair") as i64, hash40("hair_hide") as i64);
//         }
//         slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
//     }
//     frame(fighter.lua_state_agent, 12.0);
//     if macros::is_excute(fighter) {
//         macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_pierces"), 0);
//         macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
//     }
//     frame(fighter.lua_state_agent, 26.0);
//     di = false;
//     if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
//         di = true;
//     }
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("hair") as i64, hash40("hair_normal") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 27.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("front_hair") as i64, hash40("front_hair_normal") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 30.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("dragon") as i64, hash40("dragon_none") as i64);
//         }
//     }
// }

// #[acmd_script( agent = "kamui", script = "effect_throwhi", category = ACMD_EFFECT, low_priority )]
// unsafe fn kamui_uthroweff(fighter: &mut L2CAgentBase) {
//     if macros::is_excute(fighter) {
//         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_REQ_EFEECT_TRANSFORM_WIND);
//         if DRAGON_INSTALL[entry_id(fighter.module_accessor)] <= 0.0 {
//             macros::EFFECT_FLW_POS(fighter, Hash40::new("kamui_transform_splash_start"), Hash40::new("neck"), 2, -1, 0, 0, 0, 0, 1, true);
//         }
//     }
//     frame(fighter.lua_state_agent, 5.0);
//     if macros::is_excute(fighter) {
//         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFEECT_TRANSFORM_WIND);
// 	    macros::EFFECT_OFF_KIND(fighter, Hash40::new("kamui_transform_dragon"), false, false);
//         macros::EFFECT(fighter, Hash40::new("kamui_water_hamon"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
//     }
//     frame(fighter.lua_state_agent, 10.0);
//     if macros::is_excute(fighter) {
//         macros::EFFECT(fighter, Hash40::new("kamui_water_splash"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
//     }
//     frame(fighter.lua_state_agent, 17.0);
//     if macros::is_excute(fighter) {
//         macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
//     }
//     frame(fighter.lua_state_agent, 36.0);
//     if macros::is_excute(fighter) {
//         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_REQ_EFEECT_TRANSFORM_WIND);
//     }
//     frame(fighter.lua_state_agent, 43.0);
//     if macros::is_excute(fighter) {
//         if DRAGON_INSTALL[entry_id(fighter.module_accessor)] <= 0.0 {
//             macros::EFFECT_FOLLOW(fighter, Hash40::new("kamui_transform_splash_end"), Hash40::new("neck"), 2, 0, 0, 0, 0, 0, 1, true);
//         }
//         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFEECT_TRANSFORM_WIND);
//     }
// }

// #[acmd_script( agent = "kamui", script = "expression_throwhi", category = ACMD_EXPRESSION, low_priority )]
// unsafe fn kamui_uthrowexp(fighter: &mut L2CAgentBase) {
//     let mut di = false;
//     if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
//         di = true;
//     }
//     if macros::is_excute(fighter) {
//         fighter.clear_lua_stack();
//         lua_args!(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, *CAMERA_QUAKE_KIND_NONE);
//         FT_ATTACK_ABS_CAMERA_QUAKE(fighter.lua_state_agent);
//         slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("dragon") as i64, hash40("dragon_horn") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 2.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("front_hair") as i64, hash40("front_hair_hide") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 3.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("hair") as i64, hash40("hair_hide") as i64);
//         }
//         slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
//     }
//     frame(fighter.lua_state_agent, 12.0);
//     if macros::is_excute(fighter) {
//         macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
//         macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
//     }
//     frame(fighter.lua_state_agent, 43.0);
//     di = false;
//     if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
//         di = true;
//     }
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("hair") as i64, hash40("hair_normal") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 44.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("front_hair") as i64, hash40("front_hair_normal") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 47.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("dragon") as i64, hash40("dragon_none") as i64);
//         }
//     }
// }

// #[acmd_script( agent = "kamui", script = "effect_throwlw", category = ACMD_EFFECT, low_priority )]
// unsafe fn kamui_dthroweff(fighter: &mut L2CAgentBase) {
//     if macros::is_excute(fighter) {
//         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_REQ_EFEECT_TRANSFORM_WIND);
//         if DRAGON_INSTALL[entry_id(fighter.module_accessor)] <= 0.0 {
// 	        macros::EFFECT_FLW_POS(fighter, Hash40::new("kamui_transform_splash_start"), Hash40::new("neck"), 2, -1, 0, 0, 0, 0, 1, true);
//         }
//     }
//     frame(fighter.lua_state_agent, 7.0);
//     if macros::is_excute(fighter) {
//         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFEECT_TRANSFORM_WIND);
// 	    macros::EFFECT_OFF_KIND(fighter, Hash40::new("kamui_transform_dragon"), false, false);
//     }
//     frame(fighter.lua_state_agent, 26.0);
//     if macros::is_excute(fighter) {
//         macros::EFFECT(fighter, Hash40::new("kamui_counter_ripple"), Hash40::new("top"), 0, 0, 6.5, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
// 	    macros::EFFECT(fighter, Hash40::new("kamui_water_splash"), Hash40::new("top"), 0, 0, 5.5, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, true);
//     }
//     frame(fighter.lua_state_agent, 33.0);
//     if macros::is_excute(fighter) {
//         macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 15, 15, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
//     }
//     frame(fighter.lua_state_agent, 40.0);
//     if macros::is_excute(fighter) {
//         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_REQ_EFEECT_TRANSFORM_WIND);
//     }
//     frame(fighter.lua_state_agent, 45.0);
//     if macros::is_excute(fighter) {
//         if DRAGON_INSTALL[entry_id(fighter.module_accessor)] <= 0.0 {
//             macros::EFFECT_FOLLOW(fighter, Hash40::new("kamui_transform_splash_end"), Hash40::new("neck"), 2, -1, 0, 0, 0, 0, 1, true);
//         }
//         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFEECT_TRANSFORM_WIND);
//     }
// }

// #[acmd_script( agent = "kamui", script = "expression_throwlw", category = ACMD_EXPRESSION, low_priority )]
// unsafe fn kamui_dthrowexp(fighter: &mut L2CAgentBase) {
//     let mut di = false;
//     if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
//         di = true;
//     }
//     if macros::is_excute(fighter) {
//         fighter.clear_lua_stack();
//         lua_args!(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, *CAMERA_QUAKE_KIND_NONE);
//         FT_ATTACK_ABS_CAMERA_QUAKE(fighter.lua_state_agent);
//         slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("dragon") as i64, hash40("dragon_horn") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 2.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("front_hair") as i64, hash40("front_hair_hide") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 3.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("hair") as i64, hash40("hair_hide") as i64);
//         }
//         slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
//     }
//     frame(fighter.lua_state_agent, 26.0);
//     if macros::is_excute(fighter) {
//         macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
//         macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
//     }
//     frame(fighter.lua_state_agent, 45.0);
//     di = false;
//     if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
//         di = true;
//     }
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("hair") as i64, hash40("hair_normal") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 46.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("front_hair") as i64, hash40("front_hair_normal") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 49.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("dragon") as i64, hash40("dragon_none") as i64);
//         }
//     }
// }

// #[acmd_script( agent = "kamui", script = "effect_attackairn", category = ACMD_EFFECT, low_priority )]
// unsafe fn kamui_naireff(fighter: &mut L2CAgentBase) {
//     if macros::is_excute(fighter) {
//         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_REQ_EFEECT_TRANSFORM_WIND);
//         if DRAGON_INSTALL[entry_id(fighter.module_accessor)] <= 0.0 {
//     	    macros::EFFECT_FLW_POS(fighter, Hash40::new("kamui_transform_splash_start"), Hash40::new("neck"), 2, -1, 0, 0, 0, 0, 1, true);
//         }
//     }
//     frame(fighter.lua_state_agent, 5.0);
//     if macros::is_excute(fighter) {
//         macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_kamui_sword1"), Hash40::new("tex_kamui_sword2"), 5, Hash40::new("haver"), 0.0, 0.1, 0.0, Hash40::new("haver"), -0.0, 15.0, 0.0, true, Hash40::new("kamui_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
//         let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
//         if color == 0 {
//             macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_kamui_spear_01"), Hash40::new("tex_kamui_spear"), 7, Hash40::new("arml"), 2.0, 0.0, 0.0, Hash40::new("arml"), 13.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("arml"), 6, 0, 0, 0, 90, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.1, 0.2);
//         }
//         if color == 1 {
//             macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_kamui_spear_01"), Hash40::new("tex_kamui_spear"), 7, Hash40::new("arml"), 2.0, 0.0, 0.0, Hash40::new("arml"), 13.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("arml"), 6, 0, 0, 0, 90, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.1, 0.2);
//         }
//         if color == 2 {
//             macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_kamui_spear_03"), Hash40::new("tex_kamui_spear"), 7, Hash40::new("arml"), 2.0, 0.0, 0.0, Hash40::new("arml"), 13.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("arml"), 6, 0, 0, 0, 90, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.1, 0.2);
//         }
//         if color == 3 {
//             macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_kamui_spear_04"), Hash40::new("tex_kamui_spear"), 7, Hash40::new("arml"), 2.0, 0.0, 0.0, Hash40::new("arml"), 13.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("arml"), 6, 0, 0, 0, 90, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.1, 0.2);
//         }
//         if color == 4 {
//             macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_kamui_spear_05"), Hash40::new("tex_kamui_spear"), 7, Hash40::new("arml"), 2.0, 0.0, 0.0, Hash40::new("arml"), 13.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("arml"), 6, 0, 0, 0, 90, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.1, 0.2);
//         }
//         if color == 5 {
//             macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_kamui_spear_06"), Hash40::new("tex_kamui_spear"), 7, Hash40::new("arml"), 2.0, 0.0, 0.0, Hash40::new("arml"), 13.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("arml"), 6, 0, 0, 0, 90, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.1, 0.2);
//         }
//         if color == 6 {
//             macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_kamui_spear_07"), Hash40::new("tex_kamui_spear"), 7, Hash40::new("arml"), 2.0, 0.0, 0.0, Hash40::new("arml"), 13.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("arml"), 6, 0, 0, 0, 90, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.1, 0.2);
//         }
//         if color == 7 {
//             macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_kamui_spear_08"), Hash40::new("tex_kamui_spear"), 7, Hash40::new("arml"), 2.0, 0.0, 0.0, Hash40::new("arml"), 13.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("arml"), 6, 0, 0, 0, 90, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.1, 0.2);
//         }
//     }
//     frame(fighter.lua_state_agent, 20.0);
//     if macros::is_excute(fighter) {
//         macros::AFTER_IMAGE_OFF(fighter, 5);
//     }
//     frame(fighter.lua_state_agent, 34.0);
//     if macros::is_excute(fighter) {
//         if DRAGON_INSTALL[entry_id(fighter.module_accessor)] <= 0.0 {
//             macros::EFFECT_FOLLOW(fighter, Hash40::new("kamui_transform_splash_end"), Hash40::new("neck"), 2, -1, 0, 0, 0, 0, 1, true);
//         }
//     }
//     frame(fighter.lua_state_agent, 39.0);
//     if macros::is_excute(fighter) {
//         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFEECT_TRANSFORM_WIND);
//     }
// }

// #[acmd_script( agent = "kamui", script = "expression_attackairn", category = ACMD_EXPRESSION, low_priority )]
// unsafe fn kamui_nairexp(fighter: &mut L2CAgentBase) {
//     let mut di = false;
//     if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
//         di = true;
//     }
//     if macros::is_excute(fighter) {
//         ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("dragon") as i64, hash40("dragon_horn") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 4.0);
//     if macros::is_excute(fighter) {
//         ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, 0);
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("front_hair") as i64, hash40("front_hair_hide") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 5.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("hair") as i64, hash40("hair_hide") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 15.0);
//     if macros::is_excute(fighter) {
//         ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, 0);
//     }
//     frame(fighter.lua_state_agent, 37.0);
//     di = false;
//     if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
//         di = true;
//     }
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("hair") as i64, hash40("hair_normal") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 38.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("front_hair") as i64, hash40("front_hair_normal") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 41.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("dragon") as i64, hash40("dragon_none") as i64);
//         }
//     }
// }

// #[acmd_script( agent = "kamui", script = "effect_landingairn", category = ACMD_EFFECT, low_priority )]
// unsafe fn kamui_nairlandeff(fighter: &mut L2CAgentBase) {
//     if macros::is_excute(fighter) {
//         if DRAGON_INSTALL[entry_id(fighter.module_accessor)] <= 0.0 {
//     	    macros::EFFECT_FLW_POS(fighter, Hash40::new("kamui_transform_splash_start"), Hash40::new("neck"), 2, -1, 0, 0, 0, 0, 1, true);
//         }
//         macros::LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
//     }
// }

#[acmd_script( agent = "kamui", script = "game_attackairf", category = ACMD_GAME, low_priority )]
unsafe fn kamui_fair(fighter: &mut L2CAgentBase) {
    let mut di = false;
    if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
        di = true;
    }
    frame(fighter.lua_state_agent, 7.0);
    let mut hold = false;
    if di && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        hold = true;
    }
    if hold {
        macros::FT_MOTION_RATE(fighter, 5.0);
    }
    frame(fighter.lua_state_agent, 8.0);
    if hold {
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        if !hold {
            macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 9.0, 65, 87, 0, 40, 4.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 9.0, 65, 87, 0, 40, 4.0, -1.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 9.0, 65, 87, 0, 40, 4.0, -1.0, 8.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        if hold {
            macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 2.0, 367, 35, 0, 22, 4.0, -1.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 2.0, 367, 35, 0, 22, 4.0, -1.0, 4.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 2.0, 367, 35, 0, 22, 4.0, -1.0, 8.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    if hold {
        macros::FT_MOTION_RATE(fighter, 4.0);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        if hold {
            AttackModule::clear_all(fighter.module_accessor);
            macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 3.0, 65, 87, 0, 60, 4.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 3.0, 65, 87, 0, 60, 4.0, -1.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 3.0, 65, 87, 0, 60, 4.0, -1.0, 8.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    if hold {
        macros::FT_MOTION_RATE(fighter, 2.0);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    if hold {
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 59.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "kamui", script = "effect_attackairf", category = ACMD_EFFECT, low_priority )]
unsafe fn kamui_faireff(fighter: &mut L2CAgentBase) {
    let mut di = false;
    if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
        di = true;
    }
    frame(fighter.lua_state_agent, 7.0);
    let mut hold = false;
    if di && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        hold = true;
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        if !hold {
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_kamui_sword1"), Hash40::new("tex_kamui_sword2"), 5, Hash40::new("haver"), 0.0, 0.1, 0.0, Hash40::new("haver"), -0.0, 15.0, 0.0, true, Hash40::new("kamui_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        }
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        if hold {
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_kamui_sword1"), Hash40::new("tex_kamui_sword2"), 5, Hash40::new("haver"), 0.0, 0.1, 0.0, Hash40::new("haver"), -0.0, 15.0, 0.0, true, Hash40::new("kamui_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        }
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE_OFF(fighter, 3);
    }
}

#[acmd_script( agent = "kamui", script = "game_attackairb", category = ACMD_GAME, low_priority )]
unsafe fn kamui_bair(fighter: &mut L2CAgentBase) {
    let mut di = false;
    if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
        di = true;
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        if di {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 1.0, y: 0.5, z: 0.0});
        }
        else {
            macros::SET_SPEED_EX(fighter, 1.2, 0.25, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 361, 100, 0, 40, 7.5, 0.0, 10.0, -12.5, Some(0.0), Some(11.0), Some(-17.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 361, 100, 0, 40, 5.7, 0.0, 10.7, -12.5, Some(0.0), Some(12.2), Some(-21.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 42.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

// #[acmd_script( agent = "kamui", script = "effect_attackairb", category = ACMD_EFFECT, low_priority )]
// unsafe fn kamui_baireff(fighter: &mut L2CAgentBase) {
//     if macros::is_excute(fighter) {
//         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_REQ_EFEECT_TRANSFORM_WIND);
//         if DRAGON_INSTALL[entry_id(fighter.module_accessor)] <= 0.0 {
//     	    macros::EFFECT_FLW_POS(fighter, Hash40::new("kamui_transform_splash_start"), Hash40::new("neck"), 2, -1, 0, 0, 0, 0, 1, true);
//         }
//     }
//     frame(fighter.lua_state_agent, 12.0);
//     if macros::is_excute(fighter) {
//         macros::EFFECT_FLW_POS(fighter, Hash40::new("kamui_transform_splash_start"), Hash40::new("neck"), 2, 0, 0, 0, 0, 0, 1, true);
//         macros::LAST_EFFECT_SET_RATE(fighter, 1.3);
//     }
//     frame(fighter.lua_state_agent, 13.0);
//     if macros::is_excute(fighter) {
//         macros::EFFECT_FLW_POS(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 12.3, -21, 0, 0, 0, 1.1, true);
//     }
//     frame(fighter.lua_state_agent, 39.0);
//     if macros::is_excute(fighter) {
//         if DRAGON_INSTALL[entry_id(fighter.module_accessor)] <= 0.0 {
//             macros::EFFECT_FOLLOW(fighter, Hash40::new("kamui_transform_splash_end"), Hash40::new("neck"), 2, -1, 0, 0, 0, 0, 1, true);
//         }
//     }
// }

// #[acmd_script( agent = "kamui", script = "expression_attackairb", category = ACMD_EXPRESSION, low_priority )]
// unsafe fn kamui_bairexp(fighter: &mut L2CAgentBase) {
//     let mut di = false;
//     if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
//         di = true;
//     }
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("dragon") as i64, hash40("dragon_wing_no_horn") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 2.0);
//     if macros::is_excute(fighter) {
//         VisibilityModule::set_int64(fighter.module_accessor, hash40("dragon") as i64, hash40("dragon_wing") as i64);
//         if di {
//             ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_mask"), true);
//         }
//     }
//     frame(fighter.lua_state_agent, 5.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("front_hair") as i64, hash40("front_hair_hide") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 6.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("hair") as i64, hash40("hair_hide") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 11.0);
//     if macros::is_excute(fighter) {
//         ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohit_wing"), 0, false, 0);
//     }
//     frame(fighter.lua_state_agent, 13.0);
//     if macros::is_excute(fighter) {
//         macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
//     }
//     frame(fighter.lua_state_agent, 40.0);
//     di = false;
//     if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
//         di = true;
//     }
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("hair") as i64, hash40("hair_normal") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 41.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("front_hair") as i64, hash40("front_hair_normal") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 44.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("dragon") as i64, hash40("dragon_wing_no_horn") as i64);
//         }
//         else {
//             ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_mask"), true);
//             ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_fronthair"), false);
//             ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_hair"), false);
//         }
//     }
// }

// #[acmd_script( agent = "kamui", script = "expression_landingairb", category = ACMD_EXPRESSION, low_priority )]
// unsafe fn kamui_bairlandexp(fighter: &mut L2CAgentBase) {
//     let mut di = false;
//     if DRAGON_INSTALL[entry_id(fighter.module_accessor)] >= 0.0 {
//         di = true;
//     }
//     if macros::is_excute(fighter) {
//         if !di {
//     	    VisibilityModule::set_int64(fighter.module_accessor, hash40("dragon") as i64, hash40("dragon_wing") as i64);
// 	        VisibilityModule::set_int64(fighter.module_accessor, hash40("front_hair") as i64, hash40("front_hair_hide") as i64);
// 	        VisibilityModule::set_int64(fighter.module_accessor, hash40("hair") as i64, hash40("hair_hide") as i64);
//         }
//         ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_lands"), 0, false, 0);
//         slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
//     }
//     frame(fighter.lua_state_agent, 20.0);
//     di = false;
//     if DRAGON_INSTALL[entry_id(fighter.module_accessor)] >= 0.0 {
//         di = true;
//     }
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("hair") as i64, hash40("hair_normal") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 22.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("front_hair") as i64, hash40("front_hair_normal") as i64);
//         }
//     }
//     frame(fighter.lua_state_agent, 25.0);
//     if macros::is_excute(fighter) {
//         VisibilityModule::set_int64(fighter.module_accessor, hash40("dragon") as i64, 0x1349b2d9a3 as i64);
//         if di {
//             ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_mask"), true);
//         }
//     }
// }

#[acmd_script( agent = "kamui", script = "game_attackairhi", category = ACMD_GAME, low_priority )]
unsafe fn kamui_uair(fighter: &mut L2CAgentBase) {
    let mut di = false;
    if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
        di = true;
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 6.0);
    let mut hold = false;
    if di && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        hold = true;
    }
    if macros::is_excute(fighter) {
        if hold {
            macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 2.0, 367, 35, 0, 22, 5.5, 0.0, 0.0, -1.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 2.0, 367, 35, 0, 22, 5.5, 0.0, 4.0, -1.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 2.0, 367, 35, 0, 22, 5.5, 0.0, 8.0, -1.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    if hold {
        macros::FT_MOTION_RATE(fighter, 4.0);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        if !hold {
            macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 10.0, 70, 97, 0, 55, 5.5, 0.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 10.0, 70, 97, 0, 55, 5.5, 0.0, 4.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 10.0, 70, 97, 0, 55, 5.5, 0.0, 8.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        if hold {
            AttackModule::clear_all(fighter.module_accessor);
            macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 4.0, 70, 97, 0, 55, 5.5, 0.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 4.0, 70, 97, 0, 55, 5.5, 0.0, 4.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 4.0, 70, 97, 0, 55, 5.5, 0.0, 8.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    if hold {
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
    wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 38.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "kamui", scripts = ["game_specialsjump", "game_specialairsjump"], category = ACMD_GAME, low_priority )]
unsafe fn kamui_specialsjump(fighter: &mut L2CAgentBase) {
    let mut di = false;
    if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
        di = true;
    }
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, false, 0);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, Hash40::new("special_s_jump"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_S_FLAG_ATTACK);
    }
    frame(fighter.lua_state_agent, 8.0);
    if di {
        if macros::is_excute(fighter) {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_S_FLAG_ATTACK);
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        }
    }
}

#[acmd_script( agent = "kamui", scripts = ["game_speciallwhit", "game_specialairlwhit"], category = ACMD_GAME, low_priority )]
unsafe fn kamui_speciallwhit(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, false, 0);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        VisibilityModule::set_whole(fighter.module_accessor, false);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, 0.5);
    }
    macros::FT_MOTION_RATE(fighter, 2.0);
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, 1.0);
    }
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        VisibilityModule::set_whole(fighter.module_accessor, true);
    }
}

// #[acmd_script( agent = "kamui", scripts = ["expression_speciallwhit", "expression_specialairlwhit"], category = ACMD_EXPRESSION, low_priority )]
// unsafe fn kamui_speciallwhitexp(fighter: &mut L2CAgentBase) {
//     if macros::is_excute(fighter) {
//         ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_counter"), 0, false, 0);
//         slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
//         ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_mask"), true);
//         ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_fronthair"), false);
//         ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_hair"), false);
//     }
//     frame(fighter.lua_state_agent, 26.0);
//     if macros::is_excute(fighter) {
//         macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
//         ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosionm"), 0, false, 0);
//     }
//     frame(fighter.lua_state_agent, 46.0);
//     let mut di = false;
//     if macros::is_excute(fighter) {
//         if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
//             di = true;
//         }
//         if !di {
//             ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_hair"), true);
//         }
//     }
//     frame(fighter.lua_state_agent, 47.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_fronthair"), true);
//         }
//     }
//     frame(fighter.lua_state_agent, 50.0);
//     if macros::is_excute(fighter) {
//         if !di {
//             ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_mask"), false);
//         }
//     }
// }

#[acmd_script( agent = "kamui_waterdragon", scripts = ["game_speciallwhit", "game_specialairlwhit"], category = ACMD_GAME, low_priority )]
unsafe fn kamui_waterdragon_speciallwhit(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 361, 30, 0, 60, 12.0, 0.0, 6.0, 11.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 361, 30, 0, 60, 12.0, 0.0, 6.0, -11.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 14.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 14.0, false);
        AttackModule::set_optional_hit_sound(fighter.module_accessor, 0, Hash40::new("se_kamui_criticalhit"));
        AttackModule::set_optional_hit_sound(fighter.module_accessor, 1, Hash40::new("se_kamui_criticalhit"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 361, 30, 0, 60, 8.0, 0.0, 21.0, 11.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 8.0, 361, 30, 0, 60, 8.0, 0.0, 21.0, -11.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 14.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 3, 14.0, false);
        AttackModule::set_optional_hit_sound(fighter.module_accessor, 2, Hash40::new("se_kamui_criticalhit"));
        AttackModule::set_optional_hit_sound(fighter.module_accessor, 3, Hash40::new("se_kamui_criticalhit"));
    }
    frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            DRAGON_INSTALL[entry_id(fighter.module_accessor)] = 600.0;
            _TIME_COUNTER[entry_id(fighter.module_accessor)] = 24;
            // SET_DRAGON_OFF[entry_id(fighter.module_accessor)] = false;
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        kamui_frame
    );
    smashline::install_status_scripts!(
        kamui_speciallwpre,
        kamui_speciallw,
        kamui_speciallwend
    );
    smashline::install_acmd_scripts!(
        // kamui_utauntexp,
        kamui_jab2,
        kamui_jab3,
        // kamui_rapidjabendeff,
        // kamui_rapidjabendexp,
        kamui_ftilt,
        kamui_utilt,
        kamui_dtilt,
        // kamui_fsmasheff,
        // kamui_fsmashhieff,
        // kamui_fsmashlweff,
        // kamui_fsmashexp,
        // kamui_usmasheff,
        // kamui_usmashexp,
        // kamui_dsmasheff,
        // kamui_dsmashexp,
        // kamui_fthroweff,
        // kamui_fthrowexp,
        // kamui_bthroweff,
        // kamui_bthrowexp,
        // kamui_uthroweff,
        // kamui_uthrowexp,
        // kamui_dthroweff,
        // kamui_dthrowexp,
        // kamui_naireff,
        // kamui_nairexp,
        // kamui_nairlandeff,
        kamui_fair,
        kamui_faireff,
        kamui_bair,
        // kamui_baireff,
        // kamui_bairexp,
        // kamui_bairlandexp,
        kamui_uair,
        kamui_specialsjump,
        kamui_speciallwhit,
        // kamui_speciallwhitexp,
        kamui_waterdragon_speciallwhit
    );
}