use crate::imports::status_imports::*;
use crate::fighter::common::status::escape::escape_air_slide::*;

#[status("lucario", FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE)]
unsafe fn lucario_escape_air_slide_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let landing_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    escape_air_slide_end_inner(fighter);
    if VarModule::is_flag(fighter.battle_object, lucario::instance::flag::FORCE_LANDING_FALL_SPECIAL) {
        WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    }
    0.into()
}

pub fn install() {
    lucario_escape_air_slide_end::install();
}