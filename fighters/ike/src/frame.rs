use super::*;

extern "C" {
    #[link_name = "common_fighter_frame"]
    pub fn common_fighter_frame(fighter: &mut L2CFighterCommon);
}

unsafe extern "C" fn on_main(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);

    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        if fighter.global_table[STATUS_FRAME].get_f32() >= 30.0 {
            VarModule::on_flag(fighter.module_accessor, vars::ike::status::flag::SPECIAL_LW_WHIFF);
            fighter.change_status(FIGHTER_IKE_STATUS_KIND_SPECIAL_LW_HIT.into(), false.into());
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, on_main);
}