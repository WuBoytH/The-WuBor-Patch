use crate::imports::status_imports::*;

#[status_script(agent = "koopa_breath", status = WEAPON_KOOPA_BREATH_STATUS_KIND_MOVE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn koopa_breath_move_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_param_float(weapon.module_accessor, hash40("param_breath"), hash40("life")) as i32;
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    let speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_breath"), hash40("max_speed"));
    let lr = PostureModule::lr(weapon.module_accessor);
    let speed_mul = WorkModule::get_float(weapon.module_accessor, *WEAPON_KOOPA_BREATH_INSTANCE_WORK_ID_FLOAT_SPEED_MUL);
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed * lr * speed_mul
    );
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    let scale = WorkModule::get_float(weapon.module_accessor, *WEAPON_KOOPA_BREATH_INSTANCE_WORK_ID_FLOAT_SIZE_RATE);
    PostureModule::set_scale(weapon.module_accessor, scale, false);
    AttackModule::set_power_mul_status(weapon.module_accessor, speed_mul);
    AttackModule::set_reaction_mul(weapon.module_accessor, speed_mul);
    if !StopModule::is_stop(weapon.module_accessor) {
        koopa_breah_move_substatus(weapon, false.into());
    }
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(koopa_breah_move_substatus as *const () as _));
    MotionModule::change_motion(
        weapon.module_accessor,
        Hash40::new("move"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    weapon.fastshift(L2CValue::Ptr(koopa_breath_move_fastshift as *const () as _))
}

unsafe extern "C" fn koopa_breah_move_substatus(weapon: &mut L2CWeaponCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    0.into()
}

unsafe extern "C" fn koopa_breath_move_fastshift(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life <= 0 {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    if AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_ALL)
    || GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        let pos = PostureModule::pos(weapon.module_accessor);
        EffectModule::req(
            weapon.module_accessor,
            Hash40::new("sys_bomb_b"),
            pos,
            &ZERO_VECTOR,
            1.0,
            0,
            -1,
            false,
            0
        );
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        koopa_breath_move_main
    );
}