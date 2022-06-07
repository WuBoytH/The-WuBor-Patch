#![allow(unused_mut)]
#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;

use {
    std::{collections::HashMap, sync::Arc},
    parking_lot::RwLock,
    smash::{
        phx::*,
        app::*
    }
};

lazy_static! {
    static ref CUSTOM_VAR_MANAGER: RwLock<CustomVarManager> = RwLock::new(CustomVarManager::new());
}

pub struct CustomVarManager {
    pub modules: Arc<RwLock<HashMap<i32, VarModule>>>
}

impl CustomVarManager {
    pub(crate) fn new() -> Self {
        Self {
            modules: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    #[export_name = "CustomVarManager__reset_var_module"]
    pub extern "Rust" fn reset_var_module(object: *mut BattleObject) -> bool {
        let entry_id = unsafe{
            Fighter::get_fighter_entry_id((*object).battle_object_id)
        };
        let mut manager = CUSTOM_VAR_MANAGER.write();
        let x = if let Some(mut modules) = manager.modules.try_write() {
            let _ = modules.insert(entry_id, VarModule::new());
            true
        }
        else {
            false
        };

        x
    }
}

pub struct VarModule {
    int: [Vec<i32>; 3],
    int64: [Vec<u64>; 3],
    float: [Vec<f32>; 3],
    flag: [Vec<bool>; 3]
}

/// An additional module to be used with Smash's `BattleObject` class. This handles storing and retrieving primitive variables
/// that you want to associate with a specific object (such as associating a gimmick timer with mario or dk)
impl VarModule {
    /// Resets all integers that are within the common array.
    pub const RESET_COMMON_INT:   u16 = 0b000000000001;
    /// Resets all 64-bit values that are within the common array
    pub const RESET_COMMON_INT64: u16 = 0b000000000010;
    /// Resets all floats that are within the common array
    pub const RESET_COMMON_FLOAT: u16 = 0b000000000100;
    /// Resets all flags that are within the common array (default is `false`)
    pub const RESET_COMMON_FLAG:  u16 = 0b000000001000;

    /// Resets all integers that are within the instance array
    pub const RESET_INSTANCE_INT:   u16 = 0b000000010000;
    /// Resets all 64-bit values that are within the instance array
    pub const RESET_INSTANCE_INT64: u16 = 0b000000100000;
    /// Resets all floats that are within the instance array
    pub const RESET_INSTANCE_FLOAT: u16 = 0b000001000000;
    /// Resets all flags that are within the instance array
    pub const RESET_INSTANCE_FLAG:  u16 = 0b000010000000;

    /// Resets all integers that are within the instance array
    pub const RESET_STATUS_INT:   u16 = 0b000100000000;
    /// Resets all 64-bit values that are within the instance array
    pub const RESET_STATUS_INT64: u16 = 0b001000000000;
    /// Resets all floats that are within the instance array
    pub const RESET_STATUS_FLOAT: u16 = 0b010000000000;
    /// Resets all flags that are within the instance array
    pub const RESET_STATUS_FLAG:  u16 = 0b100000000000;

    /// Resets all integers
    pub const RESET_INT:   u16 = Self::RESET_COMMON_INT | Self::RESET_INSTANCE_INT | Self::RESET_STATUS_INT;
    /// Resets all 64-bit values
    pub const RESET_INT64: u16 = Self::RESET_COMMON_INT64 | Self::RESET_INSTANCE_INT64 | Self::RESET_STATUS_INT64;
    /// Resets all floats
    pub const RESET_FLOAT: u16 = Self::RESET_COMMON_FLOAT | Self::RESET_INSTANCE_FLOAT | Self::RESET_STATUS_FLOAT;
    /// Resets all flags
    pub const RESET_FLAG:  u16 = Self::RESET_COMMON_FLAG | Self::RESET_INSTANCE_FLAG | Self::RESET_STATUS_FLAG;

    /// Resets all values in the common array
    pub const RESET_COMMON:   u16 = 0xF;
    /// Resets all values in the instance array
    pub const RESET_INSTANCE: u16 = 0xF0;
    /// Resets all values in the status array
    pub const RESET_STATUS:   u16 = 0xF00;
    /// Resets all values
    pub const RESET_ALL:      u16 = 0xFFF;

    /// Constructs a new instance of `VarModule` that defaults all values to either `0` or `false`
    /// # Returns
    /// A blank `VarModule` instance
    pub(crate) fn new() -> Self {
        Self {
            int: [vec![0; 0x1000], vec![0; 0x1000], vec![0; 0x1000]],
            int64: [vec![0; 0x1000], vec![0; 0x1000], vec![0; 0x1000]],
            float: [vec![0.0; 0x1000], vec![0.0; 0x1000], vec![0.0; 0x1000]],
            flag: [vec![false; 0x1000], vec![false; 0x1000], vec![false; 0x1000]]
        }
    }

    /// Resets various `VarModule` arrays depending on the mask
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `mask` - A mask of the reset values to determine what to reset
    #[export_name = "VarModule__reset"]
    pub extern "Rust" fn reset(object: *mut BattleObject, mask: u16) {
        let entry_id = unsafe{
            Fighter::get_fighter_entry_id((*object).battle_object_id)
        };
        let mut manager = CUSTOM_VAR_MANAGER.read();
        let mut modules = manager.modules.write();
        if let Some(mut module) = modules.get_mut(&entry_id) {
            if mask & Self::RESET_COMMON_INT != 0 {
                module.int[0].fill(0);
            }
            if mask & Self::RESET_INSTANCE_INT != 0 {
                module.int[1].fill(0);
            }
            if mask & Self::RESET_STATUS_INT != 0 {
                module.int[2].fill(0);
            }
            if mask & Self::RESET_COMMON_INT64 != 0 {
                module.int64[0].fill(0);
            }
            if mask & Self::RESET_INSTANCE_INT64 != 0 {
                module.int64[1].fill(0);
            }
            if mask & Self::RESET_STATUS_INT64 != 0 {
                module.int64[2].fill(0);
            }
            if mask & Self::RESET_COMMON_FLOAT != 0 {
                module.float[0].fill(0.0);
            }
            if mask & Self::RESET_INSTANCE_FLOAT != 0 {
                module.float[1].fill(0.0);
            }
            if mask & Self::RESET_STATUS_FLOAT != 0 {
                module.float[2].fill(0.0);
            }
            if mask & Self::RESET_COMMON_FLAG != 0 {
                module.flag[0].fill(false);
            }
            if mask & Self::RESET_INSTANCE_FLAG != 0 {
                module.flag[1].fill(false);
            }
            if mask & Self::RESET_INSTANCE_FLAG != 0 {
                module.flag[1].fill(false);
            }
        }
    }

    /// Retrieves an integer
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - The variable to retrieve
    /// # Returns
    /// The variable requested
    #[export_name = "VarModule__get_int"]
    pub extern "Rust" fn get_int(object: *mut BattleObject, what: i32) -> i32 {
        let entry_id = unsafe{
            Fighter::get_fighter_entry_id((*object).battle_object_id)
        };
        let mut manager = CUSTOM_VAR_MANAGER.read();
        let mut modules = manager.modules.write();
        if let Some(mut module) = modules.get_mut(&entry_id) {
            let vec_index = (what >> 0xC) as usize;
            let index = (what & 0xFFF) as usize;
            module.int[vec_index][index]   
        }
        else {
            0
        }
    }

    /// Retrieves a float
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - The variable to retrieve
    /// # Returns
    /// The variable requested
    #[export_name = "VarModule__get_float"]
    pub extern "Rust" fn get_float(object: *mut BattleObject, what: i32) -> f32 {
        let entry_id = unsafe{
            Fighter::get_fighter_entry_id((*object).battle_object_id)
        };
        let mut manager = CUSTOM_VAR_MANAGER.read();
        let mut modules = manager.modules.write();
        if let Some(mut module) = modules.get_mut(&entry_id) {
            let vec_index = (what >> 0xC) as usize;
            let index = (what & 0xFFF) as usize;
            module.float[vec_index][index]
        }
        else {
            0.0
        }
    }

    /// Retrieves a 64-bit value
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - The variable to retrieve
    /// # Returns
    /// The variable requested
    #[export_name = "VarModule__get_int64"]
    pub extern "Rust" fn get_int64(object: *mut BattleObject, what: i32) -> u64 {
        let entry_id = unsafe{
            Fighter::get_fighter_entry_id((*object).battle_object_id)
        };
        let mut manager = CUSTOM_VAR_MANAGER.read();
        let mut modules = manager.modules.write();
        if let Some(mut module) = modules.get_mut(&entry_id) {
            let vec_index = (what >> 0xC) as usize;
            let index = (what & 0xFFF) as usize;
            module.int64[vec_index][index]
        }
        else {
            0
        }
    }

    /// Retrieves a flag
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - The variable to retrieve
    /// # Returns
    /// The variable requested
    #[export_name = "VarModule__is_flag"]
    pub extern "Rust" fn is_flag(object: *mut BattleObject, what: i32) -> bool {
        let entry_id = unsafe{
            Fighter::get_fighter_entry_id((*object).battle_object_id)
        };
        let mut manager = CUSTOM_VAR_MANAGER.read();
        let mut modules = manager.modules.write();
        if let Some(mut module) = modules.get_mut(&entry_id) {
            let vec_index = (what >> 0xC) as usize;
            let index = (what & 0xFFF) as usize;
            module.flag[vec_index][index]
        }
        else {
            false
        }
    }

    /// Sets an integer value
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - The variable to set
    /// * `val` - The value to set the variable to
    #[export_name = "VarModule__set_int"]
    pub extern "Rust" fn set_int(object: *mut BattleObject, what: i32, val: i32) {
        let entry_id = unsafe{
            Fighter::get_fighter_entry_id((*object).battle_object_id)
        };
        let mut manager = CUSTOM_VAR_MANAGER.read();
        let mut modules = manager.modules.write();
        if let Some(mut module) = modules.get_mut(&entry_id) {
            let vec_index = (what >> 0xC) as usize;
            let index = (what & 0xFFF) as usize;
            module.int[vec_index][index] = val;
        }
    }

    /// Sets a float value
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - The variable to set
    /// * `val` - The value to set the variable to
    #[export_name = "VarModule__set_float"]
    pub extern "Rust" fn set_float(object: *mut BattleObject, what: i32, val: f32) {
        let entry_id = unsafe{
            Fighter::get_fighter_entry_id((*object).battle_object_id)
        };
        let mut manager = CUSTOM_VAR_MANAGER.read();
        let mut modules = manager.modules.write();
        if let Some(mut module) = modules.get_mut(&entry_id) {
            let vec_index = (what >> 0xC) as usize;
            let index = (what & 0xFFF) as usize;
            module.float[vec_index][index] = val;   
        }
    }

    /// Sets a 64-bit value
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - The variable to set
    /// * `val` - The value to set the variable to
    #[export_name = "VarModule__set_int64"]
    pub extern "Rust" fn set_int64(object: *mut BattleObject, what: i32, val: u64) {
        let entry_id = unsafe{
            Fighter::get_fighter_entry_id((*object).battle_object_id)
        };
        let mut manager = CUSTOM_VAR_MANAGER.read();
        let mut modules = manager.modules.write();
        if let Some(mut module) = modules.get_mut(&entry_id) {
            let vec_index = (what >> 0xC) as usize;
            let index = (what & 0xFFF) as usize;
            module.int64[vec_index][index] = val;
        }
    }

    /// Sets a flag
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - The variable to set
    /// * `val` - The value to set the variable to
    #[export_name = "VarModule__set_flag"]
    pub extern "Rust" fn set_flag(object: *mut BattleObject, what: i32, val: bool) {
        let entry_id = unsafe{
            Fighter::get_fighter_entry_id((*object).battle_object_id)
        };
        let mut manager = CUSTOM_VAR_MANAGER.read();
        let mut modules = manager.modules.write();
        if let Some(mut module) = modules.get_mut(&entry_id) {
            let vec_index = (what >> 0xC) as usize;
            let index = (what & 0xFFF) as usize;
            module.flag[vec_index][index] = val;
        }
    }

    /// Sets a flag to false
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - The variable to set
    /// # Note
    /// This method is equivalent to `VarModule::set_flag(object, what, false)`
    #[export_name = "VarModule__off_flag"]
    pub extern "Rust" fn off_flag(object: *mut BattleObject, what: i32) {
        Self::set_flag(object, what, false);
    }

    /// Sets a flag to true
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - The variable to set
    /// # Note
    /// This method is equivalent to `VarModule::set_flag(object, what, true)`
    #[export_name = "VarModule__on_flag"]
    pub extern "Rust" fn on_flag(object: *mut BattleObject, what: i32) {
        Self::set_flag(object, what, true);
    }

    /// Countdowns an integer
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - The variable to count down
    /// * `min` - The minimum value that variable should be before we are done counting down
    /// # Returns
    /// * `true` - `what` was less than `min` before or after decrementing
    /// * `false` - `what` remains greater than or equal to `min` after decrementing
    #[export_name = "VarModule__countdown_int"]
    pub extern "Rust" fn countdown_int(object: *mut BattleObject, what: i32, min: i32) -> bool {
        if Self::get_int(object, what) < min {
            true
        } else {
            Self::dec_int(object, what);
            Self::get_int(object, what) < min
        }
    }

    /// Adds a value to an integer
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - The variable to add to
    /// * `amount` - The value to add to the variable
    #[export_name = "VarModule__add_int"]
    pub extern "Rust" fn add_int(object: *mut BattleObject, what: i32, amount: i32) {
        let entry_id = unsafe{
            Fighter::get_fighter_entry_id((*object).battle_object_id)
        };
        let mut manager = CUSTOM_VAR_MANAGER.read();
        let mut modules = manager.modules.write();
        if let Some(mut module) = modules.get_mut(&entry_id) {
            let vec_index = (what >> 0xC) as usize;
            let index = (what & 0xFFF) as usize;
            module.int[vec_index][index] += amount;
        }
    }

    /// Subtracts a value from an integer
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - The variable to subtract from
    /// * `amount` - The value to subtract from the variable
    #[export_name = "VarModule__sub_int"]
    pub extern "Rust" fn sub_int(object: *mut BattleObject, what: i32, amount: i32) {
        let entry_id = unsafe{
            Fighter::get_fighter_entry_id((*object).battle_object_id)
        };
        let mut manager = CUSTOM_VAR_MANAGER.read();
        let mut modules = manager.modules.write();
        if let Some(mut module) = modules.get_mut(&entry_id) {
            let vec_index = (what >> 0xC) as usize;
            let index = (what & 0xFFF) as usize;
            module.int[vec_index][index] -= amount;
        }
    }

    /// Increments an integer
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - The variable to increment
    /// # Note
    /// This is functionally equivalent to `VarModule::add_int(object, what, 1)`
    #[export_name = "VarModule__inc_int"]
    pub extern "Rust" fn inc_int(object: *mut BattleObject, what: i32) {
        Self::add_int(object, what, 1);
    }

    /// Decrements an integer
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - The variable to decrement
    /// # Note
    /// This is functionally equivalent to `VarModule::sub_int(object, what, 1)`
    #[export_name = "VarModule__dec_int"]
    pub extern "Rust" fn dec_int(object: *mut BattleObject, what: i32) {
        Self::sub_int(object, what, 1);
    }

    /// Adds a value to float
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - The variable to add on to
    /// * `amount` - The amount to add to the variable
    #[export_name = "VarModule__add_float"]
    pub extern "Rust" fn add_float(object: *mut BattleObject, what: i32, amount: f32) {
        let entry_id = unsafe{
            Fighter::get_fighter_entry_id((*object).battle_object_id)
        };
        let mut manager = CUSTOM_VAR_MANAGER.read();
        let mut modules = manager.modules.write();
        if let Some(mut module) = modules.get_mut(&entry_id) {
            let vec_index = (what >> 0xC) as usize;
            let index = (what & 0xFFF) as usize;
            module.float[vec_index][index] += amount;
        }
    }

    /// Subtracts a value from a float
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - The variable to subtract from
    /// * `amount` - The amount to subtract from the variable
    #[export_name = "VarModule__sub_float"]
    pub extern "Rust" fn sub_float(object: *mut BattleObject, what: i32, amount: f32) {
        let entry_id = unsafe{
            Fighter::get_fighter_entry_id((*object).battle_object_id)
        };
        let mut manager = CUSTOM_VAR_MANAGER.read();
        let mut modules = manager.modules.write();
        if let Some(mut module) = modules.get_mut(&entry_id) {
            let vec_index = (what >> 0xC) as usize;
            let index = (what & 0xFFF) as usize;
            module.float[vec_index][index] -= amount;
        }
    }

    /// Sets a 2-dimensional vector
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - Where to start setting the vector
    /// * `val` - The vector to set
    /// # Panics
    /// This function requires that the last 3 bytes of `what` are less than `0xFFF`
    #[export_name = "VarModule__set_vec2"]
    pub extern "Rust" fn set_vec2(object: *mut BattleObject, what: i32, val: Vector2f) {
        let entry_id = unsafe{
            Fighter::get_fighter_entry_id((*object).battle_object_id)
        };
        let mut manager = CUSTOM_VAR_MANAGER.read();
        let mut modules = manager.modules.write();
        if let Some(mut module) = modules.get_mut(&entry_id) {
            let vec_index = (what >> 0xC) as usize;
            let index = (what & 0xFFF) as usize;
            if index >= 0xFFF {
                panic!("Cannot set Vec2 on index that will overflow!");
            }
            module.float[vec_index][index + 0] = val.x;
            module.float[vec_index][index + 1] = val.y;
        }
    }

    /// Sets a 3-dimensional vector
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - Where to start setting the vector
    /// * `val` - The vector to set
    /// # Panics
    /// This function requires that the last 3 bytes of `what` are less than `0xFFE`
    #[export_name = "VarModule__set_vec3"]
    pub extern "Rust" fn set_vec3(object: *mut BattleObject, what: i32, val: Vector3f) {
        let entry_id = unsafe{
            Fighter::get_fighter_entry_id((*object).battle_object_id)
        };
        let mut manager = CUSTOM_VAR_MANAGER.read();
        let mut modules = manager.modules.write();
        if let Some(mut module) = modules.get_mut(&entry_id) {
            let vec_index = (what >> 0xC) as usize;
            let index = (what & 0xFFF) as usize;
            if index >= 0xFFE {
                panic!("Cannot set Vec2 on index that will overflow!");
            }
            module.float[vec_index][index + 0] = val.x;
            module.float[vec_index][index + 1] = val.y;
            module.float[vec_index][index + 2] = val.z;
        }
    }

    /// Sets a 4-dimensional vector
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - Where to start setting the vector
    /// * `val` - The vector to set
    /// # Panics
    /// This function requires that the last 3 bytes of `what` are less than `0xFFD`
    #[export_name = "VarModule__set_vec4"]
    pub extern "Rust" fn set_vec4(object: *mut BattleObject, what: i32, val: Vector4f) {
        let entry_id = unsafe{
            Fighter::get_fighter_entry_id((*object).battle_object_id)
        };
        let mut manager = CUSTOM_VAR_MANAGER.read();
        let mut modules = manager.modules.write();
        if let Some(mut module) = modules.get_mut(&entry_id) {
            let vec_index = (what >> 0xC) as usize;
            let index = (what & 0xFFF) as usize;
            if index >= 0xFFD {
                panic!("Cannot set Vec2 on index that will overflow!");
            }
            module.float[vec_index][index + 0] = val.x;
            module.float[vec_index][index + 1] = val.y;
            module.float[vec_index][index + 2] = val.z;
            module.float[vec_index][index + 3] = val.w;
        }
    }

    /// Gets a 2-dimensional vector
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - Where to start setting the vector
    /// # Returns
    /// The 2-dimensional vector starting at the value specified
    /// # Panics
    /// This function requires that the last 3 bytes of `what` are less than `0xFFF`
    #[export_name = "VarModule__get_vec2"]
    pub extern "Rust" fn get_vec2(object: *mut BattleObject, what: i32) -> Vector2f {
        let entry_id = unsafe{
            Fighter::get_fighter_entry_id((*object).battle_object_id)
        };
        let mut manager = CUSTOM_VAR_MANAGER.read();
        let mut modules = manager.modules.write();
        if let Some(mut module) = modules.get_mut(&entry_id) {
            let vec_index = (what >> 0xC) as usize;
            let index = (what & 0xFFF) as usize;
            if index >= 0xFFF {
                panic!("Cannot get Vec2 with index that will overflow!");
            }
            Vector2f {
                x: module.float[vec_index][index + 0],
                y: module.float[vec_index][index + 1]
            }
        }
        else {
            Vector2f{x: 0.0, y: 0.0}
        }
    }

    /// Gets a 3-dimensional vector
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - Where to start setting the vector
    /// # Returns
    /// The 3-dimensional vector starting at the value specified
    /// # Panics
    /// This function requires that the last 3 bytes of `what` are less than `0xFFE`
    #[export_name = "VarModule__get_vec3"]
    pub extern "Rust" fn get_vec3(object: *mut BattleObject, what: i32) -> Vector3f {
        let entry_id = unsafe{
            Fighter::get_fighter_entry_id((*object).battle_object_id)
        };
        let mut manager = CUSTOM_VAR_MANAGER.read();
        let mut modules = manager.modules.write();
        if let Some(mut module) = modules.get_mut(&entry_id) {
            let vec_index = (what >> 0xC) as usize;
            let index = (what & 0xFFF) as usize;
            if index >= 0xFFE {
                panic!("Cannot get Vec2 with index that will overflow!");
            }
            Vector3f {
                x: module.float[vec_index][index + 0],
                y: module.float[vec_index][index + 1],
                z: module.float[vec_index][index + 2]
            }
        }
        else {
            Vector3f{x: 0.0, y: 0.0, z: 0.0}
        }
    }


    /// Gets a 4-dimensional vector
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - Where to start setting the vector
    /// # Returns
    /// The 4-dimensional vector starting at the value specified
    /// # Panics
    /// This function requires that the last 3 bytes of `what` are less than `0xFFD`
    #[export_name = "VarModule__get_vec4"]
    pub extern "Rust" fn get_vec4(object: *mut BattleObject, what: i32) -> Vector4f {
        let entry_id = unsafe{
            Fighter::get_fighter_entry_id((*object).battle_object_id)
        };
        let mut manager = CUSTOM_VAR_MANAGER.read();
        let mut modules = manager.modules.write();
        if let Some(mut module) = modules.get_mut(&entry_id) {
            let vec_index = (what >> 0xC) as usize;
            let index = (what & 0xFFF) as usize;
            if index >= 0xFFD {
                panic!("Cannot get Vec2 with index that will overflow!");
            }
            Vector4f {
                x: module.float[vec_index][index + 0],
                y: module.float[vec_index][index + 1],
                z: module.float[vec_index][index + 2],
                w: module.float[vec_index][index + 3]
            }
        }
        else {
            Vector4f{x: 0.0, y: 0.0, z: 0.0, w: 0.0}
        }
    }
}

pub struct GenericModule {
    vtable: *const u64,
    owner: *mut BattleObjectModuleAccessor,
    // ...
}

#[skyline::hook( offset = 0x4deb70 )]
pub unsafe fn set_work_keep_hook(module: *mut GenericModule, float: u32, int: u32, flag: u32) {
    let mut mask = 0;
    if flag == 0xFFFFF000 {
        mask += VarModule::RESET_STATUS_FLAG;
    }
    if int == 0xFFFFFFE0 {
        mask += VarModule::RESET_STATUS_INT;
        mask += VarModule::RESET_STATUS_INT64;
    }
    if float == 0xFFFFFF80 {
        mask += VarModule::RESET_STATUS_FLOAT;
    }
    let module_accessor = (*module).owner;
    let object_id = (*module_accessor).battle_object_id;
    let object = sv_system::battle_object(object_id as u64);
    VarModule::reset(object, mask);
    original!()(module, flag, int, float)
}
