use super::*;

extern "C" {
    #[link_name = "escape_air_slide_end"]
    pub fn escape_air_slide_end(fighter: &mut L2CFighterCommon) -> L2CValue;
}

unsafe extern "C" fn lucario_escape_air_slide_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let landing_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    escape_air_slide_end(fighter);
    if VarModule::is_flag(fighter.module_accessor, vars::lucario::instance::flag::FORCE_LANDING_FALL_SPECIAL) {
        WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, lucario_escape_air_slide_end);
}