use smash::{
    lua2cpp::L2CFighterCommon,
    lib::{lua_const::*, L2CValue}
};
use smashline::*;
use crate::{
    daisy::{
        daisy_specials_restrict,
        daisy_speciallw_restrict,
        daisy_itemtoss_special
    }
};

#[fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        // fighter.global_table[0x32].assign(&L2CValue::Bool(false));
        if smash::app::utility::get_kind(&mut *fighter.module_accessor) == *FIGHTER_KIND_DAISY {
            // fighter.global_table[0x26].assign(&L2CValue::Bool(false));
            fighter.global_table[0x28].assign(&L2CValue::Ptr(daisy_itemtoss_special as *const () as _));
            fighter.global_table[0x2d].assign(&L2CValue::Ptr(daisy_itemtoss_special as *const () as _));
            fighter.global_table[0x39].assign(&L2CValue::Ptr(daisy_specials_restrict as *const () as _));
            // fighter.global_table[0x3b].assign(&L2CValue::Ptr(daisy_speciallw_restrict as *const () as _));
        }
        else if smash::app::utility::get_kind(&mut *fighter.module_accessor) == *FIGHTER_KIND_PEACH {
            // fighter.global_table[0x26].assign(&L2CValue::Bool(false));
            fighter.global_table[0x28].assign(&L2CValue::Ptr(daisy_itemtoss_special as *const () as _));
            fighter.global_table[0x2d].assign(&L2CValue::Ptr(daisy_itemtoss_special as *const () as _));
        }
    }
}

pub fn install() {
    install_agent_init_callbacks!(
        agent_init
    );
}