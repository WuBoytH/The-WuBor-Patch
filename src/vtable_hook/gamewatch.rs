use crate::imports::*;

#[skyline::hook(offset = 0xa83010)]
unsafe extern "C" fn gamewatch_change_status_callback(vtable: u64, fighter: &mut Fighter) {
    original!()(vtable, fighter);

    let module_accessor = fighter.battle_object.module_accessor;
    let status = StatusModule::status_kind(module_accessor);

    if status == fighter::status::GUARD_CANCEL_ATTACK {
        ArticleModule::remove_exist(module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST));
        VisibilityModule::set_material_anim_priority(module_accessor, Hash40::new("head"), true);
        VisibilityModule::set_material_anim_priority(module_accessor, Hash40::new("helmet"), true);
        VisibilityModule::set_material_anim_priority(module_accessor, Hash40::new("lhand"), true);
        VisibilityModule::set_material_anim_priority(module_accessor, Hash40::new("rhand"), true);
        VisibilityModule::set_material_anim_priority(module_accessor, Hash40::new("hand"), true);
        VisibilityModule::set_material_anim_priority(module_accessor, Hash40::new("body"), true);
    }
}

pub fn install() {
    skyline::install_hooks!(
        gamewatch_change_status_callback
    );
}