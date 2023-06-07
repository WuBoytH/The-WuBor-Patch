use crate::imports::status_imports::*;
use smash_rs::app::work_ids;

#[skyline::hook(replace = L2CFighterCommon_sub_escape_uniq_process_initStatus)]
unsafe fn sub_escape_uniq_process_initstatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_escape_uniq_process_common_initStatus();
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_escape_uniq_process_common_initStatus_common)]
unsafe fn sub_escape_uniq_process_common_initstatus_common(fighter: &mut L2CFighterCommon, _param_1: L2CValue, _param_2: L2CValue) {
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() != *FIGHTER_STATUS_KIND_ESCAPE_AIR {
        sv_kinetic_energy!(
            clear_speed_ex,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_DAMAGE
        );
    }
    else {
        let prev_status = fighter.global_table[PREV_STATUS_KIND].get_i32();
        if [
            *FIGHTER_STATUS_KIND_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
            *FIGHTER_STATUS_KIND_SAVING_DAMAGE_FLY
        ].contains(&prev_status) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_PREV_STATUS_PASSIVE_GROUND);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_PREV_STATUS_PASSIVE_AIR);
            sv_kinetic_energy!(
                controller_set_accel_x_mul,
                fighter,
                0.0
            );
            sv_kinetic_energy!(
                controller_set_accel_x_add,
                fighter,
                0.0
            );
        }
        if [
            *FIGHTER_STATUS_KIND_DAMAGE_FALL,
            *FIGHTER_STATUS_KIND_TREAD_FALL
        ].contains(&prev_status) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_PREV_STATUS_PASSIVE_GROUND);
        }
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
                sv_kinetic_energy!(
                    reset_energy,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                    ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
                    0.0,
                    result,
                    0.0,
                    0.0,
                    0.0
                );
                sv_kinetic_energy!(
                    enable,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_GRAVITY
                );
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
        // used_escape = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_USED_ESCAPE_AIR);
        penalty_motion_rate = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("escape_air_penalty_motion_rate"));
        used_escape = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_USED_ESCAPE_AIR); // new
        hit_xlu_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_hit_xlu_frame"));
        penalty_hit_xlu_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_penalty_hit_xlu_frame"));
        hit_normal_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_hit_normal_frame"));
        penalty_hit_normal_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_penalty_hit_normal_frame"));
        if MotionModule::is_flag_start_1_frame_from_motion_kind(fighter.module_accessor, Hash40::new("escape_air")) {
            hit_xlu_frame -= 1.0;
            penalty_hit_xlu_frame -= 1.0;
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
    if VarModule::is_flag(fighter.battle_object, fighter::instance::flag::CANCEL_ESCAPE_TO_ESCAPE_FB) {
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
    if VarModule::is_flag(fighter.battle_object, fighter::instance::flag::CANCEL_ESCAPE_TO_ESCAPE_FB) {
        final_motion_rate = 1.0 / (penalty_motion_rate * escape_penalty_max_count as f32 + 1.0);
        VarModule::off_flag(fighter.battle_object, fighter::instance::flag::CANCEL_ESCAPE_TO_ESCAPE_FB);
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
                VarModule::on_flag(fighter.battle_object, fighter::instance::flag::CANCEL_ESCAPE_TO_ESCAPE_FB);
                fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_F.into(), true.into());
                return 0.into();
            }
            if cat & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_B != 0 {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ESCAPE_XLU_START_1F);
                VarModule::on_flag(fighter.battle_object, fighter::instance::flag::CANCEL_ESCAPE_TO_ESCAPE_FB);
                fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_B.into(), true.into());
                return 0.into();
            }
        }
        let enable_attack = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_INT_ESCAPE_ATTACK);
        if enable_attack == *FIGHTER_ESCAPE_ATTACK_MODE_ENABLE {
            let is_catch = fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH == 0;
            if !is_catch
            && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                return 0.into();
            }
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        if MotionModule::is_end(fighter.module_accessor)
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 0.into();
        }
        fighter.sub_escape_check_rumble();
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

#[skyline::hook(offset = 0x645bc0)]
unsafe fn handle_escape_staling(work: &mut smash_rs::app::WorkModule, status: i32) {
    // This is actually completely vanilla, I just like having it here
    if work.is_flag(work_ids::fighter::instance::DISABLE_ESCAPE_PENALTY) {
        return;
    }
    if 0 < work.get_int(work_ids::fighter::instance::ESCAPE_PENALTY_FRAME) {
        let escape_penalty_max_count = work.get_param_int(smash_rs::phx::Hash40::new("common"), smash_rs::phx::Hash40::new("escape_penalty_max_count")) as f32;
        
        let id_status_pair = [
            (work_ids::fighter::instance::USED_ESCAPE, 0x1f),
            (work_ids::fighter::instance::USED_ESCAPE_F, 0x20),
            (work_ids::fighter::instance::USED_ESCAPE_B, 0x21),
            (work_ids::fighter::instance::USED_ESCAPE_AIR, 0x22),
        ];

        for compare_status in id_status_pair.iter() {
            let add_penalty = if status != compare_status.1 {
                0.5
            }
            else {
                1.0
            };
            let used_escape = work.get_float(compare_status.0);
            let new_escape = if used_escape + add_penalty > escape_penalty_max_count {
                escape_penalty_max_count
            }
            else {
                used_escape + add_penalty
            };
            work.set_float(new_escape, compare_status.0);
        }
    }
    let escape_penalty_frame = work.get_param_int(smash_rs::phx::Hash40::new("common"), smash_rs::phx::Hash40::new("escape_penalty_frame"));
    work.set_int(escape_penalty_frame, work_ids::fighter::instance::ESCAPE_PENALTY_FRAME);
    let escape_penalty_recovry_frame = work.get_param_int(smash_rs::phx::Hash40::new("common"), smash_rs::phx::Hash40::new("escape_penalty_recovry_frame"));
    work.set_int(escape_penalty_recovry_frame, work_ids::fighter::instance::ESCAPE_RECOVERY_FRAME);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_escape_uniq_process_initstatus,
            sub_escape_uniq_process_common_initstatus_common,
            status_escape_main
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
    skyline::install_hooks!(
        handle_escape_staling
    );
}