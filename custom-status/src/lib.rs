#![allow(unused_mut)]
#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;

// Thank you blujay for all of the help on this one

use {
    std::{collections::HashMap, sync::Arc},
    parking_lot::RwLock,
    smash::{
        lua2cpp::*,
        phx::Hash40,
        app::*,
        lib::L2CValue
    }
};

pub type StatusFunc = unsafe extern "C" fn(&mut L2CFighterCommon) -> L2CValue;

pub struct StatusInfo {
    pub pre: Option<StatusFunc>,
    pub main: Option<StatusFunc>,
    pub end: Option<StatusFunc>,
    pub init: Option<StatusFunc>,
    pub exec: Option<StatusFunc>,
    pub exec_stop: Option<StatusFunc>,
    pub exec_post: Option<StatusFunc>,
    pub exit: Option<StatusFunc>,
    pub map_correction: Option<StatusFunc>,
    pub fix_camera: Option<StatusFunc>,
    pub fix_pos_slow: Option<StatusFunc>,
    pub check_damage: Option<StatusFunc>,
    pub check_attack: Option<StatusFunc>,
    pub on_change_lr: Option<StatusFunc>,
    pub leave_stop: Option<StatusFunc>,
    pub notify_event_gimmick: Option<StatusFunc>,
    pub calc_param: Option<StatusFunc>
}

impl StatusInfo {
    pub fn new() -> StatusInfo {
        StatusInfo {
            pre: None,
            main: None,
            end: None,
            init: None,
            exec: None,
            exec_stop: None,
            exec_post: None,
            exit: None,
            map_correction: None,
            fix_camera: None,
            fix_pos_slow: None,
            check_damage: None,
            check_attack: None,
            on_change_lr: None,
            leave_stop: None,
            notify_event_gimmick: None,
            calc_param: None,
        }
    }

    pub fn with_pre(mut self, pre: StatusFunc) -> Self {
        self.pre = Some(pre);
        self
    }

    pub fn with_main(mut self, main: StatusFunc) -> Self {
        self.main = Some(main);
        self
    }

    pub fn with_end(mut self, end: StatusFunc) -> Self {
        self.end = Some(end);
        self
    }

    pub fn with_init(mut self, init: StatusFunc) -> Self {
        self.init = Some(init);
        self
    }

    pub fn with_exec(mut self, exec: StatusFunc) -> Self {
        self.exec = Some(exec);
        self
    }

    pub fn with_exec_stop(mut self, exec_stop: StatusFunc) -> Self {
        self.exec_stop = Some(exec_stop);
        self
    }

    pub fn with_exec_post(mut self, exec_post: StatusFunc) -> Self {
        self.exec_post = Some(exec_post);
        self
    }

    pub fn with_exit(mut self, exit: StatusFunc) -> Self {
        self.exit = Some(exit);
        self
    }

    pub fn with_map_correction(mut self, map_correction: StatusFunc) -> Self {
        self.map_correction = Some(map_correction);
        self
    }

    pub fn with_fix_camera(mut self, fix_camera: StatusFunc) -> Self {
        self.fix_camera = Some(fix_camera);
        self
    }

    pub fn with_fix_pos_slow(mut self, fix_pos_slow: StatusFunc) -> Self {
        self.fix_pos_slow = Some(fix_pos_slow);
        self
    }

    pub fn with_check_damage(mut self, check_damage: StatusFunc) -> Self {
        self.check_damage = Some(check_damage);
        self
    }

    pub fn with_check_attack(mut self, check_attack: StatusFunc) -> Self {
        self.check_attack = Some(check_attack);
        self
    }

    pub fn with_on_change_lr(mut self, on_change_lr: StatusFunc) -> Self {
        self.on_change_lr = Some(on_change_lr);
        self
    }

    pub fn with_leave_stop(mut self, leave_stop: StatusFunc) -> Self {
        self.leave_stop = Some(leave_stop);
        self
    }

    pub fn with_notify_event_gimmick(mut self, notify_event_gimmick: StatusFunc) -> Self {
        self.notify_event_gimmick = Some(notify_event_gimmick);
        self
    }

    pub fn with_calc_param(mut self, calc_param: StatusFunc) -> Self {
        self.calc_param = Some(calc_param);
        self
    }

}

lazy_static! {
    static ref CUSTOM_STATUS_MANAGER: RwLock<CustomStatusManager> = RwLock::new(CustomStatusManager::new());
}

pub struct CustomStatusManager {
    pub modules: Arc<RwLock<HashMap<Hash40, CustomStatusModule>>>,
    pub agent_statuses: HashMap<Hash40, Arc<RwLock<HashMap<i32, StatusInfo>>>>,
    pub common_statuses: Arc<RwLock<HashMap<i32, StatusInfo>>>,
}

impl CustomStatusManager {
    pub(crate) fn new() -> Self {
        Self {
            modules: Arc::new(RwLock::new(HashMap::new())),
            agent_statuses: HashMap::new(),
            common_statuses: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    #[export_name = "CustomStatusManager__add_new_agent_module"]
    pub extern "Rust" fn add_new_agent_module(agent: Hash40, module: CustomStatusModule) -> bool {
        let mut manager = CUSTOM_STATUS_MANAGER.write();
        let x = if let Some(mut modules) = manager.modules.try_write() {
            let _ = modules.insert(agent, module);
            true
        }
        else {
            false
        };

        x
    }

    #[export_name = "CustomStatusManager__add_new_common_status_script"]
    pub extern "Rust" fn add_new_common_status_script(id: i32, info: StatusInfo) -> bool {
        let mut manager = CUSTOM_STATUS_MANAGER.write();
        let x = if let Some(mut common_statuses) = manager.common_statuses.try_write() {
            let _ = common_statuses.insert(id, info);
            true
        } else {
            false
        };

        x
    }

    #[export_name = "CustomStatusManager__add_new_agent_status_script"]
    pub extern "Rust" fn add_new_agent_status_script(agent: Hash40, id: i32, info: StatusInfo) -> bool {
        let mut manager = CUSTOM_STATUS_MANAGER.write();
        if let Some(agent_statuses) = manager.agent_statuses.get_mut(&agent) {
            if let Some(mut agent_statuses) = agent_statuses.try_write() {
                let _ = agent_statuses.insert(id, info);
                true
            } else {
                false
            }
        } else {
            let mut map = HashMap::new();
            let _ = map.insert(id, info);
            let _ = manager.agent_statuses.insert(agent, Arc::new(RwLock::new(map)));
            true
        }
    }

    #[export_name = "CustomStatusManager__get_common_status_scripts"]
    pub extern "Rust" fn get_common_status_scripts() -> Arc<RwLock<HashMap<i32, StatusInfo>>> {
        CUSTOM_STATUS_MANAGER.read().common_statuses.clone()
    }

    #[export_name = "CustomStatusManager__get_agent_status_scripts"]
    pub extern "Rust" fn get_agent_status_scripts(agent: Hash40) -> Option<Arc<RwLock<HashMap<i32, StatusInfo>>>> {
        CUSTOM_STATUS_MANAGER.read().agent_statuses.get(&agent).map(|x| x.clone())
    }
}

pub struct CustomStatusModule {
    pub common_statuses: Arc<RwLock<HashMap<i32, StatusInfo>>>,
    pub agent_statuses: Option<Arc<RwLock<HashMap<i32, StatusInfo>>>>,
    pub vanilla_status_max: i32,
    pub common_status_end: i32,
}

impl CustomStatusModule {
    pub fn new() -> Self {
        Self {
            common_statuses: CustomStatusManager::get_common_status_scripts(),
            agent_statuses: None,
            vanilla_status_max: 0,
            common_status_end: 0
        }
    }

    pub fn init_from_hash(object: *mut BattleObject, max: i32) -> i32 {
        let agent_hash;
        unsafe {
            agent_hash = (*object).agent_kind_hash;
        }
        CustomStatusManager::add_new_agent_module(agent_hash, CustomStatusModule::new());
        let mut mgr = CUSTOM_STATUS_MANAGER.read();
        let mut modules = mgr.modules.write();
        let module = modules.get_mut(&agent_hash).unwrap();
        println!("[INIT] Got CustomStatusModule for {:#x}", agent_hash.hash);

        module.agent_statuses = CustomStatusManager::get_agent_status_scripts(agent_hash);
        println!("[INIT] Got agent status scripts for {:#x}", agent_hash.hash);
        module.vanilla_status_max = max;

        let mut common_max = -1;
        let mut agents_max = -1;

        let common = module.common_statuses.read();
        for (id, _) in common.iter() {
            common_max = common_max.max(*id);
        }
        
        if let Some(agents) = module.agent_statuses.as_ref().map(|x| x.read()) {
            for (id, _) in agents.iter() {
                agents_max = agents_max.max(*id);
            }
        }

        println!("[INIT] Number of added common statuses: {}", common_max);
        println!("[INIT] Number of added agent statuses: {}", agents_max);

        module.common_status_end = module.vanilla_status_max + common_max + 1;
        module.common_status_end + agents_max + 1
    }

    pub fn install_statuses(object: *mut BattleObject, agent: &mut L2CFighterCommon) {
        let agent_hash;
        unsafe {
            agent_hash = (*object).agent_kind_hash;
        }
        let mut mgr = CUSTOM_STATUS_MANAGER.read();
        let mut modules = mgr.modules.write();
        if let Some(mut module) = modules.get_mut(&agent_hash) {
            println!("[INSTALL] Got CustomStatusModule for {:#x}", agent_hash.hash);
            let common = module.common_statuses.read();

            for (id, info) in common.iter() {
                println!("[INSTALL] Installing common status with ID {}", id);

                let kind = module.vanilla_status_max + *id;
                if let Some(func) = info.pre.clone() {
                    set_status_func(agent, kind, 0, func);
                }
                if let Some(func) = info.main.clone() {
                    set_status_func(agent, kind, 1, func);
                }
                if let Some(func) = info.end.clone() {
                    set_status_func(agent, kind, 2, func);
                }
                if let Some(func) = info.init.clone() {
                    set_status_func(agent, kind, 3, func);
                }
                if let Some(func) = info.exec.clone() {
                    set_status_func(agent, kind, 4, func);
                }
                if let Some(func) = info.exec_stop.clone() {
                    set_status_func(agent, kind, 5, func);
                }
                if let Some(func) = info.exec_post.clone() {
                    set_status_func(agent, kind, 6, func);
                }
                if let Some(func) = info.exit.clone() {
                    set_status_func(agent, kind, 7, func);
                }
                if let Some(func) = info.map_correction.clone() {
                    set_status_func(agent, kind, 8, func);
                }
                if let Some(func) = info.fix_camera.clone() {
                    set_status_func(agent, kind, 9, func);
                }
                if let Some(func) = info.fix_pos_slow.clone() {
                    set_status_func(agent, kind, 10, func);
                }
                if let Some(func) = info.check_damage.clone() {
                    set_status_func(agent, kind, 11, func);
                }
                if let Some(func) = info.check_attack.clone() {
                    set_status_func(agent, kind, 12, func);
                }
                if let Some(func) = info.on_change_lr.clone() {
                    set_status_func(agent, kind, 13, func);
                }
                if let Some(func) = info.leave_stop.clone() {
                    set_status_func(agent, kind, 14, func);
                }
                if let Some(func) = info.notify_event_gimmick.clone() {
                    set_status_func(agent, kind, 15, func);
                }
                if let Some(func) = info.calc_param.clone() {
                    set_status_func(agent, kind, 16, func);
                }
            }

            if let Some(agents) = module.agent_statuses.as_ref().map(|x| x.read()) {
                
                for (id, info) in agents.iter() {
                    let kind = module.common_status_end + *id;
                    println!("[INSTALL] Installing agent statuse with ID {} | {}", id, kind);
                    if let Some(func) = info.pre.clone() {
                        set_status_func(agent, kind, 0, func);
                    }
                    if let Some(func) = info.main.clone() {
                        set_status_func(agent, kind, 1, func);
                    }
                    if let Some(func) = info.end.clone() {
                        set_status_func(agent, kind, 2, func);
                    }
                    if let Some(func) = info.init.clone() {
                        set_status_func(agent, kind, 3, func);
                    }
                    if let Some(func) = info.exec.clone() {
                        set_status_func(agent, kind, 4, func);
                    }
                    if let Some(func) = info.exec_stop.clone() {
                        set_status_func(agent, kind, 5, func);
                    }
                    if let Some(func) = info.exec_post.clone() {
                        set_status_func(agent, kind, 6, func);
                    }
                    if let Some(func) = info.exit.clone() {
                        set_status_func(agent, kind, 7, func);
                    }
                    if let Some(func) = info.map_correction.clone() {
                        set_status_func(agent, kind, 8, func);
                    }
                    if let Some(func) = info.fix_camera.clone() {
                        set_status_func(agent, kind, 9, func);
                    }
                    if let Some(func) = info.fix_pos_slow.clone() {
                        set_status_func(agent, kind, 10, func);
                    }
                    if let Some(func) = info.check_damage.clone() {
                        set_status_func(agent, kind, 11, func);
                    }
                    if let Some(func) = info.check_attack.clone() {
                        set_status_func(agent, kind, 12, func);
                    }
                    if let Some(func) = info.on_change_lr.clone() {
                        set_status_func(agent, kind, 13, func);
                    }
                    if let Some(func) = info.leave_stop.clone() {
                        set_status_func(agent, kind, 14, func);
                    }
                    if let Some(func) = info.notify_event_gimmick.clone() {
                        set_status_func(agent, kind, 15, func);
                    }
                    if let Some(func) = info.calc_param.clone() {
                        set_status_func(agent, kind, 16, func);
                    }
                }
            }
        }
        println!("[INSTALL] Finished installing status scripts!");
    }

    #[export_name = "CustomStatusModule__get_agent_status_kind"]
    pub extern "Rust" fn get_agent_status_kind(object: *mut BattleObject, id: i32) -> i32 {
        let agent_hash;
        unsafe {
            agent_hash = (*object).agent_kind_hash;
        }
        let mut mgr = CUSTOM_STATUS_MANAGER.read();
        let mut modules = mgr.modules.write();
        let module = modules.get_mut(&agent_hash).unwrap();
        println!("[AGENT] Got CustomStatusModule for {:#x}", agent_hash.hash);

        module.common_status_end + id
    }

    #[export_name = "CustomStatusModule__get_common_status_kind"]
    pub extern "Rust" fn get_common_status_kind(object: *mut BattleObject, id: i32) -> i32 {
        let agent_hash;
        unsafe {
            agent_hash = (*object).agent_kind_hash;
        }
        let mut mgr = CUSTOM_STATUS_MANAGER.read();
        let mut modules = mgr.modules.write();
        let module = modules.get_mut(&agent_hash).unwrap();
        println!("[COMMON] Got CustomStatusModule for {:#x}", agent_hash.hash);

        module.vanilla_status_max + id
    }
}

fn set_status_func(agent: &mut L2CFighterBase, kind: i32, condition: i32, func: StatusFunc) {
    unsafe {
        agent.sv_set_status_func(
            kind.into(),
            condition.into(),
            std::mem::transmute(func)
        )
    }
}

