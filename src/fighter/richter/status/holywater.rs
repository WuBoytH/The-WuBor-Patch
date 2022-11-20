use {
    smash::{
        phx::*,
        app::{lua_bind::*, *},
        lib::{lua_const::*, *}
    },
    smash_script::*
};

extern "C" {
    #[link_name = "\u{1}_ZN3app4item12disable_areaEP9lua_Statei"]
    pub fn item__disable_area(lua_state: u64, area_kind: i32);

    #[link_name = "\u{1}_ZN3app22kinetic_energy_gravity9set_accelEP9lua_Statef"]
    pub fn kinetic_energy_gravity__set_accel(lua_state: u64, accel: f32);

    #[link_name = "\u{1}_ZN3app22kinetic_energy_gravity15set_limit_speedEP9lua_Statef"]
    pub fn kinetic_energy_gravity__set_limit_speed(lua_state: u64, accel: f32);

    #[link_name = "\u{1}_ZN3app22kinetic_energy_control6enableEP9lua_State"]
    pub fn kinetic_energy_control__enable(lua_state: u64);

    #[link_name = "\u{1}_ZN3app22kinetic_energy_control9set_accelEP9lua_StateRKN3phx8Vector2fE"]
    pub fn kinetic_energy_control__set_accel(lua_state: u64, accel: *const Vector2f);

    #[link_name = "\u{1}_ZN3app22kinetic_energy_control9set_brakeEP9lua_StateRKN3phx8Vector2fE"]
    pub fn kinetic_energy_control__set_brake(lua_state: u64, accel: *const Vector2f);

    #[link_name = "\u{1}_ZN3app22kinetic_energy_control16set_stable_speedEP9lua_StateRKN3phx8Vector2fE"]
    pub fn kinetic_energy_control__set_stable_speed(lua_state: u64, accel: *const Vector2f);

    #[link_name = "\u{1}_ZN3app22kinetic_energy_control15set_limit_speedEP9lua_StateRKN3phx8Vector2fE"]
    pub fn kinetic_energy_control__set_limit_speed(lua_state: u64, accel: *const Vector2f);

    #[link_name = "\u{1}_ZN3app22kinetic_energy_control9set_speedEP9lua_StateRKN3phx8Vector2fE"]
    pub fn kinetic_energy_control__set_speed(lua_state: u64, accel: *const Vector2f);

    #[link_name = "\u{1}_ZN3app26kinetic_energy_control_rot12set_rotationEP9lua_StateRKN3phx8Vector3fE"]
    pub fn kinetic_energy_control_rot__set_rotation(lua_state: u64, rotation: *const Vector3f);

    #[link_name = "\u{1}_ZN3app18kinetic_energy_rot12set_rotationEP9lua_StateRKN3phx8Vector3fE"]
    pub fn kinetic_energy_rot__set_rotation(lua_state: u64, rotation: *const Vector3f);

    #[link_name = "\u{1}_ZN3app9holywater35HOLYWATER_FIRE_PILLAR_GRAVITY_ACCELENS_11FighterKindE"]
    pub fn HOLYWATER_FIRE_PILLAR_GRAVITY_ACCEL(kind: FighterKind) -> f32;

    #[link_name = "\u{1}_ZN3app9holywater39HOLYWATER_FIRE_PILLAR_GRAVITY_ACCEL_MAXENS_11FighterKindE"]
    pub fn HOLYWATER_FIRE_PILLAR_GRAVITY_ACCEL_MAX(kind: FighterKind) -> f32;

    #[link_name = "\u{1}_ZN3app9holywater29HOLYWATER_FIRE_PILLAR_SPEED_YENS_11FighterKindE"]
    pub fn HOLYWATER_FIRE_PILLAR_SPEED_Y(kind: FighterKind) -> f32;
}

pub static mut RICHTER_HOLYWATER : usize = 0x758e00;

#[skyline::hook(replace = RICHTER_HOLYWATER)]
unsafe fn richter_holywater_born_some_status(item: &mut L2CAgent) -> L2CValue {
    // (item.unk20 as L2CValue)[0x1257816e00 as u64].assign(&L2CValue::I32(0));
    item.clear_lua_stack();
    lua_args!(item, ITEM_AREA_KIND_BODY);
    item__disable_area(item.lua_state_agent, *ITEM_AREA_KIND_BODY);
    item.clear_lua_stack();
    lua_args!(item, ITEM_AREA_KIND_PICKUP);
    item__disable_area(item.lua_state_agent, *ITEM_AREA_KIND_PICKUP);
    HitModule::set_whole(item.module_accessor, HitStatus(*HIT_STATUS_OFF), 0);
    WorkModule::off_flag(item.module_accessor, *ITEM_INSTANCE_WORK_FLAG_AUTO_PLAY_LOST_EFFECT);
    KineticModule::clear_speed_all(item.module_accessor);
    let kind = richter_holywater_something(item).get_i32();
    let gravity_accel = HOLYWATER_FIRE_PILLAR_GRAVITY_ACCEL(FighterKind(kind));
    // println!("gravity: {}", -gravity_accel);
    item.clear_lua_stack();
    lua_args!(item, -gravity_accel);
    kinetic_energy_gravity__set_accel(item.lua_state_agent, -gravity_accel);
    let gravity_accel_max = HOLYWATER_FIRE_PILLAR_GRAVITY_ACCEL_MAX(FighterKind(kind));
    // println!("gravity max: {}", gravity_accel_max);
    item.clear_lua_stack();
    lua_args!(item, gravity_accel_max);
    kinetic_energy_gravity__set_limit_speed(item.lua_state_agent, gravity_accel_max);
    item.clear_lua_stack();
    lua_args!(item, 0, 0, 0);
    kinetic_energy_control_rot__set_rotation(item.lua_state_agent, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
    item.clear_lua_stack();
    lua_args!(item, 0, 0, 0);
    kinetic_energy_rot__set_rotation(item.lua_state_agent, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
    if !GroundModule::is_touch(item.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        let speed_y = HOLYWATER_FIRE_PILLAR_SPEED_Y(FighterKind(kind));
        // println!("speed y: {}", speed_y);
        KineticModule::add_speed(item.module_accessor, &Vector3f{x: 0.0, y: speed_y, z: 0.0});
    }
    // <WuBor>
    if !GroundModule::is_touch(item.module_accessor, (*GROUND_TOUCH_FLAG_LEFT | *GROUND_TOUCH_FLAG_RIGHT) as u32) {
        let speed_x = 0.4;
        let lr = PostureModule::lr(item.module_accessor);
        item.clear_lua_stack();
        lua_args!(item, 0, 0);
        kinetic_energy_control__set_accel(item.lua_state_agent, &Vector2f{x: 0.0, y: 0.0});
        item.clear_lua_stack();
        lua_args!(item, 0, 0);
        kinetic_energy_control__set_brake(item.lua_state_agent, &Vector2f{x: 0.0, y: 0.0});
        item.clear_lua_stack();
        lua_args!(item, speed_x * lr, 0);
        kinetic_energy_control__set_stable_speed(item.lua_state_agent, &Vector2f{x: speed_x * lr, y: 0.0});
        item.clear_lua_stack();
        lua_args!(item, speed_x * lr, 0);
        kinetic_energy_control__set_limit_speed(item.lua_state_agent, &Vector2f{x: speed_x * lr, y: 0.0});
        item.clear_lua_stack();
        lua_args!(item, speed_x * lr, 0);
        kinetic_energy_control__set_speed(item.lua_state_agent, &Vector2f{x: speed_x * lr, y: 0.0});
        item.clear_lua_stack();
        kinetic_energy_control__enable(item.lua_state_agent);
    }
    // </WuBor>
    PostureModule::set_rot(item.module_accessor, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 0);
    0.into()
}

unsafe extern "C" fn richter_holywater_something(_item: &mut L2CAgent) -> L2CValue {
    // Checks which holywater, but I'm lazy...
    0x44.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "item" {
        unsafe {
            RICHTER_HOLYWATER += (*info.module.ModuleObject).module_base as usize;
            skyline::install_hooks!(
                richter_holywater_born_some_status
            );
        }
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}