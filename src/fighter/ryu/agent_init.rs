use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::*,
        lib::{lua_const::*/*, L2CValue*/}
    },
    smashline::*,
    custom_var::*,
    wubor_utils::vars::*,
};

#[fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_RYU {
            return;
        }
        VarModule::set_float(fighter.module_accessor, ryu::instance::float::SEC_SEN_TIMER, -0.6);
        // fighter.global_table[STATUS_END_CONTROL].assign(&false.into());
    }
}

pub fn install() {
    install_agent_init_callbacks!(
        agent_init
    );
}
