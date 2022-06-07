#![allow(non_upper_case_globals)]

use {
    std::sync::Once,
    smash::{
        phx::Vector3f,
        app::*,
    }
};

// System
pub static mut INT_OFFSET : usize = 0x4E19D0;
// pub static mut INT64_OFFSET : usize = 0x4E19F0;
pub static mut FLOAT_OFFSET : usize = 0x4E19D0;
pub static mut NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET : usize = 0x675A20;
pub static mut DEFINE_LUA_CONSTANT_OFFSET : usize = 0x3727390; //13.0.1
pub static INT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x11, 0x40, 0xf9,
];
pub static FLOAT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x19, 0x40, 0xf9,
];
// pub static INT64_SEARCH_CODE: &[u8] = &[
//     0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x15, 0x40, 0xf9,
// ];
pub static NOTIFY_LOG_EVENT_COLLISION_HIT_SEARCH_CODE: &[u8] = &[
    0xff, 0x03, 0x03, 0xd1,
    0xe8, 0x2b, 0x00, 0xfd,
    0xfc, 0x6f, 0x06, 0xa9,
    0xfa, 0x67, 0x07, 0xa9,
    0xf8, 0x5f, 0x08, 0xa9,
    0xf6, 0x57, 0x09, 0xa9,
    0xf4, 0x4f, 0x0a, 0xa9,
    0xfd, 0x7b, 0x0b, 0xa9,
    0xfd, 0xc3, 0x02, 0x91,
    0xfb, 0x03, 0x00, 0xaa
];

// Common
pub const ZERO_VECTOR : Vector3f = Vector3f { x: 0.0, y: 0.0, z: 0.0 };
pub static mut FGC_TRAINING : bool = false;

pub const ATTACK_AIR_N_MASK : i32 = 0b00001;
pub const ATTACK_AIR_F_MASK : i32 = 0b00010;
pub const ATTACK_AIR_B_MASK : i32 = 0b00100;
pub const ATTACK_AIR_HI_MASK : i32 = 0b01000;
pub const ATTACK_AIR_LW_MASK : i32 = 0b10000;

pub const ATTACK_N_MASK : i32 = 0b0000001;
pub const ATTACK_S3_MASK : i32 = 0b0000010;
pub const ATTACK_HI3_MASK : i32 = 0b0000100;
pub const ATTACK_LW3_MASK : i32 = 0b0001000;
pub const ATTACK_S4_MASK : i32 = 0b0010000;
pub const ATTACK_HI4_MASK : i32 = 0b0100000;
pub const ATTACK_LW4_MASK : i32 = 0b1000000;

pub mod commons {
    pub mod instance {
        pub mod flag {
            pub const DISABLE_SPECIAL_N : i32 = 0x0;
            pub const DISABLE_SPECIAL_S : i32 = 0x1;
            pub const DISABLE_SPECIAL_HI : i32 = 0x2;
            pub const DISABLE_SPECIAL_LW : i32 = 0x3;
            pub const GUARD_OFF_ATTACK_CANCEL : i32 = 0x4;
            pub const IS_FGC : i32 = 0x5;
            pub const CANCEL_ESCAPE_TO_ESCAPE_FB : i32 = 0x6;
            pub const SUPER_JUMP : i32 = 0x7;
            pub const FORCE_ESCAPE_AIR_SLIDE : i32 = 0x8;
        }
        pub mod int {
            pub const TARGET_ID : i32 = 0x0;
            pub const USED_GROUND_NORMALS : i32 = 0x1;
            pub const USED_AERIALS : i32 = 0x2;
            // pub const CUSTOM_COMMAND_236_STEP : i32 = 0x3;
            // pub const CUSTOM_COMMAND_236_TIMER : i32 = 0x4;
            // pub const CUSTOM_COMMAND_214_STEP : i32 = 0x5;
            // pub const CUSTOM_COMMAND_214_TIMER : i32 = 0x6;
            // pub const CUSTOM_COMMAND_623_STEP : i32 = 0x7;
            // pub const CUSTOM_COMMAND_623_TIMER : i32 = 0x8;
            // pub const CUSTOM_COMMAND_236236_STEP : i32 = 0x9;
            // pub const CUSTOM_COMMAND_236236_TIMER : i32 = 0xA;
        }
        pub mod float {
            pub const FLICK_DOWN : i32 = 0x0;
            pub const SUPER_JUMP_FRAME : i32 = 0x1;
        }
    }
    pub mod status {
        pub mod flag {
            pub const JUMP_CANCEL : i32 = 0x2000;
            pub const NORMAL_CANCEL : i32 = 0x2001;
            // pub const DASH_CANCEL : i32 = 0x2002;
            // pub const SPECIAL_CANCEL : i32 = 0x2003;
        }
        pub mod int {
            pub const ENABLED_AERIALS : i32 = 0x2000;
        }
        pub mod float {
            pub const HIT_FRAME : i32 = 0x2000;
        }
    }
}

pub mod attack_air {
    pub mod flag {
        pub const WHIFF : i32 = 0x2100;
    }
}

pub mod guard_off {
    pub mod int {
        pub const ATTACK_CANCEL_FRAME : i32 = 0x2100;
    }
}

pub mod appeal {
    pub mod flag {
        pub const HOLD : i32 = 0x2100;
        pub const LOOP : i32 = 0x2101;
        pub const ENABLE_ACTION : i32 = 0x2102;
    }
    pub mod int {
        pub const HOLD_BUTTON : i32 = 0x2100;
        pub const ACTION_BUTTON : i32 = 0x2101;
        pub const RESTART_FRAME : i32 = 0x2102;
    }
    pub mod int64 {
        pub const ACTION_MOT : i32 = 0x2100;
        pub const LOOP_MOT : i32 = 0x2101;
    }
}

pub mod toonlink {
    pub mod attack_air_lw {
        pub mod flag {
            pub const BOUNCE : i32 = 0x2100;
        }
    }
    pub mod special_hi {
        pub mod float {
            pub const SPIN_SPEED : i32 = 0x2100;
            pub const SPIN_SPEED_MAX : i32 = 0x2101;
        }
    }
}

pub mod wario {
    pub mod throw {
        pub mod flag {
            pub const MOVE : i32 = 0x2100;
        }
        pub mod float {
            pub const SPIN_SPEED : i32 = 0x2100;
        }
    }
}

pub mod singletons {
    // All credit for this to blujay, macros are very cool
    use super::*;
    use skyline::nn::ro::LookupSymbol;
    
    static INIT : Once = Once::new();

    pub static mut FIGHTER_MANAGER : *const *mut smash::app::FighterManager = 0 as _;
    pub static mut FIGHTER_CUTIN_MANAGER : *const *mut smash::app::FighterCutInManager = 0 as _;

    extern "C" {
        #[link_name = "\u{1}_ZN3lib9SingletonIN3app16BattleObjectSlowEE9instance_E"]
        pub static mut BATTLE_OBJECT_SLOW: *mut u8;
    }
    
    macro_rules! expose_singleton {
        ($($public:ident, $private:ident)*) => {
            $(
                #[inline(always)]
                #[allow(non_snake_case)]
                pub fn $public() -> *mut $public {
                    unsafe {
                        *$private
                    }
                }
            )*
        }
    }

    macro_rules! assign_symbol {
        ($id:ident, $e:expr) => {{
            unsafe {
                let mut sym = 0usize;
                LookupSymbol(&mut sym as *mut usize, $e.as_ptr() as _);
                assert!(sym != 0, "Failed to find symbol {}", $e);
                $id = std::mem::transmute(sym)
            }
        }}
    }

    expose_singleton!(
        FighterManager, FIGHTER_MANAGER
        FighterCutInManager, FIGHTER_CUTIN_MANAGER
    );

    pub fn init() {
        INIT.call_once(|| {
            assign_symbol!(
                FIGHTER_MANAGER,
                "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\0"
            );
            assign_symbol!(
                FIGHTER_CUTIN_MANAGER,
                "_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E\0"
            );
        });
    }
}

pub fn install() {
    singletons::init();
}
