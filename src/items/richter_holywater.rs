use crate::imports::*;
use crate::system::func_links;

pub static mut RICHTER_HOLYWATER_BORN : usize = 0x758e00;
pub static mut RICHTER_HOLYWATER_BORN_LOOP : usize = 0x759600;

#[skyline::hook(replace = RICHTER_HOLYWATER_BORN)]
unsafe extern "C" fn richter_holywater_born_some_status(item: &mut L2CAgent) -> L2CValue {
    // (item.unk20 as L2CValue)[0x1257816e00 as u64].assign(&L2CValue::I32(0));
    item.clear_lua_stack();
    lua_args!(item, ITEM_AREA_KIND_BODY);
    func_links::Item::disable_area(item.lua_state_agent, *ITEM_AREA_KIND_BODY);
    item.clear_lua_stack();
    lua_args!(item, ITEM_AREA_KIND_PICKUP);
    func_links::Item::disable_area(item.lua_state_agent, *ITEM_AREA_KIND_PICKUP);
    HitModule::set_whole(item.module_accessor, HitStatus(*HIT_STATUS_OFF), 0);
    WorkModule::off_flag(item.module_accessor, *ITEM_INSTANCE_WORK_FLAG_AUTO_PLAY_LOST_EFFECT);
    KineticModule::clear_speed_all(item.module_accessor);
    let kind = richter_holywater_something(item).get_i32();
    let gravity_accel = func_links::HOLYWATER::FIRE_PILLAR_GRAVITY_ACCEL(FighterKind(kind));
    // println!("gravity: {}", -gravity_accel);
    item.clear_lua_stack();
    lua_args!(item, -gravity_accel);
    func_links::KineticEnergyGravity::set_accel(item.lua_state_agent, -gravity_accel);
    let gravity_accel_max = func_links::HOLYWATER::FIRE_PILLAR_GRAVITY_ACCEL_MAX(FighterKind(kind));
    // println!("gravity max: {}", gravity_accel_max);
    item.clear_lua_stack();
    lua_args!(item, gravity_accel_max);
    func_links::KineticEnergyGravity::set_limit_speed(item.lua_state_agent, gravity_accel_max);
    item.clear_lua_stack();
    lua_args!(item, 0, 0, 0);
    func_links::KineticEnergyControlRot::set_rotation(item.lua_state_agent, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
    item.clear_lua_stack();
    lua_args!(item, 0, 0, 0);
    func_links::KineticEnergyRot::set_rotation(item.lua_state_agent, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
    if !GroundModule::is_touch(item.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        let speed_y = func_links::HOLYWATER::FIRE_PILLAR_SPEED_Y(FighterKind(kind));
        // println!("speed y: {}", speed_y);
        KineticModule::add_speed(item.module_accessor, &Vector3f{x: 0.0, y: speed_y, z: 0.0});
    }
    // <WuBor>
    if !GroundModule::is_touch(item.module_accessor, (*GROUND_TOUCH_FLAG_LEFT | *GROUND_TOUCH_FLAG_RIGHT) as u32) {
        let normal_x = GroundModule::get_touch_normal_x(item.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
        let normal_y = GroundModule::get_touch_normal_y(item.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
        // println!("normal: {}, {}", normal_x, normal_y);
        let angle = normal_x.atan2(normal_y);
        // println!("angle: {}", angle.to_degrees());
        let speed = 0.4;
        let speed_x = speed * angle.cos();
        let speed_y = speed * angle.sin();
        // println!("speed: {}, {}", speed_x, speed_y);
        let lr = PostureModule::lr(item.module_accessor);
        item.clear_lua_stack();
        lua_args!(item, 0, 0);
        func_links::KineticEnergyControl::set_accel(item.lua_state_agent, &Vector2f{x: 0.0, y: 0.0});
        item.clear_lua_stack();
        lua_args!(item, 0, 0);
        func_links::KineticEnergyControl::set_brake(item.lua_state_agent, &Vector2f{x: 0.0, y: 0.0});
        item.clear_lua_stack();
        lua_args!(item, speed, speed);
        func_links::KineticEnergyControl::set_stable_speed(item.lua_state_agent, &Vector2f{x: speed, y: speed});
        item.clear_lua_stack();
        lua_args!(item, speed, speed);
        func_links::KineticEnergyControl::set_limit_speed(item.lua_state_agent, &Vector2f{x: speed, y: speed});
        item.clear_lua_stack();
        lua_args!(item, speed_x.abs() * lr, -speed_y * lr);
        func_links::KineticEnergyControl::set_speed(item.lua_state_agent, &Vector2f{x: speed_x.abs() * lr, y: -speed_y * lr});
        item.clear_lua_stack();
        func_links::KineticEnergyControl::enable(item.lua_state_agent);
    }
    // </WuBor>
    PostureModule::set_rot(item.module_accessor, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 0);
    0.into()
}

unsafe extern "C" fn richter_holywater_something(_item: &mut L2CAgent) -> L2CValue {
    // Checks which holywater, but I'm lazy...
    0x44.into()
}

#[skyline::hook(replace = RICHTER_HOLYWATER_BORN_LOOP)]
unsafe extern "C" fn richter_holywater_born_loop(item: &mut L2CAgent) -> L2CValue {
    original!()(item);
    item.clear_lua_stack();
    let speed_x= func_links::KineticEnergyControl::get_speed_x(item.lua_state_agent);
    if (GroundModule::is_touch(item.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32) && speed_x.signum() < 0.0)
    || (GroundModule::is_touch(item.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32) && speed_x.signum() > 0.0) {
        item.clear_lua_stack();
        lua_args!(item, 0.0, 0.0);
        func_links::KineticEnergyControl::set_speed(item.lua_state_agent, &Vector2f{x: 0.0, y: 0.0});
    }
    else if GroundModule::is_touch(item.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        let normal_x = GroundModule::get_touch_normal_x(item.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
        let normal_y = GroundModule::get_touch_normal_y(item.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
        let angle = normal_x.atan2(normal_y);
        let speed = 0.4;
        let speed_x = speed * angle.cos();
        let speed_y = speed * angle.sin();
        let lr = PostureModule::lr(item.module_accessor);
        item.clear_lua_stack();
        lua_args!(item, speed_x.abs() * lr, -speed_y * lr);
        func_links::KineticEnergyControl::set_speed(item.lua_state_agent, &Vector2f{x: speed_x.abs() * lr, y: -speed_y * lr});
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "item" {
        unsafe {
            let base = (*info.module.ModuleObject).module_base as usize;
            RICHTER_HOLYWATER_BORN += base;
            RICHTER_HOLYWATER_BORN_LOOP += base;
            skyline::install_hooks!(
                richter_holywater_born_some_status,
                richter_holywater_born_loop
            );
        }
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}