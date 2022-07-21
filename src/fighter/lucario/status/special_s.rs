use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::{Hash40, Vector3f, Vector2f},
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    smashline::*,
    custom_var::*,
    wubor_utils::{vars::*, table_const::*},
    super::super::helper::*
};

#[status_script(agent = "lucario", status = FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_S_THROW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucario_special_s_throw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
    let enhance = lucario_drain_aura(fighter, false);
    let mot = if !enhance {
        hash40("special_s_throw")
    }
    else {
        hash40("special_s_throw_2")
    };
    WorkModule::set_int64(fighter.module_accessor, mot as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
    let mot = if !enhance {
        hash40("special_air_s_throw")
    }
    else {
        hash40("special_air_s_throw_2")
    };
    WorkModule::set_int64(fighter.module_accessor, mot as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AIR_MOT);

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
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
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
            if VarModule::is_flag(fighter.battle_object, lucario::status::flag::SPECIAL_S_THROW_ENABLE_GRAVITY) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                VarModule::off_flag(fighter.battle_object, lucario::status::flag::SPECIAL_S_THROW_ENABLE_GRAVITY);
                VarModule::on_flag(fighter.battle_object, lucario::status::flag::SPECIAL_S_THROW_POST_GRAVITY);
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLAG_CRITICAL_HIT) {
            if VarModule::get_int(fighter.battle_object, lucario::instance::int::AURA_LEVEL) > 1 {
                let pos = ModelModule::joint_global_position(
                    fighter.module_accessor,
                    Hash40::new("throw"),
                    &mut Vector3f{x: 0.0, y: 0.0, z: 0.0},
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
                let mut event = crate::function_hooks::LinkEventThrow::new_l2c_table();
                event["link_event_kind_"].assign(&L2CValue::Hash40(Hash40::new("throw")));
                let callable: extern "C" fn() -> *mut smash::app::LinkEvent = std::mem::transmute(event["new_instance_lua_"].get_ptr());
                let link_event = callable();
                smash::app::lua_bind::LinkEvent::load_from_l2c_table(link_event, &event);
                LinkModule::send_event_nodes_struct(fighter.module_accessor, *LINK_NO_CAPTURE, link_event, 0);
                event = smash::app::lua_bind::LinkEvent::store_l2c_table(link_event);
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

pub fn install() {
    install_status_scripts!(
        lucario_special_s_throw_main
    );
}