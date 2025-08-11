use crate::imports::*;

#[skyline::hook(offset = 0xcc8f20)]
unsafe extern "C" fn mariod_init(_vtable: u64, fighter: &mut Fighter) {
    let module_accessor = fighter.battle_object.module_accessor;
    WorkModule::off_flag(module_accessor, *FIGHTER_MARIOD_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP);
    WorkModule::off_flag(module_accessor, *FIGHTER_MARIOD_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BUOYANCY);
    EffectModule::remove_common(module_accessor, Hash40::new("charge_max"));
}

#[skyline::hook(offset = 0xcc9770)]
unsafe extern "C" fn mariod_on_status_change(_vtable: u64, fighter: &mut Fighter) {
    let module_accessor = fighter.battle_object.module_accessor;
    let prev_status = StatusModule::prev_status_kind(module_accessor, 0);
    if [*FIGHTER_STATUS_KIND_APPEAL, *FIGHTER_STATUS_KIND_DEMO].contains(&prev_status) {
        ArticleModule::remove_exist(module_accessor, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRCAPSULE, ArticleOperationTarget(0));
    }
}

pub fn install() {
    skyline::install_hooks!(
        mariod_init,
        mariod_on_status_change
    );
}