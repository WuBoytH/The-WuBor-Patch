use {
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    wubor_utils::{vars::*, table_const::*}
};

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
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_STATUS_APPEAL_WORK_FLAG_APPEAL_HOLD)
        || WorkModule::is_flag(fighter.module_accessor, FIGHTER_STATUS_APPEAL_WORK_FLAG_APPEAL_LOOP) {
            // New logic for taunt holds/loops/actions.
            let loop_mot = WorkModule::get_int64(fighter.module_accessor, FIGHTER_STATUS_APPEAL_WORK_INT_APPEAL_LOOP_MOT);
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
            let is_loop = WorkModule::is_flag(fighter.module_accessor, FIGHTER_STATUS_APPEAL_WORK_FLAG_APPEAL_LOOP);
            taunt_holds(fighter, is_loop);
        }
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_STATUS_APPEAL_WORK_FLAG_APPEAL_ENABLE_ACTION) {
            let action_button = WorkModule::get_int(fighter.module_accessor, FIGHTER_STATUS_APPEAL_WORK_INT_APPEAL_ACTION_BUTTON);
            if ControlModule::check_button_on_trriger(fighter.module_accessor, action_button) {
                WorkModule::off_flag(fighter.module_accessor, FIGHTER_STATUS_APPEAL_WORK_FLAG_APPEAL_HOLD);
                WorkModule::off_flag(fighter.module_accessor, FIGHTER_STATUS_APPEAL_WORK_FLAG_APPEAL_LOOP);
                let action_mot = WorkModule::get_int64(fighter.module_accessor, FIGHTER_STATUS_APPEAL_WORK_INT_APPEAL_ACTION_MOT);
                MotionModule::change_motion(
                    fighter.module_accessor,
                    Hash40::new_raw(action_mot),
                    0.0,
                    1.0,
                    false,
                    0.0,
                    false,
                    false
                );
                WorkModule::off_flag(fighter.module_accessor, FIGHTER_STATUS_APPEAL_WORK_FLAG_APPEAL_ENABLE_ACTION);
            }
        }
        if [
            WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_APPEAL_WORK_INT_MOTION_KIND_L),
            WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_APPEAL_WORK_INT_MOTION_KIND_R)
        ].contains(&MotionModule::motion_kind(fighter.module_accessor)) {
            // Original logic but now within an else statement, so the taunt actions take priority.
            let motion_frame = fighter.global_table[MOTION_FRAME].get_f32();
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

/// Used specifically for taunts that we've made loop,
/// but we want to break the loop early.
unsafe fn taunt_holds(fighter: &mut L2CFighterCommon, is_loop: bool) {
    let button = WorkModule::get_int(fighter.module_accessor, FIGHTER_STATUS_APPEAL_WORK_INT_APPEAL_HELD_BUTTON);
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
        let restart_frame = WorkModule::get_int(fighter.module_accessor, FIGHTER_STATUS_APPEAL_WORK_INT_APPEAL_RESTART_FRAME) as f32;
        MotionModule::change_motion_force_inherit_frame(
            fighter.module_accessor,
            Hash40::new_raw(mot),
            restart_frame,
            1.0,
            0.0
        );
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_STATUS_APPEAL_WORK_FLAG_APPEAL_HOLD);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_STATUS_APPEAL_WORK_FLAG_APPEAL_LOOP);
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_appeal_common,
            status_appeal_main
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}