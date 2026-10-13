use super::*;
use wubor_utils::offsets;

#[skyline::from_offset(offsets::fighter::gamewatch::CHANGE_STATUS_CALLBACK)]
unsafe extern "C" fn gamewatch_unk43_inner(vtable: u64, fighter: &mut Fighter);

unsafe extern "C" fn gamewatch_unk43(vtable: u64, fighter: &mut Fighter) {
    gamewatch_unk43_inner(vtable, fighter);

    let module_accessor = fighter.battle_object.module_accessor;
    let status = StatusModule::status_kind(module_accessor);

    if status == vars::fighter::status::GUARD_CANCEL_ATTACK {
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
    replace_fighter_vtable(
        *FIGHTER_KIND_GAMEWATCH,
        FighterVtableSlot::Unk43,
        gamewatch_unk43 as *const () as _
    );
}
