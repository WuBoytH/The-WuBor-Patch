use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::*,
        lib::lua_const::*
    },
    smashline::*,
    custom_var::*,
    wubor_utils::vars::*
};

#[fighter_init]
fn on_start(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_SAMUSD {
            return;
        }
        VarModule::set_int(fighter.module_accessor, samusd::instance::int::CSHOT_ID, *BATTLE_OBJECT_ID_INVALID);
    }
}

pub fn install(agent : &mut smashline::Agent) {
    agent.on_start(on_start);
}
