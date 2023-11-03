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
    // fighter.FighterStatusDamage_init_damage_speed_up(reaction_frame.into(), degrees.into(), false.into());
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
unsafe fn fighterstatusuniqprocessdamage_leave_stop(fighter: &mut L2CFighterCommon, _arg2: L2CValue, arg3: L2CValue) -> L2CValue {
    // <WuBor>
    ShakeModule::stop(fighter.module_accessor);
    // </WuBor>
    if !arg3.get_bool() {
        return 0.into();
    }
    let some = fighter.local_func__fighter_status_damage_2();
    let absolute = some["absolute"].get_bool();
    // fighter.clear_lua_stack();
    // lua_args!(fighter, hash40("attr"));
    // sv_information::damage_log_value(fighter.lua_state_agent);
    // let attr = fighter.pop_lua_stack(1).get_u64();
    if !absolute {
    // || attr == 0x193bdcb0cc { // Kazuya's unique normal hit effect
        fighter.FighterStatusUniqProcessDamage_check_hit_stop_delay(some);
    }

    FighterUtil::cheer_damage(fighter.module_accessor);
    fighter.check_ryu_final_damage_03(true.into());

    let release_action = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_INT_STOP_RELEASE_ACTION);
    if release_action == *FIGHTER_STATUS_DAMAGE_STOP_RELEASE_ACTION_GROUND_TO_AIR {
        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), false);
        let situation = fighter.global_table[SITUATION_KIND].clone();
        fighter.global_table[PREV_SITUATION_KIND].assign(&situation);
        fighter.global_table[SITUATION_KIND].assign(&SITUATION_KIND_AIR.into());
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_FLY_AIR);
    }
    else {
        // There's stuff here but... it goes unused?
    }
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_STOP_RELEASE_ACTION_NONE, *FIGHTER_STATUS_DAMAGE_WORK_INT_STOP_RELEASE_ACTION);

    let mut damage_motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_INT_MOTION_KIND);
    if damage_motion == hash40("damage_fly_roll") {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINISH_CAMERA_TARGET) {
            damage_motion = hash40("damage_fly_n");
        }
    }
    let damage_lr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_DAMAGE_LR);
    if damage_lr != 0.0 {
        let status = StatusModule::status_kind(fighter.module_accessor);
        let lr = PostureModule::lr(fighter.module_accessor);
        if damage_lr * lr >= 0.0 || status == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL || status == *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR {
            PostureModule::set_lr(fighter.module_accessor, damage_lr);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
            WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_DAMAGE_LR);
        }
        else {
            if !(status == *FIGHTER_STATUS_KIND_DAMAGE_FLY
            && (damage_motion == hash40("wall_damage")
            || MotionModule::motion_kind(fighter.module_accessor) == hash40("wall_damage")))
            && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_KNOCKOUT) {
                TurnModule::set_turn(
                    fighter.module_accessor,
                    Hash40::new("back_damage"),
                    lr,
                    false,
                    false,
                    true
                );
                PostureModule::reverse_lr(fighter.module_accessor);
                let back_damage_effective_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("back_damage_effective_frame"));
                WorkModule::set_int(fighter.module_accessor, back_damage_effective_frame, *FIGHTER_INSTANCE_WORK_ID_INT_BACK_DAMAGE_EFFECTIVE_FRAME);
            }
            else {
                WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_DAMAGE_LR);
            }
        }
    }

    let mut start_frame = 0.0;
    if damage_motion != hash40("invalid") {
        if damage_motion == hash40("wall_damage") {
            start_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("wall_damage_start_frame"));
            let start_1_frame = MotionModule::is_flag_start_1_frame_from_motion_kind(fighter.module_accessor, Hash40::new("wall_damage"));
            if start_1_frame {
                start_frame -= 1.0;
            }
        }
        let status = StatusModule::status_kind(fighter.module_accessor);
        if status == *FIGHTER_STATUS_KIND_DAMAGE_FLY {
            if fighter.global_table[DAMAGE_MOTION_KIND_CALLBACK].get_bool() {
                let callable: extern "C" fn(&mut L2CFighterCommon, L2CValue) -> L2CValue = std::mem::transmute(fighter.global_table[DAMAGE_MOTION_KIND_CALLBACK].get_ptr());
                damage_motion = callable(fighter, damage_motion.into()).get_u64();
            }
        }
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new_raw(damage_motion),
            start_frame,
            1.0,
            false,
            0.0,
            false,
            false
        );
        if status == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINISH_CAMERA_TARGET) {
                let angle_compose = fighter.sub_FighterStatusDamage_get_damage_fly_angle_compose();
                let angle = FighterUtil::set_damage_fly_angle(fighter.module_accessor, 0.0, 1.0, 180.0, MotionNodeRotateCompose { _address: angle_compose.get_i32() as u8 });
                WorkModule::set_float(fighter.module_accessor, angle, *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_ROT_ANGLE);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_FLY_ROLL_SET_ANGLE);
            }
            let mut cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new_raw(damage_motion), true);
            if cancel_frame <= 0.0 {
                cancel_frame = MotionModule::end_frame(fighter.module_accessor);
            }
            let speed_up = fighter.reaction_frame_mul_speed_up();
            if 0.0 < speed_up.get_f32() {
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINISH_CAMERA_TARGET) {
                    let frame_sub = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x255c556cd3);
                    let diff = speed_up.get_f32() - frame_sub;
                    let modulo = diff % cancel_frame;
                    if 0.0 < modulo {
                        MotionModule::set_frame(fighter.module_accessor, cancel_frame - modulo, true);
                    }
                }
                else {
                    MotionModule::set_rate(fighter.module_accessor, cancel_frame / speed_up.get_f32());
                }
            }
        }
        else if [
            *FIGHTER_STATUS_KIND_DAMAGE_AIR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR
        ].contains(&status) {
            let pierce = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_TO_PIERCE);
            fighter.set_damage_motion_rate(damage_motion.into(), start_frame.into(), pierce.into());
            let angle_compose = fighter.sub_FighterStatusDamage_get_damage_fly_angle_compose();
            let angle = FighterUtil::set_damage_fly_angle(fighter.module_accessor, 0.0, 1.0, 360.0, MotionNodeRotateCompose { _address: angle_compose.get_i32() as u8 });
            WorkModule::set_float(fighter.module_accessor, angle, *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_ROT_ANGLE);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_FLY_ROLL_SET_ANGLE);
        }
        WorkModule::set_int64(fighter.module_accessor, hash40("invalid") as i64, *FIGHTER_STATUS_DAMAGE_WORK_INT_MOTION_KIND);
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_damage_uniq_process_mainStop)]
unsafe fn sub_damage_uniq_process_mainstop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_ELEC) {
        let stop_frame = FighterStopModuleImpl::get_damage_stop_frame(fighter.module_accessor);
        if stop_frame == 1 {
            fighter.FighterStatusDamage__req_fly_roll_smoke_first();
        }
        fighter.sub_FighterStatusDamage_correctDamageVectorExecStop();
    }
    // else {
    //     fighter.exec_damage_elec_hit_stop();
    // }

    // if StopModule::is_damage(fighter.module_accessor) {
    //     fighter.clear_lua_stack();
    //     lua_args!(fighter, Hash40::new_raw(0x8a6df7656));
    //     sv_information::damage_log_value(fighter.lua_state_agent);
    //     if !fighter.pop_lua_stack(1).get_bool() {
    //         if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_PARALYZE_STOP) {
    //             let something = fighter.local_func__fighter_status_damage_2();
    //             fighter.FighterStatusUniqProcessDamage_check_hit_stop_delay_flick(something);
    //         }
    //     }
    // }

    if [
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR
    ].contains(&status) {
        fighter.sub_ftStatusUniqProcessDamageFlyRoll_execStop();
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_exec_damage_elec_hit_stop)]
unsafe fn exec_damage_elec_hit_stop(fighter: &mut L2CFighterCommon) {
    let mut hit_stop_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_INT_HIT_STOP_FRAME);
    if hit_stop_frame != 0 {
        hit_stop_frame -= 1;
        WorkModule::set_int(fighter.module_accessor, hit_stop_frame, *FIGHTER_STATUS_DAMAGE_WORK_INT_HIT_STOP_FRAME);
    }
    let damage_stop_frame = FighterStopModuleImpl::get_damage_stop_frame(fighter.module_accessor);
    if damage_stop_frame == 1 {
        fighter.FighterStatusDamage__req_fly_roll_smoke_first();
    }
    fighter.sub_FighterStatusDamage_correctDamageVectorExecStop();
    
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_KOZUKATA_DAMAGE) {
        if ControlModule::get_clatter_time(fighter.module_accessor, 0) <= 0.0 {
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_DAMAGE_WORK_INT_HIT_STOP_FRAME);
            ShakeModule::stop(fighter.module_accessor);
        }
    }

    if damage_stop_frame <= 0 {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_ELEC);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_ENABLE_KINE_GRAVITY) {
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new_raw(0x8a6df7656));
        sv_information::damage_log_value(fighter.lua_state_agent);
        if !fighter.pop_lua_stack(1).get_bool() {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_PARALYZE_STOP) {
                let something = fighter.local_func__fighter_status_damage_2();
                fighter.FighterStatusUniqProcessDamage_check_hit_stop_delay(something);
            }
        }
        StatusModule::set_keep_situation_air(fighter.module_accessor, false);
        let release_action = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_INT_STOP_RELEASE_ACTION);
        if release_action == *FIGHTER_STATUS_DAMAGE_STOP_RELEASE_ACTION_GROUND_TO_AIR {
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), false);
            let situation = fighter.global_table[SITUATION_KIND].clone();
            fighter.global_table[PREV_SITUATION_KIND].assign(&situation);
            fighter.global_table[SITUATION_KIND].assign(&SITUATION_KIND_AIR.into());
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_FLY_AIR);
        }
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_STOP_RELEASE_ACTION_NONE, *FIGHTER_STATUS_DAMAGE_WORK_INT_STOP_RELEASE_ACTION);
        fighter.virtual_ftStatusUniqProcessDamage_init(true.into());
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x244371e88f));
        let damage_lr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_DAMAGE_LR);
        let lr = PostureModule::lr(fighter.module_accessor);
        let status = StatusModule::status_kind(fighter.module_accessor);
        if damage_lr * lr >= 0.0 || status == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL || status == *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR {
            if damage_lr * lr > 0.0 {
                PostureModule::set_lr(fighter.module_accessor, damage_lr);
                PostureModule::update_rot_y_lr(fighter.module_accessor);
                WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_DAMAGE_LR);
            }
        }
        else {
            let motion = MotionModule::motion_kind(fighter.module_accessor);
            if status == *FIGHTER_STATUS_KIND_DAMAGE_FLY
            || motion == hash40("wall_damage") {
                WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_DAMAGE_LR);
            }
            else {
                TurnModule::set_turn(
                    fighter.module_accessor,
                    Hash40::new("back_damage"),
                    lr,
                    false,
                    false,
                    true
                );
                PostureModule::reverse_lr(fighter.module_accessor);
                let back_damage_effective_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("back_damage_effective_frame"));
                WorkModule::set_int(fighter.module_accessor, back_damage_effective_frame, *FIGHTER_INSTANCE_WORK_ID_INT_BACK_DAMAGE_EFFECTIVE_FRAME);
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_PARALYZE_EFFECT) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_PARALYZE_EFFECT);
        }
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_PARALYZE_STOP);
    }
    // else {
    //     fighter.clear_lua_stack();
    //     lua_args!(fighter, Hash40::new_raw(0x8a6df7656));
    //     sv_information::damage_log_value(fighter.lua_state_agent);
    //     if !fighter.pop_lua_stack(1).get_bool() {
    //         if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_PARALYZE_STOP) {
    //             let something = fighter.local_func__fighter_status_damage_2();
    //             fighter.FighterStatusUniqProcessDamage_check_hit_stop_delay_flick(something);
    //         }
    //     }
    // }
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
            sub_damage_uniq_process_mainstop,
            exec_damage_elec_hit_stop,
            fighterstatusdamage__requestvectoradjusteffect
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}