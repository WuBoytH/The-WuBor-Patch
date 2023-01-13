use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::*,
        app::*,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    custom_cancel::*,
    wubor_utils::table_const::*,
    crate::fighter::element::fgc::*
};

#[fighter_reset]
fn agent_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_EFLAME {
            return;
        }
        let agent = (*fighter.battle_object).agent_kind_hash;
        element_fgc(agent);
        fighter.global_table[CHECK_AIR_ITEM_THROW_UNIQ].assign(&L2CValue::Bool(false));
        fighter.global_table[FALL_BRAKE_UNIQ].assign(&L2CValue::Bool(false));
    }
}

pub fn install() {
    CustomCancelManager::initialize_agent(Hash40::new("fighter_kind_eflame"));
    install_agent_resets!(
        agent_reset
    );
}
