use super::*;

unsafe extern "C" fn guard_cancel_attack_start_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_PIKMIN_PIKMIN_ACTION_COMP,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(0),
        false,
        *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    0.into()
}

unsafe extern "C" fn guard_cancel_attack_start_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    HitModule::set_status_all(weapon.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);

    let action_comp_frame = WorkModule::get_param_int(weapon.module_accessor, hash40("param_pikmin"), hash40("action_comp_frame"));
    WorkModule::set_int(weapon.module_accessor, action_comp_frame, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_ACTION_COMP_FRAME);

    if !StopModule::is_stop(weapon.module_accessor) {
        guard_cancel_attack_start_substatus(weapon, false.into());
    }
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(guard_cancel_attack_start_substatus as *const () as _));

    MotionModule::change_motion(
        weapon.module_accessor,
        Hash40::new("guard_cancel_attack_wait"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    ModelModule::set_render_offset_position(weapon.module_accessor, &Vector3f{x: 0.0, y: 0.0, z: 3.0});

    weapon.fastshift(L2CValue::Ptr(guard_cancel_attack_start_fastshift as *const () as _))
}

unsafe extern "C" fn guard_cancel_attack_start_substatus(weapon: &mut L2CWeaponCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        WorkModule::count_down_int(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_ACTION_COMP_FRAME, -1);
    }
    0.into()
}

unsafe extern "C" fn guard_cancel_attack_start_fastshift(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if MotionModule::is_end(weapon.module_accessor) {
        MotionModule::change_motion(
            weapon.module_accessor,
            Hash40::new("guard_cancel_attack_charge"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        return weapon.fastshift(L2CValue::Ptr(guard_cancel_attack_charge_fastshift as *const () as _));
    }
    0.into()
}

unsafe extern "C" fn guard_cancel_attack_charge_fastshift(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn guard_cancel_attack_start_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    ModelModule::set_render_offset_position(weapon.module_accessor, &Vector3f{x: 0.0, y: 0.0, z: 0.0});

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, vars::pikmin_pikmin::status::GUARD_CANCEL_ATTACK_START, guard_cancel_attack_start_pre);
    agent.status(Main, vars::pikmin_pikmin::status::GUARD_CANCEL_ATTACK_START, guard_cancel_attack_start_main);
    agent.status(End, vars::pikmin_pikmin::status::GUARD_CANCEL_ATTACK_START, guard_cancel_attack_start_end);
}