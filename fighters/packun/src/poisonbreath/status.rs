use super::*;

unsafe extern "C" fn start_init(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    let speed_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_poisonbreath"), hash40("speed_x"));
    let speed_x_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_poisonbreath"), hash40("speed_x_max"));
    let lr = PostureModule::lr(weapon.module_accessor);

    let charge_ratio = WorkModule::get_float(weapon.module_accessor, *WEAPON_PACKUN_POISONBREATH_INSTANCE_WORK_ID_FLOAT_CHARGE_RATIO);

    let speed = speed_x + ((speed_x_max - speed_x) * charge_ratio);

    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_PACKUN_POISONBREATH_KINETIC_ENERGY_ID_NORMAL,
        speed * lr,
        0.0
    );
    sv_kinetic_energy!(
        set_stable_speed,
        weapon,
        WEAPON_PACKUN_POISONBREATH_KINETIC_ENERGY_ID_NORMAL,
        speed_x_max,
        0.0
    );
    sv_kinetic_energy!(
        set_limit_speed,
        weapon,
        WEAPON_PACKUN_POISONBREATH_KINETIC_ENERGY_ID_NORMAL,
        speed_x_max,
        0.0
    );

    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_PACKUN_POISONBREATH_KINETIC_ENERGY_ID_NORMAL);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_PACKUN_POISONBREATH_KINETIC_ENERGY_ID_ENV_WIND);

    0.into()
}

unsafe extern "C" fn start_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    // let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let init_life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    // println!("Life: {}, Max Life: {}", life, init_life);

    VarModule::set_int(weapon.module_accessor, vars::packun_poisonbreath::instance::int::INIT_LIFE, init_life);

    let base_life = 120;
    let max_life = 240;
    let charge_ratio = WorkModule::get_float(weapon.module_accessor, *WEAPON_PACKUN_POISONBREATH_INSTANCE_WORK_ID_FLOAT_CHARGE_RATIO);

    let life = base_life + ((max_life - base_life) as f32 * charge_ratio) as i32;
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);

    let start_scale_mul = WorkModule::get_param_float(weapon.module_accessor, hash40("param_poisonbreath"), hash40("start_scale_mul"));
    let base_scale = PostureModule::base_scale(weapon.module_accessor);
    WorkModule::set_float(weapon.module_accessor, base_scale, *WEAPON_PACKUN_POISONBREATH_STATUS_WORK_FLOAT_START_SCALE);
    PostureModule::set_scale(weapon.module_accessor, start_scale_mul * base_scale, false);

    MotionModule::change_motion(
        weapon.module_accessor,
        Hash40::new("start"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    if !StopModule::is_stop(weapon.module_accessor) {
        start_substatus(weapon, false.into());
    }
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(start_substatus as *const () as _));

    weapon.fastshift(L2CValue::Ptr(start_fastshift as *const () as _))
}

unsafe extern "C" fn start_substatus(weapon: &mut L2CWeaponCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        if WorkModule::count_down_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE, 0) {
            notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        }
    }
    0.into()
}

unsafe extern "C" fn start_fastshift(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn burst_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NONE,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(0),
        false,
        *WEAPON_PACKUN_POISONBREATH_STATUS_WORK_KEEP_FLAG_SHOOT_FLAG,
        *WEAPON_PACKUN_POISONBREATH_STATUS_WORK_KEEP_FLAG_SHOOT_INT,
        *WEAPON_PACKUN_POISONBREATH_STATUS_WORK_KEEP_FLAG_SHOOT_FLOAT,
        0
    );
    0.into()
}

unsafe extern "C" fn burst_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = VarModule::get_int(weapon.module_accessor, vars::packun_poisonbreath::instance::int::INIT_LIFE);

    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);

    MotionModule::change_motion(
        weapon.module_accessor,
        Hash40::new("burst"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    weapon.fastshift(L2CValue::Ptr(burst_fastshift as *const () as _))
}

unsafe extern "C" fn burst_fastshift(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if !StatusModule::is_changing(weapon.module_accessor) {
        let to_normal_scale_frame = WorkModule::get_param_int(weapon.module_accessor, hash40("param_poisonbreath"), hash40("to_normal_scale_frame"));
        let start_scale_mul = WorkModule::get_param_float(weapon.module_accessor, hash40("param_poisonbreath"), hash40("start_scale_mul"));
        let charge_max_scale_mul = WorkModule::get_param_float(weapon.module_accessor, hash40("param_poisonbreath"), hash40("charge_max_scale_mul"));

        let charge_ratio = WorkModule::get_float(weapon.module_accessor, *WEAPON_PACKUN_POISONBREATH_INSTANCE_WORK_ID_FLOAT_CHARGE_RATIO);

        let add_scale = ((((charge_max_scale_mul - 1.0) * charge_ratio) + 1.0) - start_scale_mul) / to_normal_scale_frame as f32;
        // println!("add_scale: {}", add_scale);

        let start_scale = WorkModule::get_float(weapon.module_accessor, *WEAPON_PACKUN_POISONBREATH_STATUS_WORK_FLOAT_START_SCALE);
        let frame = weapon.global_table[STATUS_FRAME].get_f32();
        let new_scale = ((frame * add_scale) + start_scale_mul) * start_scale;
        // println!("new_scale: {}", new_scale);
        PostureModule::set_scale(weapon.module_accessor, new_scale, false);
        WorkModule::set_float(weapon.module_accessor, new_scale, *WEAPON_PACKUN_POISONBREATH_STATUS_WORK_FLOAT_SCALE);

        if to_normal_scale_frame as f32 <= frame {
            weapon.change_status(WEAPON_PACKUN_POISONBREATH_STATUS_KIND_SHOOT.into(), false.into());
        }
    }

    0.into()
}

unsafe extern "C" fn shoot_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NONE,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(0),
        false,
        *WEAPON_PACKUN_POISONBREATH_STATUS_WORK_KEEP_FLAG_SHOOT_FLAG,
        *WEAPON_PACKUN_POISONBREATH_STATUS_WORK_KEEP_FLAG_SHOOT_INT,
        *WEAPON_PACKUN_POISONBREATH_STATUS_WORK_KEEP_FLAG_SHOOT_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK
    );
    0.into()
}

unsafe extern "C" fn shoot_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let speed_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_poisonbreath"), hash40("speed_x"));
    let brake_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_poisonbreath"), hash40("brake_x"));
    sv_kinetic_energy!(
        set_stable_speed,
        weapon,
        WEAPON_PACKUN_POISONBREATH_KINETIC_ENERGY_ID_NORMAL,
        speed_x,
        0.0
    );
    sv_kinetic_energy!(
        set_brake,
        weapon,
        WEAPON_PACKUN_POISONBREATH_KINETIC_ENERGY_ID_NORMAL,
        brake_x,
        0.0
    );

    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_PACKUN_POISONBREATH_KINETIC_ENERGY_ID_NORMAL);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_PACKUN_POISONBREATH_KINETIC_ENERGY_ID_ENV_WIND);

    0.into()
}

unsafe extern "C" fn shoot_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(
        weapon.module_accessor,
        Hash40::new("shoot"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    if !StopModule::is_stop(weapon.module_accessor) {
        shoot_substatus(weapon, false.into());
    }
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(shoot_substatus as *const () as _));

    weapon.fastshift(L2CValue::Ptr(shoot_fastshift as *const () as _))
}

unsafe extern "C" fn shoot_substatus(weapon: &mut L2CWeaponCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        if WorkModule::count_down_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE, 0) {
            notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        }
    }
    0.into()
}

unsafe extern "C" fn shoot_fastshift(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let to_small_scale_start_frame = WorkModule::get_param_int(weapon.module_accessor, hash40("param_poisonbreath"), hash40("to_small_scale_start_frame"));
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life <= to_small_scale_start_frame {
        let frames_left = to_small_scale_start_frame - life;
        let last_scale = WorkModule::get_param_float(weapon.module_accessor, hash40("param_poisonbreath"), hash40("last_scale"));
        let start_scale = WorkModule::get_float(weapon.module_accessor, *WEAPON_PACKUN_POISONBREATH_STATUS_WORK_FLOAT_START_SCALE);
        let final_scale = last_scale * start_scale;
        let scale = WorkModule::get_float(weapon.module_accessor, *WEAPON_PACKUN_POISONBREATH_STATUS_WORK_FLOAT_SCALE);
        let new_scale = scale - (((scale - final_scale) / to_small_scale_start_frame as f32) * frames_left as f32);
        PostureModule::set_scale(weapon.module_accessor, new_scale, false);
    }

    let off_effect_frame = WorkModule::get_param_int(weapon.module_accessor, hash40("param_poisonbreath"), 0x20b120d4df);
    if life <= off_effect_frame {
        if !WorkModule::is_flag(weapon.module_accessor, *WEAPON_PACKUN_POISONBREATH_STATUS_SHOOT_WORK_FLAG_DONE_REMOVE_EFFECT) {
            effect!(
                weapon,
                MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND,
                Hash40::new("packun_poison_gas"),
                false,
                true
            );
            WorkModule::on_flag(weapon.module_accessor, *WEAPON_PACKUN_POISONBREATH_STATUS_SHOOT_WORK_FLAG_DONE_REMOVE_EFFECT)
        }
    }

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *WEAPON_PACKUN_POISONBREATH_STATUS_KIND_START, start_init);
    agent.status(Main, *WEAPON_PACKUN_POISONBREATH_STATUS_KIND_START, start_main);

    agent.status(Pre, vars::packun_poisonbreath::status::BURST, burst_pre);
    agent.status(Main, vars::packun_poisonbreath::status::BURST, burst_main);

    agent.status(Pre, *WEAPON_PACKUN_POISONBREATH_STATUS_KIND_SHOOT, shoot_pre);
    agent.status(Init, *WEAPON_PACKUN_POISONBREATH_STATUS_KIND_SHOOT, shoot_init);
    agent.status(Main, *WEAPON_PACKUN_POISONBREATH_STATUS_KIND_SHOOT, shoot_main);
}