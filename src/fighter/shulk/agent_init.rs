use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::{lua_bind::WorkModule, *},
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    crate::fighter::common::agent_inits::*,
    wubor_utils::table_const::*
};

#[fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_SHULK {
            return;
        }
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_SHULK_MONAD_TYPE_NONE, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_TYPE);
        fighter.global_table[SPECIAL_LW_PRE].assign(&L2CValue::Ptr(speciallw_pre_generic as *const () as _));
    }
}

pub fn install() {
    install_agent_init_callbacks!(
        agent_init
    );
}
