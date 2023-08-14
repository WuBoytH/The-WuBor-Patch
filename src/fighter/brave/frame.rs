use {
    crate::imports::status_imports::*
};

#[line("brave", main)]
fn brave_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if smashball::is_training_mode() {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                    let fighta = fighter.global_table[FIGHTER].get_ptr() as *mut Fighter;
                    FighterSpecializer_Brave::add_sp(fighta, 50.0);
                }
            }
        }
    }
}

pub fn install() {
    brave_frame::install();
}