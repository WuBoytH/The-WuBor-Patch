#![allow(unused_must_use)]

use {
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        hash40,
        phx::{Hash40, Vector3f},
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    wubor_utils::{
        wua_bind::*,
        vars::*,
        table_const::*
    }
};

#[skyline::hook(replace = L2CFighterCommon_sub_escape_uniq_process_common_initStatus_common)]
unsafe fn sub_escape_uniq_process_common_initstatus_common(fighter: &mut L2CFighterCommon, param_1: L2CValue, param_2: L2CValue) {
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() != *FIGHTER_STATUS_KIND_ESCAPE_AIR {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        sv_kinetic_energy::clear_speed_ex(fighter.lua_state_agent);
    }
    else {
        let prev_status = fighter.global_table[PREV_STATUS_KIND].get_i32();
        let stick_x = param_1.get_f32();
        let stick_y = param_2.get_f32();
        let length = sv_math::vec2_length(stick_x, stick_y);
        let escape_air_slide_stick = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("escape_air_slide_stick"));
        if escape_air_slide_stick <= length {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE);
        }
        if prev_status != *FIGHTER_STATUS_KIND_DAMAGE_FALL {
            if prev_status == *FIGHTER_STATUS_KIND_TREAD_FALL {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_PREV_STATUS_PASSIVE_GROUND);
            }
            else if [
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
                *FIGHTER_STATUS_KIND_SAVING_DAMAGE_FLY
            ].contains(&prev_status) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_PREV_STATUS_PASSIVE_GROUND);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_PREV_STATUS_PASSIVE_AIR);
                fighter.clear_lua_stack();
                lua_args!(fighter, 0.0);
                sv_kinetic_energy::controller_set_accel_x_mul(fighter.lua_state_agent);
                fighter.clear_lua_stack();
                lua_args!(fighter, 0.0);
                sv_kinetic_energy::controller_set_accel_x_add(fighter.lua_state_agent);
            }
        }
        else {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_PREV_STATUS_PASSIVE_GROUND);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
        }
        else {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
            let escape_air_stiff_start_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_stiff_start_frame"));
            WorkModule::set_float(fighter.module_accessor, escape_air_stiff_start_frame, *FIGHTER_STATUS_ESCAPE_AIR_STIFF_START_FRAME);
            let escape_air_stiff_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_stiff_frame"));
            WorkModule::set_float(fighter.module_accessor, escape_air_stiff_frame, *FIGHTER_STATUS_ESCAPE_AIR_STIFF_FRAME);
            let some_bool = WorkModule::get_param_int(fighter.module_accessor, 0x2ea659cf56, 0) == 1;
            if some_bool {
                let mot = MotionModule::motion_kind(fighter.module_accessor);
                if mot == hash40("jump_aerial_f") || mot == hash40("jump_aerial_b") {
                    let some_float = WorkModule::get_param_float(fighter.module_accessor, 0x271984ee09, 0);
                    let some_float2 = WorkModule::get_param_float(fighter.module_accessor, 0x2bf4bef265, 0);
                    let frame = MotionModule::frame(fighter.module_accessor);
                    let mut result = some_float - (some_float2 * frame);
                    let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
                    if result < -air_speed_y_stable {
                        result = -air_speed_y_stable;
                    }
                    fighter.clear_lua_stack();
                    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, result, 0.0, 0.0, 0.0);
                    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                    fighter.clear_lua_stack();
                    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    sv_kinetic_energy::enable(fighter.lua_state_agent);
                }
            }
        }
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
    }
    let status_kind_interrupt = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    let mut used_escape = 0.0;
    let mut penalty_motion_rate = 1.0;
    let mut hit_xlu_frame = 0.0;
    let mut penalty_hit_xlu_frame = 0.0;
    let mut hit_normal_frame = 0.0;
    let mut penalty_hit_normal_frame = 0.0;
    if status_kind_interrupt == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
        used_escape = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_USED_ESCAPE_AIR);
        penalty_motion_rate = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("escape_air_penalty_motion_rate"));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
            hit_xlu_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_hit_xlu_frame"));
            penalty_hit_xlu_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_penalty_hit_xlu_frame"));
            hit_normal_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_hit_normal_frame"));
            penalty_hit_normal_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_penalty_hit_normal_frame"));
            if MotionModule::is_flag_start_1_frame_from_motion_kind(fighter.module_accessor, Hash40::new("escape_air_slide")) {
                hit_xlu_frame -= 1.0;
                penalty_hit_xlu_frame -= 1.0;
            }
        }
        else {
            hit_xlu_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_hit_xlu_frame"));
            penalty_hit_xlu_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_penalty_hit_xlu_frame"));
            hit_normal_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_hit_normal_frame"));
            penalty_hit_normal_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_penalty_hit_normal_frame"));
            if MotionModule::is_flag_start_1_frame_from_motion_kind(fighter.module_accessor, Hash40::new("escape_air")) {
                hit_xlu_frame -= 1.0;
                penalty_hit_xlu_frame -= 1.0;
            }
        }
    }
    if status_kind_interrupt == *FIGHTER_STATUS_KIND_ESCAPE_B {
        used_escape = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_USED_ESCAPE_B);
        penalty_motion_rate = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("escape_b_penalty_motion_rate"));
        hit_xlu_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_b_hit_xlu_frame"));
        penalty_hit_xlu_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_b_penalty_hit_xlu_frame"));
        hit_normal_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_b_hit_normal_frame"));
        penalty_hit_normal_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_b_penalty_hit_normal_frame"));
        if MotionModule::is_flag_start_1_frame_from_motion_kind(fighter.module_accessor, Hash40::new("escape_b")) {
            hit_xlu_frame -= 1.0;
            penalty_hit_xlu_frame -= 1.0;
        }
    }
    if status_kind_interrupt == *FIGHTER_STATUS_KIND_ESCAPE_F {
        used_escape = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_USED_ESCAPE_F);
        penalty_motion_rate = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("escape_f_penalty_motion_rate"));
        hit_xlu_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_f_hit_xlu_frame"));
        penalty_hit_xlu_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_f_penalty_hit_xlu_frame"));
        hit_normal_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_f_hit_normal_frame"));
        penalty_hit_normal_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_f_penalty_hit_normal_frame"));
        if MotionModule::is_flag_start_1_frame_from_motion_kind(fighter.module_accessor, Hash40::new("escape_f")) {
            hit_xlu_frame -= 1.0;
            penalty_hit_xlu_frame -= 1.0;
        }
    }
    if status_kind_interrupt == *FIGHTER_STATUS_KIND_ESCAPE {
        used_escape = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_USED_ESCAPE);
        penalty_motion_rate = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("escape_penalty_motion_rate"));
        hit_xlu_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_n_hit_xlu_frame"));
        penalty_hit_xlu_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_n_penalty_hit_xlu_frame"));
        hit_normal_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_n_hit_normal_frame"));
        penalty_hit_normal_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_n_penalty_hit_normal_frame"));
        if MotionModule::is_flag_start_1_frame_from_motion_kind(fighter.module_accessor, Hash40::new("escape")) {
            hit_xlu_frame -= 1.0;
            penalty_hit_xlu_frame -= 1.0;
        }
    }
    let escape_penalty_max_count = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("escape_penalty_max_count"));
    let mut part1 = used_escape / escape_penalty_max_count as f32;
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_CANCEL_ESCAPE_TO_ESCAPE_FB) {
        part1 = 1.0;
    }
    if part1 >= 0.0 {
        if 1.0 < part1 {
            part1 = 1.0;
        }
    }
    else {
        part1 = 0.0;
    }
    let final_motion_rate;
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_CANCEL_ESCAPE_TO_ESCAPE_FB) {
        final_motion_rate = 1.0 / (penalty_motion_rate * escape_penalty_max_count as f32 + 1.0);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_CANCEL_ESCAPE_TO_ESCAPE_FB);
    }
    else {
        let part2 = penalty_motion_rate * used_escape;
        let part3 = part2 + 1.0;
        final_motion_rate = 1.0 / part3;
    }
    WorkModule::set_float(fighter.module_accessor, final_motion_rate, *FIGHTER_STATUS_ESCAPE_WORK_FLOAT_MOTION_RATE_PENALTY);
    let xlu_interp = fighter.lerp(hit_xlu_frame.into(), penalty_hit_xlu_frame.into(), part1.into()).get_f32();
    let normal_interp = fighter.lerp(hit_normal_frame.into(), penalty_hit_normal_frame.into(), part1.into()).get_f32();
    let escape_invincible_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("escape_invincible_frame"), 0);
    let interp_invuln = normal_interp * escape_invincible_frame;
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ESCAPE_XLU_START_1F) {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ESCAPE_XLU_START_1F);
    }
    WorkModule::set_int(fighter.module_accessor, xlu_interp as i32, *FIGHTER_STATUS_ESCAPE_WORK_INT_HIT_XLU_FRAME);
    WorkModule::set_int(fighter.module_accessor, interp_invuln as i32, *FIGHTER_STATUS_ESCAPE_WORK_INT_HIT_NORMAL_FRAME);
    if status_kind_interrupt == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
        let add_xlu_start_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_AIR_ESCAPE_ADD_XLU_START_FRAME);
        if 0 < add_xlu_start_frame {
            WorkModule::add_int(fighter.module_accessor, add_xlu_start_frame, *FIGHTER_STATUS_ESCAPE_WORK_INT_HIT_XLU_FRAME);
            WorkModule::set_int(fighter.module_accessor, add_xlu_start_frame, *FIGHTER_STATUS_ESCAPE_AIR_ADD_XLU_START_FRAME);
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_AIR_ESCAPE_ADD_XLU_START_FRAME);
        }
    }
    FighterWorkModuleImpl::calc_escape_air_slide_param(fighter.module_accessor, part1);
}

#[skyline::hook(replace = L2CFighterCommon_status_Escape_Main)]
unsafe fn status_escape_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 0.into();
        }
    }
    else {
        let normal_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_INT_HIT_NORMAL_FRAME);
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
        && normal_frame == 1 {
            let cat = fighter.global_table[CMD_CAT1].get_i32();
            if cat & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_F != 0 {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ESCAPE_XLU_START_1F);
                WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_CANCEL_ESCAPE_TO_ESCAPE_FB);
                fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_F.into(), true.into());
                return 0.into();
            }
            if cat & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_B != 0 {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ESCAPE_XLU_START_1F);
                WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_CANCEL_ESCAPE_TO_ESCAPE_FB);
                fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_B.into(), true.into());
                return 0.into();
            }
        }
        let enable_attack = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_INT_ESCAPE_ATTACK);
        if enable_attack == *FIGHTER_ESCAPE_ATTACK_MODE_ENABLE {
            let is_catch = fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH == 0;
            if is_catch as i32 & 1 != 0 {
                if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                    return 0.into();
                }
            }
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        if MotionModule::is_end(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                return 0.into();
            }
        }
        fighter.sub_escape_check_rumble();
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_setup_escape_air_slide_common)]
pub unsafe fn setup_escape_air_slide_common(fighter: &mut L2CFighterCommon, param_1: L2CValue, param_2: L2CValue) {
    let stickx = param_1.get_f32();
    let sticky = param_2.get_f32();
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
        let normalize = sv_math::vec2_normalize(stickx, sticky);
        let mut dirx = normalize.x;
        let mut diry = normalize.y;
        WorkModule::set_float(fighter.module_accessor, dirx, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_X);
        WorkModule::set_float(fighter.module_accessor, diry, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y);
        let escape_air_slide_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_speed"));
        let slide_speed_x = escape_air_slide_speed * dirx;
        let slide_speed_y = escape_air_slide_speed * diry;
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_FREE, slide_speed_x, slide_speed_y, 0.0, 0.0, 0.0);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, -1.0, -1.0);
        sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
        let boma = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, boma);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, boma);
        let escape_air_slide_stiff_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_stiff_frame"));
        let escape_air_slide_u_stiff_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_u_stiff_frame"));
        let escape_air_slide_d_stiff_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_d_stiff_frame"));
        dirx = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_X);
        diry = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y);
        let arctangent = diry.atan2(dirx.abs());
        let stiff_lerp;
        if 0.0 > arctangent.to_degrees() {
            stiff_lerp = fighter.lerp(escape_air_slide_stiff_frame.into(), escape_air_slide_d_stiff_frame.into(),
                (arctangent.to_degrees() / 90.0).into()
            );
        }
        else {
            stiff_lerp = fighter.lerp(escape_air_slide_stiff_frame.into(), escape_air_slide_u_stiff_frame.into(),
                (arctangent.to_degrees() / 90.0).into()
            );
        }
        let escape_air_slide_stiff_start_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_stiff_start_frame"));
        WorkModule::set_float(fighter.module_accessor, escape_air_slide_stiff_start_frame, *FIGHTER_STATUS_ESCAPE_AIR_STIFF_START_FRAME);
        let escape_air_slide_back_end_frame;
        if WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC)
        && !MiscModule::is_damage_check(boma, true) {
            escape_air_slide_back_end_frame = 0;
        }
        else {
            escape_air_slide_back_end_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_back_end_frame"));
        }
        let add_xlu = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_ADD_XLU_START_FRAME);
        WorkModule::set_int(fighter.module_accessor, escape_air_slide_back_end_frame + add_xlu, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_BACK_END_FRAME);
        WorkModule::set_float(fighter.module_accessor, stiff_lerp.get_f32(), *FIGHTER_STATUS_ESCAPE_AIR_STIFF_FRAME);
        EffectModule::req_on_joint(
            fighter.module_accessor,
            Hash40::new("sys_smash_flash_s"),
            Hash40::new("hip"),
            &Vector3f{x: 0.0, y: 4.0, z: 8.0},
            &ZERO_VECTOR,
            1.1,
            &Vector3f{x: 18.0, y: 12.0, z: 0.0},
            &ZERO_VECTOR,
            false,
            0,
            0,
            0
        );
    }
}

#[skyline::hook(replace = L2CFighterCommon_exec_escape_air_slide)]
pub unsafe fn exec_escape_air_slide(fighter: &mut L2CFighterCommon) {
    let mut slide_step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_STEP);
    let back_end_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_BACK_END_FRAME);
    if slide_step == 0 {
        let slide_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_FRAME);
        if back_end_frame <= slide_frame {
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_FRAME);
            slide_step = 1;
            WorkModule::set_int(fighter.module_accessor, slide_step, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_STEP);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_KINE_FALL) {
        return;
    }
    if slide_step != 0 {
        if slide_step != 1 {
            return;
        }
        let slide_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_FRAME);
        if slide_frame != 0 {
            if 1 <= slide_frame {
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_KEEP_AIR_TURNED_OFF) {
                    StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), false);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_KEEP_AIR_TURNED_OFF);
                }
                if !WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING) {
                    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
                }
            }
        }
        else {
            let dir_x = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_X);
            let dir_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y);
            fighter.clear_lua_stack();
            lua_args!(
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                ENERGY_STOP_RESET_TYPE_ESCAPE_AIR_SLIDE,
                dir_x,
                dir_y,
                0.0,
                0.0,
                0.0
            );
            sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        }
        let something = WorkModule::get_param_float(fighter.module_accessor, 0x15f2c6719b, 0);
        if something <= slide_frame as f32 {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
            let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
            let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
            fighter.clear_lua_stack();
            lua_args!(
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST_NO_CAP,
                speed_x,
                0.0,
                0.0,
                0.0,
                0.0
            );
            sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            sv_kinetic_energy::enable(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
                0.0,
                speed_y,
                0.0,
                0.0,
                0.0
            );
            sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            sv_kinetic_energy::enable(fighter.lua_state_agent);
            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_STOP, fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_KINE_FALL);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_GRAVITY);
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_FRAME);
            WorkModule::set_int(fighter.module_accessor, 2, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_STEP);
        }
    }
    else {
        let slide_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_FRAME);
        let frame = back_end_frame as f32 - 1.0;
        let result = slide_frame as f32 / frame;
        let back_accel = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_BACK_ACCEL);
        let curve = sv_math::bezier_curve(0.5, 0.8, 0.9, 1.0, result);
        let accel = curve - back_accel;
        let escape_air_slide_back_distance;
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC)
        && !MiscModule::is_damage_check(fighter.module_accessor, true) {
            escape_air_slide_back_distance = 0.0;
        }
        else {
            escape_air_slide_back_distance = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_back_distance"));
        }
        let end = accel * -escape_air_slide_back_distance;
        WorkModule::set_float(fighter.module_accessor, curve, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_BACK_ACCEL);
        let dir_x = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_X);
        let dir_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, dir_x * end, dir_y * end);
        sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    }
}

#[skyline::hook(replace = L2CFighterCommon_status_end_EscapeAir)]
unsafe fn status_end_escapeair(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status == *FIGHTER_STATUS_KIND_FALL
    || status == *FIGHTER_STATUS_KIND_LANDING {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
            let landing_frame_escape_air_slide = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("landing_frame_escape_air_slide"));
            let landing_frame_escape_air_slide_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("landing_frame_escape_air_slide_max"));
            let frame = MotionModule::frame(fighter.module_accessor);
            let end_frame = MotionModule::end_frame(fighter.module_accessor);
            let frame_ratio = frame / end_frame;
            let landing_frame = fighter.lerp(landing_frame_escape_air_slide.into(), landing_frame_escape_air_slide_max.into(), frame_ratio.into()).get_f32();
            WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            let escape_air_slide_landing_speed_max = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("escape_air_slide_landing_speed_max"));
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
            let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
            let landing_speed_mul_escape_air_slide = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("landing_speed_mul_escape_air_slide"));
            let mut landing_speed = speed_x * landing_speed_mul_escape_air_slide;
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
            let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
            if escape_air_slide_landing_speed_max < landing_speed.abs() {
                if landing_speed < 0.0 {
                    landing_speed = -escape_air_slide_landing_speed_max;
                }
                else {
                    landing_speed = escape_air_slide_landing_speed_max;
                }
            }
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, landing_speed, speed_y);
            sv_kinetic_energy::set_speed(fighter.lua_state_agent);
        }
        else {
            let landing_frame_escape_air = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("landing_frame_escape_air"));
            WorkModule::set_float(fighter.module_accessor, landing_frame_escape_air, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        }
        if status == *FIGHTER_STATUS_KIND_LANDING {
            if !MotionModule::is_end(fighter.module_accessor) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_TURN);
            }
            // WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_LANDING_CLIFF_STOP);
        }
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_escape_uniq_process_common_initstatus_common,
            status_escape_main,
            setup_escape_air_slide_common,
            exec_escape_air_slide,
            status_end_escapeair
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}