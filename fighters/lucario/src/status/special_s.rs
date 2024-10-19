use super::*;
use super::super::helper::*;

#[allow(non_snake_case)]
pub mod LinkEventThrow {
    use super::*;
    extern "C" {
        #[link_name = "\u{1}_ZN3app14LinkEventThrow13new_l2c_tableEv"]
        pub fn new_l2c_table() -> L2CValue;
    }
}

unsafe extern "C" fn lucario_special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        VarModule::on_flag(fighter.module_accessor, vars::fighter::instance::flag::DISABLE_SPECIAL_S);
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_s") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_s") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AIR_MOT);
    lucario_special_s_set_kinetic(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(lucario_special_s_main_loop as *const () as _))
}

unsafe extern "C" fn lucario_special_s_set_kinetic(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        lucario_special_set_air(fighter);
        lucario_special_air_mot_helper(fighter);
        if VarModule::is_flag(fighter.module_accessor, vars::lucario::status::flag::SPECIAL_S_SET_MOTION) {
            if VarModule::is_flag(fighter.module_accessor, vars::lucario::status::flag::SPECIAL_S_POST_GRAVITY) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            }
            else {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            }
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        }
    }
    else {
        lucario_special_set_ground(fighter);
        lucario_special_ground_mot_helper(fighter);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
}

unsafe extern "C" fn lucario_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if !MotionModule::is_end(fighter.module_accessor) {
        if VarModule::is_flag(fighter.module_accessor, vars::lucario::status::flag::SPECIAL_S_CHECK_ENHANCE) {
            VarModule::off_flag(fighter.module_accessor, vars::lucario::status::flag::SPECIAL_S_CHECK_ENHANCE);
            VarModule::on_flag(fighter.module_accessor, vars::lucario::status::flag::SPECIAL_S_SET_MOTION);
            let enhance = VarModule::get_int(fighter.module_accessor, vars::lucario::instance::int::AURA_LEVEL) > 0;
            if enhance && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                VarModule::on_flag(fighter.module_accessor, vars::lucario::status::flag::SPECIAL_S_ENHANCE);
                lucario_drain_aura(fighter, false);
            }
            lucario_special_s_set_kinetic(fighter);
        }
        if VarModule::is_flag(fighter.module_accessor, vars::lucario::status::flag::SPECIAL_S_ENABLE_GRAVITY) {
            VarModule::off_flag(fighter.module_accessor, vars::lucario::status::flag::SPECIAL_S_ENABLE_GRAVITY);
            VarModule::on_flag(fighter.module_accessor, vars::lucario::status::flag::SPECIAL_S_POST_GRAVITY);
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                sv_kinetic_energy!(
                    controller_set_accel_x_add,
                    fighter,
                    0.02
                );
                sv_kinetic_energy!(
                    controller_set_accel_x_mul,
                    fighter,
                    0.02
                );
            }
        }
        if !StatusModule::is_changing(fighter.module_accessor)
        && StatusModule::is_situation_changed(fighter.module_accessor) {
            lucario_special_s_set_kinetic(fighter);
        }
    }
    else {
        let status = if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_FALL
        }
        else {
            FIGHTER_STATUS_KIND_WAIT
        };
        fighter.change_status(status.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn lucario_special_s_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    // if VarModule::is_flag(fighter.module_accessor, vars::lucario::status::flag::SPECIAL_S_ENHANCE) {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
        let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
        let mul = 0.5;
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            speed_x * mul,
            0.0
        );
    // }
    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_S_THROW {
        VarModule::off_flag(fighter.module_accessor, vars::fighter::instance::flag::DISABLE_SPECIAL_S);
    }
    0.into()
}

unsafe extern "C" fn lucario_special_s_throw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
    VarModule::off_flag(fighter.module_accessor, vars::lucario::status::flag::SPECIAL_S_ENABLE_GRAVITY);
    VarModule::off_flag(fighter.module_accessor, vars::lucario::status::flag::SPECIAL_S_POST_GRAVITY);
    let mot_g;
    let mot_a;
    if VarModule::is_flag(fighter.module_accessor, vars::lucario::status::flag::SPECIAL_S_ENHANCE) {
        mot_g = hash40("special_s_throw_2");
        mot_a = hash40("special_air_s_throw_2");
    }
    else {
        mot_g = hash40("special_s_throw");
        mot_a = hash40("special_air_s_throw");
    };
    WorkModule::set_int64(fighter.module_accessor, mot_g as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
    WorkModule::set_int64(fighter.module_accessor, mot_a as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AIR_MOT);

    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_INT_FRAME);
    lucario_special_s_throw_set_kinetic(fighter);
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(lucario_special_s_throw_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(lucario_special_s_throw_main_loop as *const () as _))
}

unsafe extern "C" fn lucario_special_s_throw_set_kinetic(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        lucario_special_set_air(fighter);
        lucario_special_air_mot_helper(fighter);
        let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            ENERGY_STOP_RESET_TYPE_FREE,
            speed_x,
            speed_y.clamp(-1.0, 1.0),
            0.0,
            0.0,
            0.0
        );
        sv_kinetic_energy!(
            set_brake,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            0.06,
            0.06
        );
        sv_kinetic_energy!(
            clear_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY
        );
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    else {
        lucario_special_set_ground(fighter);
        lucario_special_ground_mot_helper(fighter);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
}

unsafe extern "C" fn lucario_special_s_throw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if !MotionModule::is_end(fighter.module_accessor) {
        // Original but unused
        // if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        //     if 1 <= WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_INT_FRAME) {
        //         fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
        //         return 0.into();
        //     }
        // }
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            if VarModule::is_flag(fighter.module_accessor, vars::lucario::status::flag::SPECIAL_S_ENABLE_GRAVITY) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                VarModule::off_flag(fighter.module_accessor, vars::lucario::status::flag::SPECIAL_S_ENABLE_GRAVITY);
                VarModule::on_flag(fighter.module_accessor, vars::lucario::status::flag::SPECIAL_S_POST_GRAVITY);
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLAG_CRITICAL_HIT) {
            if VarModule::get_int(fighter.module_accessor, vars::lucario::instance::int::AURA_LEVEL) > 1 {
                let mut pos = Vector3f{x: 0.0, y: 0.0, z: 0.0};
                ModelModule::joint_global_position(
                    fighter.module_accessor,
                    Hash40::new("throw"),
                    &mut pos,
                    true
                );
                let capture_object = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE) as u32;
                FighterUtil::request_critical_hit_cut_in_force(
                    fighter.module_accessor,
                    capture_object,
                    &Vector2f{x: pos.x, y: pos.y},
                    *FIGHTER_KIND_LUCARIO,
                    Hash40::new("param_special_s"),
                    *LINK_NO_NONE,
                    true,
                    0,
                    true
                );
            }
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLAG_CRITICAL_HIT);
        }
        let throw_done = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLAG_THROW_DONE);
        let request_throw = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLAG_REQUEST_THROW);
        if !throw_done {
            if request_throw {
                let mut event = LinkEventThrow::new_l2c_table();
                event["link_event_kind_"].assign(&L2CValue::Hash40(Hash40::new("throw")));
                let callable: extern "C" fn() -> *mut smash::app::LinkEvent = std::mem::transmute(event["new_instance_lua_"].get_ptr());
                let link_event = callable();
                lua_bind::LinkEvent::load_from_l2c_table(link_event, &event);
                LinkModule::send_event_nodes_struct(fighter.module_accessor, *LINK_NO_CAPTURE, link_event, 0);
                event = lua_bind::LinkEvent::store_l2c_table(link_event);
                let deleter: extern "C" fn(*mut smash::app::LinkEvent) = std::mem::transmute(*((*(link_event as *const u64) + 0x8) as *const u64));
                deleter(link_event);
    
                let node_obj_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
                WorkModule::set_int(fighter.module_accessor, node_obj_id as i32, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
                WorkModule::set_int(fighter.module_accessor, event["hit_group_"].get_i32(), *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
                WorkModule::set_int(fighter.module_accessor, event["hit_no_"].get_i32(), *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
                WorkModule::set_float(fighter.module_accessor, event["motion_rate_"].get_f32(), *FIGHTER_STATUS_THROW_WORK_FLOAT_MOTION_RATE);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLAG_THROW_DONE); 
            }
        }
        // if !StatusModule::is_changing(fighter.module_accessor)
        // && StatusModule::is_situation_changed(fighter.module_accessor) {
        //     lucario_special_s_throw_set_kinetic(fighter);
        // }
    }
    else {
        let status = if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_FALL
        }
        else {
            FIGHTER_STATUS_KIND_WAIT
        };
        fighter.change_status(status.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn lucario_special_s_throw_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_INT_FRAME);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, lucario_special_s_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, lucario_special_s_end);

    agent.status(Main, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_S_THROW, lucario_special_s_throw_main);
}