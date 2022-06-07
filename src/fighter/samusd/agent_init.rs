use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::*,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    custom_var::*,
    wubor_utils::vars::*,
    super::fgc::*
};

#[fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_SAMUSD {
            return;
        }
        VarModule::set_int(fighter.battle_object, samusd::instance::int::CSHOT_ID, *BATTLE_OBJECT_ID_INVALID);
        fighter.global_table["fgc_func"].assign(&L2CValue::Ptr(samusd_fgc as *const () as _));
    }
}

pub fn install() {
    install_agent_init_callbacks!(
        agent_init
    );
}
