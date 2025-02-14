use super::*;

unsafe extern "C" fn fly_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
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

    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_flare1"), hash40("life"));
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_EDGE_FLARE1_INSTANCE_WORK_ID_INT_LIFE);

    let speed_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_flare1"), hash40("speed_x"));
    let speed_mul = WorkModule::get_param_float(weapon.module_accessor, hash40("param_flare1"), hash40("speed_mul"));
    let ratio = WorkModule::get_float(weapon.module_accessor, *WEAPON_EDGE_FLARE1_INSTANCE_WORK_ID_FLOAT_RATIO);

    let lerp = weapon.lerp(1.0_f32.into(), speed_mul.into(), ratio.into()).get_f32();
    let angle = WorkModule::get_float(weapon.module_accessor, *WEAPON_EDGE_FLARE1_INSTANCE_WORK_ID_FLOAT_ANGLE);
    let speed = sv_math::vec2_rot(speed_x * speed_mul, 0.0, angle);
    let lr = PostureModule::lr(weapon.module_accessor);
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed.x * lerp * lr,
        speed.y
    );
    sv_kinetic_energy!(
        set_limit_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed.x * lerp * lr,
        speed.y
    );

    AttackModule::set_base_offset(weapon.module_accessor, &Vector2f{x: -speed.x * lerp, y: 0.0});
    weapon.fastshift(L2CValue::Ptr(fly_fastshift as *const () as _))
}

unsafe extern "C" fn fly_fastshift(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if WorkModule::get_int(weapon.module_accessor, *WEAPON_EDGE_FLARE1_INSTANCE_WORK_ID_INT_LIFE) <= 0 {
        weapon.change_status(WEAPON_EDGE_FLARE1_STATUS_KIND_END.into(), false.into());
    }

    if weapon.sub_ground_module_is_touch_all_consider_speed().get_bool() {
        weapon.change_status(WEAPON_EDGE_FLARE1_STATUS_KIND_END.into(), false.into());
    }

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_EDGE_FLARE1_STATUS_KIND_FLY, fly_main);
}