use super::*;

extern "C" {
    #[link_name = "common_fighter_frame"]
    pub fn common_fighter_frame(fighter: &mut L2CFighterCommon);
}

unsafe extern "C" fn brave_training_mode_handler(fighter: &mut L2CFighterCommon) {
    if smashball::is_training_mode() {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                let fighta = fighter.global_table[FIGHTER].get_ptr() as *mut Fighter;
                FighterSpecializer_Brave::add_sp(fighta, 50.0);
            }
        }
    }
}

unsafe extern "C" fn on_main(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    brave_training_mode_handler(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, on_main);
}