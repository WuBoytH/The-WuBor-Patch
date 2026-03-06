use super::*;

unsafe extern "C" fn attack_dash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighta = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
    FighterSpecializer_Pikmin::hold_pikmin(fighta, 1);
    FighterSpecializer_Pikmin::update_hold_pikmin_param(fighta);
    let pikmin_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
    if pikmin_num > 0 {
        let pikmin_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_OBJECT_ID_0) as u32;
        if LinkModule::link(fighter.module_accessor, *FIGHTER_PIKMIN_LINK_NO_PIKMIN_ATTACK, pikmin_id) != 0 {
            let mut link_event = FighterPikminLinkEventWeaponPikminChangeStatus__new_l2c_table();

            link_event["status_kind_"].assign(&L2CValue::I32(vars::pikmin_pikmin::status::ATTACK_DASH));

            link_event["link_event_kind_"].assign(&L2CValue::Hash40(Hash40::new("fighter_pikmin_link_event_weapon_pikmin_change_status")));

            let object_id = fighter.global_table[OBJECT_ID].get_u32();
            link_event["sender_id_"].assign(&L2CValue::U32(object_id));

            link_event_store_l2c_table(fighter, FIGHTER_PIKMIN_LINK_NO_PIKMIN_ATTACK.into(), link_event);

            let mut link_event = FighterPikminLinkEventWeaponPikminSyncLR__new_l2c_table();

            link_event["link_event_kind_"].assign(&L2CValue::Hash40(Hash40::new("fighter_pikmin_link_event_weapon_pikmin_sync_lr")));

            let lr = PostureModule::lr(fighter.module_accessor);
            link_event["lr_"].assign(&L2CValue::F32(lr));

            let object_id = fighter.global_table[OBJECT_ID].get_u32();
            link_event["sender_id_"].assign(&L2CValue::U32(object_id));

            link_event_store_l2c_table(fighter, FIGHTER_PIKMIN_LINK_NO_PIKMIN_ATTACK.into(), link_event);

            LinkModule::set_attribute(
                fighter.module_accessor,
                *FIGHTER_PIKMIN_LINK_NO_PIKMIN_ATTACK,
                LinkAttribute{ _address: *LINK_ATTRIBUTE_REFERENCE_PARENT_STOP as u8 },
                true
            );

            LinkModule::set_attribute(
                fighter.module_accessor,
                *FIGHTER_PIKMIN_LINK_NO_PIKMIN_ATTACK,
                LinkAttribute{ _address: *LINK_ATTRIBUTE_REFERENCE_PARENT_ATTACK_STOP as u8 },
                true
            );

            LinkModule::unlink(fighter.module_accessor, *FIGHTER_PIKMIN_LINK_NO_PIKMIN_ATTACK);
        }
    }

    fighter.status_AttackDash()
}

unsafe extern "C" fn attack_dash_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighta = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
    FighterSpecializer_Pikmin::update_hold_pikmin_param(fighta);
    let pikmin_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
    if pikmin_num > 0 {
        let pikmin_id_0 = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_OBJECT_ID_0) as u32;
        if LinkModule::link(fighter.module_accessor, *FIGHTER_PIKMIN_LINK_NO_PIKMIN_ATTACK, pikmin_id_0) != 0 {
            let mut link_event = FighterPikminLinkEventWeaponPikminChangeStatus__new_l2c_table();
    
            link_event["status_kind_"].assign(&L2CValue::I32(*WEAPON_PIKMIN_PIKMIN_STATUS_KIND_FALL));
    
            link_event["link_event_kind_"].assign(&L2CValue::Hash40(Hash40::new("fighter_pikmin_link_event_weapon_pikmin_change_status")));
    
            let object_id = fighter.global_table[OBJECT_ID].get_u32();
            link_event["sender_id_"].assign(&L2CValue::U32(object_id));
    
            link_event_store_l2c_table(fighter, FIGHTER_PIKMIN_LINK_NO_PIKMIN_ATTACK.into(), link_event);
    
            LinkModule::unlink(fighter.module_accessor, *FIGHTER_PIKMIN_LINK_NO_PIKMIN_ATTACK);
        }
        FighterSpecializer_Pikmin::reduce_pikmin_all(fighta);
    }

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_DASH, attack_dash_main);
    agent.status(Exit, *FIGHTER_STATUS_KIND_ATTACK_DASH, attack_dash_exit);
}