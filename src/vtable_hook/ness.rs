use crate::imports::*;

#[skyline::hook(offset = 0xdefdf0)]
unsafe extern "C" fn ness_init(vtable: u64, fighter: &mut Fighter) {
    original!()(vtable, fighter);
    let module_accessor = fighter.battle_object.module_accessor;
    let shield_data = ShieldData::new(
        0.0,
        6.0,
        12.0,
        0.0,
        0.0,
        0.0,
        10.0,
        Hash40::new("top"),
        *COLLISION_SHAPE_TYPE_SPHERE as u8,
        *SHIELD_TYPE_UNDEFINED as u8
    );
    let shield_datas = &mut (ShieldDatas2::new().add(shield_data, 0));
    let resource = &mut ShieldGroupResource2::new(
        shield_datas,
        1,
        1.4,
        1.4,
        150.0,
        0.5,
        false,
        0
    );
    MiscModule::add_reflector_group(module_accessor, resource, 4);
    ReflectorModule::set_hop(module_accessor, true, 42.0, 4);

    let shield_data = ShieldData::new(
        0.0,
        6.0,
        12.0,
        0.0,
        0.0,
        0.0,
        10.0,
        Hash40::new("top"),
        *COLLISION_SHAPE_TYPE_SPHERE as u8,
        *SHIELD_TYPE_UNDEFINED as u8
    );
    let shield_datas = &mut (ShieldDatas2::new().add(shield_data, 0));
    let resource = &mut ShieldGroupResource2::new(
        shield_datas,
        1,
        1.4,
        1.4,
        150.0,
        0.5,
        false,
        0
    );
    MiscModule::add_reflector_group(module_accessor, resource, 5);
    ReflectorModule::set_hop(module_accessor, true, 347.0, 5);
}

pub fn install() {
    skyline::install_hooks!(
        ness_init
    );
}