use {
    smash::{
        phx::Hash40,
        app::{lua_bind::*, *}
    },
    // crate::{
    //     common_funcs::*,
    //     vars::*
    // }
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
