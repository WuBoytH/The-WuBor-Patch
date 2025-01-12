use super::*;

extern "C" {
    #[link_name = "speciallw_pre_generic"]
    fn speciallw_pre_generic(fighter: &mut L2CFighterCommon) -> L2CValue;
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[CHECK_SPECIAL_LW_UNIQ].assign(&L2CValue::Ptr(speciallw_pre_generic as *const () as _));
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
}
