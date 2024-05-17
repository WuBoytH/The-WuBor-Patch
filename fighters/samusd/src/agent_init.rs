use {
    smash::{
        lua2cpp::L2CFighterCommon,
        lib::lua_const::*
    },
    custom_var::*,
    wubor_utils::vars::*
};

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    VarModule::set_int(fighter.module_accessor, vars::samusd::instance::int::CSHOT_ID, *BATTLE_OBJECT_ID_INVALID);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_start(on_start);
}
