#![allow(unused_must_use)]

use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    wubor_utils::vars::*
};

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_DamageAir_Main)]
unsafe fn status_damageair_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC) {
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
    }
    call_original!(fighter)
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_damageair_main
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}