use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Texture;
//use std::fs;
//use sdl2::tff::init;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::process::Command;
use sdl2::surface::Surface;
use sdl2::rect::Rect;

/* pub fn get_font(font_path : &str) -> Result<Font> {
    let data = std::fs::read(&font_path)?;
    let font = Font::try_from_vec(data).unwrap();
    return OK(font);
} */

pub fn main() {  
    //Creating the window.
    let context = sdl2::init().unwrap();
    let video_system = context.video().unwrap();
    let window = video_system.window("myterm", 640, 480).position(0, 0).build().unwrap();
    let mut canvas : Canvas<Window> = window.into_canvas().present_vsync().build().unwrap();
    let texture_creator = canvas.texture_creator();
    canvas.set_draw_color(Color::RGB(34, 34, 34));
    canvas.clear(); 
    canvas.present();
    let font_context = sdl2::ttf::init().unwrap();
    println!("Finding Fonts");
    //path to font
    let font_path = "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf";
    //loaded font context
    let font = font_context.load_font(font_path, 12).unwrap();
    //rendered text
    let rendering : Surface = font.render("hello world").solid(Color::RGB(255, 255, 255)).unwrap();
    //size of rendered text
    let (width, height) = rendering.size();
    //rectangle covered by renedered text.
    let rect = Rect::new(0, 0, width, height);
    //rendered text as a texture
    let text_texture : Texture = rendering.as_texture(&texture_creator).unwrap();
    //copy rendered text to canvas
    canvas.copy(&text_texture, None, Some(rect));
    canvas.present();
    //let font = get_font(font_path).unwrap();
    let mut paused = false;
    let mut event_pump = context.event_pump().unwrap();
    //let font_output = Command::new("fc-list").args(":spacing=100");
    'running : loop {
        if !paused {
            for event in event_pump.wait_iter(){
                match event {
                    Event::Quit{..} | 
                    Event::KeyDown {keycode : Some(Keycode::Escape), ..} => {
                        println!("Loop Terminating");
                        break 'running;
                    }
                    Event::KeyDown {keycode : Some(Keycode::Space), ..} => {
                        paused = true;
                    }
                    _ => {}
                }
            }
        }
    }
    /* let font_path = "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf"
    let font = sdl2_ttf_context.load_font(font_path, 16);
    let my_str = "Hello World";
    let result = canvas.with_texture_canvas(&mut main_texture, |texture_canvas| {
        texture_canvas.copy(&another_texture, None, None).expect("Cant copy");
    }); */
}


