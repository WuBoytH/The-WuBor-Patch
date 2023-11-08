use {
    crate::imports::status_imports::*,
    crate::fighter::common::frame::common_fighter_frame
};

unsafe extern "C" fn krool_propeller_early_cancel(fighter: &mut L2CFighterCommon) {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status == *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI
    && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        fighter.change_status(FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_AIR_END.into(), true.into());
    }
}

unsafe extern "C" fn krool_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    krool_propeller_early_cancel(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, krool_frame);
}