use crate::imports::status_imports::*;

pub unsafe fn jack_special_n_landing_handler(fighter: &mut L2CFighterCommon, landing_frame: L2CValue) {
    if !CancelModule::is_enable_cancel(fighter.module_accessor) {
        WorkModule::set_float(fighter.module_accessor, landing_frame.get_f32(), *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_CANCEL);
        fighter.change_status(FIGHTER_JACK_STATUS_KIND_SPECIAL_N_LANDING.into(), false.into());
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
    }
}