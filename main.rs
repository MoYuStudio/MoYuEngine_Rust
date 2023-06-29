
use raylib::prelude::*;

fn main() {
    
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("MoYuEngine")
        .build();

    let image = Image::load_image("assets/block/1.png").expect("Failed to load image");
    let texture = rl.load_texture_from_image(&thread, &image).expect("Failed to load texture");

    // 加载窗口图标
    let icon = Image::load_image("assets/block/1.png").expect("Failed to load icon");
    rl.set_window_icon(&icon);

    let tile_size: f32 = 64.0; // 瓷砖的宽度和高度
    let map_size: i32 = 64; // 地图的大小（瓷砖数量）
    let move_speed: f32 = 0.0001 * tile_size as f32 * map_size as f32; // 移动速度
    // let mut zoom_speed: f32 = 0.1;

    let mut map_x: f32 = 0.0;
    let mut map_y: f32 = 0.0;
    let mut scale: f32 = 4.0; // 初始化缩放因子为 1.0

    while !rl.window_should_close() {
        if rl.is_key_down(KeyboardKey::KEY_A) {
            map_x += move_speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            map_x -= move_speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_W) {
            map_y += move_speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            map_y -= move_speed;
        }
        // if rl.is_key_down(KeyboardKey::KEY_Q) {
        //     scale += zoom_speed;
        // }
        // if rl.is_key_down(KeyboardKey::KEY_E) {
        //     scale -= zoom_speed;
        // }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        // d.draw_texture_ex(
        //     &texture,
        //     Vector2::default(),
        //     0.0,
        //     1.0,
        //     Color::WHITE,
        // );

        for x in 0..map_size {
            for y in 0..map_size {
                let tile_x = (x - y) as f32 * tile_size as f32 / 2.0 + map_x;
                let tile_y = (x + y) as f32 * tile_size as f32 / 4.0 + map_y;

                d.draw_texture_ex(
                    &texture,
                    Vector2::new(tile_x, tile_y),
                    0.0,
                    scale,
                    Color::WHITE,
                );
            }
        }
        d.draw_fps(10, 10)
    }
}
