use crate::imports::status_imports::*;

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_ESCAPE_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe extern "C" fn lucario_escape_air_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let landing_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    fighter.status_end_EscapeAir();
    if VarModule::is_flag(fighter.module_accessor, lucario::instance::flag::FORCE_LANDING_FALL_SPECIAL) {
        WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        lucario_escape_air_end
    );
}