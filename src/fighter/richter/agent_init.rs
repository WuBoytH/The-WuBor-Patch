use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::*,
        app::*,
        lib::{lua_const::*, L2CValue}
    },
    custom_cancel::*,
    smashline::*,
    crate::fighter::common::agent_inits::*,
    wubor_utils::table_const::*,
    super::fgc
};

#[fighter_reset]
fn agent_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_RICHTER {
            return;
        }
        fighter.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(specialhi_pre_generic as *const () as _));
        fgc::install();
    }
}

pub fn install() {
    CustomCancelManager::initialize_agent(Hash40::new("fighter_kind_richter"));
    install_agent_resets!(
        agent_reset
    );
}
