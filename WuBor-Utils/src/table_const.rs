#![allow(dead_code)]
// Huge shoutouts to Blujay for helping me with this

pub const KIND:                            i32 = 0x2;
pub const OBJECT_ID:                       i32 = 0x3;
pub const FIGHTER:                         i32 = 0x4;
pub const MODULE_ACCESSOR:                 i32 = 0x5;
 
pub const INIT_STATUS_FUNC:                i32 = 0x7;
pub const IS_STOP:                         i32 = 0x8;
pub const STATUS_KIND_INTERRUPT:           i32 = 0x9;
pub const PREV_STATUS_KIND:                i32 = 0xA;
pub const STATUS_KIND:                     i32 = 0xB;
pub const STATUS_COUNT:                    i32 = 0xC;

pub const STATUS_FRAME:                    i32 = 0xE;
pub const STATUS_FRAME_NO_INTERP:          i32 = 0xF;

pub const SUB_STATUS3:                     i32 = 0x13;
pub const SUB_STATUS2:                     i32 = 0x14;
pub const SUB_STATUS:                      i32 = 0x15;
pub const SITUATION_KIND:                  i32 = 0x16;
pub const PREV_SITUATION_KIND:             i32 = 0x17;
pub const PREV_STATUS_FRAME:               i32 = 0x18;

pub const STICK_X:                         i32 = 0x1A;
pub const STICK_Y:                         i32 = 0x1B;
pub const FLICK_X:                         i32 = 0x1C;
pub const FLICK_Y:                         i32 = 0x1D;
pub const FLICK_Y_DIR:                     i32 = 0x1E;
pub const PAD_FLAG:                        i32 = 0x1F;
pub const CMD_CAT1:                        i32 = 0x20;
pub const CMD_CAT2:                        i32 = 0x21;
pub const CMD_CAT3:                        i32 = 0x22;
pub const CMD_CAT4:                        i32 = 0x23;

pub const CHECK_AIR_SPECIAL_UNIQ:          i32 = 0x26;
pub const CHECK_GROUND_SPECIAL_UNIQ:       i32 = 0x27;
pub const CHECK_GROUND_ATTACK_UNIQ:        i32 = 0x28;
pub const DASH_COMMON_UNIQ:                i32 = 0x29;
pub const RUN_MAIN_UNIQ:                   i32 = 0x2A;
pub const JUMP_SQUAT_MAIN_UNIQ:            i32 = 0x2B;
pub const CHECK_AIR_LANDING_UNIQ:          i32 = 0x2C;
pub const CHECK_AIR_ITEM_THROW_UNIQ:       i32 = 0x2D;
pub const CHECK_AIR_ATTACK_UNIQ:           i32 = 0x2E;
pub const CHECK_AIR_ESCAPE_UNIQ:           i32 = 0x2F;
pub const CHECK_AIR_TREAD_JUMP_UNIQ:       i32 = 0x30;
pub const CHECK_AIR_WALL_JUMP_UNIQ:        i32 = 0x31;
pub const CHECK_AIR_JUMP_UNIQ:             i32 = 0x32;
pub const CHECK_AIR_JUMP_AERIAL_UNIQ:      i32 = 0x33;
pub const GUARD_CONT_UNIQ:                 i32 = 0x34;
pub const TURN_UNIQ:                       i32 = 0x35;
pub const CHECK_AIR_CLIFF_LASSO_UNIQ:      i32 = 0x36;
pub const LANDING_UNIQ_CHECK_STRANS_UNIQ:  i32 = 0x37;
pub const CHECK_SPECIAL_N_UNIQ:            i32 = 0x38;
pub const CHECK_SPECIAL_S_UNIQ:            i32 = 0x39;
pub const CHECK_SPECIAL_HI_UNIQ:           i32 = 0x3A;
pub const CHECK_SPECIAL_LW_UNIQ:           i32 = 0x3B;
pub const CHECK_SPECIAL_COMMAND:           i32 = 0x3C;
pub const WAZA_CUSTOMIZE_CONTROL:          i32 = 0x3D;
pub const STATUS_END_CONTROL:              i32 = 0x3E;

pub const DAMAGE_MOTION_KIND_CALLBACK:     i32 = 0x42;
pub const SUB_UNIQ_DAMAGE_FLY_UNIQ:        i32 = 0x43;
pub const DOWN_DAMAGE_UNIQ:                i32 = 0x44;
pub const THROW_F_STATUS_KIND:             i32 = 0x45;
pub const THROW_B_STATUS_KIND:             i32 = 0x46;
pub const THROW_HI_STATUS_KIND:            i32 = 0x47;
pub const THROW_LW_STATUS_KIND:            i32 = 0x48;
pub const DAMAGE_STOP_MOTION_INTP_FRAME:   i32 = 0x49;
pub const SUB_REBIRTH_UNIQ_INIT_CORE_UNIQ: i32 = 0x4A;
pub const SUB_REBIRTH_UNIQ_EXEC_UNIQ:      i32 = 0x4B;
pub const SUB_DEAD_UNIQ_INIT_UNIQ:         i32 = 0x4C;
pub const SUB_ROULETTE_SET_SETP_UNIQ:      i32 = 0x4D;
pub const FALL_BRAKE_UNIQ:                 i32 = 0x4E;
pub const CHECK_GROUND_GUARD_UNIQ:         i32 = 0x4F;
pub const CHECK_GROUND_CATCH_UNIQ:         i32 = 0x50;
pub const CHECK_COMMAND_WALK_UNIQ:         i32 = 0x51;
pub const CHECK_GROUND_JUMP_MINI_ATTACK:   i32 = 0x52;
pub const CHECK_AIR_ITEM_THROW_POST:       i32 = 0x53;
pub const IS_ITEM_SHOOT_STATUS_UNIQ:       i32 = 0x54;
pub const CHECK_ATTACK_3_UNIQ:             i32 = 0x55;
pub const CHECK_ATTACK_N_UNIQ:             i32 = 0x56;
pub const CHECK_ATTACK_S4_UNIQ:            i32 = 0x57;
pub const CHECK_ATTACK_HI4_UNIQ:           i32 = 0x58;
pub const CHECK_ATTACK_LW4_UNIQ:           i32 = 0x59;
pub const SQUAT_COMMON_UNIQ:               i32 = 0x5A;