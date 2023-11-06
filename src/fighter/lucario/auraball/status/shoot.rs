use crate::imports::status_imports::*;

unsafe extern "C" fn lucario_auraball_shoot_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MiscModule::get_vars_from_pocket(weapon.module_accessor);
    if VarModule::is_flag(weapon.module_accessor, lucario_auraball::instance::flag::SPIRIT_BOMB) {
        let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
        let owner_object = MiscModule::get_battle_object_from_id(owner_id);
        let lucario = sv_battle_object::category(owner_id) == *BATTLE_OBJECT_CATEGORY_FIGHTER
        && sv_battle_object::kind(owner_id) == *FIGHTER_KIND_LUCARIO
        && VarModule::is_flag((*owner_object).module_accessor, lucario::status::flag::SPECIAL_N_START_FROM_GROUND);
        if StatusModule::situation_kind((*owner_object).module_accessor) == *SITUATION_KIND_GROUND
        || lucario {
            WorkModule::set_customize_no(weapon.module_accessor, 1, 0);
        }
        else {
            VarModule::on_flag(weapon.module_accessor, lucario_auraball::status::flag::FROM_AIR);
            WorkModule::set_customize_no(weapon.module_accessor, 2, 0);
        }
    }
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(0),
        false,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
    0.into()
}

unsafe extern "C" fn lucario_auraball_shoot_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_auraball"), hash40("life"));
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
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
    let max_charge = WorkModule::get_int(weapon.module_accessor, *WEAPON_LUCARIO_AURABALL_INSTANCE_WORK_ID_INT_PARAM_MAX_CHARGE_FRAME);
    let charge = WorkModule::get_int(weapon.module_accessor, *WEAPON_LUCARIO_AURABALL_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
    let ratio = charge as f32 / max_charge as f32;
    AttackModule::set_lerp_ratio(weapon.module_accessor, ratio, 0);
    AttackModule::set_attack_scale(weapon.module_accessor, 1.0, false);
    if VarModule::is_flag(weapon.module_accessor, lucario_auraball::instance::flag::SPIRIT_BOMB) {
        GroundModule::set_passable_check(weapon.module_accessor, true);
    }
    if !StopModule::is_stop(weapon.module_accessor) {
        lucario_auraball_shoot_substatus(weapon, false.into());
    }
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(lucario_auraball_shoot_substatus as *const () as _));
    weapon.fastshift(L2CValue::Ptr(lucario_auraball_shoot_main_fastshift as *const () as _))
}

unsafe extern "C" fn lucario_auraball_shoot_substatus(weapon: &mut L2CWeaponCommon, param_3: L2CValue) -> L2CValue {
    let mot = MotionModule::motion_kind(weapon.module_accessor);
    if !param_3.get_bool() && mot != hash40("explosion") {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        if life <= 0 {
            if VarModule::is_flag(weapon.module_accessor, lucario_auraball::instance::flag::SPIRIT_BOMB) {
                MotionModule::change_motion(
                    weapon.module_accessor,
                    Hash40::new("explosion"),
                    0.0,
                    1.0,
                    false,
                    0.0,
                    false,
                    false
                );
                let max_charge = WorkModule::get_int(weapon.module_accessor, *WEAPON_LUCARIO_AURABALL_INSTANCE_WORK_ID_INT_PARAM_MAX_CHARGE_FRAME);
                let charge = WorkModule::get_int(weapon.module_accessor, *WEAPON_LUCARIO_AURABALL_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
                let ratio = charge as f32 / max_charge as f32;
                AttackModule::set_lerp_ratio(weapon.module_accessor, ratio, 0);
                AttackModule::set_attack_scale(weapon.module_accessor, 1.0, false);
            }
            else {
                notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
            }
        }
        let touch_flag = GroundModule::get_touch_moment_flag(weapon.module_accessor);
        if VarModule::is_flag(weapon.module_accessor, lucario_auraball::instance::flag::SPIRIT_BOMB)
        && touch_flag & (*GROUND_TOUCH_FLAG_DOWN | *GROUND_TOUCH_FLAG_RIGHT | *GROUND_TOUCH_FLAG_LEFT) as u64 != 0
        && MotionModule::motion_kind(weapon.module_accessor) != hash40("explosion") {
            MotionModule::change_motion(
                weapon.module_accessor,
                Hash40::new("explosion"),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            let max_charge = WorkModule::get_int(weapon.module_accessor, *WEAPON_LUCARIO_AURABALL_INSTANCE_WORK_ID_INT_PARAM_MAX_CHARGE_FRAME);
            let charge = WorkModule::get_int(weapon.module_accessor, *WEAPON_LUCARIO_AURABALL_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
            let ratio = charge as f32 / max_charge as f32;
            AttackModule::set_lerp_ratio(weapon.module_accessor, ratio, 0);
            AttackModule::set_attack_scale(weapon.module_accessor, 1.0, false);
        }
        if touch_flag & (*GROUND_TOUCH_FLAG_LEFT | *GROUND_TOUCH_FLAG_RIGHT) as u64 != 0 {
            if VarModule::is_flag(weapon.module_accessor, lucario_auraball::instance::flag::SPIRIT_BOMB)
            && MotionModule::motion_kind(weapon.module_accessor) != hash40("explosion") {
                MotionModule::change_motion(
                    weapon.module_accessor,
                    Hash40::new("explosion"),
                    0.0,
                    1.0,
                    false,
                    0.0,
                    false,
                    false
                );
                let max_charge = WorkModule::get_int(weapon.module_accessor, *WEAPON_LUCARIO_AURABALL_INSTANCE_WORK_ID_INT_PARAM_MAX_CHARGE_FRAME);
                let charge = WorkModule::get_int(weapon.module_accessor, *WEAPON_LUCARIO_AURABALL_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
                let ratio = charge as f32 / max_charge as f32;
                AttackModule::set_lerp_ratio(weapon.module_accessor, ratio, 0);
                AttackModule::set_attack_scale(weapon.module_accessor, 1.0, false);
            }
            else {
                let pos = PostureModule::pos(weapon.module_accessor);
                EffectModule::req(
                    weapon.module_accessor,
                    Hash40::new_raw(0x15cff20136),
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
        }
    }
    0.into()
}

unsafe extern "C" fn lucario_auraball_shoot_main_fastshift(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if VarModule::is_flag(weapon.module_accessor, lucario_auraball::instance::flag::SPIRIT_BOMB) {
        if AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_HIT)
        || AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_SHIELD)
        && !AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_REFLECTOR) {
            if !VarModule::is_flag(weapon.module_accessor, lucario_auraball::status::flag::EXPLOSION) {
                WorkModule::set_int(weapon.module_accessor, 30, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
                VarModule::on_flag(weapon.module_accessor, lucario_auraball::status::flag::EXPLOSION);
            }
        }
    }
    let mut x_val = 0.0;
    let mut y_val = 0.0;
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        x_val = GroundModule::get_touch_normal_x_consider_gravity(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
        y_val = GroundModule::get_touch_normal_y_consider_gravity(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
    }
    else if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_UP as u32) {
        x_val = GroundModule::get_touch_normal_x_consider_gravity(weapon.module_accessor, *GROUND_TOUCH_FLAG_UP as u32);
        y_val = GroundModule::get_touch_normal_y_consider_gravity(weapon.module_accessor, *GROUND_TOUCH_FLAG_UP as u32);
    }
    let length = sv_math::vec2_length(x_val, y_val);
    if 0.0 < length {
        let atan = x_val.atan2(y_val).abs();
        let deactivate_angle = WorkModule::get_param_float(weapon.module_accessor, hash40("param_auraball"), hash40("deactivate_angle"));
        let rad = deactivate_angle.to_radians();
        let angle = std::f32::consts::PI - rad;
        if rad < atan && atan < angle {
            let lr = PostureModule::lr(weapon.module_accessor);
            x_val *= lr;
            if x_val < 0.0 {
                let pos = PostureModule::pos(weapon.module_accessor);
                EffectModule::req(
                    weapon.module_accessor,
                    Hash40::new_raw(0x15cff20136),
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
        }
    }
    0.into()
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Pre, *WEAPON_LUCARIO_AURABALL_STATUS_KIND_SHOOT, lucario_auraball_shoot_pre);
    agent.status(smashline::Main, *WEAPON_LUCARIO_AURABALL_STATUS_KIND_SHOOT, lucario_auraball_shoot_main);
}