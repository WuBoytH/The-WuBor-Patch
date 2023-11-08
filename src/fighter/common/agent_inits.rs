#![allow(dead_code)]

use {
    smash::{
        lua2cpp::L2CFighterCommon,
        lib::L2CValue
    },
    custom_var::*,
    wubor_utils::vars::*
};

// #[macro_export]
// macro_rules! dump_trace {
//     () => {{
//         let text = ::skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
//         dump_trace!(text)
//     }};
//     ($base:expr) => {{
//         const MAXIMUM_BT_LEN: usize = 0x20;
//         let text = $base;
//         println!("Current text address: {:#x}", text);
//         let mut lr: *const u64;
//         // unsafe {
//             asm!("mov {}, x30", out(reg) lr);
//         // }
//         let mut fp: *const u64;
//         // unsafe {
//             asm!("mov {}, x29", out(reg) fp);
//         // }
//         println!("Current LR:\t\t{:#x} ({:#x})", (lr as u64) - text, (lr as u64));
//         let mut counter = 0usize;
//         while !fp.is_null() && counter < MAXIMUM_BT_LEN {
//             lr = *fp.offset(1) as *const u64;
//             if !lr.is_null() {
//                 println!("[{}]: {:#x} ({:#x})", counter, (lr as u64), (lr as u64) - text);
//                 counter += 1;
//             }
//             fp = *fp as *const u64;
//         }
//     }}
// }

pub unsafe extern "C" fn specialn_pre_generic(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, fighter::instance::flag::DISABLE_SPECIAL_N) {
        return 0.into();
    }
    1.into()
}

pub unsafe extern "C" fn specials_pre_generic(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, fighter::instance::flag::DISABLE_SPECIAL_S) {
        return 0.into();
    }
    1.into()
}

pub unsafe extern "C" fn specialhi_pre_generic(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, fighter::instance::flag::DISABLE_SPECIAL_HI) {
        return 0.into();
    }
    1.into()
}

pub unsafe extern "C" fn speciallw_pre_generic(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, fighter::instance::flag::DISABLE_SPECIAL_LW) {
        return 0.into();
    }
    1.into()
}

// #[fighter_init]
// fn agent_init(fighter: &mut L2CFighterCommon) {
//     unsafe {
//         if !LISTENER_INSTALLED {
//             LISTENER_INSTALLED = true;
//             smash_rs::app::FighterManager::instance_mut().unwrap().add_global_event_listener(event_callback);
//         }
//     }
// }

// extern "C" fn event_callback(event: &smash_rs::app::FighterEvent, _: *mut u8) {
//     println!("Event triggered with ID: {}. Printing stack trace...", event.get_raw_event_id());
//     unsafe {
//         dump_trace!();
//     }
// }

// static mut LISTENER_INSTALLED : bool = false;

// #[fighter_reset]
// fn fighter_reset(_fighter: &mut L2CFighterCommon) {
//     unsafe {
//         LISTENER_INSTALLED = false;
//     }
// }

// pub fn install() {
//     install_agent_init_callbacks!(
//         agent_init
//     );
//     install_agent_resets!(
//         fighter_reset
//     );
// }
