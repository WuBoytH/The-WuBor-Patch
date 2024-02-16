use skyline::hooks::InlineCtx;

#[skyline::from_offset(0x37a1ef0)]
unsafe fn set_text_string(pane: u64, string: *const u8);

// unsafe fn get_pane_by_name(arg: u64, arg2: *const u8) -> [u64; 4] {
//     let func_addr = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x37752e0);
//     let callable: extern "C" fn(u64, *const u8, ...) -> [u64; 4] = std::mem::transmute(func_addr);
//     callable(arg, arg2)
// }

// unsafe fn set_room_text(arg: u64, string: String) {
//     let func_addr = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x3778c50);
//     let callable: extern "C" fn(u64, *const u8, usize, *const u16, ...) = std::mem::transmute(func_addr);
//     callable(arg, b"mnu_online_room_inside_room_id\0".as_ptr(), 1, string.encode_utf16().collect::<Vec<u16>>().as_ptr())
// }

static mut CURRENT_PANE_HANDLE: usize = 0;
static mut CURRENT_ARENA_ID: String = String::new();
static mut CURRENT_INPUT_BUFFER: isize = -1;
static mut MOST_RECENT_AUTO: isize = -1;

const MAX_INPUT_BUFFER: isize = 25;
const MIN_INPUT_BUFFER: isize = -1;

#[skyline::hook(offset = 0x18881d0, inline)]
unsafe fn non_hdr_update_room_hook(_: &skyline::hooks::InlineCtx) {
    static mut CURRENT_COUNTER: usize = 0;
    if ninput::any::is_press(ninput::Buttons::RIGHT) {
        if CURRENT_COUNTER == 0 {
            CURRENT_INPUT_BUFFER += 1;
        }
        CURRENT_COUNTER = (CURRENT_COUNTER + 1) % 10;
    } else if ninput::any::is_press(ninput::Buttons::LEFT) {
        if CURRENT_COUNTER == 0 {
            CURRENT_INPUT_BUFFER -= 1;
        }
        CURRENT_COUNTER = (CURRENT_COUNTER + 1) % 10;
    } else {
        CURRENT_COUNTER = 0;
    }

    CURRENT_INPUT_BUFFER = CURRENT_INPUT_BUFFER.clamp(MIN_INPUT_BUFFER, MAX_INPUT_BUFFER);
    if CURRENT_INPUT_BUFFER == -1 {
        if MOST_RECENT_AUTO == -1 {
            set_text_string(
                CURRENT_PANE_HANDLE as u64,
                format!("{}\nInput Latency: Auto\0", CURRENT_ARENA_ID).as_ptr(),
            );
        } else {
            set_text_string(
                CURRENT_PANE_HANDLE as u64,
                format!("{}\nInput Latency: Auto ({})\0", CURRENT_ARENA_ID, MOST_RECENT_AUTO).as_ptr()
            )
        }
    } else {
        set_text_string(CURRENT_PANE_HANDLE as u64, format!("{}\nInput Latency: {}\0", CURRENT_ARENA_ID, CURRENT_INPUT_BUFFER).as_ptr());
    }
}

#[skyline::hook(offset = 0x188702c, inline)]
unsafe fn non_hdr_set_room_id(ctx: &skyline::hooks::InlineCtx) {
    let panel = *((*((*ctx.registers[0].x.as_ref() + 8) as *const u64) + 0x10) as *const u64);
    CURRENT_PANE_HANDLE = panel as usize;
    CURRENT_ARENA_ID = dbg!(String::from_utf16(std::slice::from_raw_parts(*ctx.registers[3].x.as_ref() as *const u16, 5)).unwrap());
}

static mut IS_USABLE: bool = false;

#[skyline::hook(offset = 0x16ccc58, inline)]
unsafe fn non_hdr_set_online_latency(ctx: &InlineCtx) {
    let auto = *(*ctx.registers[19].x.as_ref() as *mut u8);
    if IS_USABLE {
        MOST_RECENT_AUTO = auto as isize;
        if CURRENT_INPUT_BUFFER != -1 {
            *(*ctx.registers[19].x.as_ref() as *mut u8) = CURRENT_INPUT_BUFFER as u8;
        }
    }
}

#[skyline::hook(offset = 0x22d9cd4, inline)]
unsafe fn online_melee_any_scene_create(_: &InlineCtx) {
    IS_USABLE = false;
}

#[skyline::hook(offset = 0x22d9c04, inline)]
unsafe fn bg_matchmaking_seq(_: &InlineCtx) {
    IS_USABLE = false;
}

#[skyline::hook(offset = 0x22d9b34, inline)]
unsafe fn arena_seq(_: &InlineCtx) {
    IS_USABLE = true;
}

#[skyline::hook(offset = 0x235a630, inline)]
unsafe fn main_menu(_: &InlineCtx) {
    IS_USABLE = false;
}

pub fn install() {
    skyline::install_hooks!(
        non_hdr_set_room_id,
        non_hdr_update_room_hook,
        non_hdr_set_online_latency,
        online_melee_any_scene_create,
        bg_matchmaking_seq,
        arena_seq,
        main_menu
    );
}