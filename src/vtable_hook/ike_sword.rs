use {
    smash::app::lua_bind::*,
    wubor_utils::vars,
};

extern "Rust" {
    #[link_name = "ike_sword_set_spawn_pos_internal"]
    fn ike_sword_set_spawn_pos_internal(ctx: &mut skyline::hooks::InlineCtx);

    #[link_name = "ike_sword_set_status_internal"]
    fn ike_sword_set_status_internal(ctx: &mut skyline::hooks::InlineCtx);

    #[link_name = "ike_sword_on_hit_internal"]
    fn ike_sword_on_hit_internal(vtable: u64, weapon: &mut smash::app::Weapon, hit_kind: u32) -> u64;
}

#[skyline::hook(offset = 0xaf9bf0, inline)]
unsafe extern "C" fn ike_sword_set_spawn_pos(ctx: &mut skyline::hooks::InlineCtx) {
    ike_sword_set_spawn_pos_internal(ctx);
}

#[skyline::hook(offset = 0xaf9cc4, inline)]
unsafe extern "C" fn ike_sword_set_status(ctx: &mut skyline::hooks::InlineCtx) {
    ike_sword_set_status_internal(ctx);
}

#[skyline::hook(offset = 0x340ac50)]
unsafe extern "C" fn ike_sword_on_hit(vtable: u64, weapon: &mut smash::app::Weapon, hit_kind: u32) -> u64 {
    ike_sword_on_hit_internal(vtable, weapon, hit_kind)
}

pub unsafe extern "C" fn ike_sword_can_pocket(_vtable: u64, weapon: &mut smash::app::Weapon) -> bool {
    let module_accessor = weapon.battle_object.module_accessor;
    let status = StatusModule::status_kind(module_accessor);
    status == vars::ike_sword::status::BLADE_BEAM
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x51cd2d0).data(ike_sword_can_pocket as u64);

    skyline::install_hooks!(
        ike_sword_set_spawn_pos,
        ike_sword_set_status,
        ike_sword_on_hit
    );
}