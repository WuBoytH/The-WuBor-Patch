use super::*;

extern "C" {
    #[link_name = "daisy_itemtoss_pre"]
    pub fn daisy_itemtoss_pre(fighter: &mut L2CFighterCommon) -> L2CValue;
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[CHECK_GROUND_ATTACK_UNIQ].assign(&L2CValue::Ptr(daisy_itemtoss_pre as *const () as _));
    fighter.global_table[CHECK_AIR_ITEM_THROW_UNIQ].assign(&L2CValue::Ptr(daisy_itemtoss_pre as *const () as _));
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
}