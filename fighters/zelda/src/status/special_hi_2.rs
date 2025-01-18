use super::*;

unsafe extern "C" fn zelda_special_hi_2_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    smashline::original_status(Init, fighter, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2)(fighter);
    fighter.clear_lua_stack();
    lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
    VarModule::set_float(fighter.module_accessor, vars::zelda::status::float::SPECIAL_HI_2_SPEED_X, speed_x);
    VarModule::set_float(fighter.module_accessor, vars::zelda::status::float::SPECIAL_HI_2_SPEED_Y, speed_y);
    0.into()
}

unsafe extern "C" fn zelda_special_hi_2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);

    VisibilityModule::set_whole(fighter.module_accessor, false);

    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), false);

    WorkModule::set_flag(fighter.module_accessor, false, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);

    GroundModule::set_passable_check(fighter.module_accessor, true);

    let cliff_check = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_CLIFF_CHECK);
    fighter.sub_fighter_cliff_check(cliff_check.into());

    if !StopModule::is_stop(fighter.module_accessor) {
        zelda_special_hi_2_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(zelda_special_hi_2_substatus as *const () as _));

    fighter.sub_shift_status_main(L2CValue::Ptr(zelda_special_hi_2_main_loop as *const () as _))
}

unsafe extern "C" fn zelda_special_hi_2_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        let special_hi_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_FRAME);
        let move_xlu = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("move_xlu"));
        if special_hi_frame == move_xlu {
            GroundModule::set_passable_check(fighter.module_accessor, false);
        }
        let move_cliff_check = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("move_cliff_check"));
        if special_hi_frame == move_cliff_check {
            fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
        }
    }
    else {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_FRAME);
        let special_hi_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_FRAME);
        if 2 <= special_hi_frame {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_CHECK_GROUND);
        }
    }
    0.into()
}

unsafe extern "C" fn zelda_special_hi_2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    let special_hi_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_FRAME);
    let move_time = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("move_time"));
    if move_time <= special_hi_frame {
        if GroundModule::is_touch(fighter.module_accessor, (*GROUND_TOUCH_FLAG_RIGHT | *GROUND_TOUCH_FLAG_LEFT) as u32) {
            let speed_x = VarModule::get_float(fighter.module_accessor, vars::zelda::status::float::SPECIAL_HI_2_SPEED_X);
            let speed_y = VarModule::get_float(fighter.module_accessor, vars::zelda::status::float::SPECIAL_HI_2_SPEED_Y);
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                speed_x,
                speed_y
            );
        }
        fighter.change_status(FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3.into(), false.into());
        return 0.into();
    }

    if StatusModule::is_changing(fighter.module_accessor)
    || StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
    }

    zelda_special_hi_2_check_ground(fighter);

    0.into()
}

#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "\u{1}_ZN3app17sv_kinetic_energy11get_speed3fEP9lua_State"]
    pub fn get_speed3f(arg1: u64) -> smash_rs::cpp::simd::Vector3;
}

unsafe extern "C" fn zelda_special_hi_2_check_ground(fighter: &mut L2CFighterCommon) {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_CHECK_GROUND) {
        return;
    }

    if GroundModule::is_attach_cliff(fighter.module_accessor) {
        return;
    }

    let mut touch_id = *GROUND_TOUCH_ID_NONE;
    let mut touch_flag = *GROUND_TOUCH_FLAG_NONE;

    if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32) {
        touch_id = *GROUND_TOUCH_ID_RIGHT;
        touch_flag = *GROUND_TOUCH_FLAG_RIGHT;
    }
    else if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32) {
        touch_id = *GROUND_TOUCH_ID_LEFT;
        touch_flag = *GROUND_TOUCH_FLAG_LEFT;
    }
    else if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_UP as u32) {
        touch_id = *GROUND_TOUCH_ID_UP;
        touch_flag = *GROUND_TOUCH_FLAG_UP;
    }
    else if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        touch_id = *GROUND_TOUCH_ID_DOWN;
        touch_flag = *GROUND_TOUCH_FLAG_DOWN;
    }

    if touch_flag == *GROUND_TOUCH_FLAG_NONE {
        let speed_x = VarModule::get_float(fighter.module_accessor, vars::zelda::status::float::SPECIAL_HI_2_SPEED_X);
        let speed_y = VarModule::get_float(fighter.module_accessor, vars::zelda::status::float::SPECIAL_HI_2_SPEED_Y);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            speed_x,
            speed_y
        );
        return;
    }

    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
    let speed = get_speed3f(fighter.lua_state_agent);

    let mut length = sv_math::vec3_length(speed.vec[0], speed.vec[1], speed.vec[2]);
    if 0.0 < length {
        let touch_x = GroundModule::get_touch_normal_x(fighter.module_accessor, touch_flag as u32);
        let touch_y = GroundModule::get_touch_normal_y(fighter.module_accessor, touch_flag as u32);

        let touch = fighter.Vector3__create(touch_x.into(), touch_y.into(), 0.0_f32.into());
        let something = fighter.Vector3__create(0.0_f32.into(), 0.0_f32.into(), 1.0_f32.into());
        let mut cross = fighter.Vector3__cross(touch.clone(), something);

        let math = 1.0 / length;
        let speed_mul = Vector3f{x: speed.vec[0] * math, y: speed.vec[1] * math, z: speed.vec[2] * math};
        let mut final_dot = 0.0;
        if touch_flag != *GROUND_TOUCH_FLAG_DOWN
        && 0.0 < speed_mul.y {
            if cross["y"].get_f32() < 0.0 {
                final_dot = -1.0;
            }
            let x = touch["x"].get_f32();
            let y = touch["y"].get_f32();
            let deg = x.atan2(y).to_degrees().abs();
            let deg = 180.0 - deg;
            let deg = deg.abs();
            let something = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x158bb5418d);
            if deg <= something {
                length = speed_mul.x.abs();
                cross["x"].assign(&L2CValue::F32(speed_mul.x.signum()));
            }
        }
        else {
            final_dot = sv_math::vec3_dot(cross["x"].get_f32(), cross["y"].get_f32(), cross["z"].get_f32(), speed_mul.x, speed_mul.y, speed_mul.z);
            if -0.00001 <= final_dot
            && final_dot <= 0.00001 {
                if touch_flag == *GROUND_TOUCH_FLAG_RIGHT
                || touch_flag == *GROUND_TOUCH_FLAG_LEFT {
                    final_dot = sv_math::vec3_dot(cross["x"].get_f32(), cross["y"].get_f32(), cross["z"].get_f32(), 0.0, 1.0, 0.0);
                }
                else {
                    let lr = PostureModule::lr(fighter.module_accessor);
                    final_dot = sv_math::vec3_dot(cross["x"].get_f32(), cross["y"].get_f32(), cross["z"].get_f32(), lr, 0.0, 0.0);
                }
            }
        }

        if final_dot < 0.0 {
            let x = cross["x"].get_f32();
            let y = cross["y"].get_f32();
            let z = cross["z"].get_f32();
            cross["x"].assign(&L2CValue::F32(x * -1.0));
            cross["y"].assign(&L2CValue::F32(y * -1.0));
            cross["z"].assign(&L2CValue::F32(z * -1.0));
        }
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            cross["x"].get_f32() * length,
            cross["y"].get_f32() * length,
            cross["z"].get_f32() * length
        );
        let situation = StatusModule::situation_kind(fighter.module_accessor);
        if situation == *SITUATION_KIND_GROUND {
            let line = GroundModule::get_touch_line_raw(fighter.module_accessor, GroundTouchID(touch_id)) as *mut GroundCollisionLine;
            let is_floor = sv_ground_collision_line::is_floor(line);
            GroundModule::set_attach_ground(fighter.module_accessor, is_floor);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2, zelda_special_hi_2_init);
    agent.status(Main, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2, zelda_special_hi_2_main);
}