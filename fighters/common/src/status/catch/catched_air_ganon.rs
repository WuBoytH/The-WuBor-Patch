use super::*;

#[skyline::hook(replace = L2CFighterCommon_sub_uniq_process_CatchedAirGanon_init)]
pub unsafe extern "C" fn sub_uniq_process_catchedairganon_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = LinkModule::get_parent_lr(fighter.module_accessor, *LINK_NO_CAPTURE);
    PostureModule::set_lr(fighter.module_accessor, lr);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    capture!(fighter, MA_MSC_CMD_CAPTURE_THROWN);

    LinkModule::set_attribute(
        fighter.module_accessor,
        *LINK_NO_CAPTURE,
        LinkAttribute{ _address: *LINK_ATTRIBUTE_REFERENCE_PARENT_COLOR_BLEND as u8 },
        false
    );

    if LinkModule::is_link(fighter.module_accessor, *LINK_NO_CAPTURE) {
        let offset_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("ganon_special_air_s_offset_x"));
        let offset_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("ganon_special_air_s_offset_y"));
        LinkModule::set_model_constraint_flag(
            fighter.module_accessor,
            (
                *CONSTRAINT_FLAG_ORIENTATION |
                *CONSTRAINT_FLAG_POSITION |
                *CONSTRAINT_FLAG_OFFSET_TRANSLATE
            ) as u32
        );
        LinkModule::set_constraint_translate_offset(
            fighter.module_accessor,
            &Vector3f{x: 0.0, y: offset_y, z: offset_x}
        );
    }

    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_CatchedAirGanon)]
pub unsafe extern "C" fn status_catchedairganon(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_share = WorkModule::get_param_int(fighter.module_accessor, 0xcad2ee25e_u64, 0xc07d88ea0_u64);
    if motion_share == *FIGHTER_MOTION_SHARE_TYPE_TARO {
        FighterMotionModuleImpl::add_body_type_hash(
            fighter.module_accessor,
            Hash40::new("catched_air_ganon"),
            *BODY_TYPE_MOTION_DX
        );
    }
    else if motion_share == *FIGHTER_MOTION_SHARE_TYPE_GIRL {
        FighterMotionModuleImpl::add_body_type_hash(
            fighter.module_accessor,
            Hash40::new("catched_air_ganon"),
            *BODY_TYPE_MOTION_GIRL
        );
    }
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("catched_air_ganon"),
        1.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_CatchedAirGanon_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_status_CatchedAirGanon_Main)]
pub unsafe extern "C" fn status_catchedairganon_main(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_uniq_process_CatchedAirGanon_exit)]
pub unsafe extern "C" fn sub_uniq_process_catchedairganon_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status != *FIGHTER_STATUS_KIND_THROWN {
        CaptureModule::thrown_cut(fighter.module_accessor, false, false);
    }
    0.into()
}

// #[skyline::hook(replace = L2CFighterCommon_status_CatchedAirEndGanon)]
// pub unsafe extern "C" fn status_catchedairendganon(fighter: &mut L2CFighterCommon) -> L2CValue {
//     let motion_share = WorkModule::get_param_int(fighter.module_accessor, 0xad2ee25eu64, 0x7d88ea0u64);
//     let throw_motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROWN_WORK_INT_MOTION_KIND);
//     if motion_share == *FIGHTER_MOTION_SHARE_TYPE_TARO {
//         FighterMotionModuleImpl::add_body_type_hash(
//             fighter.module_accessor,
//             Hash40::new_raw(throw_motion),
//             *BODY_TYPE_MOTION_DX
//         );
//     }
//     else if motion_share == *FIGHTER_MOTION_SHARE_TYPE_GIRL {
//         FighterMotionModuleImpl::add_body_type_hash(
//             fighter.module_accessor,
//             Hash40::new_raw(throw_motion),
//             *BODY_TYPE_MOTION_GIRL
//         );
//     }
//     MotionModule::change_motion(
//         fighter.module_accessor,
//         Hash40::new_raw(throw_motion),
//         1.0,
//         1.0,
//         false,
//         0.0,
//         false,
//         false
//     );
//     fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_CatchedAirEndGanon_Main as *const () as _))
// }

// #[skyline::hook(replace = L2CFighterCommon_status_CatchedAirEndGanon_Main)]
// pub unsafe extern "C" fn status_catchedairendganon_main(_fighter: &mut L2CFighterCommon) -> L2CValue {
//     0.into()
// }

// #[skyline::hook(replace = L2CFighterCommon_sub_uniq_process_CatchedAirEndGanon_exit)]
// pub unsafe extern "C" fn sub_uniq_process_catchedairendganon_exit(_fighter: &mut L2CFighterCommon) -> L2CValue {
//     0.into()
// }

#[skyline::hook(replace = L2CFighterCommon_status_end_CatchedAirEndGanon)]
pub unsafe extern "C" fn status_end_catchedairendganon(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_prev = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status == *FIGHTER_STATUS_KIND_DAMAGE_FALL {
        let flag = if status_prev == *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON
        || status_prev == *FIGHTER_STATUS_KIND_CATCHED_AIR_FALL_GANON {
            *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_AIR
        }
        else {
            *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_GROUND
        };
        WorkModule::on_flag(fighter.module_accessor, flag);
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_uniq_process_catchedairganon_init,
            status_catchedairganon,
            status_catchedairganon_main,
            sub_uniq_process_catchedairganon_exit,
            // status_catchedairendganon,
            // status_catchedairendganon_main,
            // sub_uniq_process_catchedairendganon_exit,
            // status_end_catchedairendganon
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}