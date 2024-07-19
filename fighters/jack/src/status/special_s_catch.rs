use super::*;

#[allow(non_snake_case)]
pub mod LinkEventThrow {
    use super::*;
    extern "C" {
        #[link_name = "\u{1}_ZN3app14LinkEventThrow13new_l2c_tableEv"]
        pub fn new_l2c_table() -> L2CValue;
    }
}

pub unsafe extern "C" fn jack_special_s_catch_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_RESET,
        *GROUND_CORRECT_KIND_NONE as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        true,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

pub unsafe extern "C" fn jack_special_s_catch_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_s1_catch"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(jack_special_s_catch_main_loop as *const () as _))
}

unsafe extern "C" fn jack_special_s_catch_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(vars::jack::status::SPECIAL_S_CATCH_JUMP.into(), false.into());
        return 0.into();
    }
    if VarModule::is_flag(fighter.module_accessor, vars::jack::status::flag::SPECIAL_S_CATCH_THROW) {
        VarModule::off_flag(fighter.module_accessor, vars::jack::status::flag::SPECIAL_S_CATCH_THROW);
        jack_special_s_catch_helper(fighter);
    }
    CatchModule::update_pos_cling(fighter.module_accessor);
    0.into()
}

#[allow(unused_assignments)]
unsafe extern "C" fn jack_special_s_catch_helper(fighter: &mut L2CFighterCommon) {
    let object_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE) as u32;
    let category = sv_battle_object::category(object_id);
    let kind = sv_battle_object::kind(object_id);
    let mut motion = hash40("clung_thrown_jump_diddy");
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        let capture_module_accessor = sv_battle_object::module_accessor(object_id);
        if [
            *FIGHTER_KIND_KIRBY,
            *FIGHTER_KIND_PURIN,
            *FIGHTER_KIND_GAMEWATCH
        ].contains(&kind) {
            motion = FighterMotionModuleImpl::add_body_type_hash(fighter.module_accessor, Hash40::new("clung_thrown_jump_diddy"), *BODY_TYPE_MOTION_DX);
        }
        else {
            let motion_share = WorkModule::get_param_int(capture_module_accessor, hash40("param_motion"), hash40("motion_share"));
            if motion_share == *FIGHTER_MOTION_SHARE_TYPE_GIRL {
                motion = FighterMotionModuleImpl::add_body_type_hash(fighter.module_accessor, Hash40::new("clung_thrown_jump_diddy"), *BODY_TYPE_MOTION_GIRL);
            }
        }
    }
    let mut event = LinkEventThrow::new_l2c_table();
    event["link_event_kind_"].assign(&L2CValue::Hash40(Hash40::new("cling_attack_diddy")));
    event["motion_kind_"].assign(&L2CValue::Hash40(Hash40::new_raw(motion)));
    let callable: extern "C" fn() -> *mut smash::app::LinkEvent = std::mem::transmute(event["new_instance_lua_"].get_ptr());
    let link_event = callable();
    lua_bind::LinkEvent::load_from_l2c_table(link_event, &event);
    LinkModule::send_event_nodes_struct(fighter.module_accessor, *LINK_NO_CAPTURE, link_event, 0);
    event = lua_bind::LinkEvent::store_l2c_table(link_event);
    let deleter: extern "C" fn(*mut smash::app::LinkEvent) = std::mem::transmute(*((*(link_event as *const u64) + 0x8) as *const u64));
    deleter(link_event);

    WorkModule::set_int(fighter.module_accessor, object_id as i32, *FIGHTER_DIDDY_STATUS_MONKEY_FLIP_WORK_INT_TARGET_TASK);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_DIDDY_STATUS_MONKEY_FLIP_WORK_INT_TARGET_HIT_GROUP);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_DIDDY_STATUS_MONKEY_FLIP_WORK_INT_TARGET_HIT_NO);
    WorkModule::set_int(fighter.module_accessor, object_id as i32, *FIGHTER_DIDDY_STATUS_MONKEY_FLIP_WORK_INT_CLING_TASK_ID);
}

#[allow(unused_assignments)]
unsafe extern "C" fn jack_special_s_catch_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != vars::jack::status::SPECIAL_S_CATCH_JUMP {
        CatchModule::cling_cut(fighter.module_accessor, false);
    }
    else {
        let mut event = LinkEventThrow::new_l2c_table();
        event["link_event_kind_"].assign(&L2CValue::Hash40(Hash40::new("cling_throw_diddy")));
        event["motion_kind_"].assign(&L2CValue::Hash40(Hash40::new("invalid")));
        let callable: extern "C" fn() -> *mut smash::app::LinkEvent = std::mem::transmute(event["new_instance_lua_"].get_ptr());
        let link_event = callable();
        lua_bind::LinkEvent::load_from_l2c_table(link_event, &event);
        LinkModule::send_event_nodes_struct(fighter.module_accessor, *LINK_NO_CAPTURE, link_event, 0);
        event = lua_bind::LinkEvent::store_l2c_table(link_event);
        let deleter: extern "C" fn(*mut smash::app::LinkEvent) = std::mem::transmute(*((*(link_event as *const u64) + 0x8) as *const u64));
        deleter(link_event);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, vars::jack::status::SPECIAL_S_CATCH, jack_special_s_catch_pre);
    agent.status(Main, vars::jack::status::SPECIAL_S_CATCH, jack_special_s_catch_main);
    agent.status(End, vars::jack::status::SPECIAL_S_CATCH, jack_special_s_catch_end);
}