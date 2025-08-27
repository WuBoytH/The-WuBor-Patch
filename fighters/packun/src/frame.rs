use super::*;

extern "C" {
    #[link_name = "common_fighter_frame"]
    pub fn common_fighter_frame(fighter: &mut L2CFighterCommon);
}

unsafe extern "C" fn piranhacopter_mash(fighter: &mut L2CFighterCommon) {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status == *FIGHTER_STATUS_KIND_SPECIAL_HI
    && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        VarModule::inc_int(fighter.module_accessor, vars::packun::status::int::SPECIAL_HI_MASH_COUNT);
    }
}

unsafe extern "C" fn on_main(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    piranhacopter_mash(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, on_main);
}