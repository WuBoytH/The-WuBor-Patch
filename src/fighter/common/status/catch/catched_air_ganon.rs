use crate::imports::status_imports::*;

#[skyline::hook(replace = L2CFighterCommon_status_CatchedAirGanon)]
pub unsafe fn status_catchedairganon(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_share = WorkModule::get_param_int(fighter.module_accessor, 0xad2ee25eu64, 0x7d88ea0u64);
    if motion_share == *FIGHTER_MOTION_SHARE_TYPE_TARO {
        FighterMotionModuleImpl::add_body_type_hash(
            fighter.module_accessor,
            Hash40::new("catched_ganon"),
            *BODY_TYPE_MOTION_DX
        );
    }
    else if motion_share == *FIGHTER_MOTION_SHARE_TYPE_GIRL {
        FighterMotionModuleImpl::add_body_type_hash(
            fighter.module_accessor,
            Hash40::new("catched_ganon"),
            *BODY_TYPE_MOTION_GIRL
        );
    }
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("catched_ganon"),
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
pub unsafe fn status_catchedairganon_main(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_CatchedAirEndGanon)]
pub unsafe fn status_catchedairendganon(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_share = WorkModule::get_param_int(fighter.module_accessor, 0xad2ee25eu64, 0x7d88ea0u64);
    let throw_motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROWN_WORK_INT_MOTION_KIND);
    if motion_share == *FIGHTER_MOTION_SHARE_TYPE_TARO {
        FighterMotionModuleImpl::add_body_type_hash(
            fighter.module_accessor,
            Hash40::new_raw(throw_motion),
            *BODY_TYPE_MOTION_DX
        );
    }
    else if motion_share == *FIGHTER_MOTION_SHARE_TYPE_GIRL {
        FighterMotionModuleImpl::add_body_type_hash(
            fighter.module_accessor,
            Hash40::new_raw(throw_motion),
            *BODY_TYPE_MOTION_GIRL
        );
    }
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new_raw(throw_motion),
        1.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_CatchedAirEndGanon_Main as *const () as _))
}

// #[skyline::hook(replace = L2CFighterCommon_status_CatchedAirEndGanon_Main)]
// pub unsafe fn status_catchedairendganon_main(_fighter: &mut L2CFighterCommon) -> L2CValue {
//     0.into()
// }

// #[skyline::hook(replace = L2CFighterCommon_sub_uniq_process_CatchedAirEndGanon_exit)]
// pub unsafe fn sub_uniq_process_catchedairendganon_exit(_fighter: &mut L2CFighterCommon) -> L2CValue {
//     0.into()
// }

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_catchedairganon,
            status_catchedairganon_main,
            status_catchedairendganon,
            // status_catchedairendganon_main,
            // sub_uniq_process_catchedairendganon_exit
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}