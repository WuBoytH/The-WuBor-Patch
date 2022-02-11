#![allow(non_upper_case_globals)]

use {
    std::sync::Once,
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::Vector3f,
        app::*,
    },
    smashline::*
};

// System
pub static mut INT_OFFSET : usize = 0x4E19D0;
// pub static mut INT64_OFFSET : usize = 0x4E19F0;
pub static mut FLOAT_OFFSET : usize = 0x4E19D0;
pub static mut NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET : usize = 0x675A20;
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

// System Mechanics
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_GUARD_OFF_ATTACK_CANCEL : i32 = 0x20000116;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_N : i32 = 0x20000117;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S : i32 = 0x20000118;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI : i32 = 0x20000119;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_LW : i32 = 0x2000011A;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_AIR_ATTACK_WHIFF : i32 = 0x2000011B;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FUNNY : i32 = 0x2000011C;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC : i32 = 0x2000011D;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HITSTUN : i32 = 0x2000011E;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_BY_SPECIAL_HITSTUN : i32 = 0x2000011F;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_CANCEL_ESCAPE_TO_ESCAPE_FB : i32 = 0x200001A0;
pub static mut FGC_TRAINING : bool = false;

pub const FIGHTER_INSTANCE_WORK_ID_INT_GUARD_HOLD_FRAME : i32 = 0x100000ED;
pub const FIGHTER_INSTANCE_WORK_ID_INT_COUNTER_HIT_STATE : i32 = 0x100000EE;
pub const FIGHTER_INSTANCE_WORK_ID_INT_TARGET_ID : i32 = 0x100000EF;
pub const FIGHTER_INSTANCE_WORK_ID_INT_COMMAND_INPUT_TIMER : i32 = 0x100000F0;
pub const FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236_STEP : i32 = 0x100000F1;
pub const FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_214_STEP : i32 = 0x100000F2;
pub const FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_623_STEP : i32 = 0x100000F3;
pub const FIGHTER_INSTANCE_WORK_ID_INT_SUPER_COMMAND_INPUT_TIMER : i32 = 0x100000F4;
pub const FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236236_STEP : i32 = 0x100000F5;
pub const FIGHTER_INSTANCE_WORK_ID_INT_USED_GROUND_NORMALS : i32 = 0x100000F6;
pub const FIGHTER_INSTANCE_WORK_ID_INT_USED_AERIALS : i32 = 0x100000F7;

pub const FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_PREV : i32 = 0x5F;
pub const FIGHTER_INSTANCE_WORK_ID_FLOAT_FGC_HITSTUN_MUL : i32 = 0x60;

pub const FIGHTER_STATUS_GUARD_OFF_WORK_INT_ATTACK_CANCEL_FRAME : i32 = 0x11000010;

// pub const FIGHTER_STATUS_ESCAPE_AIR_FLAG_FORCE_SLIDE_F : i32 = 21000019;

pub const FIGHTER_STATUS_WORK_ID_FLOAT_CANCEL_TIMER : i32 = 0x1000026;

pub const FIGHTER_STATUS_WORK_ID_INT_ENABLED_AERIALS : i32 = 0x11000026;
// Set int like so: WorkModule::set_int(fighter.module_accessor, 0b11111, FIGHTER_STATUS_WORK_ID_INT_ENABLED_AERIALS);
// Bitmask is 0bXXXXX, where each X represents an aerial. 0b(LW)(HI)(B)(F)(N)

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

pub const FIGHTER_STATUS_WORK_ID_FLAG_JUMP_CANCEL : i32 = 0x2100002B;
// pub const FIGHTER_STATUS_WORK_ID_FLAG_DASH_CANCEL : i32 = 0x2100002C;
pub const FIGHTER_STATUS_WORK_ID_FLAG_NORMAL_CANCEL : i32 = 0x2100002D;
// pub const FIGHTER_STATUS_WORK_ID_FLAG_SPECIAL_CANCEL : i32 = 0x2100002E;

// Character Specific

// Mario
pub const FIGHTER_MARIO_STATUS_SPECIAL_N_FLAG_FGC_CANCEL : i32 = 0x2100000E;
pub const FIGHTER_MARIO_STATUS_SPECIAL_LW_FLAG_LANDING : i32 = 0x2100000B;
pub const FIGHTER_MARIO_STATUS_SPECIAL_LW_FLAG_BLJ : i32 = 0x2100000C;
pub const FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BLJ_PREV : i32 = 0x200000E4;
pub const FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_KIND : i32 = 0x100000C0;
pub const FIGHTER_MARIO_SPECIAL_LW_KIND_LONG_JUMP : i32 = 0;
pub const FIGHTER_MARIO_SPECIAL_LW_KIND_GROUND_POUND : i32 = 1;
pub const FIGHTER_MARIO_SPECIAL_LW_KIND_GROUND_POUND_CANCEL : i32 = 2;
pub const FIGHTER_MARIO_STATUS_SPECIAL_LW_INT_LONG_JUMP_KIND : i32 = 0x11000005;
pub const FIGHTER_MARIO_LONG_JUMP_W : i32 = 0;
pub const FIGHTER_MARIO_LONG_JUMP_M : i32 = 1;
pub const FIGHTER_MARIO_LONG_JUMP_S : i32 = 2;
pub const FIGHTER_MARIO_LONG_JUMP_B : i32 = 3;

// Donkey Kong
pub static mut DK_COUNT : u64 = 0;

// Samus

pub const FIGHTER_SAMUS_INSTANCE_WORK_ID_FLAG_BEAM_RAPID : i32 = 0x200000E8;

// pub const FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_ATTACK_HI_3_COMBO : i32 = 0x11000011;

// pub const FIGHTER_SAMUS_STATUS_ATTACK_HI_3_FLAG_CHECK_STEP : i32 = 0x21000021;
// pub const FIGHTER_SAMUS_STATUS_ATTACK_HI_3_FLAG_INC_STEP : i32 = 0x21000022;

pub const WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_ANGLE : i32 = 0xA;

// Dark Samus
pub const FIGHTER_SAMUSD_INSTANCE_WORK_ID_INT_CSHOT_ID : i32 = 0x100000C4;

// Luigi
pub const FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CANCEL : i32 = 0x200000E8;

// Kirby
pub const FIGHTER_KIRBY_STATUS_ATTACK_LW3_FLAG_BOUNCE : i32 = 0x2100000D;

// Dark Samus
pub const FIGHTER_SAMUSD_STATUS_SPECIAL_LW_FLAG_BOUNCE : i32 = 0x21000012;

// Falco
pub const FIGHTER_FALCO_INSTANCE_WORK_ID_FLAG_KAA : i32 = 0x200000E5;

// Ganondorf
pub const FIGHTER_GANON_STATUS_WORK_ID_INT_TELEPORT_STEP : i32 = 0x11000008;
pub const FIGHTER_GANON_TELEPORT_STEP_INIT : i32 = 1;
pub const FIGHTER_GANON_TELEPORT_STEP_MOVE : i32 = 2;
pub const FIGHTER_GANON_TELEPORT_STEP_CHECK_FEINT : i32 = 3;
pub const FIGHTER_GANON_TELEPORT_STEP_END : i32 = 4;
pub const FIGHTER_GANON_STATUS_WORK_ID_FLAG_TELEPORT_FEINT : i32 = 0x21000012;
pub const FIGHTER_GANON_STATUS_WORK_ID_FLAG_TELEPORT_STOP : i32 = 0x21000013;
pub const FIGHTER_GANON_STATUS_WORK_ID_FLOAT_TELEPORT_OG_POS_X : i32 = 0x1000008;
pub const FIGHTER_GANON_STATUS_WORK_ID_FLOAT_TELEPORT_OG_POS_Y : i32 = 0x1000009;
pub const FIGHTER_GANON_STATUS_WORK_ID_FLOAT_TELEPORT_TELE_POS_X : i32 = 0x100000A;
pub const FIGHTER_GANON_STATUS_WORK_ID_FLOAT_TELEPORT_TELE_POS_Y : i32 = 0x100000B;
pub const FIGHTER_GANON_STATUS_WORK_ID_FLAG_TELEPORT_START_GROUND : i32 = 0x21000014;

// Wario
pub const FIGHTER_WARIO_INSTANCE_WORK_ID_INT_FINISH_SIGN : i32 = 0x100000C4;

// Toon Link
pub const FIGHTER_TOONLINK_STATUS_ATTACK_AIR_LW_FLAG_BOUNCE : i32 = 0x21000017;
pub const FIGHTER_TOONLINK_STATUS_WORK_ID_FLOAT_SPECIAL_HI_SPIN_SPEED : i32 = 0x1000008;
pub const FIGHTER_TOONLINK_STATUS_WORK_ID_FLOAT_SPECIAL_HI_SPIN_SPEED_MAX : i32 = 0x1000009;

// Lucario
pub const FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_IS_SPIRIT_BOMB : i32 = 0x200000E5;
pub const FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_IS_SUPER_DASH_CANCEL : i32 = 0x200000E6;

// Shulk
pub const FIGHTER_SHULK_INSTANCE_WORK_ID_FLOAT_BURST_RECOVER : i32 = 0x53;
pub const FIGHTER_SHULK_INSTANCE_WORK_ID_FLOAT_BURST_COOLDOWN : i32 = 0x54;

// Ryu
pub const FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_DISABLE_EX_FOCUS : i32 = 0x200000EC;
pub const FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_EX_FOCUS : i32 = 0x200000ED;
pub const FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_EX_FLASH : i32 = 0x200000EE;
pub const FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SEC_SEN_STATE : i32 = 0x200000EF;
pub const FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SECRET_SENSATION : i32 = 0x200000F0;
pub const FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SEC_SEN_CAMERA : i32 = 0x200000F1;

pub const FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_RYU_X : i32 = 0x50;
pub const FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_RYU_Y : i32 = 0x51;
pub const FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_TARGET_X : i32 = 0x52;
pub const FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_TARGET_Y : i32 = 0x53;
pub const FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_DISABLE_EX_FOCUS_TIMER : i32 = 0x54;
pub const FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_SEC_SEN_TIMER : i32 = 0x55;
pub const FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_OPPONENT_DIREC : i32 = 0x56;
pub const FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_VERT_EXTRA : i32 = 0x57;

pub const FIGHTER_RYU_INSTANCE_WORK_ID_INT_FLASH_TIMER : i32 = 0x100000CA;

// Ken
pub const FIGHTER_KEN_INSTANCE_WORK_ID_INT_SPECIAL_LW_TYPE : i32 = 0x100000CA;
pub const FIGHTER_KEN_SPECIAL_LW_TYPE_QUICK_STEP : i32 = 0;
pub const FIGHTER_KEN_SPECIAL_LW_TYPE_HEAT_RUSH : i32 = 1;
pub const FIGHTER_KEN_INSTANCE_WORK_ID_INT_QUICK_STEP_STATE : i32 = 0x100000CB;
pub const FIGHTER_KEN_QUICK_STEP_STATE_ENABLE : i32 = 0;
pub const FIGHTER_KEN_QUICK_STEP_STATE_RUN : i32 = 1;
pub const FIGHTER_KEN_QUICK_STEP_STATE_DISABLE : i32 = 2;
pub const FIGHTER_KEN_INSTANCE_WORK_ID_INT_FLASH_MAX : i32 = 0x100000CC;
pub const FIGHTER_KEN_INSTANCE_WORK_ID_INT_FLASH_COUNTER : i32 = 0x100000CD;
pub const FIGHTER_KEN_INSTANCE_WORK_ID_INT_SHORYUREPPA : i32 = 0x100000CE;
pub const FIGHTER_KEN_INSTANCE_WORK_ID_INT_V_TRIGGER_EFF_TIMER : i32 = 0x100000CF;

pub const FIGHTER_KEN_STATUS_SPECIAL_LW_FLAG_STEP_KICK : i32 = 0x21000017;
pub const FIGHTER_KEN_STATUS_ATTACK_FLAG_VS1_CANCEL : i32 = 0x21000018;
pub const FIGHTER_KEN_STATUS_ATTACK_FLAG_VT1_CANCEL : i32 = 0x21000019;
pub const FIGHTER_KEN_STATUS_GUARD_FLAG_V_SHIFT : i32 = 0x2100001A;

pub const FIGHTER_KEN_INSTANCE_WORK_ID_FLAG_V_TRIGGER : i32 = 0x200000EC;

pub const FIGHTER_KEN_INSTANCE_WORK_ID_FLOAT_V_GAUGE : i32 = 0x50;
pub const FIGHTER_KEN_INSTANCE_WORK_ID_FLOAT_DIFF_X : i32 = 0x51;

// Corrin
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_DRAGON_INSTALL : i32 = 0x4C;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_DRAGON_INSTALL_TIMER : i32 = 0x4D;

// Incineroar
pub const FIGHTER_GAOGAEN_INSTANCE_WORK_ID_INT_REVENGE : i32 = 0x100000C6;

// Joker
pub const FIGHTER_JACK_STATUS_WORK_ID_FLAG_SPECIAL_S_FEINT : i32 = 0x21000010;

// Terry
pub const FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_D_TILT_CHAIN_COUNT : i32 = 0x100000CD;
pub const FIGHTER_DOLLY_STATUS_WORK_ID_FLAG_SPECIAL_N_FEINT : i32 = 0x21000010;

// Sephiroth
pub const FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CANCEL : i32 = 0x200000F0;

// Pyra
pub const FIGHTER_EFLAME_SPECIAL_S_FLAG_ROTATE : i32 = 0x2100000D;

// Mythra
pub const FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_S_CANCEL : i32 = 0x200000E8;

// Yu Narukami
pub const FIGHTER_YU_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_N_S : i32 = 0x200000E2;
pub const FIGHTER_YU_INSTANCE_WORK_ID_FLAG_AWAKENING : i32 = 0x200000E3;
pub const FIGHTER_YU_INSTANCE_WORK_ID_FLAG_SHADOW_FRENZY : i32 = 0x200000E4;
pub const FIGHTER_YU_INSTANCE_WORK_ID_FLAG_CAN_ONE_MORE : i32 = 0x200000E5;
pub const FIGHTER_YU_INSTANCE_WORK_ID_FLAG_ROMAN_ON_HIT : i32 = 0x200000E7;
pub const FIGHTER_YU_INSTANCE_WORK_ID_FLAG_HEROIC_GRAB : i32 = 0x200000E8;
pub const FIGHTER_YU_INSTANCE_WORK_ID_FLAG_COMMAND : i32 = 0x200000E9;
// pub const FIGHTER_YU_INSTANCE_WORK_ID_FLAG_TRAINING_TOOLS : i32 = 0x200000EA;

pub const FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE : i32 = 0x4C;
pub const FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SPENT_SP : i32 = 0x4D;
pub const FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE_MAX : i32 = 0x4E;
pub const FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAIN_PENALTY : i32 = 0x4F;

pub const FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_LEVEL : i32 = 0x100000C0;
pub const FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_EFFECT_TIMER : i32 = 0x100000C1;
pub const FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_FLASH_TIMER : i32 = 0x100000C2;
pub const FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_GLOW_TIMER : i32 = 0x100000C3;

pub const FIGHTER_YU_STATUS_FLAG_IS_EX : i32 = 0x21000015;

pub const FIGHTER_YU_STATUS_ATTACK_DASH_FLAG_BIG_GAMBLE : i32 = 0x2100000F;

pub const FIGHTER_YU_STATUS_SPECIAL_LW_WORK_ID_FLOAT_ROMAN_MOVE : i32 = 0x100000A;
pub const FIGHTER_YU_STATUS_SPECIAL_LW_FLAG_DECIDE_ROMAN_DIREC : i32 = 0x21000015;
pub const FIGHTER_YU_STATUS_SPECIAL_LW_FLAG_ROMAN_MOVE : i32 = 0x21000016;

pub const SP_1 : Vector3f = Vector3f{x: 0.0, y: 22.0, z: -6.0};
pub const SP_2 : Vector3f = Vector3f{x: 0.0, y: 22.0, z: -2.0};
pub const SP_3 : Vector3f = Vector3f{x: 0.0, y: 22.0, z: 2.0};
pub const SP_4 : Vector3f = Vector3f{x: 0.0, y: 22.0, z: 6.0};
pub const SP_5 : Vector3f = Vector3f{x: 0.0, y: 27.0, z: -2.0};
pub const SP_6 : Vector3f = Vector3f{x: 0.0, y: 27.0, z: 2.0};
pub static YU_AUDIO: [&'static str; 36] = ["appeal01", "appeal02", "attack01", "attack02", "attack03", "attack04", "attack05", "attack06", "attack07", "cliffcatch", "damage_twinkle", "damage01", "damage02", "damage03", "damagefly01", "damagefly02", "final", "furafura", "furasleep", "heavyget", "jump01", "missfoot01", "missfoot02", "ottotto", "passive", "special_h01", "special_l01", "special_l02", "special_n01", "swimup", "win01", "win02", "win03", "win_marth", "win_ike", "knockout"];
pub static YU_SEQ: [&'static str; 8] = ["attack", "special_n", "special_l", "special_h", "futtobi01", "futtobi02", "jump", "ottotto"];

// Chrom
pub const FIGHTER_CHROM_STATUS_SPECIAL_LW_FLAG_CHANGE_KINETIC : i32 = 0x21000015;

#[fighter_reset]
fn fighter_reset(_fighter: &mut L2CFighterCommon) {
    unsafe {
        DK_COUNT = 0;
        if !smashball::is_training_mode() {
            FGC_TRAINING = false;
        }
    }
}

pub mod singletons {
    // All credit for this to blujay, macros are very cool
    use super::*;
    use skyline::nn::ro::LookupSymbol;
    
    static INIT : Once = Once::new();

    pub static mut FIGHTER_CUTIN_MANAGER : *const *mut smash::app::FighterCutInManager = 0 as _;

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
        FighterCutInManager, FIGHTER_CUTIN_MANAGER
    );

    pub fn init() {
        INIT.call_once(|| {
            assign_symbol!(
                FIGHTER_CUTIN_MANAGER,
                "_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E\0"
            );
        });
    }
}

pub fn install() {
    singletons::init();
    install_agent_resets!(
        fighter_reset
    );
}
