use super::*;

extern "C" {
    #[link_name = "common_fighter_frame"]
    pub fn common_fighter_frame(fighter: &mut L2CFighterCommon);
}

unsafe extern "C" fn piranhacopter_early_cancel(fighter: &mut L2CFighterCommon) {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status == *FIGHTER_STATUS_KIND_SPECIAL_HI
    && VarModule::is_flag(fighter.module_accessor, vars::packun::status::flag::SPECIAL_HI_ENABLE_CANCEL)
    && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        fighter.change_status(FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END.into(), true.into());
    }
}

unsafe extern "C" fn on_main(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    piranhacopter_early_cancel(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, on_main);
}