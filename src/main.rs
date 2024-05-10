use std::vec;

use macroquad::{miniquad::window::screen_size, prelude::*, rand};

const WINDOW_HEIGHT: i32 = 1000;
const WINDOW_WIDTH: i32 = 1000;
const MAX_ROTATION: f32 = 2.0 * 3.14;

const HEARTS_AMOUNT: usize = 100;
const STARS_AMOUNT: usize = 150;

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

enum WhatDraw {
    Hearts,
    Stars,
}

struct SceneConfig<'a> {
    what_draw: WhatDraw,
    bg_color: Color,
    texture: &'a Texture2D,
    texts: Vec<&'static str>,
    text_colors: (Color, Color),
    colors: Vec<Color>,
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
    for heart in hearts {
        draw_texture(heart_texture, heart.x, heart.y, heart.color);
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

fn draw_texture_at_x_y(
    image_texture: &Texture2D,
    x: f32,
    y: f32,
    rotation: f32,
    y_offset: Option<f32>,
) {
    let texture_y: f32;
    match y_offset {
        None => texture_y = y,
        Some(offset) => texture_y = y + offset,
    }
    draw_texture_ex(
        &image_texture,
        x,
        texture_y,
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

fn recreate_texture_coords(
    window_size: (f32, f32),
    texture: &Texture2D,
    y_offset: f32,
) -> (f32, f32) {
    let x = (window_size.0 - texture.width()) / 2.0;
    let y = (window_size.1 - texture.width()) / 2.0 + y_offset;
    return (x, y);
}

fn recreate_hearts(window_size: (f32, f32), colors: &Vec<Color>) -> Vec<Heart> {
    let mut hearts = vec![];
    for color in colors.iter() {
        let x = rand::gen_range(0.0, window_size.0);
        let y = rand::gen_range(0.0, window_size.1);

        hearts.push(Heart {
            x,
            y,
            color: *color,
        })
    }
    return hearts;
}

fn recreate_stars(window_size: (f32, f32), colors: &Vec<Color>) -> Vec<Star> {
    let mut stars = vec![];
    for color in colors.iter() {
        let x = rand::gen_range(0.0, window_size.0);
        let y = rand::gen_range(0.0, window_size.1);
        stars.push(Star {
            x,
            y,
            color: *color,
            rotation: rand::gen_range(0.0, MAX_ROTATION),
        })
    }
    return stars;
}

fn get_next_font_index(fonts: &Vec<Font>, current_index: usize) -> usize {
    match fonts.get(current_index + 1) {
        None => 0,
        Some(_) => current_index + 1,
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

fn generate_colors(
    amount: usize,
    main_color_range: (f32, f32),
    other_color_multiplier: f32,
    rgb_distribution: (bool, bool, bool),
) -> Vec<Color> {
    let mut colors: Vec<Color> = vec![];
    for _ in 0..amount {
        let main_color =
            rand::gen_range(main_color_range.0, main_color_range.1) / main_color_range.1;
        let other_color =
            rand::gen_range(0.0, main_color * other_color_multiplier) / main_color_range.1;

        colors.push(Color {
            r: if rgb_distribution.0 {
                main_color
            } else {
                other_color
            },
            g: if rgb_distribution.1 {
                main_color
            } else {
                other_color
            },
            b: if rgb_distribution.2 {
                main_color
            } else {
                other_color
            },
            a: 1.0,
        })
    }
    return colors;
}

async fn game() {
    let fonts = vec![
        load_ttf_font("fonts/Swampy Clean.ttf").await.unwrap(),
        load_ttf_font("fonts/MorfinSans-Regular.ttf").await.unwrap(),
        load_ttf_font("fonts/Ramona-Bold.ttf").await.unwrap(),
    ];

    let mut font_index = 0;
    let mut window_size = screen_size();
    let rotation_step = 0.025;

    let y_offset = 150.0;
    let star_image = load_image("images/star.png").await.unwrap();
    let star_texture = Texture2D::from_image(&star_image);
    let heart_image = load_image("images/heart.png").await.unwrap();
    let heart_texture = Texture2D::from_image(&heart_image);
    let mut texture_rotation = 0.0;

    let texture_y_offset: Option<f32> = None;

    let mut scene_index = 0;

    // TEXTURES
    let flower_texture = Texture2D::from_image(&load_image("images/flower.png").await.unwrap());
    let present_texture = Texture2D::from_image(&load_image("images/present.png").await.unwrap());
    let flower_white_texture =
        Texture2D::from_image(&load_image("images/flower-white.png").await.unwrap());
    let flower_blue_texture =
        Texture2D::from_image(&load_image("images/flower-blue.png").await.unwrap());
    let flower_red_texture =
        Texture2D::from_image(&load_image("images/flower-red.png").await.unwrap());
    let sad_texture = Texture2D::from_image(&load_image("images/sad.png").await.unwrap());
    let sun_texture = Texture2D::from_image(&load_image("images/sun.png").await.unwrap());

    let scenes = vec![
        SceneConfig {
            what_draw: WhatDraw::Hearts,
            bg_color: Color {
                r: 0.1,
                g: 0.2,
                b: 0.1,
                a: 1.0,
            },
            texture: &flower_texture,
            texts: vec![
                "Азалька!!!!!",
                "я тебя люблю",
                "очень сильно!",
                "Сегодня я подготовил",
                "для тебя сюрприз <3",
            ],
            text_colors: (WHITE, BLACK),
            colors: generate_colors(HEARTS_AMOUNT, (500.0, 1000.0), 200.0, (false, true, false)),
        },
        SceneConfig {
            what_draw: WhatDraw::Stars,
            bg_color: Color {
                r: 1.0,
                g: 1.0,
                b: 0.9,
                a: 1.0,
            },
            texture: &present_texture,
            texts: vec!["У меня даже есть", "для тебя!!!", "подарок!!!"],
            text_colors: (BLACK, WHITE),
            colors: generate_colors(STARS_AMOUNT, (900.0, 1000.0), 500.0, (true, true, false)),
        },
        SceneConfig {
            what_draw: WhatDraw::Hearts,
            bg_color: Color {
                r: 0.7,
                g: 0.7,
                b: 1.0,
                a: 1.0,
            },
            texture: &flower_white_texture,
            texts: vec!["Но сначала я хочу", "сказать тебе", "какая ты ..."],
            text_colors: (WHITE, BLACK),
            colors: generate_colors(HEARTS_AMOUNT, (700.0, 1000.0), 1000.0, (false, false, true)),
        },
        SceneConfig {
            what_draw: WhatDraw::Stars,
            bg_color: Color {
                r: 0.7,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            texture: &flower_blue_texture,
            texts: vec![
                "Замечательная!",
                "Умная! Красивая!",
                "Добрая! Милая!",
                "Гениальная!",
                "Душещипательная!",
            ],
            text_colors: (WHITE, BLACK),
            colors: generate_colors(STARS_AMOUNT, (500.0, 1000.0), 200.0, (false, true, true)),
        },
        SceneConfig {
            what_draw: WhatDraw::Hearts,
            bg_color: Color {
                r: 1.0,
                g: 0.7,
                b: 0.65,
                a: 1.0,
            },
            texture: &flower_red_texture,

            texts: vec![
                "Я так рад,",
                "что встретил тебя",
                "и продолжаю",
                "встречать",
                "уже целых 2!!!",
                "года!!!",
            ],
            text_colors: (WHITE, BLACK),
            colors: generate_colors(HEARTS_AMOUNT, (700.0, 1000.0), 200.0, (true, false, false)),
        },
        SceneConfig {
            what_draw: WhatDraw::Stars,
            bg_color: Color {
                r: 0.33,
                g: 0.7,
                b: 0.7,
                a: 1.0,
            },
            texture: &sad_texture,

            texts: vec![
                "Если бы не ты",
                "я б так и был",
                "холостым (фуууу)",
                "и одиноким :(((",
            ],
            text_colors: (BLACK, WHITE),
            colors: generate_colors(HEARTS_AMOUNT, (850.0, 1000.0), 1000.0, (true, true, true)),
        },
        SceneConfig {
            what_draw: WhatDraw::Hearts,
            bg_color: Color {
                r: 0.0,
                g: 0.6,
                b: 1.0,
                a: 1.0,
            },
            texture: &sun_texture,
            texts: vec![
                "ТЫ МОЕ СОЛНЫШКО!!!",
                "я тобой очень сильно",
                "горжусь!",
                "И не устаю",
                "тобой удивляться!!!",
            ],
            text_colors: (BLACK, WHITE),
            colors: generate_colors(HEARTS_AMOUNT, (999.0, 1000.0), 500.0, (true, true, true)),
        },
        // SceneConfig {},
    ];

    let mut hearts: Vec<Heart> = recreate_hearts(window_size, &scenes[scene_index].colors);
    let mut stars: Vec<Star> = recreate_stars(window_size, &scenes[scene_index].colors);

    let (mut current_x, mut current_y) =
        recreate_texture_coords(window_size, &scenes[scene_index].texture, y_offset);

    loop {
        let (mouse_x, mouse_y) = mouse_position();
        let current_window_size = screen_size();

        let current_scene = &scenes[scene_index];

        clear_background(current_scene.bg_color);

        if (mouse_x > current_x)
            & (mouse_y > current_y)
            & (mouse_x < current_x + current_scene.texture.width())
            & (mouse_y < current_y + current_scene.texture.height())
            & is_mouse_button_pressed(MouseButton::Left)
        {
            let next_index = scene_index + 1;
            if next_index < scenes.len() {
                scene_index += 1;
                let new_scene = &scenes[scene_index];
                (current_x, current_y) =
                    recreate_texture_coords(window_size, new_scene.texture, y_offset);

                match new_scene.what_draw {
                    WhatDraw::Hearts => {
                        hearts = recreate_hearts(window_size, &new_scene.colors);
                    }
                    WhatDraw::Stars => {
                        stars = recreate_stars(window_size, &new_scene.colors);
                    }
                }
            }
        }

        if current_window_size != window_size {
            window_size = current_window_size;
            (current_x, current_y) =
                recreate_texture_coords(window_size, &current_scene.texture, y_offset);
            hearts = recreate_hearts(window_size, &current_scene.colors);
            stars = recreate_stars(window_size, &current_scene.colors);
        }

        match &current_scene.what_draw {
            WhatDraw::Stars => {
                draw_stars(&star_texture, &stars);
                update_star_positions(&mut stars, star_texture.width(), window_size);
            }
            WhatDraw::Hearts => {
                draw_hearts(&heart_texture, &hearts);
                update_hearts_positions(&mut hearts, heart_image.width as f32, window_size);
            }
        }

        draw_multiline_text_in_the_center(
            &current_scene.texts,
            fonts.get(font_index).unwrap(),
            100,
            window_size,
            y_offset,
            10.0,
            current_scene.text_colors.0,
            current_scene.text_colors.1,
        );

        draw_texture_at_x_y(
            &current_scene.texture,
            current_x,
            current_y,
            texture_rotation,
            texture_y_offset,
        );

        if texture_rotation < MAX_ROTATION {
            texture_rotation += rotation_step;
        } else {
            texture_rotation = 0.0;
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
