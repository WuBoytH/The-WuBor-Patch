use super::*;

unsafe extern "C" fn normal_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    // println!("normal");
    if WorkModule::is_flag(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_FLAG_SWALLOWED) {
        WorkModule::set_float(weapon.module_accessor, 1.0, *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_FLOAT_SWALLOWED_SPEED_MUL);
    }

    let is_air = WorkModule::is_flag(weapon.module_accessor, *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_FLAG_TYPE_AIR);
    let strength = WorkModule::get_int(weapon.module_accessor, *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_INT_STRENGTH);
    let (correct, motion) = if is_air {
        let motion = if strength == *FIGHTER_DOLLY_STRENGTH_W {
            "normal_air_w"
        }
        else {
            "normal_air"
        };
        (*GROUND_CORRECT_KIND_AIR, motion)
    }
    else {
        let motion = if strength == *FIGHTER_DOLLY_STRENGTH_W {
            "normal_w"
        }
        else {
            "normal"
        };
        (*GROUND_CORRECT_KIND_GROUND, motion)
    };
    GroundModule::correct(weapon.module_accessor, GroundCorrectKind(correct));
    MotionModule::change_motion(
        weapon.module_accessor,
        Hash40::new(motion),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    normal_speed_helper(weapon);

    let situation = weapon.global_table[SITUATION_KIND].get_i32();
    weapon.global_table[PREV_SITUATION_KIND].assign(&L2CValue::I32(situation));

    weapon.fastshift(L2CValue::Ptr(normal_fastshift as *const () as _))
}

unsafe extern "C" fn normal_speed_helper(weapon: &mut L2CWeaponCommon) {
    let mul = if WorkModule::is_flag(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_FLAG_SWALLOWED) {
        WorkModule::get_float(weapon.module_accessor, *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_FLOAT_SWALLOWED_SPEED_MUL)
    }
    else {
        1.0
    };

    if WorkModule::is_flag(weapon.module_accessor, *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_FLAG_TYPE_AIR) {
        let speed_air = WorkModule::get_float(weapon.module_accessor, *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_FLOAT_SPEED_AIR);
        let lr = PostureModule::lr(weapon.module_accessor);
        sv_kinetic_energy!(
            set_speed,
            weapon,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            speed_air * lr * mul,
            0.0
        );
    }
    else {
        if weapon.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            let speed_g = WorkModule::get_float(weapon.module_accessor, *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_FLOAT_SPEED_GROUND);
            let lr = PostureModule::lr(weapon.module_accessor);
            sv_kinetic_energy!(
                set_speed,
                weapon,
                WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
                speed_g * lr * mul,
                0.0
            );
        }
    }
}

unsafe extern "C" fn normal_fastshift(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if StatusModule::is_changing(weapon.module_accessor) {
        return 0.into();
    }

    let flag = if PostureModule::lr(weapon.module_accessor) == 1.0 {
        *GROUND_TOUCH_FLAG_RIGHT
    }
    else {
        *GROUND_TOUCH_FLAG_LEFT
    };
    if GroundModule::is_touch(weapon.module_accessor, (flag | *GROUND_TOUCH_FLAG_UP) as u32) {
        EffectModule::detach_all(weapon.module_accessor, 5);
        notify_event_msc_cmd!(weapon, 0x199c462b5d_u64);
        return 0.into();
    }

    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        let normal_x = GroundModule::get_touch_normal_x(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
        let normal_y = GroundModule::get_touch_normal_y(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
        let angle = normal_x.atan2(normal_y);
        let lr = PostureModule::lr(weapon.module_accessor);
        let degrees = (angle * lr).to_degrees();
        // println!("degrees: {}", degrees);
        WorkModule::set_float(weapon.module_accessor, degrees, *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_FLOAT_GROUND_ANGLE);
        let angle_kill = WorkModule::get_param_float(weapon.module_accessor, hash40("param_wave"), 0x1d5e336640);
        if degrees.abs() > angle_kill {
            EffectModule::detach_all(weapon.module_accessor, 5);
            notify_event_msc_cmd!(weapon, 0x199c462b5d_u64);
            return 0.into();
        }
    }

    if StatusModule::is_situation_changed(weapon.module_accessor) {
        let is_air = weapon.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR;
        let strength = WorkModule::get_int(weapon.module_accessor, *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_INT_STRENGTH);
        let (correct, motion) = if is_air {
            let motion = if strength == *FIGHTER_DOLLY_STRENGTH_W {
                "normal_air_w"
            }
            else {
                "norma_air"
            };
            (*GROUND_CORRECT_KIND_AIR, motion)
        }
        else {
            let motion = if strength == *FIGHTER_DOLLY_STRENGTH_W {
                "normal_w"
            }
            else {
                "norma"
            };
            (*GROUND_CORRECT_KIND_GROUND, motion)
        };
        GroundModule::correct(weapon.module_accessor, GroundCorrectKind(correct));
        MotionModule::change_motion_inherit_frame(
            weapon.module_accessor,
            Hash40::new(motion),
            -1.0,
            1.0,
            0.0,
            false,
            false
        );
        normal_speed_helper(weapon);
    }

    0.into()
}

unsafe extern "C" fn normal_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if weapon.global_table[STATUS_FRAME].get_f32() <= 1.0 {
        return 0.into();
    }

    if WorkModule::count_down_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE, 0) {
        EffectModule::detach_all(weapon.module_accessor, 5);
        notify_event_msc_cmd!(weapon, 0x199c462b5d_u64);
        return 0.into();
    }

    // usually has an air check
    let angle = WorkModule::get_float(weapon.module_accessor, *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_FLOAT_GROUND_ANGLE).to_radians();
    let angle_kill = WorkModule::get_param_float(weapon.module_accessor, hash40("param_wave"), 0x1d5e336640);
    if angle.abs() < angle_kill {
        let rot_x = PostureModule::rot_x(weapon.module_accessor, 0);
        let time_to_adjust_angle = WorkModule::get_param_float(weapon.module_accessor, hash40("param_wave"), hash40("time_to_adjust_angle"));
        let rot = PostureModule::rot(weapon.module_accessor, 0) as *mut Vector3f;
        let sub = angle.abs() - rot_x;
        let add = sub / time_to_adjust_angle;
        let final_rot = (*rot).x + add;
        PostureModule::set_rot(weapon.module_accessor, &Vector3f::new(final_rot, (*rot).y, (*rot).z), 0);
    }

    let speed = if weapon.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        WorkModule::get_float(weapon.module_accessor, *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_FLOAT_SPEED_GROUND)
    }
    else {
        WorkModule::get_float(weapon.module_accessor, *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_FLOAT_SPEED_AIR)
    };
    // let brake = if VarModule::is_flag(weapon.module_accessor, vars::ike_wave::status::flag::BLADE_BEAM_KINETIC_SHIFT) {
    //     0.1
    // }
    // else {
    //     0.0
    // };
    let mul = if WorkModule::is_flag(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_FLAG_SWALLOWED) {
        WorkModule::get_float(weapon.module_accessor, *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_FLOAT_SWALLOWED_SPEED_MUL)
    }
    else {
        1.0
    };
    // println!("base speed: {}", speed);
    // println!("base brake: {}", brake);
    // println!("angle: {}", angle);
    let lr = PostureModule::lr(weapon.module_accessor);
    let speed_x = speed * angle.cos().abs() * lr;
    // let brake_x = brake * angle.cos().abs();
    let speed_y;
    // let brake_y;
    if weapon.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        speed_y = speed * angle.sin() * -1.0;
        // brake_y = brake * angle.sin().abs();
    }
    else {
        speed_y = 0.0;
        // brake_y = 0.0;
    }
    // println!("speed: {}, {}", speed_x, speed_y);
    // println!("brake: {}, {}", brake_x, brake_y);
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed_x * mul,
        speed_y * mul
    );
    // sv_kinetic_energy!(
    //     set_brake,
    //     weapon,
    //     WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
    //     brake_x,
    //     brake_y
    // );

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_DOLLY_WAVE_STATUS_KIND_NORMAL, normal_main);
    agent.status(Exec, *WEAPON_DOLLY_WAVE_STATUS_KIND_NORMAL, normal_exec);
}