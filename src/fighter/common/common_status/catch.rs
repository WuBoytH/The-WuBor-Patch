#![allow(unused_must_use)]

use {
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    wubor_utils::table_const::*
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
unsafe fn fighterstatuscapture_set_invalid_capture(_fighter: &mut L2CFighterCommon) {
    // Haha there's nothing here now
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