use {
    smash::{
        lua2cpp::L2CFighterCommon,
        // phx::*,
        app::*,
        lib::lua_const::*
    },
    smashline::*,
    custom_var::*,
    // custom_cancel::*,
    wubor_utils::vars::*,
    // super::fgc
};

// #[event(start)]
// fn agent_reset(fighter: &mut L2CFighterCommon) {
//     unsafe {
//         let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
//         if fighter_kind != *FIGHTER_KIND_SAMUSD {
//             return;
//         }
//         fgc::install();
//     }
// }

#[event("samusd", initialize)]
fn agent_init(fighter: &mut L2CFighterCommon) {
    VarModule::set_int(fighter.battle_object, samusd::instance::int::CSHOT_ID, *BATTLE_OBJECT_ID_INVALID);
}

pub fn install() {
    // let agent = Hash40::new("fighter_kind_samusd");
    // CustomCancelManager::initialize_agent(agent);
    // //agent_reset//::install();
    agent_init::install();
}
