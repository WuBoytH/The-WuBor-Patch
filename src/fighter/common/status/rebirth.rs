use crate::imports::status_imports::*;

#[skyline::hook(replace = L2CFighterCommon_sub_rebirth_common_pre)]
unsafe fn sub_rebirth_common_pre(fighter: &mut L2CFighterCommon) {
    CameraModule::reset_all(fighter.module_accessor);
    ControlModule::reset_trigger(fighter.module_accessor);
    ControlModule::clear_command(fighter.module_accessor, false);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), true);
    AreaModule::set_whole(fighter.module_accessor, false);
    VisibilityModule::set_whole(fighter.module_accessor, true);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1e61567377));
    PhysicsModule::set_swing_rebirth(fighter.module_accessor, true);
    MotionModule::change_motion(
        fighter.module_accessor,
        // Hash40::new("wait"), // Old motion
        Hash40::new("down_wait_d"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_OFF), 0);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_rebirth_uniq_check(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_rebirth_uniq_check as *const () as _));
    GroundModule::set_ignore_boss(fighter.module_accessor, true);
}

#[skyline::hook(replace = L2CFighterCommon_status_Rebirth_Main)]
unsafe fn status_rebirth_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_rebirth_common().get_bool() {
        return 1.into();
    }
    rebirth_motion_handler(fighter);
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if !fighter.global_table[IS_STOP].get_bool() {
        fighter.sub_rebirth_uniq_process_exec_fix_pos();
    }
    0.into()
}

unsafe fn rebirth_motion_handler(fighter: &mut L2CFighterCommon) {
    let mot = MotionModule::motion_kind(fighter.module_accessor);
    if mot == hash40("down_wait_d") {
        let elapsed = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_REBIRTH_WORK_INT_MOVE_ELAPSED_FRAME);
        let total = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_REBIRTH_WORK_INT_MOVE_TOTAL_FRAME);
        if elapsed + 15 >= total {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("down_stand_d"),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            // HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_OFF), 0);
        }
    }
    else {
        if appeal_motion_uniq_handler(fighter) {
            let kind = fighter.global_table[KIND].get_i32();
            if MotionModule::is_end(fighter.module_accessor) {
                if [
                    *FIGHTER_KIND_ROY,
                    *FIGHTER_KIND_CHROM
                ].contains(&kind)
                && ![
                    hash40("wait"),
                    hash40("wait_2"),
                    hash40("wait_3"),
                    hash40("wait_4")
                ].contains(&mot) {
                    MotionModule::change_motion(
                        fighter.module_accessor,
                        Hash40::new("wait_4"),
                        0.0,
                        1.0,
                        false,
                        0.0,
                        false,
                        false
                    );
                }
                else if [
                    *FIGHTER_KIND_SAMUSD,
                    *FIGHTER_KIND_ELIGHT
                ].contains(&kind)
                && [
                    hash40("down_stand_d")
                ].contains(&mot) {
                    MotionModule::change_motion(
                        fighter.module_accessor,
                        Hash40::new("wait_4"),
                        0.0,
                        1.0,
                        false,
                        0.0,
                        false,
                        false
                    );
                }
                else {
                    fighter.sub_wait_motion(false.into());
                }
            }
        }
    }
}

unsafe fn appeal_motion_uniq_handler(fighter: &mut L2CFighterCommon) -> bool {
    let mot = MotionModule::motion_kind(fighter.module_accessor);
    let check_basic_taunts = [
        hash40("appeal_hi_l"),
        hash40("appeal_hi_r"),
        hash40("appeal_s_l"),
        hash40("appeal_s_r"),
        hash40("appeal_lw_l"),
        hash40("appeal_lw_r")
    ].contains(&mot);
    let kind = fighter.global_table[KIND].get_i32();
    let hi = ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI);
    let lw = ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW);
    let s = ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L);
    if kind == *FIGHTER_KIND_SNAKE {
        if MotionModule::is_end(fighter.module_accessor) {
            if check_basic_taunts {
                MotionModule::change_motion(
                    fighter.module_accessor,
                    Hash40::new("appeal_wait"),
                    0.0,
                    1.0,
                    false,
                    0.0,
                    false,
                    false
                );
                return false;
            }
        }
        if mot == hash40("appeal_wait") {
            if hi || lw || s {
                MotionModule::change_motion(
                    fighter.module_accessor,
                    Hash40::new("appeal_end"),
                    0.0,
                    1.0,
                    false,
                    0.0,
                    false,
                    false
                );
                return false;
            }
        }
        if mot == hash40("appeal_end") {
            return true;
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_REBIRTH_FLAG_MOVE_END)
    && !check_basic_taunts
    || (
        FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new_raw(mot), true) != 0.0
        && FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new_raw(mot), true) <= MotionModule::frame(fighter.module_accessor)
    ) {
        let lr = PostureModule::lr(fighter.module_accessor);
        let mot = if hi {
            if lr >= 0.0 {
                hash40("appeal_hi_r")
            }
            else {
                hash40("appeal_hi_l")
            }
        }
        else if lw {
            if lr >= 0.0 {
                hash40("appeal_lw_r")
            }
            else {
                hash40("appeal_lw_l")
            }
        }
        else if s {
            if lr >= 0.0 {
                hash40("appeal_s_r")
            }
            else {
                hash40("appeal_s_l")
            }
        }
        else {
            hash40("invalid")
        };
        if mot != hash40("invalid") {
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
            return false;
        }
    }
    true
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_rebirth_common_pre,
            status_rebirth_main
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}