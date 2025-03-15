use super::*;

unsafe extern "C" fn guard_cancel_attack_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_PIKMIN_PIKMIN_ATTACK_S4,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(0),
        false,
        *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    0.into()
}

unsafe extern "C" fn guard_cancel_attack_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    HitModule::set_status_all(weapon.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);

    MotionModule::change_motion(
        weapon.module_accessor,
        Hash40::new("guard_cancel_attack_jump"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    let pos_y = PostureModule::pos_y(weapon.module_accessor);
    WorkModule::set_float(weapon.module_accessor, pos_y, *WEAPON_PIKMIN_PIKMIN_STATUS_ATTACK_S4_WORK_FLOAT_GROUND_Y);
    WorkModule::set_float(weapon.module_accessor, pos_y, *WEAPON_PIKMIN_PIKMIN_STATUS_ATTACK_S4_WORK_FLOAT_AIR_Y);

    WorkModule::set_int(weapon.module_accessor, 2, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_JUMP_COUNT);

    weapon.fastshift(L2CValue::Ptr(guard_cancel_attack_fastshift as *const () as _))
}

unsafe extern "C" fn guard_cancel_attack_fastshift(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if AttackModule::is_attack(weapon.module_accessor, 0, false) {
        ModelModule::set_render_offset_position(weapon.module_accessor, &Vector3f{x: 0.0, y: 0.0, z: 3.0});
    }
    else {
        ModelModule::set_render_offset_position(weapon.module_accessor, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
    }

    if !StopModule::is_stop(weapon.module_accessor) {
        let air_y = WorkModule::get_float(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_ATTACK_S4_WORK_FLOAT_AIR_Y);
        let speed_y = KineticModule::get_sum_speed_y(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

        WorkModule::set_float(weapon.module_accessor, air_y + speed_y, *WEAPON_PIKMIN_PIKMIN_STATUS_ATTACK_S4_WORK_FLOAT_AIR_Y);

        let ground_y = WorkModule::get_float(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_ATTACK_S4_WORK_FLOAT_GROUND_Y);

        let mut offset = air_y + speed_y - ground_y;
        if offset < 0.0 {
            if weapon.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                weapon.change_status(WEAPON_PIKMIN_PIKMIN_STATUS_KIND_ATTACK_S4_LANDING.into(), false.into());
                return 1.into();
            }
            offset = 0.0;
        }
        GroundModule::set_offset_y(weapon.module_accessor, -offset);
    }

    0.into()
}

unsafe extern "C" fn guard_cancel_attack_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    ModelModule::set_render_offset_position(weapon.module_accessor, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
    GroundModule::set_offset_y(weapon.module_accessor, 0.0);

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, vars::pikmin_pikmin::status::GUARD_CANCEL_ATTACK, guard_cancel_attack_pre);
    agent.status(Main, vars::pikmin_pikmin::status::GUARD_CANCEL_ATTACK, guard_cancel_attack_main);
    agent.status(End, vars::pikmin_pikmin::status::GUARD_CANCEL_ATTACK, guard_cancel_attack_end);
}