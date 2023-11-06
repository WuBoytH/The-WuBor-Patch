use crate::imports::status_imports::*;
use super::super::super::vl;

unsafe extern "C" fn pikachu_dengekidama_regular_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(
        weapon.module_accessor,
        Hash40::new("regular"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    sv_kinetic_energy!(
        set_limit_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        vl::dengekidama::FRAME_MAX,
        0.0
    );
    weapon.fastshift(L2CValue::Ptr(pikachu_dengekidama_regular_main_loop as *const () as _))
}

unsafe extern "C" fn pikachu_dengekidama_regular_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let frame_min = vl::dengekidama::FRAME_MIN;
    let frame_max = vl::dengekidama::FRAME_MAX;
    let frame = weapon.global_table[STATUS_FRAME].get_f32();
    if (frame_min..=frame_max).contains(&frame) {
        let frame_range = frame_max - frame_min;
        let ratio = (frame - frame_min) / frame_range;
        let damage_diff = vl::dengekidama::DAMAGE_MAX - vl::dengekidama::DAMAGE_MIN;
        let damage = (vl::dengekidama::DAMAGE_MIN + damage_diff * ratio).clamp(vl::dengekidama::DAMAGE_MIN, vl::dengekidama::DAMAGE_MAX);
        AttackModule::set_power(weapon.module_accessor, 0, damage, false);
    }
    if VarModule::is_flag(weapon.module_accessor, pikachu_dengekidama::status::flag::SPEED_UP) {
        let lr = PostureModule::lr(weapon.module_accessor);
        sv_kinetic_energy!(
            set_accel,
            weapon,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            vl::dengekidama::ACCEL * lr,
            0.0
        );
    }
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE) <= 0 {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    if !StatusModule::is_changing(weapon.module_accessor)
    && GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        // notify_event_msc_cmd!(weapon, Hash40::new_raw(0x18b78d41a0));
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Main, *WEAPON_PIKACHU_DENGEKIDAMA_STATUS_KIND_REGULAR, pikachu_dengekidama_regular_main);
}