use crate::imports::status_imports::*;

unsafe fn piranhacopter_early_cancel(fighter: &mut L2CFighterCommon) {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status == *FIGHTER_STATUS_KIND_SPECIAL_HI
    && VarModule::is_flag(fighter.battle_object, packun::status::flag::SPECIAL_HI_ENABLE_CANCEL)
    && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        fighter.change_status(FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END.into(), true.into());
    }
}

#[line("packun", main)]
fn packun_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        piranhacopter_early_cancel(fighter);
    }
}

pub fn install() {
    packun_frame::install();
}