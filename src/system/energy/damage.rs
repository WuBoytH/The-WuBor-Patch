#![allow(non_snake_case)]

use super::*;

#[repr(C)]
pub struct FighterKineticEnergyDamage {
    parent: KineticEnergy,
    padding: u64,
    damage_target_speed: PaddedVec2,
    reset_type: EnergyStopResetType,
    elapsed_hitstop_frames: f32,
    hitstop_frames: f32,
    _xAC: f32,
    _xB0: f32,
    should_sync_damage_speed: bool,
    needs_to_sync_damage_speed: bool,
    should_start_interpolation: bool,
    interpolation_frames_remaining: u8,
    _xB8: u8,
    is_target_pos: bool,
    _xBA: bool,
    stop_over_speed: bool,
    _xBC: u32,
    _xC0: PaddedVec2
}

impl core::ops::Deref for FighterKineticEnergyDamage {
    type Target = KineticEnergy;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl core::ops::DerefMut for FighterKineticEnergyDamage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum EnergyStopResetType {
    Ground = 0x0,
    DamageGround,
    DamageAir,
    DamageAirIce,
    DamageOther,
    DamageKnockBack,
    GlideLanding,
    Air,
    AirXNormalMax,
    AirEscape,
    AirBrake,
    AirBrakeAlways,
    GuardDamage,
    Capture,
    CatchCut,
    ItemSwingDash,
    ItemDashThrow,
    SwimBrake,
    Run,
    RunBrake,
    GlideStart,
    CatchDash,
    ShieldRebound,
    Free,
    CaptureBeetle,
    AirLassoHang,
    AirLassoRewind,
    EscapeAirSlide,
    DamageGroundOrbit,
    DamageAirOrbit,
    None = 0xFFFFFFFF
}

// impl std::fmt::Display for EnergyStopResetType {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::Ground => write!(f, "Ground"),
//             Self::DamageGround => write!(f, "DamageGround"),
//             Self::DamageAir => write!(f, "DamageAir"),
//             Self::DamageAirIce => write!(f, "DamageAirIce"),
//             Self::DamageOther => write!(f, "DamageOther"),
//             Self::DamageKnockBack => write!(f, "DamageKnockBack"),
//             Self::GlideLanding => write!(f, "GlideLanding"),
//             Self::Air => write!(f, "Air"),
//             Self::AirXNormalMax => write!(f, "AirXNormalMax"),
//             Self::AirEscape => write!(f, "AirEscape"),
//             Self::AirBrake => write!(f, "AirBrake"),
//             Self::AirBrakeAlways => write!(f, "AirBrakeAlways"),
//             Self::GuardDamage => write!(f, "GuardDamage"),
//             Self::Capture => write!(f, "Capture"),
//             Self::CatchCut => write!(f, "CatchCut"),
//             Self::ItemSwingDash => write!(f, "ItemSwingDash"),
//             Self::ItemDashThrow => write!(f, "ItemDashThrow"),
//             Self::SwimBrake => write!(f, "SwimBrake"),
//             Self::Run => write!(f, "Run"),
//             Self::RunBrake => write!(f, "RunBrake"),
//             Self::GlideStart => write!(f, "GlideStart"),
//             Self::CatchDash => write!(f, "CatchDash"),
//             Self::ShieldRebound => write!(f, "ShieldRebound"),
//             Self::Free => write!(f, "Free"),
//             Self::CaptureBeetle => write!(f, "CaptureBeetle"),
//             Self::AirLassoHang => write!(f, "AirLassoHang"),
//             Self::AirLassoRewind => write!(f, "AirLassoRewind"),
//             Self::EscapeAirSlide => write!(f, "EscapeAirSlide"),
//             Self::DamageGroundOrbit => write!(f, "DamageGroundOrbit"),
//             Self::DamageAirOrbit => write!(f, "DamageAirOrbit"),
//         }
//     }
// }

#[skyline::hook(offset = 0x6d8100)]
unsafe extern "C" fn damage_initialize(energy: &mut FighterKineticEnergyDamage, module_accessor: &mut BattleObjectModuleAccessor) {
    use EnergyStopResetType::*;
    // println!("[wubor::damage] Damage Energy Init");
    // println!("Reset Type: {:#?}", energy.reset_type);

    original!()(energy, module_accessor);

    // match energy.reset_type {
    //     Ground | CatchCut | ItemSwingDash | ItemDashThrow => {
    //         let mut brake = WorkModule::get_param_float(module_accessor, hash40("ground_brake"), 0);
    //         if energy.reset_type == CatchCut {
    //             brake *= WorkModule::get_param_float(module_accessor, hash40("common"), hash40("capture_cut_brake_mul"));
    //         }
    //         else if energy.reset_type == ItemSwingDash {
    //             brake *= WorkModule::get_param_float(module_accessor, hash40("common"), hash40("item_dash_swing_brake_mul"));
    //         }
    //         else if energy.reset_type == ItemDashThrow {
    //             brake *= WorkModule::get_param_float(module_accessor, hash40("common"), hash40("item_dash_throw_brake_mul"));
    //         }

    //         if energy.stop_over_speed {
    //             brake *= WorkModule::get_param_float(module_accessor, hash40("common"), hash40("stop_over_speed_brake_mul"));
    //         }

    //         energy.speed_brake = PaddedVec2::new(brake, 0.0);
    //     },
    //     DamageGround | GuardDamage | DamageGroundOrbit => {

    //     },
    //     _ => unreachable!()
    // }

    // println!("Brake: {}, {}", energy.speed_brake.x, energy.speed_brake.y);

    if [
        DamageGround,
        GuardDamage,
        DamageGroundOrbit
    ].contains(&energy.reset_type) {
        // println!("Force brake stat!");
        energy.parent.speed_brake.x = 0.06;
    }
}

pub fn install() {
    skyline::install_hooks!(
        damage_initialize
    );
}