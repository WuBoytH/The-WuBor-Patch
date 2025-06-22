use crate::imports::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct RollerInkParam {
    pub enabled: bool,
    pub padding: [u8; 0xF],
    pub raycheck_pos: Vector2f,
    pub some_vec: Vector2f,
    pub paint_size: Vector2f,
    pub unk: u64,
    pub paint_life: f32,
    pub padding2: [u8; 0xc]
}

#[repr(C)]
struct FighterInkling {
    pub padding: [u8; 0x70],
    pub ink_params: [RollerInkParam; 10]
}

#[skyline::hook(replace = FighterSpecializer_Inkling::request_paint)]
unsafe extern "C" fn request_paint(
    fighter: &mut Fighter,
    bone: Hash40,
    offset: *const Vector3f,
    paint_size: *const Vector2f,
    raycheck_height: f32
) {
    let module_accessor = fighter.battle_object.module_accessor;
    let kind = fighter.battle_object.kind as i32;
    let ink_const = FighterSpecializer_Inkling::get_ink_work_id(kind);
    let ink = WorkModule::get_float(module_accessor, ink_const);
    if 0.0 < ink {
        let pos = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};
        ModelModule::joint_global_position_with_offset(
            module_accessor,
            bone,
            offset,
            pos,
            true
        );
        let raycheck_pos = &mut Vector2f{x: 0.0, y: 0.0};
        let line = GroundModule::ray_check_get_line_hit_pos_ignore_any(
            module_accessor,
            &Vector2f{x: pos.x, y: pos.y + 0.01},
            &Vector2f{x: 0.0, y: -raycheck_height},
            raycheck_pos,
            0x700
        );
        if line != 0 {
            let fighterinkling: &mut FighterInkling = std::mem::transmute(fighter);
            for x in 0..10 {
                let ink_param = &mut fighterinkling.ink_params[x];
                if !ink_param.enabled {
                    println!("enabling ink params for index {}", x);
                    ink_param.enabled = true;
                    ink_param.raycheck_pos = *raycheck_pos;

                    ink_param.some_vec = Vector2f{x: 0.0, y: 1.0};

                    ink_param.paint_size = *paint_size;

                    ink_param.unk = 0;

                    let paint_life = WorkModule::get_param_float(module_accessor, hash40("param_private"), hash40("paint_life"));
                    ink_param.paint_life = paint_life;

                    println!("{:#?}", ink_param);

                    fighterinkling.ink_params[x] = *ink_param;
                    break;
                }
            }
        }
    }

    // original!()(fighter, bone, offset, paint_size, raycheck_height);
    // let fighterinkling: &mut FighterInkling = std::mem::transmute(fighter);
    // for x in 0..10 {
    //     let ink_param = &mut fighterinkling.ink_params[x];
    //     if ink_param.enabled {
    //         println!("Ink Params {} enabled", x);
    //         println!("{:#?}", ink_param);
    //     }
    // }
}

pub fn install() {
    // Patches out the stage check for FighterSpecializer_Inkling::request_paint
    // let _ = skyline::patching::Patch::in_text(0xb0e5ac).nop();

    skyline::install_hooks!(
        request_paint
    );
}