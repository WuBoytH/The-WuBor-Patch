use super::*;

#[allow(non_snake_case)]
pub mod LinkEventThrow {
    use super::*;
    extern "C" {
        #[link_name = "\u{1}_ZN3app14LinkEventThrow13new_l2c_tableEv"]
        pub fn new_l2c_table() -> L2CValue;
    }
}

pub unsafe extern "C" fn jack_special_s_catch_jump_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
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

pub unsafe extern "C" fn jack_special_s_catch_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    jack_special_s_catch_helper(fighter);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_s1_catch_jump"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    let model_flag = LinkModule::get_model_constraint_flag(fighter.module_accessor) as u32;
    LinkModule::set_model_constraint_flag(fighter.module_accessor, model_flag | *CONSTRAINT_FLAG_OFFSET_ROT as u32);
    LinkModule::set_constraint_rot_offset_y(fighter.module_accessor, 180.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(jack_special_s_catch_jump_main_loop as *const () as _))
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
    smash::app::lua_bind::LinkEvent::load_from_l2c_table(link_event, &event);
    LinkModule::send_event_nodes_struct(fighter.module_accessor, *LINK_NO_CAPTURE, link_event, 0);
    event = smash::app::lua_bind::LinkEvent::store_l2c_table(link_event);
    let deleter: extern "C" fn(*mut smash::app::LinkEvent) = std::mem::transmute(*((*(link_event as *const u64) + 0x8) as *const u64));
    deleter(link_event);

    WorkModule::set_int(fighter.module_accessor, object_id as i32, *FIGHTER_DIDDY_STATUS_MONKEY_FLIP_WORK_INT_TARGET_TASK);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_DIDDY_STATUS_MONKEY_FLIP_WORK_INT_TARGET_HIT_GROUP);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_DIDDY_STATUS_MONKEY_FLIP_WORK_INT_TARGET_HIT_NO);
    WorkModule::set_int(fighter.module_accessor, object_id as i32, *FIGHTER_DIDDY_STATUS_MONKEY_FLIP_WORK_INT_CLING_TASK_ID);
}

unsafe extern "C" fn jack_special_s_catch_jump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_DIDDY_STATUS_SPECIAL_S_TRANSITION_TERM_ID_GROUND)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        return 0.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn jack_special_s_catch_jump_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    CatchModule::cling_cut(fighter.module_accessor, false);
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, vars::jack::status::SPECIAL_S_CATCH_JUMP, jack_special_s_catch_jump_pre);
    agent.status(Main, vars::jack::status::SPECIAL_S_CATCH_JUMP, jack_special_s_catch_jump_main);
    agent.status(End, vars::jack::status::SPECIAL_S_CATCH_JUMP, jack_special_s_catch_jump_end);
}