#![allow(non_snake_case)]

use crate::imports::status_imports::*;
use super::super::super::param;

#[skyline::hook(replace = L2CFighterCommon_status_pre_Damage)]
unsafe fn status_pre_damage(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_DAMAGE_GROUND,
        *GROUND_CORRECT_KIND_GROUND as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_DAMAGE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_DAMAGE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_DAMAGE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        false,
        false,
        0,
        *FIGHTER_STATUS_ATTR_DAMAGE as u32,
        0,
        0
    );
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_ftStatusUniqProcessDamage_init_common)]
unsafe fn ftstatusuniqprocessdamage_init_common(fighter: &mut L2CFighterCommon) {
    let reaction_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_REACTION_FRAME);
    // println!("reaction frame: {}", reaction_frame);
    fighter.clear_lua_stack();
    lua_args!(fighter, hash40("speed_vec_x"));
    sv_information::damage_log_value(fighter.lua_state_agent);
    let speed_vec_x = fighter.pop_lua_stack(1).get_f32();
    // println!("damage log value speed x probably: {}", speed_vec_x);
    fighter.clear_lua_stack();
    lua_args!(fighter, hash40("speed_vec_y"));
    sv_information::damage_log_value(fighter.lua_state_agent);
    let speed_vec_y = fighter.pop_lua_stack(1).get_f32();
    // println!("damage log value speed y probably: {}", speed_vec_y);
    fighter.clear_lua_stack();
    lua_args!(fighter, hash40("attr"));
    sv_information::damage_log_value(fighter.lua_state_agent);
    let attr = fighter.pop_lua_stack(1).get_u64();
    // println!("damage log value attr: {:#x}", attr);
    let _status = StatusModule::status_kind(fighter.module_accessor);
    // this isn't used in anyhthing???
    if 0 >= reaction_frame as i32 {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_SPEED_UP);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME_LAST);
    }
    else {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION);
        WorkModule::set_float(fighter.module_accessor, reaction_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
        WorkModule::set_float(fighter.module_accessor, reaction_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME_LAST);
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_FLY_AIR);
        }
        else {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_FLY_AIR);
        }
    }
    fighter.clear_lua_stack();
    lua_args!(fighter, hash40("angle"));
    sv_information::damage_log_value(fighter.lua_state_agent);
    let angle = fighter.pop_lua_stack(1).get_f32();
    // println!("damage log value angle: {}", angle);
    let degrees = angle.to_degrees();
    // println!("degrees: {}", degrees);
    let speed_vector = sv_math::vec2_length(speed_vec_x, speed_vec_y);
    // println!("speed vector: {}", speed_vector);
    // fighter.FighterStatusDamage_init_damage_speed_up(speed_vector.into(), degrees.into(), false.into());
    fighterstatusdamage_init_damage_speed_up_by_speed(fighter, speed_vector.into(), degrees.into(), false.into());
    let damage_cliff_no_catch_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("damage_cliff_no_catch_frame"));
    WorkModule::set_int(fighter.module_accessor, damage_cliff_no_catch_frame, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_NO_CATCH_FRAME);
    let cursor_fly_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("cursor_fly_speed"));
    // println!("cursor_fly_speed: {}", cursor_fly_speed);
    let pop1squared = speed_vec_x * speed_vec_x;
    // println!("pop1squared: {}", pop1squared);
    let pop2squared = speed_vec_y * speed_vec_y;
    // println!("pop2squared: {}", pop2squared);
    let combined = pop1squared + pop2squared;
    let cursor_fly_speed_squared = cursor_fly_speed * cursor_fly_speed;
    // println!("cursor_fly_speed_squared: {}", cursor_fly_speed_squared);
    if cursor_fly_speed_squared < combined {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CURSOR);
        let cursor_fly_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("cursor_fly_frame"));
        WorkModule::set_int(fighter.module_accessor, cursor_fly_frame, *FIGHTER_INSTANCE_WORK_ID_INT_CURSOR_FRAME);
    }
    let damage_fly_attack_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("damage_fly_attack_frame"));
    WorkModule::set_int(fighter.module_accessor, damage_fly_attack_frame, *FIGHTER_STATUS_DAMAGE_WORK_INT_ATTACK_DISABLE_FRAME);
    let damage_fly_escape_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("damage_fly_escape_frame"));
    WorkModule::set_int(fighter.module_accessor, damage_fly_escape_frame, *FIGHTER_STATUS_DAMAGE_WORK_INT_ESCAPE_DISABLE_FRAME);
    if [
        hash40("collision_attr_paralyze"),
        hash40("collision_attr_paralyze_ghost")
    ].contains(&attr) {
        let invalid_paralyze_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("invalid_paralyze_frame"));
        WorkModule::set_float(fighter.module_accessor, invalid_paralyze_frame, *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_PARALYZE_FRAME);
    }
}

unsafe fn fighterstatusdamage_init_damage_speed_up_by_speed(
    fighter: &mut L2CFighterCommon,
    factor: L2CValue, // Labeled this way because if shot out of a tornado, the game will pass in your hitstun frames instead of speed.
    angle: L2CValue,
    some_bool: L2CValue
) {
    let factor_min = param::damage::damage_speed_up_speed_min;
    let factor_max = param::damage::damage_speed_up_speed_max;
    let speed_up_mag = WorkModule::get_param_int(fighter.module_accessor, hash40("battle_object"), hash40("damage_fly_speed_up_max_mag")) as f32;
    if !check_damage_speed_up_by_speed(fighter.module_accessor, factor.get_f32())
    && !some_bool.get_bool() {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_SPEED_UP);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_SPEED_UP_MAX_MAG);
        return;
    }
    // println!("speeding up!");
    let speed_diff = factor.get_f32() - factor_min;
    let speed_max = factor_max - factor_min;
    let ratio = (speed_diff / speed_max).clamp(0.0, 1.0);
    let mut mag = fighter.lerp((1.0).into(), speed_up_mag.into(), ratio.into()).get_f32();
    // The logic below is actually vanilla, but this is trying to lineraly interpolate 1.0... to 1.0, so it doesn't *do* anything.
    let angle_base = WorkModule::get_param_float(fighter.module_accessor, hash40("battle_object"), hash40("damage_fly_speed_up_angle_base"));
    let angle_min_max = WorkModule::get_param_float(fighter.module_accessor, hash40("battle_object"), hash40("damage_fly_speed_up_min_max_angle"));
    let angle_min = angle_base - angle_min_max;
    let angle_max = angle_base + angle_min_max;
    if angle_min < angle.get_f32() && angle.get_f32() <= angle_base {
        // println!("Angle Min {} < Angle {} <= Angle Base {}", angle_min, angle.get_f32(), angle_base);
        mag *= init_damage_speed_up_inner(fighter, angle.get_f32(), angle_min, angle_base);
    }
    else if angle_base < angle.get_f32() && angle.get_f32() <= angle_min {
        // println!("Angle Min {} < Angle {} <= Angle Base {}", angle_base, angle.get_f32(), angle_max);
        mag *= init_damage_speed_up_inner(fighter, angle.get_f32(), angle_base, angle_max);
    }
    // println!("Speed Up Magnitude: {}", mag);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_SPEED_UP);
    WorkModule::set_float(fighter.module_accessor, mag, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_SPEED_UP_MAX_MAG);
}

unsafe fn check_damage_speed_up_by_speed(module_accessor: *mut BattleObjectModuleAccessor, speed: f32) -> bool {
    let log = DamageModule::damage_log(module_accessor);
    if log != 0 {
        let log = log as *mut u8;
        !(speed <= param::damage::damage_speed_up_speed_min || *log.add(0x8f) != 0 || *log.add(0x92) != 0
        || *log.add(0x93) != 0 || *log.add(0x98) != 0)
    }
    else {
        false
    }
}

unsafe fn init_damage_speed_up_inner(fighter: &mut L2CFighterCommon, angle: f32, lower: f32, upper: f32) -> f32 {
    let diff = angle - lower;
    let range = upper - lower;
    let ratio = (diff / range).clamp(0.0, 1.0);
    let angle_rate = WorkModule::get_param_float(fighter.module_accessor, hash40("battle_object"), hash40("damage_fly_speed_up_angle_rate"));
    fighter.lerp((1.0).into(), (angle_rate * 0.01).into(), ratio.into()).get_f32()
}

#[skyline::hook(replace = L2CFighterCommon_FighterStatusUniqProcessDamage_leave_stop)]
unsafe fn fighterstatusuniqprocessdamage_leave_stop(fighter: &mut L2CFighterCommon, arg2: L2CValue, arg3: L2CValue) -> L2CValue {
    // <WuBor>
    ShakeModule::stop(fighter.module_accessor);
    // </WuBor>
    original!()(fighter, arg2, arg3)
}

#[skyline::hook(replace = L2CFighterCommon_FighterStatusDamage__requestVectorAdjustEffect)]
unsafe fn fighterstatusdamage__requestvectoradjusteffect(_fighter: &mut L2CFighterCommon, _arg1: L2CValue, _arg2: L2CValue, _arg3: L2CValue, _arg4: L2CValue, _arg5: L2CValue, _arg6: L2CValue,) {
    // nothing lmao
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_damage,
            ftstatusuniqprocessdamage_init_common,
            fighterstatusuniqprocessdamage_leave_stop,
            fighterstatusdamage__requestvectoradjusteffect
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}