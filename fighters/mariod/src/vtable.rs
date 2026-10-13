use super::*;

unsafe extern "C" fn mariod_on_death(_vtable: u64, fighter: &mut Fighter) {
    let module_accessor = fighter.battle_object.module_accessor;
    WorkModule::off_flag(module_accessor, *FIGHTER_MARIOD_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP);
    WorkModule::off_flag(module_accessor, *FIGHTER_MARIOD_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BUOYANCY);
    EffectModule::remove_common(module_accessor, Hash40::new("charge_max"));
}

unsafe extern "C" fn mariod_unk43(_vtable: u64, fighter: &mut Fighter) {
    let module_accessor = fighter.battle_object.module_accessor;
    let prev_status = StatusModule::prev_status_kind(module_accessor, 0);
    if [*FIGHTER_STATUS_KIND_APPEAL, *FIGHTER_STATUS_KIND_DEMO].contains(&prev_status) {
        ArticleModule::remove_exist(module_accessor, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRCAPSULE, ArticleOperationTarget(0));
    }
}

pub fn install() {
    replace_fighter_vtable(
        *FIGHTER_KIND_MARIOD,
        FighterVtableSlot::OnDeath,
        mariod_on_death as *const () as _
    );
    replace_fighter_vtable(
        *FIGHTER_KIND_MARIOD,
        FighterVtableSlot::Unk43,
        mariod_unk43 as *const () as _
    );
}
