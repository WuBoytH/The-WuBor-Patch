use crate::imports::status_imports::*;

#[skyline::hook(replace = L2CFighterCommon_status_pre_Appeal_common)]
unsafe fn status_pre_appeal_common(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_APPEAL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_APPEAL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_APPEAL_FLOAT,
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
        0,
        param_1.get_u32(),
        0
    );
}

#[skyline::hook(replace = L2CFighterCommon_status_Appeal_common_uniq)]
unsafe fn status_appeal_common_uniq(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1a0473b26e));
    let mut appeal_kind = *FIGHTER_APPEAL_KIND_U;
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_APPEAL_RANDOM) {
        let cat2 = fighter.global_table[CMD_CAT2].get_i32();
        if cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0 {
            VarModule::set_int(fighter.module_accessor, appeal::int::HOLD_BUTTON, *CONTROL_PAD_BUTTON_APPEAL_HI);
            WorkModule::set_int64(fighter.module_accessor, hash40("appeal_hi_r") as i64, *FIGHTER_STATUS_APPEAL_WORK_INT_MOTION_KIND_R);
            WorkModule::set_int64(fighter.module_accessor, hash40("appeal_hi_l") as i64, *FIGHTER_STATUS_APPEAL_WORK_INT_MOTION_KIND_L);
            appeal_kind = *FIGHTER_APPEAL_KIND_U;
        }
        else if cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW != 0 {
            VarModule::set_int(fighter.module_accessor, appeal::int::HOLD_BUTTON, *CONTROL_PAD_BUTTON_APPEAL_LW);
            WorkModule::set_int64(fighter.module_accessor, hash40("appeal_lw_r") as i64, *FIGHTER_STATUS_APPEAL_WORK_INT_MOTION_KIND_R);
            WorkModule::set_int64(fighter.module_accessor, hash40("appeal_lw_l") as i64, *FIGHTER_STATUS_APPEAL_WORK_INT_MOTION_KIND_L);
            appeal_kind = *FIGHTER_APPEAL_KIND_D;
        }
        else if cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L != 0 {
            VarModule::set_int(fighter.module_accessor, appeal::int::HOLD_BUTTON, *CONTROL_PAD_BUTTON_APPEAL_S_L);
            WorkModule::set_int64(fighter.module_accessor, hash40("appeal_s_r") as i64, *FIGHTER_STATUS_APPEAL_WORK_INT_MOTION_KIND_R);
            WorkModule::set_int64(fighter.module_accessor, hash40("appeal_s_l") as i64, *FIGHTER_STATUS_APPEAL_WORK_INT_MOTION_KIND_L);
            appeal_kind = *FIGHTER_APPEAL_KIND_L;
        }
        else if cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R != 0 {
            VarModule::set_int(fighter.module_accessor, appeal::int::HOLD_BUTTON, *CONTROL_PAD_BUTTON_APPEAL_S_R);
            WorkModule::set_int64(fighter.module_accessor, hash40("appeal_s_r") as i64, *FIGHTER_STATUS_APPEAL_WORK_INT_MOTION_KIND_R);
            WorkModule::set_int64(fighter.module_accessor, hash40("appeal_s_l") as i64, *FIGHTER_STATUS_APPEAL_WORK_INT_MOTION_KIND_L);
            appeal_kind = *FIGHTER_APPEAL_KIND_R;
        }
    }
    else {
        let rand = sv_math::rand(hash40("fighter"), 3);
        match rand {
            0 => {
                VarModule::set_int(fighter.module_accessor, appeal::int::HOLD_BUTTON, *CONTROL_PAD_BUTTON_APPEAL_HI);
                WorkModule::set_int64(fighter.module_accessor, hash40("appeal_hi_r") as i64, *FIGHTER_STATUS_APPEAL_WORK_INT_MOTION_KIND_R);
                WorkModule::set_int64(fighter.module_accessor, hash40("appeal_hi_l") as i64, *FIGHTER_STATUS_APPEAL_WORK_INT_MOTION_KIND_L);
                appeal_kind = *FIGHTER_APPEAL_KIND_U;
            },
            1 => {
                VarModule::set_int(fighter.module_accessor, appeal::int::HOLD_BUTTON, *CONTROL_PAD_BUTTON_APPEAL_LW);
                WorkModule::set_int64(fighter.module_accessor, hash40("appeal_lw_r") as i64, *FIGHTER_STATUS_APPEAL_WORK_INT_MOTION_KIND_R);
                WorkModule::set_int64(fighter.module_accessor, hash40("appeal_lw_l") as i64, *FIGHTER_STATUS_APPEAL_WORK_INT_MOTION_KIND_L);
                appeal_kind = *FIGHTER_APPEAL_KIND_D;
            },
            _ => {
                WorkModule::set_int64(fighter.module_accessor, hash40("appeal_s_r") as i64, *FIGHTER_STATUS_APPEAL_WORK_INT_MOTION_KIND_R);
                WorkModule::set_int64(fighter.module_accessor, hash40("appeal_s_l") as i64, *FIGHTER_STATUS_APPEAL_WORK_INT_MOTION_KIND_L);
                let lr = sv_math::rand(hash40("fighter"), 2);
                if lr == 0 {
                    VarModule::set_int(fighter.module_accessor, appeal::int::HOLD_BUTTON, *CONTROL_PAD_BUTTON_APPEAL_S_R);
                    appeal_kind = *FIGHTER_APPEAL_KIND_R;
                }
                else {
                    VarModule::set_int(fighter.module_accessor, appeal::int::HOLD_BUTTON, *CONTROL_PAD_BUTTON_APPEAL_S_L);
                    appeal_kind = *FIGHTER_APPEAL_KIND_L;
                }
            }
        }
    }
    if param_1.get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon, L2CValue) -> L2CValue = std::mem::transmute(param_1.get_ptr());
        callable(fighter, appeal_kind.into());
    }
    let motion_r = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_APPEAL_WORK_INT_MOTION_KIND_R);
    let motion_l = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_APPEAL_WORK_INT_MOTION_KIND_L);
    let mot = if !MotionModule::is_anim_resource(fighter.module_accessor, Hash40::new_raw(motion_l)) {
        motion_r
    }
    else {
        if PostureModule::lr(fighter.module_accessor) != -1.0 {
            motion_r
        }
        else {
            motion_l
        }
    };
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new_raw(mot),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f73affc96));
    let fighter_kind = fighter.global_table[KIND].get_i32();
    let log = if fighter_kind == *FIGHTER_KIND_SNAKE {
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_APPEAL_ATTACK |
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
        *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
    }
    else if [*FIGHTER_KIND_LUIGI, *FIGHTER_KIND_GEKKOUGA].contains(&fighter_kind)
    && appeal_kind == *FIGHTER_APPEAL_KIND_D {
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_APPEAL_ATTACK |
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
        *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
    }
    else if [*FIGHTER_KIND_KIRBY, *FIGHTER_KIND_EDGE].contains(&fighter_kind)
    && [*FIGHTER_APPEAL_KIND_L, *FIGHTER_APPEAL_KIND_R].contains(&appeal_kind) {
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_APPEAL_ATTACK |
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
        *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
    }
    else {
        0
    };
    WorkModule::set_int64(fighter.module_accessor, log as i64, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    ControlModule::clear_command(fighter.module_accessor, false);
}

#[skyline::hook(replace = L2CFighterCommon_status_Appeal_Main)]
unsafe fn status_appeal_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    let sit = fighter.global_table[SITUATION_KIND].get_i32();
    if sit != *SITUATION_KIND_AIR {
        taunt_uniq_handler(fighter);
        if [
            WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_APPEAL_WORK_INT_MOTION_KIND_L),
            WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_APPEAL_WORK_INT_MOTION_KIND_R)
        ].contains(&MotionModule::motion_kind(fighter.module_accessor)) {
            // Original logic but now within an else statement, so the taunt actions take priority.
            let motion_frame = fighter.global_table[STATUS_FRAME].get_f32();
            if motion_frame >= 2.0 {
                let attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
                if 0 < attack_kind {
                    FighterStatusModuleImpl::reset_log_action_info(
                        fighter.module_accessor,
                        attack_kind
                    );
                    WorkModule::set_int64(
                        fighter.module_accessor,
                        0,
                        *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND
                    );
                }
            }
            else {
                let boma = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
                if FighterUtil::is_available_smash_appeal(boma)
                && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_SMASH != 0
                && FighterUtil::is_smash_appeal_timing(boma)
                && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_APPEAL_RANDOM) {
                    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x296b2ba1f5), true);
                    fighter.change_status(FIGHTER_STATUS_KIND_SMASH_APPEAL.into(), true.into());
                    return 0.into();
                }
            }
        }
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

// New Taunt Animation Logic
unsafe fn taunt_uniq_handler(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.module_accessor, appeal::flag::ACTION_BUTTON_CHECK) {
        let action_button = VarModule::get_int(fighter.module_accessor, appeal::int::ACTION_BUTTON);
        if ControlModule::check_button_on_trriger(fighter.module_accessor, action_button) {
            let flag = if VarModule::is_flag(fighter.module_accessor, appeal::flag::ACTION_BUTTON_ENABLE_SUCCESS) {
                appeal::flag::ACTION_BUFFER_SUCCESS
            }
            else {
                appeal::flag::ACTION_BUFFER_LOCKED
            };
            VarModule::on_flag(fighter.module_accessor, flag);
            VarModule::off_flag(fighter.module_accessor, appeal::flag::ACTION_BUTTON_CHECK);
        }
    }
    if VarModule::is_flag(fighter.module_accessor, appeal::flag::HOLD)
    || VarModule::is_flag(fighter.module_accessor, appeal::flag::LOOP) {
        // New logic for taunt holds/loops/actions.
        let loop_mot = VarModule::get_int64(fighter.module_accessor, appeal::int64::LOOP_MOT);
        if MotionModule::motion_kind(fighter.module_accessor) != loop_mot {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new_raw(loop_mot),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
        }
        let is_loop = VarModule::is_flag(fighter.module_accessor, appeal::flag::LOOP);
        taunt_holds(fighter, is_loop);
    }
    if VarModule::is_flag(fighter.module_accessor, appeal::flag::ENABLE_ACTION)
    || VarModule::is_flag(fighter.module_accessor, appeal::flag::ENABLE_ACTION_IMM) {
        let action_button = VarModule::get_int(fighter.module_accessor, appeal::int::ACTION_BUTTON);
        if !VarModule::is_flag(fighter.module_accessor, appeal::flag::ACTION_BUFFER_LOCKED)
        && (
            VarModule::is_flag(fighter.module_accessor, appeal::flag::ACTION_BUFFER_SUCCESS)
            || ControlModule::check_button_on_trriger(fighter.module_accessor, action_button)
        ) {
            VarModule::off_flag(fighter.module_accessor, appeal::flag::HOLD);
            VarModule::off_flag(fighter.module_accessor, appeal::flag::LOOP);
            let action_mot = VarModule::get_int64(fighter.module_accessor, appeal::int64::ACTION_MOT);
            let action_frame = VarModule::get_int(fighter.module_accessor, appeal::int::ACTION_FRAME);
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new_raw(action_mot),
                action_frame as f32,
                1.0,
                false,
                action_frame as f32,
                false,
                false
            );
            VarModule::off_flag(fighter.module_accessor, appeal::flag::ACTION_BUFFER_SUCCESS);
            VarModule::off_flag(fighter.module_accessor, appeal::flag::ACTION_BUFFER_LOCKED);
            VarModule::off_flag(fighter.module_accessor, appeal::flag::ACTION_BUTTON_ENABLE_SUCCESS);
            VarModule::off_flag(fighter.module_accessor, appeal::flag::ENABLE_ACTION);
        }
        VarModule::off_flag(fighter.module_accessor, appeal::flag::ENABLE_ACTION_IMM);
    }
}

/// Used specifically for taunts that we've made loop,
/// but we want to break the loop early.
unsafe fn taunt_holds(fighter: &mut L2CFighterCommon, is_loop: bool) {
    let button = VarModule::get_int(fighter.module_accessor, appeal::int::HOLD_BUTTON);
    if ControlModule::check_button_off(fighter.module_accessor, button)
    && (
        !is_loop || MotionModule::frame(fighter.module_accessor) < 1.0
    ) {
        let lr = PostureModule::lr(fighter.module_accessor);
        let mot = if lr < 0.0 {
            WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_APPEAL_WORK_INT_MOTION_KIND_L)
        }
        else {
            WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_APPEAL_WORK_INT_MOTION_KIND_R)
        };
        let restart_frame = VarModule::get_int(fighter.module_accessor, appeal::int::RESTART_FRAME) as f32;
        MotionModule::change_motion_force_inherit_frame(
            fighter.module_accessor,
            Hash40::new_raw(mot),
            restart_frame,
            1.0,
            0.0
        );
        VarModule::off_flag(fighter.module_accessor, appeal::flag::HOLD);
        VarModule::off_flag(fighter.module_accessor, appeal::flag::LOOP);
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_appeal_common,
            status_appeal_common_uniq,
            status_appeal_main
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}