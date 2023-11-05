use crate::imports::status_imports::*;

#[status_script(agent = "toonlink_boomerang", status = WN_LINK_BOOMERANG_STATUS_KIND_FLY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe extern "C" fn toonlink_boomerang_fly_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::off_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_INFLICTION);
    WorkModule::off_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_TO_HOP);
    if !StopModule::is_stop(weapon.module_accessor) {
        toonlink_boomerang_fly_substatus(weapon, false.into());
    }
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(toonlink_boomerang_fly_substatus as *const () as _));
    MotionModule::change_motion(
        weapon.module_accessor,
        Hash40::new("fly"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    weapon.fastshift(L2CValue::Ptr(toonlink_boomerang_fly_main_fastshift as *const () as _))
}

unsafe extern "C" fn toonlink_boomerang_fly_substatus(weapon: &mut L2CWeaponCommon, param_3: L2CValue) -> L2CValue {
    if param_3.get_bool() {
        if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
            notify_event_msc_cmd!(weapon, Hash40::new_raw(0x18b78d41a0));
        }
        toonlink_boomerang_dec_life(weapon);
    }
    0.into()
}

unsafe extern "C" fn toonlink_boomerang_dec_life(weapon: &mut L2CWeaponCommon) {
    WorkModule::dec_int(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_LIFE);
    let life = WorkModule::get_int(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_LIFE);
    if life <= 0 {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
}

unsafe extern "C" fn toonlink_boomerang_fly_main_fastshift(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let sum_speed_length = KineticModule::get_sum_speed_length(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_min = WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), hash40("speed_min"));
    let speed_mul = WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), hash40("speed_mul"));
    if !WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_TO_HOP) {
        if sum_speed_length <= speed_min * speed_mul {
            if WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_APPLY_FLY_SPEED) {
                weapon.change_status(WN_LINK_BOOMERANG_STATUS_KIND_TURN.into(), false.into());
                return 1.into();
            }
        }
    }
    else {
        weapon.change_status(WN_LINK_BOOMERANG_STATUS_KIND_HOP.into(), false.into());
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        toonlink_boomerang_fly_main
    );
}