use super::*;

extern "C" {
    #[link_name = "guard_cancel_attack_common"]
    fn guard_cancel_attack_common(fighter: &mut L2CFighterCommon);

    #[link_name = "guard_cancel_attack_main_loop_common"]
    fn guard_cancel_attack_main_loop_common(fighter: &mut L2CFighterCommon, is_crouch: L2CValue) -> L2CValue;
}

unsafe extern "C" fn guard_cancel_attack_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighta = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
    FighterSpecializer_Pikmin::hold_pikmin(fighta, 1);
    FighterSpecializer_Pikmin::update_hold_pikmin_param(fighta);
    let pikmin_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
    if pikmin_num > 0 {
        let pikmin_id_0 = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_OBJECT_ID_0) as u32;
        if LinkModule::link(fighter.module_accessor, *FIGHTER_PIKMIN_LINK_NO_PIKMIN_ATTACK, pikmin_id_0) != 0 {
            let mut link_event = FighterPikminLinkEventWeaponPikminChangeStatus__new_l2c_table();

            link_event["status_kind_"].assign(&L2CValue::I32(vars::pikmin_pikmin::status::GUARD_CANCEL_ATTACK_START));

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

            LinkModule::unlink(fighter.module_accessor, *FIGHTER_PIKMIN_LINK_NO_PIKMIN_ATTACK);
        }
    }
    0.into()
}

unsafe extern "C" fn guard_cancel_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    guard_cancel_attack_common(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(guard_cancel_attack_main_loop as *const () as _))
}

unsafe extern "C" fn guard_cancel_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    guard_cancel_attack_main_loop_common(fighter, false.into());
    guard_cancel_attack_handle_sync_pikmin(fighter);
    guard_cancel_attack_handle_shoot_pikmin(fighter);
    0.into()
}

unsafe extern "C" fn guard_cancel_attack_handle_sync_pikmin(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.module_accessor, vars::pikmin::status::flag::GUARD_CANCEL_ATTACK_DONE) {
        return;
    }

    let fighta = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
    FighterSpecializer_Pikmin::update_hold_pikmin_param(fighta);
    let pikmin_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);

    if pikmin_num <= 0 {
        return;
    }

    let pikmin_id_0 = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_OBJECT_ID_0) as u32;

    let pikmin_module_accessor_0 = sv_battle_object::module_accessor(pikmin_id_0);

    let lr = PostureModule::lr(fighter.module_accessor);
    let pikmin_lr = PostureModule::lr(pikmin_module_accessor_0);

    if lr * pikmin_lr > 0.0 {
        return;
    }

    if LinkModule::link(fighter.module_accessor, *FIGHTER_PIKMIN_LINK_NO_PIKMIN_ATTACK, pikmin_id_0) != 0 {
        let mut link_event = FighterPikminLinkEventWeaponPikminSyncLR__new_l2c_table();

        link_event["link_event_kind_"].assign(&L2CValue::Hash40(Hash40::new("fighter_pikmin_link_event_weapon_pikmin_sync_lr")));

        let lr = PostureModule::lr(fighter.module_accessor);
        link_event["lr_"].assign(&L2CValue::F32(lr));

        let object_id = fighter.global_table[OBJECT_ID].get_u32();
        link_event["sender_id_"].assign(&L2CValue::U32(object_id));

        link_event_store_l2c_table(fighter, FIGHTER_PIKMIN_LINK_NO_PIKMIN_ATTACK.into(), link_event);

        LinkModule::unlink(fighter.module_accessor, *FIGHTER_PIKMIN_LINK_NO_PIKMIN_ATTACK);
    }
}

unsafe extern "C" fn guard_cancel_attack_handle_shoot_pikmin(fighter: &mut L2CFighterCommon) {
    if !VarModule::is_flag(fighter.module_accessor, vars::pikmin::status::flag::GUARD_CANCEL_ATTACK_SHOOT) {
        return;
    }

    VarModule::off_flag(fighter.module_accessor, vars::pikmin::status::flag::GUARD_CANCEL_ATTACK_SHOOT);
    VarModule::on_flag(fighter.module_accessor, vars::pikmin::status::flag::GUARD_CANCEL_ATTACK_DONE);

    let fighta = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
    FighterSpecializer_Pikmin::update_hold_pikmin_param(fighta);
    let pikmin_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);

    if pikmin_num <= 0 {
        return;
    }

    let pikmin_id_0 = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_OBJECT_ID_0) as u32;

    let pikmin_module_accessor_0 = sv_battle_object::module_accessor(pikmin_id_0);
    let pikmin_status = StatusModule::status_kind(pikmin_module_accessor_0);

    let try_status_change = ![
            *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_DEATH,
            *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_DEATH_WAIT
        ].contains(&pikmin_status);

    if try_status_change {
        if LinkModule::link(fighter.module_accessor, *FIGHTER_PIKMIN_LINK_NO_PIKMIN_ATTACK, pikmin_id_0) != 0 {
            let mut link_event = FighterPikminLinkEventWeaponPikminSyncLR__new_l2c_table();

            link_event["link_event_kind_"].assign(&L2CValue::Hash40(Hash40::new("fighter_pikmin_link_event_weapon_pikmin_sync_lr")));

            let lr = PostureModule::lr(fighter.module_accessor);
            link_event["lr_"].assign(&L2CValue::F32(lr));

            let object_id = fighter.global_table[OBJECT_ID].get_u32();
            link_event["sender_id_"].assign(&L2CValue::U32(object_id));

            link_event_store_l2c_table(fighter, FIGHTER_PIKMIN_LINK_NO_PIKMIN_ATTACK.into(), link_event);


            let mut link_event = FighterPikminLinkEventWeaponPikminChangeStatus__new_l2c_table();

            link_event["status_kind_"].assign(&L2CValue::I32(vars::pikmin_pikmin::status::GUARD_CANCEL_ATTACK));

            link_event["link_event_kind_"].assign(&L2CValue::Hash40(Hash40::new("fighter_pikmin_link_event_weapon_pikmin_change_status")));

            let object_id = fighter.global_table[OBJECT_ID].get_u32();
            link_event["sender_id_"].assign(&L2CValue::U32(object_id));

            link_event_store_l2c_table(fighter, FIGHTER_PIKMIN_LINK_NO_PIKMIN_ATTACK.into(), link_event);

            LinkModule::unlink(fighter.module_accessor, *FIGHTER_PIKMIN_LINK_NO_PIKMIN_ATTACK);
        }
    }

    FighterSpecializer_Pikmin::reduce_pikmin_all(fighta);
}

unsafe extern "C" fn guard_cancel_attack_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    agent.status(Init, vars::fighter::status::GUARD_CANCEL_ATTACK, guard_cancel_attack_init);
    agent.status(Main, vars::fighter::status::GUARD_CANCEL_ATTACK, guard_cancel_attack_main);
    agent.status(Exit, vars::fighter::status::GUARD_CANCEL_ATTACK, guard_cancel_attack_exit);
}