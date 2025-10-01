use super::*;

mod acmd;
mod status;

#[no_mangle]
unsafe extern "Rust" fn ike_sword_set_spawn_pos_internal(ctx: &mut skyline::hooks::InlineCtx) {
    let module_accessor = ctx.registers[22].x() as *mut BattleObjectModuleAccessor;
    let status = StatusModule::status_kind(module_accessor);
    if status == *FIGHTER_IKE_STATUS_KIND_SPECIAL_LW_HIT {
        let lr = PostureModule::lr(module_accessor);
        let posture = PostureModule::pos(module_accessor) as *mut smash::phx::Vector3f;
        let scale = PostureModule::scale(module_accessor);
        (*posture).x += scale * 6.0 * lr;
        (*posture).y += scale * 10.0;
        ctx.registers[0].set_x(posture as u64);
    }
}

#[no_mangle]
unsafe extern "Rust" fn ike_sword_set_status_internal(ctx: &mut skyline::hooks::InlineCtx) {
    let module_accessor = ctx.registers[22].x() as *mut BattleObjectModuleAccessor;
    let status = StatusModule::status_kind(module_accessor);
    if status == *FIGHTER_IKE_STATUS_KIND_SPECIAL_LW_HIT {
        ctx.registers[8].set_w(vars::ike_sword::status::BLADE_BEAM as u32);
    }
}

#[no_mangle]
unsafe extern "Rust" fn ike_sword_on_hit_internal(vtable: u64, weapon: &mut smash::app::Weapon, hit_kind: u32) -> u64 {
    let val = MiscModule::normal_weapon_hit_handler(vtable, weapon, hit_kind);
    let module_accessor = weapon.battle_object.module_accessor;
    let status = StatusModule::status_kind(module_accessor);
    if status != vars::ike_sword::status::BLADE_BEAM {
        *(weapon as *mut smash::app::Weapon as *mut u32).add(0x3bc8 / 0x4) = 0;
    }
    val & 1
}

pub fn install() {
    let agent = &mut Agent::new("ike_sword");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}