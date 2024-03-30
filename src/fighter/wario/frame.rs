use {
    crate::imports::*,
    crate::fighter::common::frame::common_fighter_frame
};

unsafe extern "C" fn wario_training_tools(fighter: &mut L2CFighterCommon) {
    if smashball::is_training_mode() {
        if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
            let count = WorkModule::get_int(fighter.module_accessor, 0x100000bf);
            let amount = if count + 3 > 7 {
                7
            }
            else {
                count + 3
            };
            WorkModule::set_int(fighter.module_accessor, amount, 0x100000bf); // FIGHTER_WARIO_INSTANCE_WORK_ID_INT_GASS_COUNT
        }
    }
}

unsafe extern "C" fn wario_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    wario_training_tools(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, wario_frame);
}