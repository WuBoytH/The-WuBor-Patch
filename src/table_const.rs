#![allow(dead_code)]
// Huge shoutouts to Blujay for helping me with this

pub const FIGHTER_KIND:                  i32 = 0x2;
pub const OBJECT_ID:                     i32 = 0x3;

pub const MODULE_ACCESSOR:               i32 = 0x5;

pub const INIT_STATUS_FUNC:              i32 = 0x7;
pub const IS_STOPPING:                   i32 = 0x8;
pub const STATUS_KIND_INTERRUPT:         i32 = 0x9;
pub const PREV_STATUS_KIND:              i32 = 0xA;
pub const STATUS_KIND:                   i32 = 0xB;
pub const STATUS_COUNT:                  i32 = 0xC;

pub const CURRENT_FRAME:                 i32 = 0xE;
pub const CURRENT_FRAME2:                i32 = 0xF;

pub const SUB_STATUS3:                   i32 = 0x13;
pub const SUB_STATUS2:                   i32 = 0x14;
pub const SUB_STATUS:                    i32 = 0x15;
pub const SITUATION_KIND:                i32 = 0x16;
pub const PREV_SITUATION_KIND:           i32 = 0x17;
pub const PREV_STATUS_FRAME:             i32 = 0x18;

pub const STICK_X:                       i32 = 0x1A;
pub const STICK_Y:                       i32 = 0x1B;
pub const FLICK_X:                       i32 = 0x1C;
pub const FLICK_Y:                       i32 = 0x1D;
pub const FLICK_Y_DIR:                   i32 = 0x1E;
pub const PAD_FLAG:                      i32 = 0x1F;
pub const CMD_CAT1:                      i32 = 0x20;
pub const CMD_CAT2:                      i32 = 0x21;
pub const CMD_CAT3:                      i32 = 0x22;
pub const CMD_CAT4:                      i32 = 0x23;

pub const CHECK_AIR_SPECIAL_PRE:         i32 = 0x26;
pub const CHECK_GROUND_SPECIAL_PRE:      i32 = 0x27;
pub const CHECK_GROUND_ATTACK_PRE:       i32 = 0x28;
pub const CUSTOM_ROUTINE:                i32 = 0x2B;

pub const CHECK_AIR_ITEM_THROW_PRE:      i32 = 0x2D;
pub const CHECK_AIR_ATTACK_PRE:          i32 = 0x2E;
pub const CHECK_AIR_ESCAPE_PRE:          i32 = 0x2F;
pub const CHECK_AIR_TREAD_JUMP_PRE:      i32 = 0x30;
pub const CHECK_AIR_WALL_JUMP_PRE:       i32 = 0x31;
pub const CHECK_AIR_JUMP_PRE:            i32 = 0x32;
pub const CHECK_AIR_JUMP_AERIAL_POST:    i32 = 0x33;

pub const CHECK_AIR_CLIFF_AIR_LASSO_PRE: i32 = 0x36;

pub const SPECIAL_N_RESTRICT:            i32 = 0x38;
pub const SPECIAL_S_RESTRICT:            i32 = 0x39;
pub const SPECIAL_HI_RESTRICT:           i32 = 0x3A;
pub const SPECIAL_LW_RESTRICT:           i32 = 0x3B;
pub const CHECK_SPECIAL_COMMAND:         i32 = 0x3C;

pub const CHECK_GROUND_GUARD_PRE:        i32 = 0x4F;
pub const CHECK_GROUND_CATCH_PRE:        i32 = 0x50;

pub const CHECK_GROUND_JUMP_MIN_ATTACK:  i32 = 0x52;
pub const CHECK_AIR_ITEM_THROW_POST:     i32 = 0x53;

pub const ATTACK_3_RESTRICT:             i32 = 0x55;
pub const ATTACK_N_RESTRICT:             i32 = 0x56;
pub const ATTACK_S4_RESTRICT:            i32 = 0x57;
pub const ATTACK_HI4_RESTRICT:           i32 = 0x58;
pub const ATTACK_LW4_RESTRICT:           i32 = 0x59;