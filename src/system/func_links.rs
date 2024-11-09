#![allow(non_snake_case)]

use smash::{phx::*, app::*, lib::L2CValue};

pub mod HOLYWATER {
    use super::*;
    extern "C" {
        #[link_name = "\u{1}_ZN3app9holywater35HOLYWATER_FIRE_PILLAR_GRAVITY_ACCELENS_11FighterKindE"]
        pub fn FIRE_PILLAR_GRAVITY_ACCEL(kind: FighterKind) -> f32;

        #[link_name = "\u{1}_ZN3app9holywater39HOLYWATER_FIRE_PILLAR_GRAVITY_ACCEL_MAXENS_11FighterKindE"]
        pub fn FIRE_PILLAR_GRAVITY_ACCEL_MAX(kind: FighterKind) -> f32;

        #[link_name = "\u{1}_ZN3app9holywater29HOLYWATER_FIRE_PILLAR_SPEED_YENS_11FighterKindE"]
        pub fn FIRE_PILLAR_SPEED_Y(kind: FighterKind) -> f32;

        #[link_name = "\u{1}_ZN3app9holywater26HOLYWATER_THROW_ANGLE_SIDEENS_11FighterKindE"]
        pub fn THROW_ANGLE_SIDE(kind: FighterKind) -> f32;

        #[link_name = "\u{1}_ZN3app9holywater19HOLYWATER_ROT_SPEEDENS_11FighterKindE"]
        pub fn ROT_SPEED(kind: FighterKind) -> f32;

        #[link_name = "\u{1}_ZN3app9holywater34HOLYWATER_REFLECT_SHIELD_ROT_SPEEDENS_11FighterKindE"]
        pub fn REFLECT_SHIELD_ROT_SPEED(kind: FighterKind) -> f32;
    }
}

pub mod Item {
    extern "C" {
        #[link_name = "\u{1}_ZN3app4item12disable_areaEP9lua_Statei"]
        pub fn disable_area(lua_state: u64, area_kind: i32);

        #[link_name = "\u{1}_ZN3app4item26reset_gravity_energy_brakeEP9lua_State"]
        pub fn reset_gravity_energy_brake(lua_state: u64);
    }
}

pub mod KineticEnergyControl {
    use super::*;
    extern "C" {
        #[link_name = "\u{1}_ZN3app22kinetic_energy_control6enableEP9lua_State"]
        pub fn enable(lua_state: u64);

        #[link_name = "\u{1}_ZN3app22kinetic_energy_control9set_accelEP9lua_StateRKN3phx8Vector2fE"]
        pub fn set_accel(lua_state: u64, accel: *const Vector2f);

        #[link_name = "\u{1}_ZN3app22kinetic_energy_control9set_brakeEP9lua_StateRKN3phx8Vector2fE"]
        pub fn set_brake(lua_state: u64, accel: *const Vector2f);

        #[link_name = "\u{1}_ZN3app22kinetic_energy_control16set_stable_speedEP9lua_StateRKN3phx8Vector2fE"]
        pub fn set_stable_speed(lua_state: u64, accel: *const Vector2f);

        #[link_name = "\u{1}_ZN3app22kinetic_energy_control15set_limit_speedEP9lua_StateRKN3phx8Vector2fE"]
        pub fn set_limit_speed(lua_state: u64, accel: *const Vector2f);

        #[link_name = "\u{1}_ZN3app22kinetic_energy_control9set_speedEP9lua_StateRKN3phx8Vector2fE"]
        pub fn set_speed(lua_state: u64, accel: *const Vector2f);

        #[link_name = "\u{1}_ZN3app22kinetic_energy_control11get_speed_xEP9lua_State"]
        pub fn get_speed_x(lua_state: u64) -> f32;
    }
}

pub mod KineticEnergyControlRot {
    use super::*;
    extern "C" {
        #[link_name = "\u{1}_ZN3app26kinetic_energy_control_rot12set_rotationEP9lua_StateRKN3phx8Vector3fE"]
        pub fn set_rotation(lua_state: u64, rotation: *const Vector3f);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app26kinetic_energy_control_rot6enableEP9lua_State"]
        pub fn enable(lua_state: u64);
    }
}

pub mod KineticEnergyGravity {
    extern "C" {
        #[link_name = "\u{1}_ZN3app22kinetic_energy_gravity9set_accelEP9lua_Statef"]
        pub fn set_accel(lua_state: u64, accel: f32);
    
        #[link_name = "\u{1}_ZN3app22kinetic_energy_gravity15set_limit_speedEP9lua_Statef"]
        pub fn set_limit_speed(lua_state: u64, accel: f32);
    }
}

pub mod KineticEnergyRot {
    use super::*;
    extern "C" {
        #[link_name = "\u{1}_ZN3app18kinetic_energy_rot12set_rotationEP9lua_StateRKN3phx8Vector3fE"]
        pub fn set_rotation(lua_state: u64, rotation: *const Vector3f);
    }
}

pub mod LinkEventThrow {
    use super::*;
    extern "C" {
        #[link_name = "\u{1}_ZN3app14LinkEventThrow13new_l2c_tableEv"]
        pub fn new_l2c_table() -> L2CValue;
    }
}