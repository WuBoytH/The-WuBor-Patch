use super::*;

extern "C" {
    #[link_name = "element_special_lw_out_main"]
    pub fn element_special_lw_out_main(fighter: &mut L2CFighterCommon) -> L2CValue;
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_LW_OUT, element_special_lw_out_main);
}