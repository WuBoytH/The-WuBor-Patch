use smash::{
    lua2cpp::L2CFighterCommon,
    lib::{lua_const::*, L2CValue}
};
use smashline::*;
use crate::{
    daisy::daisy_specials_restrict
};

#[fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        if smash::app::utility::get_kind(&mut *fighter.module_accessor) == *FIGHTER_KIND_DAISY {
            fighter.global_table[0x39].assign(&L2CValue::Ptr(daisy_specials_restrict as *const () as _));
        }
    }
}

pub fn install() {
    install_agent_init_callbacks!(
        agent_init
    );
}