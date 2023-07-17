use {
    crate::imports::status_imports::*,
    wubor_utils::cancels::*
};

#[fighter_frame( agent = FIGHTER_KIND_JACK, main )]
fn jack_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)
        && StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_THROW
        && !CatchModule::is_catch(fighter.module_accessor) {
            let special_cancels = [
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW
            ].to_vec();
            special_cancel_common(fighter, SITUATION_KIND_GROUND.into(), special_cancels);
        }
        if smashball::is_training_mode() {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
                    FighterSpecializer_Jack::add_rebel_gauge(fighter.module_accessor, FighterEntryID(entry_id), 25.0);
                }
                if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
                    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
                    FighterSpecializer_Jack::add_rebel_gauge(fighter.module_accessor, FighterEntryID(entry_id), -25.0);
                }
            }
        }
    }
}

pub fn install() {
    install_agent_frames!(
        jack_frame
    );
}