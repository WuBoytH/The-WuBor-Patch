use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::{lua_bind::*, *},
        lib::{lua_const::*/*, L2CValue*/}
    },
    smashline::*,
    // wubor_utils::table_const::*,
    super::vars::*
};

#[fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_RYU {
            return;
        }
        WorkModule::set_float(fighter.module_accessor, -0.6, FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_SEC_SEN_TIMER);
        // fighter.global_table[STATUS_END_CONTROL].assign(&false.into());
    }
}

pub fn install() {
    install_agent_init_callbacks!(
        agent_init
    );
}
