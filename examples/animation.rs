use macroquad::prelude::*;
use macroquad_tantan_toolbox::animation::*;

#[derive(std::hash::Hash, Eq, PartialEq)]
enum MooseAnimationIdentifier {
    Run,
}
struct MouseAni {
    pub ani: AnimationInstance::<MooseAnimationIdentifier>,
    pub pos: Vec2
}

#[macroquad::main("animation")]
async fn main() {
    request_new_screen_size(1280., 720.);
    let texture: Texture2D = load_texture("examples/resources/e.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);

    let mut animations: Vec<MouseAni> = vec![];

    let render_target2 = render_target(1280, 720);
    let mut camera = Camera2D::from_display_rect(
        Rect::new(0., 0., 1280., 720.)
    );
    camera.zoom.y = -camera.zoom.y; // uncomment this to fix
    camera.render_target = Some(render_target2); // or comment out this to fix
    loop {
        set_camera(&camera);
        clear_background(BLACK);

        if is_mouse_button_pressed(MouseButton::Left) {
            let mut animation = AnimationInstance::<MooseAnimationIdentifier>::new(
                6f32,
                5f32,
                texture.clone(),
                MooseAnimationIdentifier::Run,
            );
            animation.add_animation(0, 30, None, 45f32, MooseAnimationIdentifier::Run);
            animations.push(
                MouseAni { 
                    ani: animation, 
                    pos: mouse_position().into()
                }
            );
        }

        animations.retain(|ma| ma.ani.is_end() == false);

        for ma in animations.iter_mut() {
            ma.ani.update(get_frame_time());
            ma.ani.draw(&ma.pos, false, Some(Vec2::ONE), WHITE);
        }

        /* animation.update(get_frame_time());
        animation.draw(&vec2(500f32, 500f32), false, WHITE); */

        set_default_camera();
        clear_background(BLACK);
        // draw game
        draw_texture_ex(
            &camera.render_target.as_ref().unwrap().texture,
            screen_width() * 0.5,
            screen_height() * 0.5,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(1280., 720.)),
                ..Default::default()
            },
        );
        draw_text(
            "tap space to change animation",
            screen_width() * 0.5f32 - 100f32,
            40f32,
            30f32,
            BLACK,
        );

        next_frame().await
    }
}
