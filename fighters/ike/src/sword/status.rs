use super::*;

unsafe extern "C" fn blade_beam_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
        0,
        0,
        0,
        0
    );
    0.into()
}

unsafe extern "C" fn blade_beam_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::set_int(weapon.module_accessor, 40, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::set_int(weapon.module_accessor, 40, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);

    VisibilityModule::set_model_visible(weapon.module_accessor, false);

    sv_kinetic_energy!(
        set_stable_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        -1.0,
        -1.0
    );
    sv_kinetic_energy!(
        set_limit_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        -1.0,
        -1.0
    );
    sv_kinetic_energy!(
        set_brake,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        0.0,
        0.0
    );
    let lr = PostureModule::lr(weapon.module_accessor);
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        4.8 * lr,
        0.0
    );
    sv_kinetic_energy!(
        enable,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL
    );
    0.into()
}

unsafe extern "C" fn blade_beam_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    LinkModule::remove_model_constraint(weapon.module_accessor, true);

    MotionModule::change_motion(
        weapon.module_accessor,
        Hash40::new("blade_beam"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    if !StopModule::is_stop(weapon.module_accessor) {
        blade_beam_substatus(weapon, false.into());
    }
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(blade_beam_substatus as *const () as _));

    weapon.fastshift(L2CValue::Ptr(blade_beam_fastshift as *const () as _))
}

unsafe extern "C" fn blade_beam_substatus(weapon: &mut L2CWeaponCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        if WorkModule::count_down_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE, 0) {
            notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        }
    }
    0.into()
}

unsafe extern "C" fn blade_beam_fastshift(weapon: &mut L2CWeaponCommon) -> L2CValue {
    // if !StatusModule::is_changing(weapon.module_accessor) {
    //     LinkModule::remove_model_constraint(weapon.module_accessor, true);
    // }
    if GroundModule::is_touch(weapon.module_accessor, (*GROUND_TOUCH_FLAG_LEFT | *GROUND_TOUCH_FLAG_RIGHT) as u32) {
        println!("Help");
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}

unsafe extern "C" fn blade_beam_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if VarModule::is_flag(weapon.module_accessor, vars::ike_sword::status::flag::BLADE_BEAM_KINETIC_SHIFT) {
        sv_kinetic_energy!(
            set_stable_speed,
            weapon,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            1.5,
            1.5
        );
        sv_kinetic_energy!(
            set_brake,
            weapon,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            0.1,
            0.1
        );
        VarModule::off_flag(weapon.module_accessor, vars::ike_sword::status::flag::BLADE_BEAM_KINETIC_SHIFT);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, vars::ike_sword::status::BLADE_BEAM, blade_beam_pre);
    agent.status(Init, vars::ike_sword::status::BLADE_BEAM, blade_beam_init);
    agent.status(Main, vars::ike_sword::status::BLADE_BEAM, blade_beam_main);
    agent.status(Exec, vars::ike_sword::status::BLADE_BEAM, blade_beam_exec);
}