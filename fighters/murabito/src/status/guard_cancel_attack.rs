use super::*;

extern "C" {
    #[link_name = "guard_cancel_attack_main_common"]
    fn guard_cancel_attack_main_common(fighter: &mut L2CFighterCommon);

    #[link_name = "guard_cancel_attack_main_loop_common"]
    fn guard_cancel_attack_main_loop_common(fighter: &mut L2CFighterCommon) -> L2CValue;
}

unsafe extern "C" fn guard_cancel_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    guard_cancel_attack_set_joint_translate(fighter);
    guard_cancel_attack_main_common(fighter);

    fighter.sub_shift_status_main(L2CValue::Ptr(guard_cancel_attack_main_loop as *const () as _))
}

unsafe extern "C" fn guard_cancel_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    guard_cancel_attack_set_joint_translate(fighter);

    guard_cancel_attack_main_loop_common(fighter)
}

unsafe extern "C" fn guard_cancel_attack_set_joint_translate(fighter: &mut L2CFighterCommon) {
    let handl = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};
    let handr = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};
    ModelModule::joint_global_position_with_offset(
        fighter.module_accessor,
        Hash40::new("handl"),
        &Vector3f{x: 1.0, y: 0.0, z: 0.0},
        handl,
        true
    );
    ModelModule::joint_global_position_with_offset(
        fighter.module_accessor,
        Hash40::new("handr"),
        &Vector3f{x: 1.0, y: 0.0, z: 0.0},
        handr,
        true
    );

    ModelModule::set_joint_translate(
        fighter.module_accessor,
        Hash40::new("havel"),
        &Vector3f{x: handl.x, y: handl.y, z: handl.z},
        true,
        false
    );
    ModelModule::set_joint_translate(
        fighter.module_accessor,
        Hash40::new("haver"),
        &Vector3f{x: handr.x, y: handr.y, z: handr.z},
        true,
        false
    );
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, vars::fighter::status::GUARD_CANCEL_ATTACK, guard_cancel_attack_main);
}