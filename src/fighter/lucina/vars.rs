use smash::phx::Vector3f;

pub const FIGHTER_YU_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_N_S : i32 = 0x200000E1;
pub const FIGHTER_YU_INSTANCE_WORK_ID_FLAG_AWAKENING : i32 = 0x200000E2;
pub const FIGHTER_YU_INSTANCE_WORK_ID_FLAG_SHADOW_FRENZY : i32 = 0x200000E3;
pub const FIGHTER_YU_INSTANCE_WORK_ID_FLAG_ROMAN_ON_HIT : i32 = 0x200000E4;
pub const FIGHTER_YU_INSTANCE_WORK_ID_FLAG_HEROIC_GRAB : i32 = 0x200000E5;
pub const FIGHTER_YU_INSTANCE_WORK_ID_FLAG_COMMAND : i32 = 0x200000E6;
// pub const FIGHTER_YU_INSTANCE_WORK_ID_FLAG_TRAINING_TOOLS : i32 = 0x200000E7;

pub const FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE : i32 = 0x4C;
pub const FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SPENT_SP : i32 = 0x4D;
pub const FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE_MAX : i32 = 0x4E;
pub const FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAIN_PENALTY : i32 = 0x4F;

pub const FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_LEVEL : i32 = 0x100000BF;
pub const FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_EFFECT_TIMER : i32 = 0x100000C0;
pub const FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_FLASH_TIMER : i32 = 0x100000C1;
pub const FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_GLOW_TIMER : i32 = 0x100000C2;
pub const FIGHTER_YU_INSTANCE_WORK_ID_INT_SHADOW_EFF_ID : i32 = 0x100000C3;

pub const FIGHTER_YU_STATUS_FLAG_IS_EX : i32 = 0x21000015;
pub const FIGHTER_YU_STATUS_FLAG_CAN_ONE_MORE : i32 = 0x21000016;

pub const FIGHTER_YU_STATUS_ATTACK_DASH_FLAG_BIG_GAMBLE : i32 = 0x2100000F;

pub const FIGHTER_YU_STATUS_SPECIAL_LW_WORK_ID_FLOAT_ROMAN_MOVE : i32 = 0x1000009;
pub const FIGHTER_YU_STATUS_SPECIAL_LW_FLAG_DECIDE_ROMAN_DIREC : i32 = 0x21000014;
pub const FIGHTER_YU_STATUS_SPECIAL_LW_FLAG_ROMAN_MOVE : i32 = 0x21000015;

pub const SP_1 : Vector3f = Vector3f{x: 0.0, y: 22.0, z: -6.0};
pub const SP_2 : Vector3f = Vector3f{x: 0.0, y: 22.0, z: -2.0};
pub const SP_3 : Vector3f = Vector3f{x: 0.0, y: 22.0, z: 2.0};
pub const SP_4 : Vector3f = Vector3f{x: 0.0, y: 22.0, z: 6.0};
pub const SP_5 : Vector3f = Vector3f{x: 0.0, y: 27.0, z: -2.0};
pub const SP_6 : Vector3f = Vector3f{x: 0.0, y: 27.0, z: 2.0};
pub static YU_AUDIO: [&'static str; 36] = ["appeal01", "appeal02", "attack01", "attack02", "attack03", "attack04", "attack05", "attack06", "attack07", "cliffcatch", "damage_twinkle", "damage01", "damage02", "damage03", "damagefly01", "damagefly02", "final", "furafura", "furasleep", "heavyget", "jump01", "missfoot01", "missfoot02", "ottotto", "passive", "special_h01", "special_l01", "special_l02", "special_n01", "swimup", "win01", "win02", "win03", "win_marth", "win_ike", "knockout"];
pub static YU_SEQ: [&'static str; 8] = ["attack", "special_n", "special_l", "special_h", "futtobi01", "futtobi02", "jump", "ottotto"];
