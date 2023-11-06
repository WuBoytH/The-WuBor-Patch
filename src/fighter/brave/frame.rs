use crate::imports::status_imports::*;

unsafe extern "C" fn brave_frame(fighter: &mut L2CFighterCommon) {
    if smashball::is_training_mode() {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                let fighta = fighter.global_table[FIGHTER].get_ptr() as *mut Fighter;
                FighterSpecializer_Brave::add_sp(fighta, 50.0);
            }
        }
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, brave_frame);
}