use {
    smash::{
        lua2cpp::*,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    wubor_utils::table_const::*
};

#[inline(always)]
pub unsafe fn bonker_vis(module_accessor: *mut BattleObjectModuleAccessor) {
    // if IS_BONKER[entry_id(module_accessor)] == 4 {
    //     ModelModule::set_mesh_visibility(module_accessor, Hash40::new("hammer"), false);
    //     ModelModule::set_mesh_visibility(module_accessor, Hash40::new("bonker"), true);
    // }
    // else {
        ModelModule::set_mesh_visibility(module_accessor, Hash40::new("hammer"), true);
        ModelModule::set_mesh_visibility(module_accessor, Hash40::new("bonker"), false);
    // }
}

pub unsafe extern "C" fn mario_remove_hammer(fighter: &mut L2CFighterCommon) {
    if ![
        *FIGHTER_STATUS_KIND_ATTACK_S4_START,
        *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD,
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR
    ].contains(&fighter.global_table[STATUS_KIND].get_i32()) {
        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP) {
            ArticleModule::remove(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
}
