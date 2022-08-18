use {
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        hash40,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    wubor_utils::table_const::*
};

#[skyline::hook(replace = L2CFighterCommon_sub_cliff_uniq_process_exit_Common)]
unsafe fn sub_cliff_uniq_process_exit_common(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_CLIFF) {
        let cliff_no_catch_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("cliff_no_catch_frame"));
        WorkModule::set_int(fighter.module_accessor, cliff_no_catch_frame, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_NO_CATCH_FRAME);
        // HitModule::set_xlu_frame_global(fighter.module_accessor, 0, 0);
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_CLIFF);
    if param_1.get_bool() {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_CLIFF);
        GroundModule::leave_cliff(fighter.module_accessor);
        // HitModule::set_xlu_frame_global(fighter.module_accessor, 0, 0);
    }
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_FALL {
        HitModule::set_xlu_frame_global(fighter.module_accessor, 0, 0);
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_cliff_uniq_process_exit_common
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}