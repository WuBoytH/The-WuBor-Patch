use crate::imports::status_imports::*;

unsafe fn krool_propeller_early_cancel(fighter: &mut L2CFighterCommon) {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status == *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI
    && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        fighter.change_status(FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_AIR_END.into(), true.into());
    }
}

#[line("krool", main)]
fn krool_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        krool_propeller_early_cancel(fighter);
    }
}

pub fn install() {
    krool_frame::install();
}