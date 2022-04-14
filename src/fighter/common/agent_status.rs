#![allow(unused_must_use)]

use {
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        app::*,
        lib::lua_const::*
    },
    wubor_utils::{table_const::*, custom_status::*}
};

#[skyline::hook(replace = L2CAgentBase_reserve_status_data_array)]
unsafe fn reserve_status_data_array_hook(agent: &mut L2CFighterCommon, count: i32) {
    let category = utility::get_category(&mut *agent.module_accessor);
    if category != *BATTLE_OBJECT_CATEGORY_FIGHTER {
        call_original!(agent, count);
        return;
    }
    let new_count = CustomStatusModule::init_from_hash(agent.battle_object, count);
    agent.global_table[STATUS_COUNT].assign(&new_count.into());
    call_original!(agent, new_count);

    CustomStatusModule::install_statuses(agent.battle_object, agent);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            reserve_status_data_array_hook
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}