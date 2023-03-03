use crate::imports::status_imports::*;

#[status_script(agent = "ridley_breath", status = WEAPON_RIDLEY_BREATH_STATUS_KIND_FLY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn ridley_breath_fly_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let owner_object = MiscModule::get_battle_object_from_id(owner_id);
    if StatusModule::situation_kind((*owner_object).module_accessor) != *SITUATION_KIND_GROUND {
        WorkModule::set_customize_no(weapon.module_accessor, 1, 0);
    }
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_breath"), hash40("life"));
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_breath"), hash40("max_init_speed"));
    let angle = WorkModule::get_param_float(weapon.module_accessor, hash40("param_breath"), hash40("init_degree"));
    let lr = PostureModule::lr(weapon.module_accessor);
    let speed_x = angle.to_radians().cos() * speed * lr;
    let speed_y = angle.to_radians().sin() * speed;
    weapon.clear_lua_stack();
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed_x,
        speed_y
    );
    sv_kinetic_energy!(
        set_stable_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        -1.0,
        -1.0
    );
    sv_kinetic_energy!(
        set_accel,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        0.0,
        0.0
    );
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
    let scale = WorkModule::get_param_float(weapon.module_accessor, hash40("param_breath"), hash40("max_size_mul"));
    PostureModule::set_scale(weapon.module_accessor, scale, false);
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(ridley_breath_fly_substatus as *const () as _));
    weapon.fastshift(L2CValue::Ptr(ridley_breath_fly_main_fastshift as *const () as _))
}

unsafe extern "C" fn ridley_breath_fly_substatus(weapon: &mut L2CWeaponCommon, param_3: L2CValue) -> L2CValue {
    if param_3.get_bool() {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    0.into()
}

unsafe extern "C" fn ridley_breath_fly_main_fastshift(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life <= 0
    || GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        weapon.change_status(WEAPON_RIDLEY_BREATH_STATUS_KIND_VANISH.into(), false.into());
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        ridley_breath_fly_main
    );
}