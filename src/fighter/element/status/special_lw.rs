use crate::imports::status_imports::*;
use super::super::change_helper::*;

extern "C" {
    #[link_name = "\u{1}_ZN3app29FighterElementLinkEventChange13new_l2c_tableEv"]
    pub fn FighterElementLinkEventChange__new_l2c_table() -> smash::lib::L2CValue;
}

pub unsafe fn element_special_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
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
