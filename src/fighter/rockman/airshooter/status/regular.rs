use crate::imports::status_imports::*;

unsafe extern "C" fn rockman_airshooter_regular_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_airshooter"), hash40("life"));
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let speed_y = WorkModule::get_param_float(weapon.module_accessor, hash40("param_airshooter"), hash40("speed"));
    let accel_y = WorkModule::get_param_float(weapon.module_accessor, hash40("param_airshooter"), hash40("accel"));
    let limit_speed_y = WorkModule::get_param_float(weapon.module_accessor, hash40("param_airshooter"), hash40("speed"));
    let speed_x;
    let accel_x;
    let limit_speed_x;
    match VarModule::get_int(weapon.module_accessor, rockman_airshooter::instance::int::NUM) {
        1 => {
            speed_x = 0.1;
            accel_x = 0.035;
            limit_speed_x = 0.5;
        },
        2 => {
            speed_x = -0.1;
            accel_x = -0.035;
            limit_speed_x = 0.5;
        },
        _ => {
            speed_x = 0.0;
            accel_x = 0.0;
            limit_speed_x = 0.0;
        }
    }
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed_x,
        speed_y
    );
    sv_kinetic_energy!(
        set_accel,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        accel_x,
        accel_y
    );
    sv_kinetic_energy!(
        set_limit_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        limit_speed_x,
        limit_speed_y
    );
    0.into()
}

unsafe extern "C" fn rockman_airshooter_regular_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if VarModule::is_flag(weapon.module_accessor, rockman_airshooter::status::flag::MOVE) {
        let limit_speed_y = WorkModule::get_param_float(weapon.module_accessor, hash40("param_airshooter"), hash40("limit_speed"));
        sv_kinetic_energy!(
            set_limit_speed,
            weapon,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            0.0,
            limit_speed_y
        );
        VarModule::off_flag(weapon.module_accessor, rockman_airshooter::status::flag::MOVE);
    }
    0.into()
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Init, *WEAPON_ROCKMAN_AIRSHOOTER_STATUS_KIND_REGULAR, rockman_airshooter_regular_init);
    agent.status(smashline::Exec, *WEAPON_ROCKMAN_AIRSHOOTER_STATUS_KIND_REGULAR, rockman_airshooter_regular_exec);
}