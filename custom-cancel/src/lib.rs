#![allow(unused_mut)]
#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;

use {
    std::collections::HashMap,
    parking_lot::RwLock,
    bitflags::bitflags,
    smash::{
        lua2cpp::*,
        phx::*,
        app::lua_bind::*,
        lib::lua_const::*
    },
    custom_var::*,
    wubor_utils::{wua_bind::*, cancels::*, vars::*, table_const::*}
};

pub type CancelFunc = unsafe extern "C" fn(&mut L2CFighterCommon) -> bool;

bitflags! {
    pub struct CancelType: i32 {
        const NONE  = 0b000;
        const HIT   = 0b001;
        const BLOCK = 0b010;
        const WHIFF = 0b100;
    }
    pub struct DashCancelDir: i32 {
        const NONE     = 0b00;
        const FORWARD  = 0b01;
        const BACKWARD = 0b10;
    }
    pub struct FGCFlags: i32 {
        const NONE    = 0b000000;
        const NORMAL  = 0b000001;
        const AERIAL  = 0b000010;
        const SPECIAL = 0b000100;
        const JUMP    = 0b001000;
        const DASH    = 0b010000;
        const AIRDASH = 0b100000;
        const ALL     = 0b111111;
    }
}

pub struct CancelInfo {
    pub pre: Option<CancelFunc>,
    pub normals: Vec<i32>,
    pub normal_cancel: CancelType,
    pub normal_cancel_require_flag: bool,
    pub specials: Vec<i32>,
    pub special_cancel: CancelType,
    pub special_cancel_require_flag: bool,
    pub jump_cancel: CancelType,
    pub jump_cancel_require_flag: bool,
    pub aerial_cancel: CancelType,
    pub aerial_cancel_require_flag: bool,
    pub dash_cancel: CancelType,
    pub dash_cancel_require_flag: bool,
    pub dash_cancel_direction: DashCancelDir,
    pub airdash_cancel: CancelType,
    pub airdash_cancel_require_flag: bool,
    pub fgc_flags: FGCFlags,
    pub alt_info: Option<AltInfo>,
    pub exception: Option<CancelFunc>
}

impl CancelInfo {
    pub fn new() -> CancelInfo {
        CancelInfo {
            pre: None,
            normals: vec![0; 0],
            normal_cancel: CancelType::HIT | CancelType::BLOCK,
            normal_cancel_require_flag: false,
            specials: [
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL
            ].to_vec(),
            special_cancel: CancelType::HIT | CancelType::BLOCK,
            special_cancel_require_flag: false,
            jump_cancel: CancelType::NONE,
            jump_cancel_require_flag: false,
            aerial_cancel: CancelType::NONE,
            aerial_cancel_require_flag: false,
            dash_cancel: CancelType::NONE,
            dash_cancel_require_flag: false,
            dash_cancel_direction: DashCancelDir::FORWARD,
            airdash_cancel: CancelType::NONE,
            airdash_cancel_require_flag: false,
            fgc_flags: FGCFlags::ALL,
            alt_info: None,
            exception: None
        }
    }

    pub fn pre_function(mut self, pre: CancelFunc) -> Self {
        self.pre = Some(pre);
        self
    }

    pub fn enable_normals(mut self, normals: Vec<i32>) -> Self {
        self.normals = normals;
        self
    }

    pub fn normal_cancel_require_flag(mut self) -> Self {
        self.normal_cancel_require_flag = true;
        self
    }

    pub fn enable_specials(mut self, specials: Vec<i32>) -> Self {
        self.specials = specials;
        self
    }

    pub fn special_cancel_require_flag(mut self) -> Self {
        self.special_cancel_require_flag = true;
        self
    }

    pub fn enable_jump_cancel(mut self, jump_cancel: CancelType) -> Self {
        self.jump_cancel = jump_cancel;
        self
    }

    pub fn jump_cancel_require_flag(mut self) -> Self {
        self.jump_cancel_require_flag = true;
        self
    }

    pub fn enable_aerials(mut self, aerial_cancel: CancelType) -> Self {
        self.aerial_cancel = aerial_cancel;
        self
    }

    pub fn aerial_cancel_require_flag(mut self) -> Self {
        self.aerial_cancel_require_flag = true;
        self
    }

    pub fn enable_dash_cancel(mut self, dash_cancel: CancelType) -> Self {
        self.dash_cancel = dash_cancel;
        self
    }

    pub fn dash_cancel_require_flag(mut self) -> Self {
        self.dash_cancel_require_flag = true;
        self
    }

    pub fn set_dash_cancel_direction(mut self, dash_cancel_direction: DashCancelDir) -> Self {
        self.dash_cancel_direction = dash_cancel_direction;
        self
    }

    pub fn enable_airdash_cancel(mut self, airdash_cancel: CancelType) -> Self {
        self.airdash_cancel = airdash_cancel;
        self
    }

    pub fn airdash_cancel_require_flag(mut self) -> Self {
        self.airdash_cancel_require_flag = true;
        self
    }

    pub fn set_fgc_flags(mut self, fgc_flags: FGCFlags) -> Self {
        self.fgc_flags = fgc_flags;
        self
    }

    pub fn set_alt_info(mut self, alt_info: AltInfo) -> Self {
        self.alt_info = Some(alt_info);
        self
    }

    pub fn exception_function(mut self, exception: CancelFunc) -> Self {
        self.exception = Some(exception);
        self
    }

}

pub struct AltInfo {
    pub flag: i32,
    pub normals: Vec<i32>,
    pub normal_cancel: CancelType,
    pub normal_cancel_require_flag: bool,
    pub specials: Vec<i32>,
    pub special_cancel: CancelType,
    pub special_cancel_require_flag: bool,
    pub jump_cancel: CancelType,
    pub jump_cancel_require_flag: bool,
    pub aerial_cancel: CancelType,
    pub aerial_cancel_require_flag: bool,
    pub dash_cancel: CancelType,
    pub dash_cancel_require_flag: bool,
    pub dash_cancel_direction: DashCancelDir,
    pub airdash_cancel: CancelType,
    pub airdash_cancel_require_flag: bool,
    pub fgc_flags: FGCFlags,
}

impl AltInfo {
    pub fn new() -> AltInfo {
        AltInfo {
            flag: 0x0000,
            normals: vec![0; 0],
            normal_cancel: CancelType::HIT | CancelType::BLOCK,
            normal_cancel_require_flag: false,
            specials: [
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL
            ].to_vec(),
            special_cancel: CancelType::HIT | CancelType::BLOCK,
            special_cancel_require_flag: false,
            jump_cancel: CancelType::NONE,
            jump_cancel_require_flag: false,
            aerial_cancel: CancelType::NONE,
            aerial_cancel_require_flag: false,
            dash_cancel: CancelType::NONE,
            dash_cancel_require_flag: false,
            dash_cancel_direction: DashCancelDir::FORWARD,
            airdash_cancel: CancelType::NONE,
            airdash_cancel_require_flag: false,
            fgc_flags: FGCFlags::ALL
        }
    }

    pub fn set_flag(mut self, flag: i32) -> Self {
        self.flag = flag;
        self
    }

    pub fn enable_normals(mut self, normals: Vec<i32>) -> Self {
        self.normals = normals;
        self
    }

    pub fn normal_cancel_require_flag(mut self) -> Self {
        self.normal_cancel_require_flag = true;
        self
    }

    pub fn enable_specials(mut self, specials: Vec<i32>) -> Self {
        self.specials = specials;
        self
    }

    pub fn special_cancel_require_flag(mut self) -> Self {
        self.special_cancel_require_flag = true;
        self
    }

    pub fn enable_jump_cancel(mut self, jump_cancel: CancelType) -> Self {
        self.jump_cancel = jump_cancel;
        self
    }

    pub fn jump_cancel_require_flag(mut self) -> Self {
        self.jump_cancel_require_flag = true;
        self
    }

    pub fn enable_aerials(mut self, aerial_cancel: CancelType) -> Self {
        self.aerial_cancel = aerial_cancel;
        self
    }

    pub fn aerial_cancel_require_flag(mut self) -> Self {
        self.aerial_cancel_require_flag = true;
        self
    }

    pub fn enable_dash_cancel(mut self, dash_cancel: CancelType) -> Self {
        self.dash_cancel = dash_cancel;
        self
    }

    pub fn dash_cancel_require_flag(mut self) -> Self {
        self.dash_cancel_require_flag = true;
        self
    }

    pub fn set_dash_cancel_direction(mut self, dash_cancel_direction: DashCancelDir) -> Self {
        self.dash_cancel_direction = dash_cancel_direction;
        self
    }

    pub fn enable_airdash_cancel(mut self, airdash_cancel: CancelType) -> Self {
        self.airdash_cancel = airdash_cancel;
        self
    }

    pub fn airdash_cancel_require_flag(mut self) -> Self {
        self.airdash_cancel_require_flag = true;
        self
    }

    pub fn set_fgc_flags(mut self, fgc_flags: FGCFlags) -> Self {
        self.fgc_flags = fgc_flags;
        self
    }
}

lazy_static! {
    static ref CUSTOM_CANCEL_MANAGER: RwLock<CustomCancelManager> = RwLock::new(CustomCancelManager::new());
}

pub struct CustomCancelManager {
    pub hp_values: HashMap<Hash40, f32>,
    pub cancel_infos: HashMap<Hash40, HashMap<i32, CancelInfo>>
}

impl CustomCancelManager {
    pub(crate) fn new() -> Self {
        Self {
            hp_values: HashMap::new(),
            cancel_infos: HashMap::new()
        }
    }

    #[export_name = "CustomCancelManager__initialize_agent"]
    pub extern "Rust" fn initialize_agent(agent: Hash40) -> bool {
        let mut manager = CUSTOM_CANCEL_MANAGER.write();
        if let Some(_agent_infos) = manager.cancel_infos.get_mut(&agent) {
            // println!("[CustomCancelManager] Agent {:#x} already initialized!", agent.hash);
            false
        } else {
            manager.cancel_infos.insert(agent, HashMap::new());
            // println!("[CustomCancelManager] Agent {:#x} initialized!", agent.hash);
            true
        }
    }

    #[export_name = "CustomCancelManager__add_cancel_info"]
    pub extern "Rust" fn add_cancel_info(agent: Hash40, status: i32, info: CancelInfo) -> bool {
        let mut manager = CUSTOM_CANCEL_MANAGER.write();
        if let Some(agent_infos) = manager.cancel_infos.get_mut(&agent) {
            if agent_infos.contains_key(&status) {
                // println!("[CustomCancelManager] Cannot overwrite {:#x}'s Cancel Info for status {:#x}", agent.hash, status);
            }
            else {
                agent_infos.insert(status, info);
                // println!("[CustomCancelManager] Writing {:#x}'s Cancel Info for status {:#x}", agent.hash, status);
                return true;
            }
        }
        false
    }

    #[export_name = "CustomCancelManager__add_cancel_info_force"]
    pub extern "Rust" fn add_cancel_info_force(agent: Hash40, status: i32, info: CancelInfo) -> bool {
        let mut manager = CUSTOM_CANCEL_MANAGER.write();
        if let Some(agent_infos) = manager.cancel_infos.get_mut(&agent) {
            if !agent_infos.insert(status, info).is_none() {
                // println!("[CustomCancelManager] Overwriting {:#x}'s Cancel Info for status {:#x}", agent.hash, status);
            }
            else {
                // println!("[CustomCancelManager] Writing {:#x}'s Cancel Info for status {:#x}", agent.hash, status);
                return true;
            }
        }
        false
    }

    #[export_name = "CustomCancelManager__remove_cancel_info"]
    pub extern "Rust" fn remove_cancel_info(agent: Hash40, status: i32) -> bool {
        let mut manager = CUSTOM_CANCEL_MANAGER.write();
        if let Some(agent_infos) = manager.cancel_infos.get_mut(&agent) {
            if !agent_infos.remove(&status).is_none() {
                // println!("[CustomCancelManager] Removed {:#x}'s Cancel Info for status {:#x}", agent.hash, status);
                return true;
            }
        }
        false
    }

    #[export_name = "CustomCancelManager__add_hp_value"]
    pub extern "Rust" fn add_hp_value(agent: Hash40, hp: f32) {
        let mut manager = CUSTOM_CANCEL_MANAGER.write();
        if let Some(_hp_value) = manager.hp_values.insert(agent, hp) {
            // println!("[CustomCancelManager] Overwriting {:#x}'s Old HP Value {} with {}", agent.hash, _hp_value, hp);
        }
    }

    #[export_name = "CustomCancelManager__get_hp_value"]
    pub extern "Rust" fn get_hp_value(agent: Hash40) -> f32 {
        let mut manager = CUSTOM_CANCEL_MANAGER.write();
        if let Some(hp_value) = manager.hp_values.get(&agent) {
            // println!("[CustomCancelManager] Getting {:#x}'s HP Value {}", agent.hash, hp_value);
            return *hp_value;
        }
        0.0
    }

    #[export_name = "CustomCancelManager__execute_cancel"]
    pub extern "Rust" fn execute_cancel(fighter: &mut L2CFighterCommon) -> bool {
        let mut manager = CUSTOM_CANCEL_MANAGER.write();
        let agent = unsafe{ (*fighter.battle_object).agent_kind_hash };
        if let Some(agent_infos) = manager.cancel_infos.get_mut(&agent) {
            let status = unsafe {StatusModule::status_kind(fighter.module_accessor)};
            if let Some(cancel_info) = agent_infos.get_mut(&status) {
                if !fighter.global_table[IS_STOP].get_bool() {
                    if Self::execute_cancel_inner(fighter, cancel_info) {
                        return true;
                    }
                }
            }
        }
        false
    }

    extern "Rust" fn execute_cancel_inner(fighter: &mut L2CFighterCommon, cancel_info: &CancelInfo) -> bool {
        if !fighter.global_table[IS_STOP].get_bool() {
            unsafe {
                if let Some(pre_func) = cancel_info.pre {
                    // println!("[CustomCancelModule] Pre Check found!");
                    if pre_func(fighter) {
                        return false;
                    }
                }
                let hit = AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT);
                let shield = AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD);
                let cancel_window = FGCModule::check_cancel_window(fighter);
                let situation = fighter.global_table[SITUATION_KIND].get_i32();
                let fgc = VarModule::is_flag(fighter.battle_object, commons::instance::flag::IS_FGC);

                // Input Priority

                // Special Cancel

                let check_fgc = if cancel_info.fgc_flags.contains(FGCFlags::SPECIAL) {
                    fgc
                }
                else {
                    true
                };
                if check_fgc {
                    let special_cancel = cancel_info.special_cancel;
                    
                    let mut specials = cancel_info.specials.clone();

                    let condition =
                    if cancel_info.alt_info.is_some()
                    && {
                        let alt_enable = cancel_info.alt_info.as_ref().unwrap();
                        VarModule::is_flag(fighter.battle_object, alt_enable.flag)
                    } {
                        let alt_enable = cancel_info.alt_info.as_ref().unwrap();
                        let alt_cancel = alt_enable.special_cancel;
                        if alt_cancel.contains(CancelType::WHIFF)
                        || (alt_cancel.contains(CancelType::BLOCK) && shield)
                        || (alt_cancel.contains(CancelType::HIT) && hit) {
                            specials = alt_enable.specials.clone();
                            true
                        }
                        else {
                            false
                        }
                    }
                    else if special_cancel.contains(CancelType::WHIFF)
                    || (special_cancel.contains(CancelType::BLOCK) && shield)
                    || (special_cancel.contains(CancelType::HIT) && hit) {
                        true
                    }
                    else {
                        false
                    };

                    let require_flag = if cancel_info.alt_info.is_some()
                    && {
                        let alt_enable = cancel_info.alt_info.as_ref().unwrap();
                        VarModule::is_flag(fighter.battle_object, alt_enable.flag)
                    } {
                        let alt_enable = cancel_info.alt_info.as_ref().unwrap();
                        alt_enable.special_cancel_require_flag
                    }
                    else {
                        cancel_info.special_cancel_require_flag
                    };

                    let flag_check = if require_flag {
                        VarModule::is_flag(fighter.battle_object, commons::status::flag::SPECIAL_CANCEL)
                    }
                    else {
                        cancel_window
                    };
                    
                    if flag_check && condition
                    && special_cancel_common(fighter, situation.into(), specials).get_bool() {
                        return true;
                    }
                }

                // Normal Cancel

                let check_fgc = if cancel_info.fgc_flags.contains(FGCFlags::NORMAL) {
                    fgc
                }
                else {
                    true
                };
                if check_fgc {
                    let normal_cancel = cancel_info.normal_cancel;

                    let mut normals = cancel_info.normals.clone();
                    
                    let condition =
                    if cancel_info.alt_info.is_some()
                    && {
                        let alt_enable = cancel_info.alt_info.as_ref().unwrap();
                        VarModule::is_flag(fighter.battle_object, alt_enable.flag)
                    } {
                        let alt_enable = cancel_info.alt_info.as_ref().unwrap();
                        let alt_cancel = alt_enable.normal_cancel;
                        if alt_cancel.contains(CancelType::WHIFF)
                        || (alt_cancel.contains(CancelType::BLOCK) && shield)
                        || (alt_cancel.contains(CancelType::HIT) && hit) {
                            normals = alt_enable.normals.clone();
                            true
                        }
                        else {
                            false
                        }
                    }
                    else if normal_cancel.contains(CancelType::WHIFF)
                    || (normal_cancel.contains(CancelType::BLOCK) && shield)
                    || (normal_cancel.contains(CancelType::HIT) && hit) {
                        true
                    }
                    else {
                        false
                    };

                    let require_flag = if cancel_info.alt_info.is_some()
                    && {
                        let alt_enable = cancel_info.alt_info.as_ref().unwrap();
                        VarModule::is_flag(fighter.battle_object, alt_enable.flag)
                    } {
                        let alt_enable = cancel_info.alt_info.as_ref().unwrap();
                        alt_enable.normal_cancel_require_flag
                    }
                    else {
                        cancel_info.normal_cancel_require_flag
                    };
                    
                    let flag_check = if require_flag {
                        VarModule::is_flag(fighter.battle_object, commons::status::flag::NORMAL_CANCEL)
                    }
                    else {
                        cancel_window
                    };
                    
                    if flag_check && condition
                    && normal_cancel_common(fighter, normals).get_bool() {
                        return true;
                    }
                }

                // Aerial Cancel

                // I need to see if you can jump cancel your current move so...
                let can_jump = {
                    let jump_cancel = cancel_info.jump_cancel;

                    let condition =
                    if cancel_info.alt_info.is_some()
                    && {
                        let alt_enable = cancel_info.alt_info.as_ref().unwrap();
                        VarModule::is_flag(fighter.battle_object, alt_enable.flag)
                    } {
                        let alt_enable = cancel_info.alt_info.as_ref().unwrap();
                        let alt_cancel = alt_enable.jump_cancel;
                        if alt_cancel.contains(CancelType::WHIFF)
                        || (alt_cancel.contains(CancelType::BLOCK) && shield)
                        || (alt_cancel.contains(CancelType::HIT) && hit) {
                            true
                        }
                        else {
                            false
                        }
                    }
                    else if jump_cancel.contains(CancelType::WHIFF)
                    || (jump_cancel.contains(CancelType::BLOCK) && shield)
                    || (jump_cancel.contains(CancelType::HIT) && hit) {
                        true
                    }
                    else {
                        false
                    };

                    let require_flag = if cancel_info.alt_info.is_some()
                    && {
                        let alt_enable = cancel_info.alt_info.as_ref().unwrap();
                        VarModule::is_flag(fighter.battle_object, alt_enable.flag)
                    } {
                        let alt_enable = cancel_info.alt_info.as_ref().unwrap();
                        alt_enable.jump_cancel_require_flag
                    }
                    else {
                        cancel_info.jump_cancel_require_flag
                    };
                    
                    let flag_check = if require_flag {
                        VarModule::is_flag(fighter.battle_object, commons::status::flag::JUMP_CANCEL)
                    }
                    else {
                        cancel_window
                    };
                    
                    condition && flag_check
                };

                let check_fgc = if cancel_info.fgc_flags.contains(FGCFlags::AERIAL) {
                    fgc
                }
                else {
                    true
                };
                if check_fgc {
                    let aerial_cancel = cancel_info.aerial_cancel;

                    let condition =
                    if cancel_info.alt_info.is_some()
                    && {
                        let alt_enable = cancel_info.alt_info.as_ref().unwrap();
                        VarModule::is_flag(fighter.battle_object, alt_enable.flag)
                    } {
                        let alt_enable = cancel_info.alt_info.as_ref().unwrap();
                        let alt_cancel = alt_enable.aerial_cancel;
                        if alt_cancel.contains(CancelType::WHIFF)
                        || (alt_cancel.contains(CancelType::BLOCK) && shield)
                        || (alt_cancel.contains(CancelType::HIT) && hit) {
                            true
                        }
                        else {
                            false
                        }
                    }
                    else if aerial_cancel.contains(CancelType::WHIFF)
                    || (aerial_cancel.contains(CancelType::BLOCK) && shield)
                    || (aerial_cancel.contains(CancelType::HIT) && hit) {
                        true
                    }
                    else {
                        false
                    };

                    let require_flag = if cancel_info.alt_info.is_some()
                    && {
                        let alt_enable = cancel_info.alt_info.as_ref().unwrap();
                        VarModule::is_flag(fighter.battle_object, alt_enable.flag)
                    } {
                        let alt_enable = cancel_info.alt_info.as_ref().unwrap();
                        alt_enable.aerial_cancel_require_flag
                    }
                    else {
                        cancel_info.aerial_cancel_require_flag
                    };
                    
                    let flag_check = if require_flag {
                        VarModule::is_flag(fighter.battle_object, commons::status::flag::NORMAL_CANCEL)
                    }
                    else {
                        cancel_window
                    };

                    if flag_check && condition
                    && aerial_cancel_common_revised(fighter, can_jump).get_bool() {
                        return true;
                    }
                }

                // Jump Cancel

                let check_fgc = if cancel_info.fgc_flags.contains(FGCFlags::JUMP) {
                    fgc
                }
                else {
                    true
                };
                if check_fgc {
                    if can_jump
                    && jump_cancel_common(fighter, situation.into()).get_bool() {
                        return true;
                    }
                }

                // Dash Cancel
                
                let check_fgc = if cancel_info.fgc_flags.contains(FGCFlags::DASH) {
                    fgc
                }
                else {
                    true
                };
                if check_fgc {
                    let dash_cancel = cancel_info.dash_cancel;
                    let dash_cancel_dir = cancel_info.dash_cancel_direction;

                    let condition =
                    if cancel_info.alt_info.is_some()
                    && {
                        let alt_enable = cancel_info.alt_info.as_ref().unwrap();
                        VarModule::is_flag(fighter.battle_object, alt_enable.flag)
                    } {
                        let alt_enable = cancel_info.alt_info.as_ref().unwrap();
                        let alt_cancel = alt_enable.dash_cancel;
                        if alt_cancel.contains(CancelType::WHIFF)
                        || (alt_cancel.contains(CancelType::BLOCK) && shield)
                        || (alt_cancel.contains(CancelType::HIT) && hit) {
                            true
                        }
                        else {
                            false
                        }
                    }
                    else if dash_cancel.contains(CancelType::WHIFF)
                    || (dash_cancel.contains(CancelType::BLOCK) && shield)
                    || (dash_cancel.contains(CancelType::HIT) && hit) {
                        true
                    }
                    else {
                        false
                    };

                    let require_flag = if cancel_info.alt_info.is_some()
                    && {
                        let alt_enable = cancel_info.alt_info.as_ref().unwrap();
                        VarModule::is_flag(fighter.battle_object, alt_enable.flag)
                    } {
                        let alt_enable = cancel_info.alt_info.as_ref().unwrap();
                        alt_enable.dash_cancel_require_flag
                    }
                    else {
                        cancel_info.dash_cancel_require_flag
                    };
                    
                    let flag_check = if require_flag {
                        VarModule::is_flag(fighter.battle_object, commons::status::flag::DASH_CANCEL)
                    }
                    else {
                        cancel_window
                    };
                    
                    if condition && flag_check {
                        if dash_cancel_dir.contains(DashCancelDir::FORWARD)
                        && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH != 0 {
                            fighter.change_status(FIGHTER_STATUS_KIND_DASH.into(), true.into());
                            VarModule::on_flag(fighter.battle_object, commons::status::flag::IS_DASH_CANCEL);
                            return true;
                        }
                        if dash_cancel_dir.contains(DashCancelDir::BACKWARD)
                        && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH != 0 {
                            fighter.change_status(FIGHTER_STATUS_KIND_TURN_DASH.into(), true.into());
                            VarModule::on_flag(fighter.battle_object, commons::status::flag::IS_DASH_CANCEL);
                            return true;
                        }
                    }
                }

                // Air Dash Cancel

                
                let check_fgc = if cancel_info.fgc_flags.contains(FGCFlags::AIRDASH) {
                    fgc
                }
                else {
                    true
                };
                if check_fgc {
                    let airdash_cancel = cancel_info.airdash_cancel;
                    
                    let condition =
                    if cancel_info.alt_info.is_some()
                    && {
                        let alt_enable = cancel_info.alt_info.as_ref().unwrap();
                        VarModule::is_flag(fighter.battle_object, alt_enable.flag)
                    } {
                        let alt_enable = cancel_info.alt_info.as_ref().unwrap();
                        let alt_cancel = alt_enable.airdash_cancel;
                        if alt_cancel.contains(CancelType::WHIFF)
                        || (alt_cancel.contains(CancelType::BLOCK) && shield)
                        || (alt_cancel.contains(CancelType::HIT) && hit) {
                            true
                        }
                        else {
                            false
                        }
                    }
                    else if airdash_cancel.contains(CancelType::WHIFF)
                    || (airdash_cancel.contains(CancelType::BLOCK) && shield)
                    || (airdash_cancel.contains(CancelType::HIT) && hit) {
                        true
                    }
                    else {
                        false
                    };

                    let require_flag = if cancel_info.alt_info.is_some()
                    && {
                        let alt_enable = cancel_info.alt_info.as_ref().unwrap();
                        VarModule::is_flag(fighter.battle_object, alt_enable.flag)
                    } {
                        let alt_enable = cancel_info.alt_info.as_ref().unwrap();
                        alt_enable.airdash_cancel_require_flag
                    }
                    else {
                        cancel_info.airdash_cancel_require_flag
                    };
                    
                    let flag_check = if require_flag {
                        VarModule::is_flag(fighter.battle_object, commons::status::flag::DASH_CANCEL)
                    }
                    else {
                        cancel_window
                    };
                    
                    if condition && flag_check
                    && airdash_cancel_common(fighter, situation.into()).get_bool() {
                        VarModule::on_flag(fighter.battle_object, commons::instance::flag::FORCE_ESCAPE_AIR_SLIDE);
                        return true;
                    }
                }
                if let Some(exception_func) = cancel_info.exception {
                    // println!("[CustomCancelModule] Exception found!");
                    if exception_func(fighter) {
                        return true;
                    }
                }
            }
        }
        false
    }

}
