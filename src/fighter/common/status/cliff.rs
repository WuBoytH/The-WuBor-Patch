use crate::imports::status_imports::*;

mod cliff_catch;

mod cliff_wait;

pub mod cliff_jump1;
mod cliff_jump2;

#[skyline::hook(replace = L2CFighterCommon_sub_cliff_uniq_process_exit_Common)]
unsafe extern "C" fn sub_cliff_uniq_process_exit_common(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_CLIFF) {
        let cliff_no_catch_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("cliff_no_catch_frame"));
        WorkModule::set_int(fighter.module_accessor, cliff_no_catch_frame, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_NO_CATCH_FRAME);
        if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_FALL {
            HitModule::set_xlu_frame_global(fighter.module_accessor, 0, 0);
        }
        else {
            VarModule::on_flag(fighter.module_accessor, fighter::instance::flag::LEDGE_INTANGIBILITY);
        }
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_CLIFF);
    if param_1.get_bool() {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_CLIFF);
        GroundModule::leave_cliff(fighter.module_accessor);
        if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_FALL {
            HitModule::set_xlu_frame_global(fighter.module_accessor, 0, 0);
        }
        else {
            VarModule::on_flag(fighter.module_accessor, fighter::instance::flag::LEDGE_INTANGIBILITY);
        }
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
    cliff_catch::install();

    cliff_wait::install();

    cliff_jump1::install();
    cliff_jump2::install();
}