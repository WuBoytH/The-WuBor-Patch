use super::*;
use super::change_helper::*;

extern "C" {
    #[link_name = "\u{1}_ZN3app29FighterElementLinkEventChange13new_l2c_tableEv"]
    pub fn FighterElementLinkEventChange__new_l2c_table() -> smash::lib::L2CValue;
}

#[no_mangle]
unsafe extern "C" fn element_special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
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
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64,
        (
            0x100 | // *FIGHTER_STATUS_ATTR_DISABLE_INTERRUPT_SLOW
            *FIGHTER_STATUS_ATTR_DISABLE_INTERRUPT_SPRING |
            0x8000000 | // *FIGHTER_STATUS_ATTR_DISABLE_INTERRUPT_STOP
            *FIGHTER_STATUS_ATTR_START_TURN
        ) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

#[no_mangle]
pub unsafe extern "C" fn element_special_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_attack = ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL);
    element_set_change_attack(fighter.module_accessor, is_attack);
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_ELEMENT_STATUS_KIND_SPECIAL_LW_STANDBY {
        let mut event = FighterElementLinkEventChange__new_l2c_table();
        event["link_event_kind_"].assign(&L2CValue::new_int(0x1cd83c14e3u64));
        event["object_id_"].assign(&L2CValue::I32(*BATTLE_OBJECT_ID_INVALID));
        let callable: extern "C" fn() -> *mut smash::app::LinkEvent = std::mem::transmute(event["new_instance_lua_"].get_ptr());
        let link_event = callable();
        lua_bind::LinkEvent::load_from_l2c_table(link_event, &event);
        LinkModule::send_event_nodes_struct(fighter.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, link_event, 0);
        event = lua_bind::LinkEvent::store_l2c_table(link_event);
        let deleter: extern "C" fn(*mut smash::app::LinkEvent) = std::mem::transmute(*((*(link_event as *const u64) + 0x8) as *const u64));
        deleter(link_event);
        let callable: extern "C" fn() -> *mut smash::app::LinkEvent = std::mem::transmute(event["new_instance_lua_"].get_ptr());
        let link_event = callable();
        lua_bind::LinkEvent::load_from_l2c_table(link_event, &event);
        LinkModule::send_event_nodes_struct(fighter.module_accessor, *ITEM_LINK_NO_HAVE, link_event, 0);
        // event = lua_bind::LinkEvent::store_l2c_table(link_event);
        lua_bind::LinkEvent::store_l2c_table(link_event);
        let deleter: extern "C" fn(*mut smash::app::LinkEvent) = std::mem::transmute(*((*(link_event as *const u64) + 0x8) as *const u64));
        deleter(link_event);
        AreaModule::set_whole(fighter.module_accessor,false);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, element_special_lw_pre);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, element_special_lw_end);
}