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
    vars::*,
    table_const::*,
    common_funcs::*,
    gameplay::*
};

#[inline(always)]
pub unsafe fn bonker_vis(module_accessor: *mut BattleObjectModuleAccessor) {
    if IS_BONKER[entry_id(module_accessor)] == 4 {
        ModelModule::set_mesh_visibility(module_accessor, Hash40::new("hammer"), false);
        ModelModule::set_mesh_visibility(module_accessor, Hash40::new("bonker"), true);
    }
    else {
        ModelModule::set_mesh_visibility(module_accessor, Hash40::new("hammer"), true);
        ModelModule::set_mesh_visibility(module_accessor, Hash40::new("bonker"), false);
    }
}

#[inline(always)]
pub unsafe fn mario_fgc(fighter: &mut L2CFighterCommon) {
    let status = StatusModule::status_kind(fighter.module_accessor);
    let mut allowed_cancels : Vec<i32> = [].to_vec();
    set_hp(fighter, 105.0);
    if [
        *FIGHTER_STATUS_KIND_ATTACK
    ].contains(&status) {
        allowed_cancels = [
            *FIGHTER_STATUS_KIND_ATTACK_S3,
            *FIGHTER_STATUS_KIND_ATTACK_LW3,
            *FIGHTER_STATUS_KIND_ATTACK_HI3,
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            *FIGHTER_STATUS_KIND_SPECIAL_HI
        ].to_vec();
    }
    else if [
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_DASH,
        *FIGHTER_STATUS_KIND_ATTACK_AIR
    ].contains(&status) {
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_hi") {
            jump_cancel_check_hit(fighter, false);
        }
        else if status == *FIGHTER_STATUS_KIND_ATTACK_S3 {
            cancel_exceptions(fighter, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3, true);
        }
        allowed_cancels = [
            *FIGHTER_STATUS_KIND_ATTACK_S4,
            *FIGHTER_STATUS_KIND_ATTACK_HI4,
            *FIGHTER_STATUS_KIND_ATTACK_LW4,
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            *FIGHTER_STATUS_KIND_SPECIAL_HI
        ].to_vec();
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_n") {
            allowed_cancels.append(&mut [*FIGHTER_STATUS_KIND_ATTACK_AIR].to_vec());
        }
    }
    else if [
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4
    ].contains(&status) {
        if status == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
            jump_cancel_check_hit(fighter, false);
        }
        allowed_cancels = [
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            *FIGHTER_STATUS_KIND_SPECIAL_HI
        ].to_vec();
    }
    else if status == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if FIREBALL_CANCEL[entry_id(fighter.module_accessor)] {
            jump_cancel_check_exception(fighter);
        }
    }
    cancel_system(fighter, status, allowed_cancels);
}

#[fighter_frame( agent = FIGHTER_KIND_MARIO )]
fn mario_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        // if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_DEAD {
        //     IS_BONKER[entry_id(fighter.module_accessor)] = 0;
        // }

        if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("attack_air_lw"){
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
                macros::PLAY_SE(fighter, Hash40::new("se_mario_attackair_l02"));
                let speedx = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * PostureModule::lr(fighter.module_accessor);
                macros::SET_SPEED_EX(fighter, speedx, 0.75, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
        }

        if [
            hash40("attack_air_f"),
            hash40("landing_air_f"),
            hash40("attack_s4_s"),
            hash40("attack_s4_hi"),
            hash40("attack_s4_lw"),
            hash40("attack_s4_hold")
        ].contains(&MotionModule::motion_kind(fighter.module_accessor)) == false {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP) {
                ArticleModule::remove(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            }
        }

        // if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
        //     if IS_BONKER[entry_id(fighter.module_accessor)] == 4 {
        //         IS_BONKER[entry_id(fighter.module_accessor)] = 0;
        //     }
        //     else {
        //         IS_BONKER[entry_id(fighter.module_accessor)] = 4;
        //     }
        // }

        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
        || StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_CLIFF {
            BOUNCE[entry_id(fighter.module_accessor)] = false;
            if ![
                *FIGHTER_STATUS_KIND_SPECIAL_LW,
                *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT,
                *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE
            ].contains(&StatusModule::status_kind(fighter.module_accessor)) {
                SPECIAL_LW_TYPE[entry_id(fighter.module_accessor)] = 0;
            }
        }

        if IS_FGC[entry_id(fighter.module_accessor)] {
            mario_fgc(fighter);
        }
    }
}

#[status_script(agent = "mario", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn mario_specials_init(_fighter: &mut L2CFighterCommon) -> L2CValue {
    L2CValue::I32(0)
}

#[status_script(agent = "mario", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn mario_specials_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
        let air_accel_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y_stable"), 0);
        let air_accel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
        if !BOUNCE[entry_id(fighter.module_accessor)] {
            fighter.clear_lua_stack();
            lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y);
            sv_kinetic_energy::set_accel(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_accel_x * 0.5);
            sv_kinetic_energy::set_accel_x_mul(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_accel_y_stable);
            sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
        }
        else {
            fighter.clear_lua_stack();
            lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y / 1.5);
            sv_kinetic_energy::set_accel(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_accel_x * 0.2);
            sv_kinetic_energy::set_accel_x_mul(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_accel_y_stable / 2.0);
            sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
        }
    }
    L2CValue::I32(0)
}

#[status_script(agent = "mario", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn mario_speciallw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Special Lw Type: 0 for Long Jump, 1 for Ground Pound
    if fighter.global_table[PREV_STATUS_KIND].get_i32() != *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE {
        BLJ_PREV[entry_id(fighter.module_accessor)] = false;
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        SPECIAL_LW_TYPE[entry_id(fighter.module_accessor)] = 0;
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        BLJ_PREV[entry_id(fighter.module_accessor)] = false;
        SPECIAL_LW_TYPE[entry_id(fighter.module_accessor)] = 1;
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(mario_speciallw_main_loop as *const () as _))
}

unsafe extern "C" fn mario_speciallw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT.into(), true.into());
    }
    0.into()
}

#[status_script(agent = "mario", status = FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn mario_speciallw_shoot_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    macros::SA_SET(fighter, *SITUATION_KIND_AIR);
    if SPECIAL_LW_TYPE[entry_id(fighter.module_accessor)] == 0 {
        ControlModule::clear_command(fighter.module_accessor, true);
        BLJ[entry_id(fighter.module_accessor)] = false;
        let dir = get_command_stick_direction(fighter.module_accessor, true);
        let speed_x : f32;
        let speed_y : f32;
        if [6, 3, 9].contains(&dir) {
            speed_x = 1.7;
            speed_y = 1.6;
            LONG_JUMP_KIND[entry_id(fighter.module_accessor)] = 2;
            BLJ_PREV[entry_id(fighter.module_accessor)] = false;
        }
        else if [4, 7, 1].contains(&dir) {
            BLJ[entry_id(fighter.module_accessor)] = true;
            if BLJ_PREV[entry_id(fighter.module_accessor)] {
                speed_x = -1.7;
                speed_y = 0.0;
                LONG_JUMP_KIND[entry_id(fighter.module_accessor)] = 3;
            }
            else {
                speed_x = 1.1;
                speed_y = 1.4;
                LONG_JUMP_KIND[entry_id(fighter.module_accessor)] = 1;
            }
            BLJ_PREV[entry_id(fighter.module_accessor)] = true;
        }
        else {
            speed_x = 1.4;
            speed_y = 1.4;
            LONG_JUMP_KIND[entry_id(fighter.module_accessor)] = 0;
            BLJ_PREV[entry_id(fighter.module_accessor)] = false;
        }
        macros::SET_SPEED_EX(fighter, speed_x, speed_y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    L2CValue::I32(0)
}

#[status_script(agent = "mario", status = FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn mario_speciallw_shoot_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if SPECIAL_LW_TYPE[entry_id(fighter.module_accessor)] == 0 {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let air_accel_mul : f32;
        if KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > 0.0 {
            air_accel_mul = 1.0;
        }
        else {
            air_accel_mul = 0.4;
        }
        let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0) * air_accel_mul;
        let air_accel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
        let air_accel_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y_stable"), 0);
        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y);
        sv_kinetic_energy::set_accel(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_accel_x * 0.6);
        sv_kinetic_energy::set_accel_x_mul(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_accel_y_stable);
        sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
        fighter.clear_lua_stack();
    }
    else {
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let gravity = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let mystery_param : f32 = -4.0;
        lua_bind::FighterKineticEnergyGravity::set_speed(gravity as *mut smash::app::FighterKineticEnergyGravity, mystery_param);
        lua_bind::FighterKineticEnergyGravity::set_accel(gravity as *mut smash::app::FighterKineticEnergyGravity, 0.0);
    }
    L2CValue::I32(0)
}

#[status_script(agent = "mario", status = FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn mario_speciallw_shoot_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Special Lw Type: 0 for Long Jump, 1 for Ground Pound
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    if SPECIAL_LW_TYPE[entry_id(fighter.module_accessor)] == 0 {
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        LONG_JUMP_LANDING[entry_id(fighter.module_accessor)] = false;
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_light"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_light"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(mario_speciallw_shoot_main_loop as *const () as _))
}

unsafe extern "C" fn mario_speciallw_shoot_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if SPECIAL_LW_TYPE[entry_id(fighter.module_accessor)] == 0 {
        if LONG_JUMP_LANDING[entry_id(fighter.module_accessor)] {
            if fighter.sub_air_check_fall_common().get_bool() == false {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                    fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE.into(), false.into());
                }
            }
        }
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        if !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_CATEGORY_MASK_ALL)
        && get_command_stick_direction(fighter.module_accessor, false) == 8 {
            SPECIAL_LW_TYPE[entry_id(fighter.module_accessor)] = 2;
            fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE.into(), false.into());
        }
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE.into(), false.into());
        }
    }
    0.into()
}

#[status_script(agent = "mario", status = FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn mario_speciallw_charge_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Special Lw Type: 0 for Long Jump, 1 for Ground Pound, 2 for Ground Pound Cancel
    if SPECIAL_LW_TYPE[entry_id(fighter.module_accessor)] == 0 {
        // KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_hold"), 0.0, 1.0, false, 0.0, false, false);
    }
    else if SPECIAL_LW_TYPE[entry_id(fighter.module_accessor)] == 1 {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_hold"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        macros::SET_SPEED_EX(fighter, 0.0, -2.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_heavy"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(mario_speciallw_charge_main_loop as *const () as _))
}

unsafe extern "C" fn mario_speciallw_charge_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if SPECIAL_LW_TYPE[entry_id(fighter.module_accessor)] == 2 {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        }
        else if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    else {
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            SPECIAL_LW_TYPE[entry_id(fighter.module_accessor)] = 0;
            fighter.sub_wait_ground_check_common(L2CValue::I32(0));
            fighter.sub_air_check_fall_common();
        }
        if SPECIAL_LW_TYPE[entry_id(fighter.module_accessor)] == 0 {
            cancel_exceptions(fighter, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW, false);
        }
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

#[acmd_script( agent = "mario", script = "game_attacks4", category = ACMD_GAME, low_priority )]
unsafe fn mario_fsmash(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::remove(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, Hash40::new("start"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.7, 361, 105, 0, 25, 2.0, 0.0, 5.0, 6.0, Some(0.0), Some(6.0), Some(14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 17.8, 361, 99, 0, 25, 3.0, 0.0, 6.0, 16.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "mario", script = "effect_attacks4", category = ACMD_EFFECT, low_priority )]
unsafe fn mario_fsmasheff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 2.0, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 1, 6, 3, 0, -39, 10, 1.2, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.196, 0.196, 0.216);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "mario", script = "sound_attacks4", category = ACMD_SOUND, low_priority )]
unsafe fn mario_fsmashsnd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_common_smash_start"));
	    macros::PLAY_SE(fighter, Hash40::new("vc_mario_attack05"));
    }
}

#[acmd_script( agent = "mario", script = "game_attacks4hi", category = ACMD_GAME, low_priority )]
unsafe fn mario_fsmashhi(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::remove(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, Hash40::new("start"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.7, 361, 105, 0, 25, 2.0, 0.0, 5.0, 6.0, Some(0.0), Some(9.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 17.8, 361, 99, 0, 25, 3.0, 0.0, 11.0, 15.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "mario", script = "effect_attacks4hi", category = ACMD_EFFECT, low_priority )]
unsafe fn mario_fsmashhieff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 2.0, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 1, 8, 3, -24, -39, 30, 1.2, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.196, 0.196, 0.216);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "mario", script = "sound_attacks4hi", category = ACMD_SOUND, low_priority )]
unsafe fn mario_fsmashhisnd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_common_smash_start"));
	    macros::PLAY_SE(fighter, Hash40::new("vc_mario_attack05"));
    }
}

#[acmd_script( agent = "mario", script = "game_attacks4lw", category = ACMD_GAME, low_priority )]
unsafe fn mario_fsmashlw(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::remove(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, Hash40::new("start"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.7, 361, 105, 0, 25, 2.0, 0.0, 5.0, 4.0, Some(0.0), Some(4.0), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 17.8, 361, 99, 0, 25, 3.0, 0.0, 3.0, 15.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "mario", script = "effect_attacks4lw", category = ACMD_EFFECT, low_priority )]
unsafe fn mario_fsmashlweff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 2.0, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 1, 6, 3, 12, -39, 0, 1.2, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.196, 0.196, 0.216);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "mario", script = "sound_attacks4lw", category = ACMD_SOUND, low_priority )]
unsafe fn mario_fsmashlwsnd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_common_smash_start"));
	    macros::PLAY_SE(fighter, Hash40::new("vc_mario_attack05"));
    }
}

#[acmd_script( agent = "mario", script = "game_attackairn", category = ACMD_GAME, low_priority )]
unsafe fn mario_nair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        if IS_FGC[entry_id(fighter.module_accessor)] {
            macros::ATTACK(fighter, 0, 0, Hash40::new("legr"), 5.0, 361, 90, 0, 45, 4.0, 0.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 5.0, 361, 90, 0, 45, 4.0, 0.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 8.0, 361, 100, 0, 20, 4.0, 0.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 8.0, 361, 100, 0, 20, 4.0, 0.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
        }
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        if !IS_FGC[entry_id(fighter.module_accessor)] {
            macros::ATTACK(fighter, 0, 0, Hash40::new("legr"), 5.0, 361, 90, 0, 13, 4.0, 0.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 5.0, 361, 90, 0, 13, 4.0, 0.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 39.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "mario", script = "game_attackairf", category = ACMD_GAME, low_priority )]
unsafe fn mario_fair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, Hash40::new("light"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 10.0);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        macros::FT_MOTION_RATE(fighter, 10.0);
    }
    frame(fighter.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 80, 0, 30, 3.0, 0.0, 11.0, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 361, 80, 0, 30, 3.0, 0.0, 1.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 290, 80, 0, 30, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 361, 80, 0, 30, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, Hash40::new("start"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 43.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "mario", script = "effect_attackairf", category = ACMD_EFFECT, low_priority )]
unsafe fn mario_faireff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 7, -1, -3, -11, -140, 1.5, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.8);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.196, 0.196, 0.216);
    }
}

#[acmd_script( agent = "mario", script = "expression_attackairf", category = ACMD_EXPRESSION, low_priority )]
unsafe fn mario_fairexp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, 0);
    }
}

#[acmd_script( agent = "mario", script = "game_landingairf", category = ACMD_GAME, low_priority )]
unsafe fn mario_fairland(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, Hash40::new("start"), false, -1.0);
        // ArticleModule::set_pos(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, Vector3f{x: -3.0, y: 0.0, z: 0.0});
    }
}

#[acmd_script( agent = "mario", script = "game_attackairlw", category = ACMD_GAME, low_priority )]
unsafe fn mario_dair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 6.0/9.0);
    frame(fighter.lua_state_agent, 9.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 361, 65, 90, 0, 3.0, 0.0, 0.0, -2.0, Some(0.0), Some(0.0), Some(2.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 45, 65, 90, 0, 3.0, 0.0, 0.0, -2.0, Some(0.0), Some(0.0), Some(2.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 3.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 3.0, false);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 40, 90, 0, 60, 4.0, 0.0, 0.0, -2.0, Some(0.0), Some(0.0), Some(2.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 56.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 63.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "mario", script = "effect_attackairlw", category = ACMD_EFFECT, low_priority )]
unsafe fn mario_daireff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 6.0, 0, 0, 0, 0, 0.6, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 0.0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 3.0, 0, 0, 0, 0, 0.55, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 6.0, 0, 0, 0, 0, 0.6, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 0.0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 3.0, 0, 0, 0, 0, 0.55, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 6.0, 0, 0, 0, 0, 0.6, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 0.0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 33.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 3.0, 0, 0, 0, 0, 0.55, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 44.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 6.0, 0, 0, 0, 0, 0.6, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 45.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 0.0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 47.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 3.0, 0, 0, 0, 0, 0.55, true, *EF_FLIP_YZ);
    }
}

#[acmd_script( agent = "mario", script = "sound_attackairlw", category = ACMD_SOUND, low_priority )]
unsafe fn mario_dairsnd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_mario_002"));
        macros::PLAY_SE(fighter, Hash40::new("se_mario_attackair_l01"));
    } 
}

#[acmd_script( agent = "mario", script = "game_landingairlw", category = ACMD_GAME, low_priority )]
unsafe fn mario_dairlanding(_fighter: &mut L2CAgentBase) {

}

#[acmd_script( agent = "mario", scripts = [ "game_specialn", "game_specialairn" ], category = ACMD_GAME, low_priority )]
unsafe fn mario_nspecial(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        FIREBALL_CANCEL[entry_id(fighter.module_accessor)] = false;
    }
    if !IS_FGC[entry_id(fighter.module_accessor)] {
        macros::FT_MOTION_RATE(fighter, 1.15);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_FIREBALL, false, -1);
    }
    // frame(fighter.lua_state_agent, 15.0);
    // if macros::is_excute(fighter) {
    //     if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
    //         notify_event_msc_cmd!(fighter, 0x2b94de0d96u64, *FIGHTER_LOG_ACTION_CATEGORY_ATTACK, *FIGHTER_LOG_ATTACK_KIND_SPECIAL_N, true);
    //         if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_GROUND {
    //             MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n"), 5.0, 1.0, false, 0.0, false, false);
    //         }
    //         else {
    //             MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n"), 5.0, 1.0, false, 0.0, false, false);
    //         }
    //     }
    // }
}

#[acmd_script( agent = "mario", scripts = [ "game_specials", "game_specialairs" ], category = ACMD_GAME, low_priority )]
unsafe fn mario_sspecial(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::set_float(fighter.module_accessor, 9.0, *FIGHTER_MARIO_STATUS_SPECIAL_S_WORK_ID_FLOAT_REFLECT_MOTION_FRAME);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        if !BOUNCE[entry_id(fighter.module_accessor)]
        && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
            macros::SET_SPEED_EX(fighter, 0.0, 1.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            BOUNCE[entry_id(fighter.module_accessor)] = true;
        }
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIO_REFLECTOR_KIND_MANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 80, 0, 65, 4.0, 0.0, 9.0, 10.0, Some(0.0), Some(5.0), Some(10.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 361, 50, 0, 65, 3.0, 0.0, 9.0, 7.0, Some(0.0), Some(9.0), Some(-10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 361, 50, 0, 65, 3.0, 0.0, 5.0, 7.0, Some(0.0), Some(5.0), Some(-10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIO_REFLECTOR_KIND_MANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "mario", script = "effect_specials", category = ACMD_EFFECT, low_priority )]
unsafe fn mario_sspecialeff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mario_supermant_wind_r"), Hash40::new("mario_supermant_wind_l"), Hash40::new("top"), 2.5, 5, -2.0, 22, 0, 0, 1.2, true, *EF_FLIP_NONE);
        macros::LAST_EFFECT_SET_ALPHA(fighter, 0.75);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_NONE);
    }
}

#[acmd_script( agent = "mario", script = "effect_specialairs", category = ACMD_EFFECT, low_priority )]
unsafe fn mario_sspecialaireff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mario_supermant_wind_r"), Hash40::new("mario_supermant_wind_l"), Hash40::new("top"), 2.5, 5, -2.0, 22, 0, 0, 1.2, true, *EF_FLIP_NONE);
        macros::LAST_EFFECT_SET_ALPHA(fighter, 0.75);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_NONE);
    }
}

#[acmd_script( agent = "mario", script = "game_specialhi", category = ACMD_GAME, low_priority )]
unsafe fn mario_uspecial(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 3.0);
    frame(fighter.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    let mut effect = Hash40::new("collision_attr_coin");
    let mut sound = *COLLISION_SOUND_ATTR_COIN;
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_HI_FLAG_CAPPY) {
        effect.hash = hash40("collision_attr_mario_local_coin");
        sound = *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN;
    }
    if macros::is_excute(fighter) {
        macros::SA_SET(fighter, *SITUATION_KIND_AIR);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 60, 100, 160, 0, 2.5, 0.0, 6.5, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, effect, *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 86, 100, 150, 0, 4.0, 0.0, 6.5, 5.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, effect, *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 100, 100, 150, 0, 4.0, 0.0, 6.3, 9.2, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, effect, *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_PUNCH);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.6, 60, 100, 180, 0, 3.0, 0.0, 6.5, 2.5, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, effect, *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.6, 92, 100, 170, 0, 3.8, 0.0, 6.5, 8.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, effect, *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.6, 60, 100, 110, 0, 3.0, 0.0, 11.5, 2.5, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, effect, *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 0.6, 92, 100, 110, 0, 3.0, 0.0, 11.5, 8.5, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, effect, *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_PUNCH);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 1, true, false);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 2, true, false);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 3, true, false);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(fighter.lua_state_agent, 17.0);
    if sound == *COLLISION_SOUND_ATTR_COIN {
        sound = *COLLISION_SOUND_ATTR_MARIO_COIN_LAST;
    }
    else {
        sound = *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN_LAST;
    }
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 60, 145, 0, 50, 9.0, 0.0, 11.5, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, effect, *ATTACK_SOUND_LEVEL_L, sound, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 60, 145, 0, 50, 9.0, 0.0, 11.5, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, effect, *ATTACK_SOUND_LEVEL_L, sound, *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "mario", script = "sound_speciallwstart", category = ACMD_SOUND, low_priority )]
unsafe fn mario_dspecialstartsnd(_fighter: &mut L2CAgentBase) {
}

#[acmd_script( agent = "mario", script = "game_speciallwlight", category = ACMD_GAME, low_priority )]
unsafe fn mario_dspecialjump(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        if LONG_JUMP_KIND[entry_id(fighter.module_accessor)] == 3 {
            LONG_JUMP_LANDING[entry_id(fighter.module_accessor)] = true;
        }
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        LONG_JUMP_LANDING[entry_id(fighter.module_accessor)] = true;
        if LONG_JUMP_KIND[entry_id(fighter.module_accessor)] == 0
        || LONG_JUMP_KIND[entry_id(fighter.module_accessor)] == 3 {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        if LONG_JUMP_KIND[entry_id(fighter.module_accessor)] == 1 {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        if LONG_JUMP_KIND[entry_id(fighter.module_accessor)] == 2 {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
}

#[acmd_script( agent = "mario", script = "sound_speciallwlight", category = ACMD_SOUND, low_priority )]
unsafe fn mario_dspecialjumpsnd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_mario_009"));
    }
}

#[acmd_script( agent = "mario", script = "sound_speciallwhold", category = ACMD_SOUND, low_priority )]
unsafe fn mario_dspeciallandsnd(_fighter: &mut L2CAgentBase) {
}

#[acmd_script( agent = "mario", script = "effect_speciallwhold", category = ACMD_EFFECT, low_priority )]
unsafe fn mario_dspeciallandeff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "mario", script = "game_specialairlwstart", category = ACMD_GAME, low_priority )]
unsafe fn mario_dspecialairstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        KineticModule::unable_energy_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "mario", script = "sound_specialairlwstart", category = ACMD_SOUND, low_priority )]
unsafe fn mario_dspecialairstartsnd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_mario_special_l04"));
    }
}

#[acmd_script( agent = "mario", script = "game_specialairlwlight", category = ACMD_GAME, low_priority )]
unsafe fn mario_dspecialpound(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 30, 50, 0, 50, 5.0, 0.0, 2.8, -2.0, Some(0.0), Some(2.8), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HIP);
    }
}

#[acmd_script( agent = "mario", script = "sound_specialairlwlight", category = ACMD_SOUND, low_priority )]
unsafe fn mario_dspecialpoundsnd(_fighter: &mut L2CAgentBase) {
}

#[acmd_script( agent = "mario", script = "expression_specialairlwlight", category = ACMD_EXPRESSION, low_priority )]
unsafe fn mario_dspecialpoundexp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, 0);
    }
}

#[acmd_script( agent = "mario", script = "game_specialairlwheavy", category = ACMD_GAME, low_priority )]
unsafe fn mario_dspecialcancel(_fighter: &mut L2CAgentBase) {
}

#[acmd_script( agent = "mario", script = "sound_specialairlwheavy", category = ACMD_SOUND, low_priority )]
unsafe fn mario_dspecialcancelsnd(_fighter: &mut L2CAgentBase) {
}

#[acmd_script( agent = "mario", script = "game_specialairlwhold", category = ACMD_GAME, low_priority )]
unsafe fn mario_dspecialpoundland(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 30, 50, 0, 60, 5.0, 0.0, 2.8, -3.0, Some(0.0), Some(2.8), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HIP);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "mario", script = "sound_specialairlwhold", category = ACMD_SOUND, low_priority )]
unsafe fn mario_dspecialpoundlandsnd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_mario_special_l03"));
    }
}

#[acmd_script( agent = "mario", script = "effect_specialairlwhold", category = ACMD_EFFECT, low_priority )]
unsafe fn mario_dspecialpoundlandeff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "mario", script = "expression_specialairlwhold", category = ACMD_EXPRESSION, low_priority )]
unsafe fn mario_dspecialpoundlandexp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_impact"), 0, false, 0);
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

#[acmd_script( agent = "mario_fireball", script = "game_regular", category = ACMD_GAME, low_priority )]
unsafe fn mario_fireball_regular(weapon: &mut L2CAgentBase) {
    let angle : u64;
    let bkb : i32;
    let kbg : i32;
    if IS_FGC[entry_id(weapon.module_accessor)] {
        angle = 80;
        bkb = 90;
        kbg = 0;
    }
    else {
        angle = 361;
        bkb = 35;
        kbg = 20;
    }
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 5.0, angle, kbg, 0, bkb, 2.4, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MARIO_FIREBALL, *ATTACK_REGION_NONE);
        AttackModule::enable_safe_pos(weapon.module_accessor);
    }
    frame(weapon.lua_state_agent, 5.0);
    if macros::is_excute(weapon) {
        if !IS_FGC[entry_id(weapon.module_accessor)] {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 5.0, 361, 15, 0, 28, 2.2, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MARIO_FIREBALL, *ATTACK_REGION_NONE);
        }
    }
    frame(weapon.lua_state_agent, 30.0);
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 4.0, 361, 10, 0, 22, 2.0, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MARIO_FIREBALL, *ATTACK_REGION_NONE);
    }
}

#[acmd_script( agent = "mario_pump", script = "effect_start", category = ACMD_EFFECT, low_priority )]
unsafe fn mario_pump_starteff(weapon: &mut L2CAgentBase) {
    if macros::is_excute(weapon) {
        bonker_vis(weapon.module_accessor);
        LinkModule::set_model_constraint_pos_ort(weapon.module_accessor, *LINK_NO_ARTICLE, Hash40::new("have"), Hash40::new("haver"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32, false);
    }
}

#[acmd_script( agent = "mario_pump", script = "effect_light", category = ACMD_EFFECT, low_priority )]
unsafe fn mario_pump_lighteff(weapon: &mut L2CAgentBase) {
    if macros::is_excute(weapon) {
        bonker_vis(weapon.module_accessor);
        LinkModule::set_model_constraint_pos_ort(weapon.module_accessor, *LINK_NO_ARTICLE, Hash40::new("have"), Hash40::new("havel"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32, false);
    }
}

pub fn install() {
    install_agent_frames!(
        mario_frame
    );
    install_status_scripts!(
        mario_specials_init,
        mario_specials_exec,
        mario_speciallw_main,
        mario_speciallw_shoot_init,
        mario_speciallw_shoot_exec,
        mario_speciallw_shoot_main,
        mario_speciallw_charge_main
    );
    install_acmd_scripts!(
        mario_fsmash,
        mario_fsmasheff,
        mario_fsmashsnd,
        mario_fsmashhi,
        mario_fsmashhieff,
        mario_fsmashhisnd,
        mario_fsmashlw,
        mario_fsmashlweff,
        mario_fsmashlwsnd,
        mario_nair,
        mario_fair,
        mario_faireff,
        mario_fairexp,
        mario_fairland,
        mario_dair,
        mario_daireff,
        mario_dairsnd,
        mario_dairlanding,
        mario_nspecial,
        mario_sspecial,
        mario_sspecialeff,
        mario_sspecialaireff,
        mario_uspecial,
        mario_dspecialstartsnd,
        mario_dspecialjump,
        mario_dspecialjumpsnd,
        mario_dspeciallandsnd,
        mario_dspeciallandeff,
        mario_dspecialairstart,
        mario_dspecialairstartsnd,
        mario_dspecialpound,
        mario_dspecialpoundsnd,
        mario_dspecialpoundexp,
        mario_dspecialcancel,
        mario_dspecialcancelsnd,
        mario_dspecialpoundland,
        mario_dspecialpoundlandsnd,
        mario_dspecialpoundlandeff,
        mario_dspecialpoundlandexp,
        mario_fireball_regular,
        mario_pump_starteff,
        mario_pump_lighteff
    );
}