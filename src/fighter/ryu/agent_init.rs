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

#[event("ryu", initialize)]
unsafe fn agent_init(fighter: &mut L2CFighterCommon) {
    VarModule::set_float(fighter.battle_object, ryu::instance::float::SEC_SEN_TIMER, -0.6);
    // fighter.global_table[STATUS_END_CONTROL].assign(&false.into());
}

pub fn install() {
    agent_init::install();
}
