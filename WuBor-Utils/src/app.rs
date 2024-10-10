#![allow(non_snake_case)]

#[repr(C)]
pub struct CollisionLogScuffed {
    pub x00: *const u64,
    pub x08: *const u64,
    pub location: smash_rs::cpp::simd::Vector3,
    pub x20: u8,
    pub x21: u8,
    pub x22: u8,
    pub x23: u8,
    pub opponent_object_id: u32,
    pub opponent_object_category: u8,
    pub x29: u8,
    pub x2A: u8,
    pub x2B: u8,
    pub x2C: u8,
    pub x2D: u8,
    pub x2E: u8,
    pub collision_kind: u8,
    pub receiver_part_id: u8,
    pub collider_part_id: u8,
    pub receiver_id: u8,
    pub collider_id: u8,
}

// SHIELD BOXES

#[repr(C)]
pub struct ShieldDataResource {
    pub offset: smash_rs::cpp::simd::Vector3,
    pub offset2: smash_rs::cpp::simd::Vector3,
    pub size: f32,
    pub x24: u32,
    pub joint: smash::phx::Hash40,
    pub shape: u8,
    pub shield_type: u8,
}

impl ShieldDataResource {
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
        x2: f32,
        y2: f32,
        z2: f32,
        size: f32,
        joint: smash::phx::Hash40,
        shape: u8,
        shield_type: u8
    ) -> Self {
        ShieldDataResource {
            offset: smash_rs::cpp::simd::Vector3{vec: [x, y, z]},
            offset2: smash_rs::cpp::simd::Vector3{vec: [x2, y2, z2]},
            size: size,
            x24: 0,
            joint: joint,
            shape: shape,
            shield_type: shield_type
        }
    }
}

#[repr(C)]
pub struct ShieldDatas {
    pub datas: [ShieldDataResource; 8]
}

impl ShieldDatas {
    pub fn new() -> ShieldDatas {
        ShieldDatas { datas: [
            ShieldDataResource::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldDataResource::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldDataResource::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldDataResource::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldDataResource::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldDataResource::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldDataResource::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldDataResource::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0)
        ] }
    }
    pub fn add(mut self, shield_data: ShieldDataResource, index: usize) -> ShieldDatas {
        self.datas[index] = shield_data;
        self
    }
}

#[repr(C)]
pub struct ShieldGroupResource {
    pub shield_datas: *const ShieldDatas,
    pub count: u64,
    pub front: u8,
    pub hop: bool,
    pub turn: bool,
    pub no_hop: bool
}

impl ShieldGroupResource {
    pub fn new(
        shield_datas: *const ShieldDatas,
        count: u64,
        front: u8,
        hop: bool,
        turn: bool,
        no_hop: bool
    ) -> Self {
        ShieldGroupResource {
            shield_datas: shield_datas,
            count: count,
            front: front,
            hop: hop,
            turn: turn,
            no_hop: no_hop
        }
    }
}

// REFLECTOR BOXES

#[repr(C)]
pub struct ShieldData {
    pub offset: smash_rs::cpp::simd::Vector3,
    pub offset2: smash_rs::cpp::simd::Vector3,
    pub size: f32,
    pub x24: u32,
    pub joint: smash::phx::Hash40,
    pub shape: u8,
    pub shield_type: u8,
    pub x32: u16,
    pub x34: u32,
    pub x38: u64,
    pub status: i32,
    pub x44: u32,
    pub x48: u64,
}

impl ShieldData {
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
        x2: f32,
        y2: f32,
        z2: f32,
        size: f32,
        joint: smash::phx::Hash40,
        shape: u8,
        shield_type: u8
    ) -> Self {
        ShieldData {
            offset: smash_rs::cpp::simd::Vector3{vec: [x, y, z]},
            offset2: smash_rs::cpp::simd::Vector3{vec: [x2, y2, z2]},
            size: size,
            x24: 0,
            joint: joint,
            shape: shape,
            shield_type: shield_type,
            x32: 0,
            x34: 0,
            x38: 0,
            status: 0,
            x44: 0,
            x48: 0,
        }
    }
}

#[repr(C)]
pub struct ShieldDatas2 {
    pub datas: [ShieldData; 8]
}

impl ShieldDatas2 {
    pub fn new() -> ShieldDatas2 {
        ShieldDatas2 { datas: [
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0)
        ] }
    }
    pub fn add(mut self, shield_data: ShieldData, index: usize) -> ShieldDatas2 {
        self.datas[index] = shield_data;
        self
    }
}

#[repr(C)]
pub struct ShieldGroupResource2 {
    pub shield_datas: *const ShieldDatas2,
    pub count: u64,
    pub attack_mul: f32,
    pub speed_mul: f32,
    pub attack_limit: f32,
    pub life_mul: f32,
    pub no_m_ball: bool,
    pub front: u8
}

impl ShieldGroupResource2 {
    pub fn new(
        shield_datas: *const ShieldDatas2,
        count: u64,
        attack_mul: f32,
        speed_mul: f32,
        attack_limit: f32,
        life_mul: f32,
        no_m_ball: bool,
        front: u8
    ) -> Self {
        ShieldGroupResource2 {
            shield_datas: shield_datas,
            count: count,
            attack_mul: attack_mul,
            speed_mul: speed_mul,
            attack_limit: attack_limit,
            life_mul: life_mul,
            no_m_ball: no_m_ball,
            front: front
        }
    }
}

pub mod Cat4 {
    pub const SPECIAL_N_COMMAND : usize = 0x0;
    pub const SPECIAL_N2_COMMAND : usize = 0x1;
    pub const SPECIAL_S_COMMAND : usize = 0x2;
    pub const SPECIAL_HI_COMMAND : usize = 0x3;
    pub const COMMAND_6N6 : usize = 0x4;
    pub const COMMAND_4N4 : usize = 0x5;
    pub const ATTACK_COMMAND1 : usize = 0x6;
    pub const SPECIAL_HI2_COMMAND : usize = 0x7;
    pub const SUPER_SPECIAL_COMMAND : usize = 0x8;
    pub const SUPER_SPECIAL_R_COMMAND : usize = 0x9;
    pub const SUPER_SPECIAL2_COMMAND : usize = 0xA;
    pub const SUPER_SPECIAL2_R_COMMAND : usize = 0xB;
    pub const COMMAND_623NB : usize = 0xC;
    pub const COMMAND_623STRICT : usize = 0xD;
    pub const COMMAND_623ALONG : usize = 0xE;
    pub const COMMAND_623BLONG : usize = 0xF;
    pub const COMMAND_623A : usize = 0x10;
    pub const COMMAND_2 : usize = 0x11;
    pub const COMMAND_3 : usize = 0x12;
    pub const COMMAND_1 : usize = 0x13;
    pub const COMMAND_6 : usize = 0x14;
    pub const COMMAND_4 : usize = 0x15;
    pub const COMMAND_8 : usize = 0x16;
    pub const COMMAND_9 : usize = 0x17;
    pub const COMMAND_7 : usize = 0x18;
    pub const COMMAND_6N6AB : usize = 0x19;
    pub const COMMAND_323CATCH : usize = 0x1A;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CommandInputState {
    pub vtable: u64,
    pub command_timer: u8,
    pub state: u8,
    pub unk2: u8,
    pub input_allow: u8,
    pub max_timer: u8,
    pub enable_timer: u8,
    pub lr: i8,
}