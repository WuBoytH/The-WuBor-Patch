use crate::imports::*;

#[skyline::hook(offset = 0x34f84f0)]
pub unsafe extern "C" fn belmont_cross_on_hit(_vtable: u64, weapon: &mut smash::app::Weapon, param_3: u64) -> u64 {
    let kind = weapon.battle_object.kind;
    let module_accessor = weapon.battle_object.module_accessor;
    let status = StatusModule::status_kind(module_accessor);
    let some_int = *(weapon as *const smash::app::Weapon as *const u32).add(0x11c / 0x4);
    if 0 < some_int {
        belmont_cross_on_hit_thing(weapon, some_int, 0, 1, -1, 1);
    }

    let check_reflect = if param_3 >> 3 & 1 == 0
    || !ReflectModule::is_reflect(module_accessor) {
        false
    }
    else {
        let team = ReflectModule::team_no(module_accessor) as i32;
        TeamModule::set_team(module_accessor, team, true);
        TeamModule::set_hit_team(module_accessor, team);
        let object_id = ReflectModule::object_id(module_accessor) as u32;
        TeamModule::set_team_owner_id(module_accessor, object_id);
        WorkModule::on_flag(module_accessor, *WEAPON_SIMON_CROSS_INSTANCE_WORK_ID_FLAG_REFLECT);
        true
    };

    if status == *WEAPON_SIMON_CROSS_STATUS_KIND_FLY
    && check_reflect
    | (
        (param_3 & 4 |
        *(weapon as *const smash::app::Weapon as *const u32).add(0x98 / 0x4) as u64 != 0)
        && kind == *WEAPON_KIND_SIMON_CROSS as u32
    ) {
        WorkModule::on_flag(module_accessor, *WEAPON_SIMON_CROSS_INSTANCE_WORK_ID_FLAG_FORCE_TURN);
        StatusModule::change_status_request(module_accessor, *WEAPON_SIMON_CROSS_STATUS_KIND_TURN, false);
    }
    return 1;
}

#[skyline::from_offset(0x33a9e60)]
unsafe extern "C" fn belmont_cross_on_hit_thing(
    weapon: &mut smash::app::Weapon,
    some_int: u32,
    some1: u32,
    some2: u64,
    some3: i32,
    some4: u64
);

pub fn install() {
    skyline::install_hooks!(
        belmont_cross_on_hit
    );
}