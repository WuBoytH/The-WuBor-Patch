#![allow(unused_must_use)]

use {
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        hash40,
        phx::Hash40,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    wubor_utils::{
        vars::*,
        table_const::*
    }
};

#[skyline::hook(replace = L2CFighterCommon_CatchCont)]
unsafe fn catchcont(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[CMD_CAT2].get_i32() & (
        *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI | *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW |
        *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L | *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R
    ) != 0 {
        if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_CATCH_WAIT {
            CatchModule::catch_cut(fighter.module_accessor, false, false);
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH_CUT.into(), true.into());
            return true.into();
        }
    }
    call_original!(fighter)
}

#[skyline::hook(replace = L2CFighterCommon_FighterStatusCapture_set_invalid_capture)]
unsafe fn fighterstatuscapture_set_invalid_capture(fighter: &mut L2CFighterCommon) {
    let invalid_capture_frame;
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FUNNY) {
        invalid_capture_frame = 1;
    }
    else {
        invalid_capture_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("invalid_capture_frame"));
    }
    WorkModule::set_int(fighter.module_accessor, invalid_capture_frame, *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_CAPTURE_FRAME);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CHECK_CATCH);
    EffectModule::req_common(fighter.module_accessor, Hash40::new("invalid_capture"), 0.0);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            catchcont,
            fighterstatuscapture_set_invalid_capture
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}