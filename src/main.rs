use std::sync::Arc;

use macroquad::{
    miniquad::window::{screen_size, set_mouse_cursor},
    prelude::*,
    rand, text,
};

const WINDOW_HEIGHT: i32 = 1000;
const WINDOW_WIDTH: i32 = 1000;
const MAX_ROTATION: f32 = 2.0 * 3.14;

struct Heart {
    x: f32,
    y: f32,
    color: Color,
}

struct Star {
    x: f32,
    y: f32,
    color: Color,
    rotation: f32,
}

enum Scene {
    First,
    Second,
    Third,
    // Fourth,
    // Fifth,
    // Sixth,
    // Seventh,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Love".to_string(),
        window_height: WINDOW_HEIGHT,
        window_width: WINDOW_WIDTH,
        window_resizable: true,
        ..Default::default()
    }
}

fn draw_hearts(heart_texture: &Texture2D, hearts: &Vec<Heart>) {
    for pos in hearts {
        draw_texture(heart_texture, pos.x, pos.y, pos.color);
    }
}

fn update_hearts_positions(hearts: &mut Vec<Heart>, heart_size: f32, window_size: (f32, f32)) {
    for heart in hearts {
        // let mut new_x = heart.x + rand::gen_range(-100, 100) as f32 / 100.0;
        let mut new_x = heart.x + 0.1;
        let mut new_y = heart.y + rand::gen_range(0, 50) as f32 / 10.0;

        if new_x > window_size.0 {
            new_x = -heart_size;
        }

        if new_y > window_size.1 {
            new_y = -heart_size;
        }

        heart.x = new_x;
        heart.y = new_y;
    }
}

fn draw_texture_at_x_y(image_texture: &Texture2D, x: f32, y: f32, rotation: f32) {
    draw_texture_ex(
        &image_texture,
        x,
        y,
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        },
        DrawTextureParams {
            rotation,
            ..Default::default()
        },
    );
}

fn draw_text_outline(
    text: &str,
    x: f32,
    y: f32,
    size: f32,
    outline_offset: (f32, f32),
    primary_color: Color,
    outline_color: Color,
    font: &Font,
) {
    draw_text_ex(
        text,
        x + outline_offset.0,
        y + outline_offset.1,
        TextParams {
            font: Some(font),
            font_size: size as u16,
            color: outline_color,
            ..Default::default()
        },
    );
    draw_text_ex(
        text,
        x,
        y,
        TextParams {
            font: Some(font),
            font_size: size as u16,
            color: primary_color,
            ..Default::default()
        },
    );
}

fn draw_line_of_text_in_the_center(
    text: &str,
    font: &Font,
    font_size: u16,
    window_size: (f32, f32),
    y_offset: f32,
    first_color: Color,
    second_color: Color,
) -> f32 {
    let font_scale = 1.0;
    let text_size = measure_text(&text, Some(font), font_size, font_scale);

    draw_text_outline(
        text,
        (window_size.0 - text_size.width) / 2.0,
        y_offset,
        font_size as f32,
        (5.0, 5.0),
        first_color,
        second_color,
        &font,
    );

    return text_size.height;
}

fn draw_multiline_text_in_the_center(
    texts: &Vec<&str>,
    font: &Font,
    font_size: u16,
    window_size: (f32, f32),
    y_offset: f32,
    lines_spacing: f32,
    first_color: Color,
    second_color: Color,
) {
    let mut line_offset = 0.0;
    for text in texts {
        let text_height = draw_line_of_text_in_the_center(
            text,
            &font,
            font_size,
            window_size,
            y_offset + line_offset,
            first_color,
            second_color,
        );
        line_offset += text_height + lines_spacing;
    }
}

fn recreate_heart_game_variables(
    window_size: (f32, f32),
    flower_texture: &Texture2D,
    y_offset: f32,
) -> (f32, f32, Vec<Heart>) {
    let flower_x = (window_size.0 - flower_texture.width()) / 2.0;
    let flower_y = (window_size.1 - flower_texture.width()) / 2.0 + y_offset;
    let mut hearts = vec![];
    for _ in 0..100 {
        let x = rand::gen_range(0.0, window_size.0);
        let y = rand::gen_range(0.0, window_size.1);

        let main_color = rand::gen_range(500.0, 1000.0) / 1000.0;
        let other_color = rand::gen_range(0.0, main_color * 1000.0 / 5.0) / 1000.0;

        hearts.push(Heart {
            x,
            y,
            color: Color {
                r: other_color,
                g: main_color,
                b: other_color,
                a: 1.0,
            },
        })
    }
    return (flower_x, flower_y, hearts);
}

fn recreate_star_game_variables(
    window_size: (f32, f32),
    present_texture: &Texture2D,
    y_offset: f32,
) -> (f32, f32, Vec<Star>) {
    let present_x = (window_size.0 - present_texture.width()) / 2.0;
    let present_y = (window_size.1 - present_texture.width()) / 2.0 + y_offset;
    let mut stars = vec![];
    for _ in 0..150 {
        let x = rand::gen_range(0.0, window_size.0);
        let y = rand::gen_range(0.0, window_size.1);

        let main_color = rand::gen_range(800.0, 1000.0) / 1000.0;
        let other_color = rand::gen_range(0.0, main_color * 300.0) / 1000.0;

        stars.push(Star {
            x,
            y,
            color: Color {
                r: main_color,
                g: main_color,
                b: other_color,
                a: 1.0,
            },
            rotation: rand::gen_range(0.0, MAX_ROTATION),
        })
    }
    return (present_x, present_y, stars);
}

fn get_next_font_index(fonts: &Vec<Font>, current_index: usize) -> usize {
    match fonts.get(current_index + 1) {
        None => 0,
        Some(_) => current_index + 1,
    }
}

fn draw_stars(star_texture: &Texture2D, stars: &Vec<Star>) {
    for star in stars {
        draw_texture_ex(
            star_texture,
            star.x,
            star.y,
            star.color,
            DrawTextureParams {
                rotation: star.rotation,
                ..Default::default()
            },
        );
    }
}

fn update_star_positions(stars: &mut Vec<Star>, star_size: f32, window_size: (f32, f32)) {
    for star in stars {
        let mut new_x = star.x + 0.1;
        let mut new_y = star.y + rand::gen_range(0, 50) as f32 / 10.0;

        if new_x > window_size.0 {
            new_x = -star_size;
        }

        if new_y > window_size.1 {
            new_y = -star_size;
        }

        star.x = new_x;
        star.y = new_y;

        let rotation_step = rand::gen_range(0.0, 0.05);
        if (star.rotation + rotation_step) < MAX_ROTATION {
            star.rotation += rotation_step;
        } else {
            star.rotation = 0.0;
        }
    }
}

async fn game() {
    let mut current_scene = Scene::First;

    let fonts = vec![
        load_ttf_font("fonts/Swampy Clean.ttf").await.unwrap(),
        load_ttf_font("fonts/MorfinSans-Regular.ttf").await.unwrap(),
        load_ttf_font("fonts/Ramona-Bold.ttf").await.unwrap(),
    ];

    let mut font_index = 0;
    let mut window_size = screen_size();
    let rotation_step = 0.025;

    let mut bg_color = Color {
        r: 0.1,
        g: 0.2,
        b: 0.1,
        a: 1.0,
    };
    let y_offset = 150.0;
    let star_image = load_image("images/star.png").await.unwrap();
    let star_texture = Texture2D::from_image(&star_image);
    let heart_image = load_image("images/heart.png").await.unwrap();
    let heart_texture = Texture2D::from_image(&heart_image);
    let mut flower_rotation = 0.0;

    // FIRST SCENE
    let flower_image = load_image("images/flower.png").await.unwrap();
    let flower_texture = Texture2D::from_image(&flower_image);

    let (mut flower_x, mut flower_y, mut hearts) =
        recreate_heart_game_variables(window_size, &flower_texture, y_offset);

    // SECOND SCENE
    let present_image = load_image("images/present.png").await.unwrap();
    let present_texture = Texture2D::from_image(&present_image);

    let (mut present_x, mut present_y, mut stars) =
        recreate_star_game_variables(window_size, &present_texture, y_offset);

    // THIRD SCENE
    let flower2_image = load_image("images/flower-white.png").await.unwrap();
    let flower2_texture = Texture2D::from_image(&flower2_image);

    loop {
        let (mouse_x, mouse_y) = mouse_position();
        let current_window_size = screen_size();

        clear_background(bg_color);

        match current_scene {
            Scene::First => {
                draw_hearts(&heart_texture, &hearts);
                update_hearts_positions(&mut hearts, heart_image.width as f32, window_size);
                draw_texture_at_x_y(&flower_texture, flower_x, flower_y, flower_rotation);

                draw_multiline_text_in_the_center(
                    &vec!["Азалия!", "я тебя люблю", "очень сильно!"],
                    fonts.get(font_index).unwrap(),
                    100,
                    window_size,
                    y_offset,
                    10.0,
                    WHITE,
                    BLACK,
                );

                if (mouse_x > flower_x)
                    & (mouse_y > flower_y)
                    & (mouse_x < flower_x + flower_texture.width())
                    & (mouse_y < flower_y + flower_texture.height())
                {
                    if is_mouse_button_pressed(MouseButton::Left) {
                        current_scene = Scene::Second;
                        bg_color = Color {
                            r: 1.0,
                            g: 1.0,
                            b: 0.9,
                            a: 1.0,
                        }
                    }
                }
            }

            Scene::Second => {
                draw_stars(&star_texture, &stars);
                update_star_positions(&mut stars, star_texture.width(), window_size);
                draw_texture_at_x_y(&present_texture, present_x, present_y, 0.0);

                draw_multiline_text_in_the_center(
                    &vec!["У меня", "для тебя", "подарок!!!"],
                    // &vec!["AAAAAA", "AAAAAAAAAAAAAA", "AAAAA"],
                    fonts.get(font_index.to_owned()).unwrap(),
                    100,
                    window_size,
                    y_offset,
                    10.0,
                    BLACK,
                    WHITE,
                );

                if (mouse_x > present_x)
                    & (mouse_y > present_y)
                    & (mouse_x < present_x + present_texture.width())
                    & (mouse_y < present_y + present_texture.height())
                    & is_mouse_button_pressed(MouseButton::Left)
                {
                    current_scene = Scene::Third;
                    bg_color = Color {
                        r: 0.7,
                        g: 0.7,
                        b: 1.0,
                        a: 1.0,
                    };
                    for heart in hearts.iter_mut() {
                        let main_color = rand::gen_range(900.0, 1000.0) / 1000.0;
                        let other_color = rand::gen_range(0.0, main_color * 1000.0) / 1000.0;

                        heart.color = Color {
                            r: other_color,
                            g: other_color,
                            b: main_color,
                            a: 1.0,
                        }
                    }
                }
            }

            Scene::Third => {
                draw_hearts(&heart_texture, &hearts);
                update_hearts_positions(&mut hearts, heart_image.width as f32, window_size);
                draw_texture_at_x_y(&flower2_texture, flower_x, flower_y, flower_rotation);

                draw_multiline_text_in_the_center(
                    &vec!["Но сначала я хочу", "сказать тебе", "какая ты ..."],
                    fonts.get(font_index).unwrap(),
                    100,
                    window_size,
                    y_offset,
                    10.0,
                    WHITE,
                    BLACK,
                );

                if (mouse_x > flower_x)
                    & (mouse_y > flower_y)
                    & (mouse_x < flower_x + flower_texture.width())
                    & (mouse_y < flower_y + flower_texture.height())
                {
                    if is_mouse_button_pressed(MouseButton::Left) {
                        current_scene = Scene::Third;
                        bg_color = Color {
                            r: 1.0,
                            g: 1.0,
                            b: 0.9,
                            a: 1.0,
                        }
                    }
                }
            }
        }

        if current_window_size != window_size {
            window_size = current_window_size;
            (flower_x, flower_y, hearts) =
                recreate_heart_game_variables(window_size, &flower_texture, y_offset);
            (present_x, present_y, stars) =
                recreate_star_game_variables(window_size, &present_texture, y_offset)
        }

        if flower_rotation < MAX_ROTATION {
            flower_rotation += rotation_step;
        } else {
            flower_rotation = 0.0;
        }

        if rand::gen_range(0, 100) > 97 {
            font_index = get_next_font_index(&fonts, font_index);
        }

        next_frame().await
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    game().await
}
