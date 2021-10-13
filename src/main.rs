use raylib::{
    ffi::{
        CheckCollisionRecs, 
        Rectangle as Rect
    }, 
    prelude::*
};

const W_WIDTH: i32 = 1280;
const W_HEIGHT: i32 = 720;
const FPS: u32 = 60;
const HAZSRD_AMOUNT: i32 = 2;
const NEB_MOVEMENT: f32 = -300.0;
const G_FORCE: f32 = 1000.0;
const J_FORCE: f32 = -600.0;

struct AnimData {
    rec: Rectangle,
    pos: Vector2,
    frame: i32,
    frame_count: i32
}

impl AnimData {
    fn sprite_animation(&mut self){
        self.rec.x = self.frame as f32 * self.rec.width;
        self.frame += 1;
        if self.frame > self.frame_count { self.frame = 0 } 
    }
}

fn main() {

    let (mut rl, thread) = raylib::init().size(W_WIDTH, W_HEIGHT).title("Test").build();

    // Important Vars
    let mut velocity = 0.0;
    const UPDATE_TIME: f32 = 1.0 / 12.0;
    let mut animation_time = 0.0;

    // Flags
    let mut is_in_air = false;
    let mut collision = false;
    let mut game_finish = false;

    // Textures
    let background = rl.load_texture(&thread,"assets/far-buildings.png").expect("Error loading background");
    let mut bg_x = 0.0;

    let midground = rl.load_texture(&thread, "assets/back-buildings.png").expect("Error loading midground");
    let mut mg_x = 0.0;

    let foreground = rl.load_texture(&thread, "assets/foreground.png").expect("Error loading foreground");
    let mut fg_x = 0.0;

    let scarfy = rl.load_texture(&thread, "assets/scarfy.png").expect("Error loading Scarfy");

    let mut scarfy_data = AnimData {
        rec: Rectangle::new(0.0, 0.0, (scarfy.width / 6 ) as f32, scarfy.height as f32),
        pos: Vector2::new((W_WIDTH / 2) as f32 - ((scarfy.width / 6 )/2) as f32, (W_HEIGHT - scarfy.height )as f32),
        frame: 0,
        frame_count: 5
    };

    let nebula = rl.load_texture(&thread, "assets/12_nebula_spritesheet.png").expect("Error loading Nebulas");

    let mut hazards: Vec<AnimData> =vec![];

    for x in 0..HAZSRD_AMOUNT {
        let lump = AnimData{
            rec: Rectangle::new(0.0, 0.0, (nebula.width / 8 ) as f32, (nebula.height / 8) as f32),
            pos: Vector2::new((W_WIDTH + (370 * x)) as f32, (W_HEIGHT - (nebula.height / 8) )as f32),
            frame: 0,
            frame_count: 5
        };

        hazards.push(lump);
    }

    let mut finish_line = hazards.last().unwrap().pos.x;


    rl.set_target_fps(FPS);

    while !rl.window_should_close() {

        let dt = rl.get_frame_time();

        if scarfy_data.pos.y >= W_HEIGHT as f32 - scarfy_data.rec.height {
            velocity = 0.0;
            is_in_air = false;
            scarfy_data.pos.y = W_HEIGHT as f32 - scarfy_data.rec.height;
        } else {
            velocity += G_FORCE * dt;
        }

        {
            if rl.is_key_pressed(KeyboardKey::KEY_SPACE) && !is_in_air {
                velocity += J_FORCE;
            }

            if rl.is_key_pressed(KeyboardKey::KEY_R) && (collision || game_finish) {
                collision = false;
                game_finish = false;

                for (x, i) in hazards.iter_mut().enumerate() {
                    i.pos.x = W_WIDTH as f32 + (370.0 * x as f32);
                }

                finish_line = hazards.last().unwrap().pos.x

            } 
        }

        animation_time += dt;

        if animation_time >= UPDATE_TIME {
            animation_time = 0.0;

            scarfy_data.sprite_animation();
            for neb in hazards.iter_mut(){
                neb.sprite_animation();
            }
        }

        if !collision {
            for neb in hazards.iter_mut(){
                neb.pos.x += NEB_MOVEMENT  * dt;
            }
    
            scarfy_data.pos.y += velocity * dt;
            finish_line += NEB_MOVEMENT * dt;
        }

        bg_x -=  20.0 * dt;
        if bg_x <= -background.width as f32 * 4.0 {bg_x = 0.0}

        mg_x -= 40.0 * dt;
        if mg_x <= -midground.width as f32 * 3.2 {mg_x = 0.0}

        fg_x -= 80.0 * dt;
        if fg_x <= -foreground.width as f32 * 2.4 {fg_x = 0.0}

        let bg_pos = vec![
            Vector2::new(bg_x, 0.0),
            Vector2::new(bg_x + background.width as f32 * 4.0, 0.0),
            Vector2::new(bg_x + (background.width as f32 * 4.0) * 2.0, 0.0)
        ];

        let mg_pos = vec![
            Vector2::new(mg_x, 105.0),
            Vector2::new(mg_x + midground.width as f32 * 3.2, 105.0),
            Vector2::new(mg_x + (midground.width as f32 * 3.2) * 2.0, 105.0)
        ];

        let fg_pos = vec![
            Vector2::new(fg_x, 280.0),
            Vector2::new(fg_x + foreground.width as f32 * 2.4, 280.0),
            Vector2::new(fg_x + (foreground.width as f32 * 2.4) * 2.0, 280.0)
        ];

        for neb in hazards.iter() {
            let pad = 40.0;

            let neb_rec = Rect {
                x: neb.pos.x + pad, 
                y: neb.pos.y + pad,
                width: neb.rec.width - 2.0 * pad, 
                height: neb.rec.height - 2.0 * pad
            };

            let scarfy_rec = Rect {
                x: scarfy_data.pos.x, 
                y: scarfy_data.pos.y,
                width: scarfy_data.rec.width, 
                height: scarfy_data.rec.height
            };

            unsafe{
                if CheckCollisionRecs(neb_rec, scarfy_rec) {
                    collision = true
                }
            }

        }

        if finish_line <= scarfy_data.pos.x {
            game_finish = true;
            collision = true;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::GRAY);

        for x in bg_pos.iter() {
            d.draw_texture_ex(&background, x, 0.0, 4.0, Color::WHITE);
        }

        for x in mg_pos.iter(){
            d.draw_texture_ex(&midground, x,0.0,3.2,Color::WHITE);
        }

        for x in fg_pos.iter(){
            d.draw_texture_ex(&foreground, x,0.0,2.4,Color::WHITE);
        }


        if collision {
            if game_finish {
                d.draw_text("You Win", W_WIDTH/2,W_HEIGHT/2, 20, Color::WHITE);
            } else {
                d.draw_text("Game over! Press R", W_WIDTH/2,W_HEIGHT/2, 20, Color::WHITE);
            }
        } else{

            d.draw_texture_rec(&scarfy, scarfy_data.rec, scarfy_data.pos, Color::WHITE);
            
            for neb in hazards.iter() {
                d.draw_texture_rec(&nebula,neb.rec,neb.pos, Color::WHITE);
            }
        }

        
    }
}