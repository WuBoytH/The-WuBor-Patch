use super::*;

unsafe extern "C" fn appeal_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Appeal_common_uniq(L2CValue::Void());
    let fighta = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
    FighterSpecializer_Pikmin::hold_pikmin(fighta, 3);
    FighterSpecializer_Pikmin::update_hold_pikmin_param(fighta);

    fighter.sub_shift_status_main(L2CValue::Ptr(appeal_main_loop as *const () as _))
}

unsafe extern "C" fn appeal_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, vars::pikmin::status::flag::APPEAL_CALL_PICKIE) {
        appeal_call_pickie(fighter);
        VarModule::inc_int(fighter.module_accessor, vars::pikmin::status::int::APPEAL_PIKMIN_COUNT);
        VarModule::off_flag(fighter.module_accessor, vars::pikmin::status::flag::APPEAL_CALL_PICKIE);
    }

    fighter.status_Appeal_Main()
}

unsafe extern "C" fn appeal_call_pickie(fighter: &mut L2CFighterCommon) {
    let fighta = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
    FighterSpecializer_Pikmin::update_hold_pikmin_param(fighta);
    let appeal_pikmin_count = VarModule::get_int(fighter.module_accessor, vars::pikmin::status::int::APPEAL_PIKMIN_COUNT);
    let pikmin_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
    if pikmin_num > 0 {
        let pikmin_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_OBJECT_ID_0 + appeal_pikmin_count) as u32;
        if LinkModule::link(fighter.module_accessor, *FIGHTER_PIKMIN_LINK_NO_PIKMIN, pikmin_id) != 0 {
            let mut link_event = FighterPikminLinkEventWeaponPikminChangeMotion__new_l2c_table();

            link_event["link_event_kind_"].assign(&L2CValue::Hash40(Hash40::new("fighter_pikmin_link_event_weapon_pikmin_change_motion")));

            let motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_APPEAL_WORK_INT_MOTION_KIND_R);
            let rand = sv_math::rand(hash40("fighter"), 10);
            let motion = if rand == 0 {
                let mut new_motion = motion;
                while new_motion == motion {
                    match sv_math::rand(hash40("fighter"), 3) {
                        0 => new_motion = hash40("appeal_hi_r"),
                        1 => new_motion = hash40("appeal_s_r"),
                        2 => new_motion = hash40("appeal_lw_r"),
                        _ => unreachable!()
                    }
                }
                new_motion
            }
            else {
                motion
            };
            link_event["motion_kind_"].assign(&L2CValue::Hash40(Hash40::new_raw(motion)));

            link_event["start_frame_"].assign(&L2CValue::F32(0.0));

            link_event["rate_"].assign(&L2CValue::F32(1.0));

            link_event["loop_"].assign(&L2CValue::Bool(false));

            let object_id = fighter.global_table[OBJECT_ID].get_u32();
            link_event["sender_id_"].assign(&L2CValue::U32(object_id));

            link_event_store_l2c_table(fighter, FIGHTER_PIKMIN_LINK_NO_PIKMIN.into(), link_event);

            let mut link_event = FighterPikminLinkEventWeaponPikminChangeStatus__new_l2c_table();

            link_event["status_kind_"].assign(&L2CValue::I32(vars::pikmin_pikmin::status::APPEAL));

            link_event["link_event_kind_"].assign(&L2CValue::Hash40(Hash40::new("fighter_pikmin_link_event_weapon_pikmin_change_status")));

            let object_id = fighter.global_table[OBJECT_ID].get_u32();
            link_event["sender_id_"].assign(&L2CValue::U32(object_id));

            link_event_store_l2c_table(fighter, FIGHTER_PIKMIN_LINK_NO_PIKMIN.into(), link_event);

            // let mut link_event = FighterPikminLinkEventWeaponPikminSyncLR__new_l2c_table();

            // link_event["link_event_kind_"].assign(&L2CValue::Hash40(Hash40::new("fighter_pikmin_link_event_weapon_pikmin_sync_lr")));

            // let lr = PostureModule::lr(fighter.module_accessor);
            // link_event["lr_"].assign(&L2CValue::F32(lr));

            // let object_id = fighter.global_table[OBJECT_ID].get_u32();
            // link_event["sender_id_"].assign(&L2CValue::U32(object_id));

            // link_event_store_l2c_table(fighter, FIGHTER_PIKMIN_LINK_NO_PIKMIN.into(), link_event);

            LinkModule::unlink(fighter.module_accessor, *FIGHTER_PIKMIN_LINK_NO_PIKMIN);
        }
    }
}

unsafe extern "C" fn appeal_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighta = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
    FighterSpecializer_Pikmin::update_hold_pikmin_param(fighta);
    let pikmin_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
    if pikmin_num > 0 {
        // let pikmin_id_0 = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_OBJECT_ID_0) as u32;
        // if LinkModule::link(fighter.module_accessor, *FIGHTER_PIKMIN_LINK_NO_PIKMIN_ATTACK, pikmin_id_0) != 0 {
        //     let mut link_event = FighterPikminLinkEventWeaponPikminChangeStatus__new_l2c_table();
    
        //     link_event["status_kind_"].assign(&L2CValue::I32(*WEAPON_PIKMIN_PIKMIN_STATUS_KIND_FALL));
    
        //     link_event["link_event_kind_"].assign(&L2CValue::Hash40(Hash40::new("fighter_pikmin_link_event_weapon_pikmin_change_status")));
    
        //     let object_id = fighter.global_table[OBJECT_ID].get_u32();
        //     link_event["sender_id_"].assign(&L2CValue::U32(object_id));
    
        //     link_event_store_l2c_table(fighter, FIGHTER_PIKMIN_LINK_NO_PIKMIN_ATTACK.into(), link_event);
    
        //     LinkModule::unlink(fighter.module_accessor, *FIGHTER_PIKMIN_LINK_NO_PIKMIN_ATTACK);
        // }
        FighterSpecializer_Pikmin::reduce_pikmin_all(fighta);
    }

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_APPEAL, appeal_main);
    agent.status(Exit, *FIGHTER_STATUS_KIND_APPEAL, appeal_exit);
}