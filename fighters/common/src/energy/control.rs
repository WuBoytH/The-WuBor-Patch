#![allow(
    unused_unsafe,
    dead_code,
    unused_unsafe,
    unused_assignments,
    improper_ctypes_definitions,
    clippy::needless_lifetimes,
    clippy::needless_return,
    clippy::transmute_ptr_to_ref
)]

use {
    smash::{
        hash40,
        phx::*,
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    wubor_utils::controls::*,
    custom_var::*,
    wubor_utils::vars,
    super::super::param
};

#[repr(C)]
pub struct KineticEnergyVTable {
    pub destructor: extern "C" fn(&mut KineticEnergy),
    pub deleter: extern "C" fn(*mut KineticEnergy),
    pub unk: extern "C" fn(&mut KineticEnergy, &mut BattleObjectModuleAccessor),
    pub update: extern "C" fn(&mut KineticEnergy, &mut BattleObjectModuleAccessor),
    pub get_speed: extern "C" fn(&mut KineticEnergy) -> *mut PaddedVec2,
    pub initialize: extern "C" fn(&mut KineticEnergy, &mut BattleObjectModuleAccessor),
    pub get_some_flag: extern "C" fn(&mut KineticEnergy) -> bool,
    pub set_some_flag: extern "C" fn(&mut KineticEnergy, bool),
    pub setup_energy: extern "C" fn(&mut KineticEnergy, u32, &Vector3f, u64, &mut BattleObjectModuleAccessor),
    pub clear_energy: extern "C" fn(&mut KineticEnergy),
    pub unk2: extern "C" fn(&mut KineticEnergy),
    pub set_speed: extern "C" fn (&mut KineticEnergy, &Vector2f),
    pub mul_accel: extern "C" fn(&mut KineticEnergy, &Vector2f),
    // ...

}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct PaddedVec2 {
    pub x: f32,
    pub y: f32,
    pub padding: u64
}

impl PaddedVec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            padding: 0
        }
    }

    pub fn zeros() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            padding: 0
        }
    }

    pub fn mag(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[repr(C)]
pub struct KineticEnergy {
    pub vtable: &'static KineticEnergyVTable,
    pub _x8: u64, // probably padding
    pub speed: PaddedVec2,
    pub rot_speed: PaddedVec2,
    pub enable: bool,
    pub unk2: [u8; 0xF], // probably padding
    pub accel: PaddedVec2,
    pub speed_max: PaddedVec2,
    pub speed_brake: PaddedVec2,
    pub speed_limit: PaddedVec2,
    pub _x80: u8,
    pub consider_ground_friction: bool,
    pub active_flag: bool, // no clue?
    pub _x83: u8,
    pub energy_reset_type: u32,
}

impl KineticEnergy {
    pub fn adjust_speed_for_ground_normal(speed: &PaddedVec2, module_accessor: &mut BattleObjectModuleAccessor) -> PaddedVec2 {
        #[skyline::from_offset(0x47b4f0)]
        extern "C" fn adjust_speed_for_ground_normal_internal(speed: smash_rs::cpp::simd::Vector2, module_accessor: &mut BattleObjectModuleAccessor) -> smash_rs::cpp::simd::Vector2;

        unsafe {
            let result = adjust_speed_for_ground_normal_internal(smash_rs::cpp::simd::Vector2 { vec: [speed.x, speed.y] }, module_accessor);
            PaddedVec2::new(result.vec[0], result.vec[1])
        }
    }

    pub fn process(&mut self, module_accessor: &mut BattleObjectModuleAccessor) {
        unsafe {
            #[skyline::from_offset(0x47bf90)]
            extern "C" fn process_energy(energy: &mut KineticEnergy, module_accessor: &mut BattleObjectModuleAccessor);

            process_energy(self, module_accessor)
        }
    }

    pub fn update(&mut self, module_accessor: &mut BattleObjectModuleAccessor) {
        unsafe {
            (self.vtable.update)(self, module_accessor)
        }
    }

    pub fn get_speed<'a>(&'a mut self) -> &'a mut PaddedVec2 {
        unsafe {
            std::mem::transmute((self.vtable.get_speed)(self))
        }
    }

    pub fn initialize(&mut self, module_accessor: &mut BattleObjectModuleAccessor) {
        unsafe {
            (self.vtable.initialize)(self, module_accessor)
        }
    }

    pub fn get_some_flag(&mut self) -> bool {
        unsafe {
            (self.vtable.get_some_flag)(self)
        }
    }

    pub fn set_some_flag(&mut self, flag: bool) {
        unsafe {
            (self.vtable.set_some_flag)(self, flag)
        }
    }

    pub fn setup_energy(&mut self, reset_type: u32, incoming_speed: &Vector3f, some: u64, module_accessor: &mut BattleObjectModuleAccessor) {
        unsafe {
            (self.vtable.setup_energy)(self, reset_type, incoming_speed, some, module_accessor)
        }
    }

    pub fn clear_energy(&mut self) {
        unsafe {
            (self.vtable.clear_energy)(self)
        }
    }

    pub fn unk2(&mut self) {
        unsafe {
            (self.vtable.unk2)(self)
        }
    }

    pub fn set_speed(&mut self, speed: &Vector2f) {
        unsafe {
            (self.vtable.set_speed)(self, speed)
        }
    }

    pub fn mul_accel(&mut self, mul: &Vector2f) {
        unsafe {
            (self.vtable.mul_accel)(self, mul)
        }
    }

}

#[repr(C)]
struct FlyData {
    pub turn_stick_x: f32,
    pub init_speed_x_mul: f32,
    pub speed_x_mul: f32,
    pub speed_x_max_mul: f32,
    pub speed_y_table_start: *const f32,
    pub speed_y_table_end: *const f32,
    pub speed_y_table_eos: *const f32,
    pub turn_param_start: *const i32,
    pub turn_param_end: *const i32,
    pub turn_param_eos: *const i32,
    pub shoot_fly_next_frame: i32
}

impl FlyData {
    pub fn get_from_fighter_kind(kind: i32) -> Option<&'static Self> {
        #[repr(C)]
        struct FlyDataResult {
            vtable: *const *const (),
            data: *const *const FlyData
        }

        unsafe {
            let accessor = *((singletons::FighterParamAccessor2() as *const u8).add((kind as usize) * 0x38 + 0x70) as *const u64);
            let function: extern "C" fn(u64, u64) -> FlyDataResult = std::mem::transmute(*(*(accessor as *const *const u64)).add(0x2));
            let result = function(accessor, hash40("fly_data"));
            if (*result.data).is_null() {
                return None;
            } else {
                return Some(&**result.data);
            }
        }

    }
}

use std::ops::{Deref, DerefMut};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum EnergyControllerResetType {
    FallAdjust = 0x0,
    FallAdjustNoCap,
    StopCeil,
    WallJump,
    FlyAdjust,
    Dash,
    ShootDash,
    ShootBackDash,
    TurnRun,
    RevolveSlashAir,
    Turn,
    Free,
    FreeTest,
    ItemLift,
    SwimRise,
    Swim,
    SwimDrown,
    MoveGround,
    MoveAir,
    TurnNoStop,
    TurnNoStopAir,
    Ladder,
    DashBack,
}

#[repr(C)]
pub struct FighterKineticEnergyControl {
    parent: KineticEnergy,
    pub lr: f32,
    pub accel_mul_x: f32,
    pub accel_add_x: f32,
    pub accel_mul_y: f32,
    pub accel_add_y: f32,
    pub _x9c: f32,
    pub _xa0: f32,
    pub unk: [u8; 4]
}

impl Deref for FighterKineticEnergyControl {
    type Target = KineticEnergy;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for FighterKineticEnergyControl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[skyline::hook(offset = 0x6d3630)]
unsafe extern "C" fn update(energy: &mut FighterKineticEnergyControl, module_accessor: &mut BattleObjectModuleAccessor) {
    let reset_type = std::mem::transmute(energy.energy_reset_type);

    let mut stick = if Buttons::from_bits_retain(ControlModule::get_button(module_accessor)).intersects(Buttons::CStickOverride) {
        Vector2f {
            x: ControlModule::get_sub_stick_x(module_accessor),
            y: ControlModule::get_sub_stick_y(module_accessor)
        }
    } else {
        Vector2f {
            x: ControlModule::get_stick_x(module_accessor),
            y: ControlModule::get_stick_y(module_accessor)
        }
    };

    let backup_max = energy.speed_max;
    let backup_brake = energy.speed_brake;

    if WorkModule::get_float(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME) > 0.0 {
        stick.x = 0.0;
    }

    let accel_add_x = if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE
    && !WorkModule::is_flag(module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL)
    {
        stick.x = 0.0;
        0.0
    } else if stick.x == 0.0 {
        0.0
    } else {
        energy.accel_add_x
    };

    let accel_add_y = if stick.y != 0.0 {
        energy.accel_add_y
    } else {
        0.0
    };

    use EnergyControllerResetType::*;

    let mut do_standard_accel = true;

    let accel_diff = match reset_type {
        FallAdjust | FallAdjustNoCap | FlyAdjust | ShootDash | ShootBackDash | RevolveSlashAir | MoveGround | MoveAir => {
            accel_add_x * stick.x.signum() + stick.x * energy.accel_mul_x
        },
        WallJump => {
            if WorkModule::get_int(module_accessor, *FIGHTER_STATUS_WALL_JUMP_WORK_INT_DISABLE_CONT_FRAME) == 0 {
                accel_add_x * stick.x.signum() + stick.x * energy.accel_mul_x
            } else {
                0.0
            }
        },
        Dash | DashBack => {
            // Don't apply or change the speed by any ammount during the first keep frames of dash
            let keep_frame = WorkModule::get_param_int(module_accessor, hash40("common"), hash40("dash_speed_keep_frame"));
            let mut zero = false;
            if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DASH || reset_type == DashBack {
                if WorkModule::get_int(module_accessor, *FIGHTER_STATUS_DASH_WORK_INT_COUNT) < keep_frame {
                    energy.speed_max.x = 0.0;
                    energy.speed_brake.x = 0.0;
                    zero = true;
                }
            } else if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_TURN_DASH {
                if WorkModule::get_int(module_accessor, *FIGHTER_STATUS_DASH_WORK_INT_TURN_DASH_FROM_DASH_COUNT) < keep_frame {
                    energy.speed_max.x = 0.0;
                    energy.speed_brake.x = 0.0;
                    zero = true;
                }
            }

            // let direction = -PostureModule::lr(module_accessor);
            // let direction = if reset_type != DashBack {
            //     -direction
            // } else {
            //     direction
            // };

            // Prevents any negative acceleration from happening during dash
            // (this kills any potential of moonwalks)
            // if !zero && stick.x * direction <= 0.0 {
            //     energy.speed_max.x = 0.0;
            //     zero = true;
            // }

            // accel add
            if zero {
                0.0
            }
            else {
                stick.x * energy.accel_mul_x + stick.x.signum() * energy.accel_add_x
            }
        },
        TurnRun => {
            let mut mul = stick.x * energy.accel_mul_x + accel_add_x * stick.x.signum();
            let mut brake = WorkModule::get_param_float(module_accessor, hash40("ground_brake"), 0)
                                    * WorkModule::get_param_float(module_accessor, hash40("common"), hash40("run_brake_brake_mul"));

            let ground_module = *(module_accessor as *const BattleObjectModuleAccessor as *const u64).add(0x88 / 0x8);
            let some_float = *(ground_module as *const f32).add(0x130 / 0x8);
            if some_float * energy.lr <= -0.1 {
                let turn_run_brake = WorkModule::get_param_float(module_accessor, hash40("common"), hash40("turn_run_stop_brake_mul"));
                mul *= turn_run_brake;
                brake *= turn_run_brake;
            }
            energy.speed_brake.x = brake;
            if 0.0 <= mul * energy.lr {
                do_standard_accel = false;
                energy.accel.x = 0.0;
                energy.speed_max.x = 0.0;
                0.0
            } else {
                mul
            }
        },
        Free => {
            energy.accel.y = accel_add_y * stick.y.signum() + stick.y * energy.accel_mul_y;
            energy.speed_max.y *= stick.y.abs();
            accel_add_x * stick.x.signum() + stick.x * energy.accel_mul_x
        },
        ItemLift => {
            let mut zero = false;
            if WorkModule::is_flag(module_accessor, *FIGHTER_STATUS_ITEM_LIFT_WORK_FLAG_STOP) {
                stick.x = 0.0;
                zero = true;
            }

            if !zero && accel_add_x * PostureModule::lr(module_accessor) <= 0.0 {
                energy.speed_max.x = 0.0;
                energy.speed_brake.x = 0.0;
                stick.x = accel_add_x; // not sure if this is accurate but it's what I think I saw in the code
                zero = true;
            }

            if zero {
                0.0
            }
            else {
                let stick_rate = WorkModule::get_float(module_accessor, *FIGHTER_STATUS_ITEM_LIFT_WORK_FLOAT_STICK_RATE);
                energy.speed_max.x *= stick_rate;
                (accel_add_x * stick.x.signum() + stick.x * energy.accel_mul_x) * stick_rate
            }
        },
        Swim => {
            let speed_mul = WorkModule::get_param_float(module_accessor, hash40("common"), hash40("swim_speed_mul"));
            energy.speed_max.x = stick.x.abs() * speed_mul;
            energy.speed_max.y = -1.0;
            accel_add_x * stick.x.signum() + stick.x * energy.accel_mul_x
        },
        SwimDrown => {
            let speed_mul = WorkModule::get_param_float(module_accessor, hash40("common"), hash40("swim_drown_speed_x_mul"))
                                    * WorkModule::get_param_float(module_accessor, hash40("common"), hash40("swim_speed_mul"));
            energy.speed_max.x = stick.x * speed_mul;
            energy.speed_max.y = -1.0;
            accel_add_x * stick.x.signum() + stick.x * energy.accel_mul_x
        },
        Turn | TurnNoStop | TurnNoStopAir => {
            if reset_type == TurnNoStop || reset_type == TurnNoStopAir {
                if (!TurnModule::is_turn(module_accessor) || energy.accel_mul_x == 0.0) && energy.speed.x == 0.0 {
                    energy.parent.enable = false;
                    return;
                }
                if ControlModule::reverse_x_frame(module_accessor) != 0 {
                    stick.x = -stick.x;
                }
            }

            (accel_add_x * stick.x.signum() + stick.x * energy.accel_mul_x) * WorkModule::get_param_float(module_accessor, hash40("common"), hash40("turn_speed_mul"))
        },
        Ladder => {
            let ladder_y = WorkModule::get_param_float(module_accessor, hash40("common"), hash40("ladder_stick_y"));
            let (speed_max, accel_y) = if ladder_y <= stick.y.abs() {
                if stick.y <= 0.0 {
                    let down_max = WorkModule::get_param_float(module_accessor, hash40("common"), hash40("ladder_speed_d_max"));
                    // lerp the down_max
                    let down_max = ((stick.y.abs() - ladder_y) / (1.0 - ladder_y)) * down_max;
                    let attack_mul = WorkModule::get_param_float(module_accessor, hash40("common"), hash40("ladder_attack_speed_mul"));
                    (down_max * attack_mul, -down_max * attack_mul)
                } else {
                    let up_max = WorkModule::get_param_float(module_accessor, hash40("common"), hash40("ladder_speed_d_max"));
                    // lerp the down_max
                    let up_max = ((stick.y - ladder_y) / (1.0 - ladder_y)) * up_max;
                    let attack_mul = WorkModule::get_param_float(module_accessor, hash40("common"), hash40("ladder_attack_speed_mul"));
                    (up_max * attack_mul, up_max * attack_mul)
                }
            } else {
                (0.0, 0.0)
            };
            do_standard_accel = false;
            energy.accel.x = 0.0;
            energy.accel.y = accel_y;
            energy.speed_max.x = 0.0;
            energy.speed_max.y = speed_max;
            0.0
        }
        _ => 0.0
    };


    if do_standard_accel {
        let mut set_speed_max = true;

        energy.accel.x = accel_diff;
        let speed_max = energy.speed_max.x * stick.x.abs();

        if energy.unk[1] != 0 {
            if !(((energy._x9c != 0.0 && (stick.x <= 0.0 || energy._xa0 <= 0.0 || speed_max.abs() <= energy._x9c.abs()))
            && (stick.x >= 0.0 || energy._xa0 >= 0.0 || speed_max.abs() <= energy._x9c.abs()))
            && ((stick.x <= 0.0 || 0.0 <= energy._xa0) && (0.0 <= stick.x || energy._xa0 <= 0.0)))
            {
                energy._x9c = speed_max;
                energy._xa0 = stick.x;
            }
            else {
                set_speed_max = false;
            }
        }

        if set_speed_max {
            energy.speed_max.x = speed_max;
        }

    }

    energy.process(module_accessor);

    let status_module = *(module_accessor as *const BattleObjectModuleAccessor as *const u64).add(0x8);
    if !*(status_module as *const bool).add(0x12a) {
        if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_AIR {
            let horizontal_limit = WorkModule::get_param_float(module_accessor, hash40("common"), hash40("common_air_speed_x_limit"));
            let vertical_limit = if energy.speed.y <= 0.0 {
                WorkModule::get_param_float(module_accessor, hash40("common"), hash40("air_speed_down_limit"))
            } else {
                WorkModule::get_param_float(module_accessor, hash40("common"), hash40("air_speed_up_limit"))
            };

            if horizontal_limit < energy.speed.x.abs() {
                energy.speed.x = vertical_limit * energy.speed.x.signum();
            }

            if vertical_limit < energy.speed.y.abs() {
                energy.speed.y = vertical_limit * energy.speed.y.signum();
            }
        } else if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND {
            let speed_limit = WorkModule::get_param_float(module_accessor, hash40("common"), hash40("ground_speed_limit"));
            if speed_limit < energy.speed.x.abs() {
                energy.speed.x = speed_limit * energy.speed.x.signum();
            }
        }
    }

    energy.speed_max = backup_max;
    energy.speed_brake = backup_brake;
}

#[skyline::hook(offset = 0x6d4060)]
unsafe extern "C" fn initialize(energy: &mut FighterKineticEnergyControl, module_accessor: &mut BattleObjectModuleAccessor) {
    use EnergyControllerResetType::*;
    let reset_type = std::mem::transmute(energy.energy_reset_type);
    match reset_type {
        FallAdjust | FallAdjustNoCap | StopCeil | WallJump => {
            let mut stable_speed = WorkModule::get_param_float(module_accessor, hash40("air_speed_x_stable"), 0);
            if reset_type == StopCeil {
                stable_speed *= WorkModule::get_param_float(module_accessor, hash40("common"), hash40("stop_ceil_speed_x_stable_mul"));
            }

            let is_jump = VarModule::is_flag(module_accessor, vars::fighter::instance::flag::JUMP_FROM_SQUAT);
            // println!("Is jumping? {is_jump}");
            let super_jump = is_jump && VarModule::is_flag(module_accessor, vars::fighter::instance::flag::SUPER_JUMP);

            energy.speed_max = PaddedVec2::new(stable_speed, -1.0);
            energy.speed_brake = PaddedVec2::new(WorkModule::get_param_float(module_accessor, hash40("air_brake_x"), 0), 0.0);

            let limit_speed = if !WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT) && energy.unk[2] == 0 {
                WorkModule::get_param_float(module_accessor, hash40("common"), hash40("air_speed_x_limit"))
            } else {
                -1.0
            };

            if super_jump && WorkModule::is_flag(module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI) {
                let mut jump_speed_x = WorkModule::get_param_float(module_accessor, hash40("jump_speed_x_max"), 0);

                if jump_speed_x < WorkModule::get_param_float(module_accessor, hash40("air_speed_x_stable"), 0) {
                    jump_speed_x = WorkModule::get_param_float(module_accessor, hash40("air_speed_x_stable"), 0);
                }

                if !VarModule::is_flag(module_accessor, vars::fighter::instance::flag::SUPER_JUMP_SET_MOMENTUM) {
                    let stick_x = if Buttons::from_bits_retain(ControlModule::get_button(module_accessor)).intersects(Buttons::CStickOverride) {
                        ControlModule::get_sub_stick_x(module_accessor)
                    }
                    else {
                        ControlModule::get_stick_x(module_accessor)
                    };
                    energy.speed.x = jump_speed_x * stick_x;
                    VarModule::on_flag(module_accessor, vars::fighter::instance::flag::SUPER_JUMP_SET_MOMENTUM);
                }
                energy.speed_max.x = jump_speed_x;
            }

            let control_mul = if super_jump {
                param::jump::special_jump_control_mul
            }
            else {
                1.0
            };
            energy.speed_limit = PaddedVec2::new(limit_speed, 0.0);
            energy.accel_mul_x = WorkModule::get_param_float(module_accessor, hash40("air_accel_x_mul"), 0) * control_mul;
            energy.accel_add_x = WorkModule::get_param_float(module_accessor, hash40("air_accel_x_add"), 0) * control_mul;
        },
        FlyAdjust => {
            let kind = smash::app::utility::get_kind(module_accessor);

            let fly_data = if let Some(data) = FlyData::get_from_fighter_kind(kind) {
                data
            } else {
                return;
            };

            let stable = WorkModule::get_param_float(module_accessor, hash40("air_speed_x_stable"), 0);
            let brake = WorkModule::get_param_float(module_accessor, hash40("air_brake_x"), 0);
            let limit = WorkModule::get_param_float(module_accessor, hash40("common"), hash40("air_speed_x_limit"));
            let accel_mul = WorkModule::get_param_float(module_accessor, hash40("air_accel_x_mul"), 0) * fly_data.speed_x_mul;
            let accel_add = WorkModule::get_param_float(module_accessor, hash40("air_accel_x_add"), 0);

            energy.speed_max = PaddedVec2::new(stable, -1.0);
            energy.speed_brake = PaddedVec2::new(brake, 0.0);
            energy.speed_limit = PaddedVec2::new(limit, 0.0);
            energy.accel_mul_x = accel_mul;
            energy.accel_add_x = accel_add;
        },
        Dash | TurnRun | DashBack => {
            energy.speed_limit = PaddedVec2::new(
                WorkModule::get_param_float(module_accessor, hash40("common"), hash40("ground_speed_limit")),
                0.0
            );
            energy.speed_max = PaddedVec2::new(
                WorkModule::get_param_float(module_accessor, hash40("run_speed_max"), 0),
                -1.0
            );
            let brake = WorkModule::get_param_float(module_accessor, hash40("ground_brake"), 0)
                                * WorkModule::get_param_float(module_accessor, hash40("common"), hash40("run_brake_brake_mul"));
            energy.speed_brake = PaddedVec2::new(brake, 0.0);
            energy.accel_mul_x = WorkModule::get_param_float(module_accessor, hash40("run_accel_mul"), 0);
            energy.accel_add_x = WorkModule::get_param_float(module_accessor, hash40("run_accel_add"), 0);
        },
        ShootDash | ShootBackDash => {
            energy.speed_limit = PaddedVec2::new(
                WorkModule::get_param_float(module_accessor, hash40("common"), hash40("ground_speed_limit")),
                0.0
            );
            energy.speed_max = PaddedVec2::new(
                WorkModule::get_param_float(module_accessor, hash40("run_speed_max"), 0),
                -1.0
            );
            let brake = WorkModule::get_param_float(module_accessor, hash40("ground_brake"), 0)
                                * WorkModule::get_param_float(module_accessor, hash40("common"), hash40("run_brake_brake_mul"));
            energy.speed_brake = PaddedVec2::new(brake, 0.0);
        },
        RevolveSlashAir => {
            let speed_max = WorkModule::get_param_float(module_accessor, hash40("air_speed_x_stable"), 0)
                                    * WorkModule::get_param_float(module_accessor, hash40("param_special_hi"), hash40("rslash_air_max_x_mul"));

            energy.speed_max = PaddedVec2::new(speed_max, -1.0);
            energy.speed_brake = PaddedVec2::new(
                WorkModule::get_param_float(module_accessor, hash40("air_brake_x"), 0),
                0.0
            );
            energy.speed_limit = PaddedVec2::new(
                WorkModule::get_param_float(module_accessor, hash40("common"), hash40("air_speed_x_limit")),
                0.0
            );
            energy.accel_mul_x = WorkModule::get_param_float(module_accessor, hash40("param_special_hi"), hash40("rslash_air_max_x_mul"));
        },
        Turn | TurnNoStop => {
            energy.speed_max = PaddedVec2::new(
                WorkModule::get_param_float(module_accessor, hash40("walk_speed_max"), 0),
                -1.0
            );
            energy.speed_limit = PaddedVec2::new(
                WorkModule::get_param_float(module_accessor, hash40("common"), hash40("ground_speed_limit")),
                0.0
            );
            let brake = WorkModule::get_param_float(module_accessor, hash40("ground_brake"), 0)
                                * WorkModule::get_param_float(module_accessor, hash40("common"), hash40("run_brake_brake_mul"));
            energy.speed_brake = PaddedVec2::new(brake, 0.0);
            energy.accel_mul_x = WorkModule::get_param_float(module_accessor, hash40("walk_accel_mul"), 0);
            energy.accel_add_x = WorkModule::get_param_float(module_accessor, hash40("walk_accel_add"), 0);
        },
        Free => {
            let speed_max = WorkModule::get_param_float(module_accessor, hash40("air_speed_x_stable"), 0);
            let speed_brake = WorkModule::get_param_float(module_accessor, hash40("air_brake_x"), 0);
            let speed_limit = WorkModule::get_param_float(module_accessor, hash40("common"), hash40("air_speed_x_limit"));
            let mul = WorkModule::get_param_float(module_accessor, hash40("air_accel_x_mul"), 0);
            let add = WorkModule::get_param_float(module_accessor, hash40("air_accel_x_add"), 0);
            energy.speed_max = PaddedVec2::new(speed_max, speed_max);
            energy.speed_brake = PaddedVec2::new(speed_brake, speed_brake);
            energy.speed_limit = PaddedVec2::new(speed_limit, speed_limit);
            energy.accel_mul_x = mul;
            energy.accel_mul_y = mul;
            energy.accel_add_x = add;
            energy.accel_add_y = add;
        },
        ItemLift => {
            let scale = PostureModule::scale(module_accessor);
            energy.speed_max = PaddedVec2::new(
                scale * WorkModule::get_param_float(module_accessor, hash40("item_lift_speed_max"), 0),
                -1.0
            );
            energy.speed_limit = PaddedVec2::new(
                scale * WorkModule::get_param_float(module_accessor, hash40("common"), hash40("ground_speed_limit")),
                0.0
            );
            let brake = WorkModule::get_param_float(module_accessor, hash40("ground_brake"), 0)
                                * WorkModule::get_param_float(module_accessor, hash40("common"), hash40("run_brake_brake_mul"));
            energy.speed_brake = PaddedVec2::new(brake, 0.0);
            energy.accel_mul_x = scale * WorkModule::get_param_float(module_accessor, hash40("item_lift_accel_mul"), 0);
            energy.accel_add_x = scale * WorkModule::get_param_float(module_accessor, hash40("item_lift_accel_add"), 0);
        },
        Swim => {
            energy.speed_brake = PaddedVec2::new(
                WorkModule::get_param_float(module_accessor, hash40("common"), hash40("swim_brake")),
                0.0
            );
            energy.accel_mul_x = WorkModule::get_param_float(module_accessor, hash40("common"), hash40("swim_accel_mul"));
        },
        SwimDrown => {
            energy.speed_brake = PaddedVec2::new(
                WorkModule::get_param_float(module_accessor, hash40("common"), hash40("swim_brake")),
                0.0
            );
            energy.accel_mul_x = WorkModule::get_param_float(module_accessor, hash40("common"), hash40("swim_accel_mul"))
                                    * WorkModule::get_param_float(module_accessor, hash40("common"), hash40("swim_drown_speed_x_mul"));
        },
        TurnNoStopAir => {
            energy.speed_max = PaddedVec2::new(
                WorkModule::get_param_float(module_accessor, hash40("air_speed_x_stable"), 0),
                -1.0
            );
            energy.speed_limit = PaddedVec2::new(
                WorkModule::get_param_float(module_accessor, hash40("common"), hash40("air_speed_x_limit")),
                0.0
            );
            energy.speed_brake = PaddedVec2::new(
                WorkModule::get_param_float(module_accessor, hash40("air_brake_x"), 0),
                0.0
            );
            energy.accel_mul_x = WorkModule::get_param_float(module_accessor, hash40("air_accel_x_mul"), 0);
            energy.accel_add_x = WorkModule::get_param_float(module_accessor, hash40("air_accel_x_add"), 0);
        },
        Ladder => {
            let up_speed = WorkModule::get_param_float(module_accessor, hash40("common"), hash40("ladder_speed_u_max"));
            let down_speed = WorkModule::get_param_float(module_accessor, hash40("common"), hash40("ladder_speed_d_max"));
            energy.speed_brake = PaddedVec2::new(0.0, up_speed.max(down_speed));
        }
        _ => {}
    }
}

#[skyline::hook(offset = 0x6d4bc0)]
unsafe extern "C" fn setup(energy: &mut FighterKineticEnergyControl, reset_type: EnergyControllerResetType, initial_speed: &Vector3f, _unk: u64, module_accessor: &mut BattleObjectModuleAccessor) {
    energy.clear_energy();

    energy.accel = PaddedVec2::zeros();
    energy.speed_max = PaddedVec2::zeros();
    energy.speed_brake = PaddedVec2::zeros();
    energy.speed_limit = PaddedVec2::new(-1.0, -1.0);
    energy.speed = PaddedVec2::new(initial_speed.x, initial_speed.y);
    energy.energy_reset_type = reset_type as u32;
    energy.accel_mul_x = 0.0;
    energy.accel_add_x = 0.0;
    energy.accel_mul_y = 0.0;
    energy.accel_add_y = 0.0;
    energy.lr = PostureModule::lr(module_accessor);
    energy.unk[3] = 1;

    use EnergyControllerResetType::*;
    match reset_type {
        FallAdjust | FallAdjustNoCap | StopCeil | WallJump => {
            energy.unk[2] = if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE) {
                1
            } else {
                0
            };
            if reset_type != FallAdjustNoCap
            && !WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT)
            && energy.unk[2] == 0 {
                // let is_jump = VarModule::is_flag(module_accessor, vars::fighter::instance::flag::JUMP_FROM_SQUAT);
                // let break_limit = is_jump || StatusModule::prev_situation_kind(module_accessor) == *SITUATION_KIND_AIR;
                // let limit_speed = if break_limit {
                //     let mut limit_speed = WorkModule::get_param_float(module_accessor, hash40("air_speed_x_stable"), 0) * 1.4;
                //     let run_speed = WorkModule::get_param_float(module_accessor, hash40("run_speed_max"), 0);
                //     let dash_speed = WorkModule::get_param_float(module_accessor, hash40("dash_speed"), 0);
                //     let ground_speed = run_speed.max(dash_speed);
                //     if limit_speed > ground_speed {
                //         limit_speed = ground_speed + ((limit_speed - ground_speed) / 2.0);
                //     }
                //     limit_speed.clamp(1.2, 1.7)
                // }
                // else {
                //     WorkModule::get_param_float(module_accessor, hash40("air_speed_x_stable"), 0)
                // };
                // if limit_speed < energy.speed.x.abs() {
                //     energy.speed = PaddedVec2::new(limit_speed * energy.speed.x.signum(), 0.0);
                // }
                let limit_speed = WorkModule::get_param_float(module_accessor, hash40("air_speed_x_stable"), 0);
                if limit_speed < energy.speed.x.abs() {
                    energy.speed = PaddedVec2::new(limit_speed * energy.speed.x.signum(), 0.0);
                }
            }
            WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
        },
        FlyAdjust => {
            let kind = smash::app::utility::get_kind(module_accessor);

            let fly_data = if let Some(data) = FlyData::get_from_fighter_kind(kind) {
                data
            } else {
                return;
            };

            let _stable_speed = WorkModule::get_param_float(module_accessor, hash40("air_speed_x_stable"), 0);
            let speed = *energy.get_speed();

            let sum = speed.x + speed.y;
            let cap = initial_speed.x * fly_data.speed_x_max_mul;

            if cap.abs() < sum.abs() {
                energy.speed.x = cap.abs() * speed.x.signum();
            }
        }, // not reached in game afaik
        Dash | TurnRun | DashBack => {
            let lr = if reset_type == DashBack {
                -energy.lr
            }
            else {
                energy.lr
            };
            energy.speed.y = 0.0;
            // let dash_speed = lr * WorkModule::get_param_float(module_accessor, hash40("dash_speed"), 0);
            // energy.speed.x = if 0.0 <= lr * energy.speed.x {
            //     dash_speed
            // }
            // else {
            //     dash_speed + energy.speed.x
            // };
            energy.speed.x = lr * WorkModule::get_param_float(module_accessor, hash40("dash_speed"), 0);
        },
        ShootDash => {
            energy.speed.x = if 0.0 <= energy.speed.x * energy.lr {
                energy.lr * WorkModule::get_param_float(module_accessor, hash40("shoot_dash_speed_f"), 0)
            } else {
                energy.speed.x + energy.lr * WorkModule::get_param_float(module_accessor, hash40("shoot_dash_speed_f"), 0)
            };
        },
        ShootBackDash => {
            energy.speed.x = if 0.0 <= energy.speed.x * energy.lr {
                -energy.lr * WorkModule::get_param_float(module_accessor, hash40("shoot_dash_speed_b"), 0)
            } else {
                energy.speed.x - energy.lr * WorkModule::get_param_float(module_accessor, hash40("shoot_dash_speed_b"), 0)
            };
        },
        RevolveSlashAir => {
            energy.speed.y = 0.0;
            energy.speed.x *= WorkModule::get_param_float(module_accessor, hash40("param_special_hi"), hash40("rslash_air_spd_x_mul"));
        },
        Free => {
            energy.speed = PaddedVec2::zeros();
        },
        MoveGround => {
            let new_speed = KineticEnergy::adjust_speed_for_ground_normal(&energy.speed, module_accessor);
            energy.speed = new_speed;
        }
        _ => {}
    }

    energy.initialize(module_accessor);
}

pub fn install() {
    skyline::install_hooks!(
        update,
        initialize,
        setup
    );
}