use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::*,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    super::fgc::*
};

#[fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if ![*FIGHTER_KIND_EFLAME, *FIGHTER_KIND_ELIGHT].contains(&fighter_kind) {
            return;
        }
        fighter.global_table["fgc_func"].assign(&L2CValue::Ptr(element_fgc as *const () as _));
    }
}

pub fn install() {
    install_agent_init_callbacks!(
        agent_init
    );
}
