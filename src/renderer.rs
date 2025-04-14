use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Texture;
//use std::fs;
//use sdl2::tff::init;
use sdl2::render::Canvas;
use sdl2::video::{Window, WindowContext};
use std::process::Command;
use sdl2::surface::Surface;
use sdl2::rect::Rect;
use core::ops::Range;
#[derive(Clone, Copy)]
pub struct Cell {
    pub character : char, //unicode character of the cell
    pub forecolor : Color, //foreground color of the cell
    pub backcolor : Color //background color of the cell
}
impl Cell {
    #[inline]
    pub const fn with_default_colors(character : char) -> Self {
        Self {
            character,
            forecolor : Color::WHITE,
            backcolor : Color::BLACK
       }
    }
    #[inline]
    pub const fn with_color(character : char, forecolor : Color, backcolor : Color) -> Self {
        Self {
            character,
            forecolor,
            backcolor
        }
    }

    pub const BLACK: Color = Color::RGBA(0, 0, 0, 255);
    pub const RED: Color = Color::RGBA(170, 0, 0, 255);
    pub const GREEN: Color = Color::RGBA(0, 170, 0, 255);
    pub const YELLOW: Color = Color::RGBA(170, 85, 0, 255); //More of a burnt orange
    pub const BLUE: Color = Color::RGBA(0, 0, 170, 255);
    pub const MAGENTA: Color = Color::RGBA(170, 0, 170, 255);
    pub const CYAN: Color = Color::RGBA(0, 170, 170, 255);
    pub const WHITE: Color = Color::RGBA(170, 170, 170, 255);
    pub const BRIGHT_BLACK: Color = Color::RGBA(85, 85, 85, 255); //Gray
    pub const BRIGHT_RED: Color = Color::RGBA(255, 85, 85, 255);
    pub const BRIGHT_GREEN: Color = Color::RGBA(85, 255, 85, 255);
    pub const BRIGHT_YELLOW: Color = Color::RGBA(255, 255, 85, 255); //More of a burnt orange
    pub const BRIGHT_BLUE: Color = Color::RGBA(85, 85, 255, 255);
    pub const BRIGHT_MAGENTA: Color = Color::RGBA(255, 85, 255, 255);
    pub const BRIGHT_CYAN: Color = Color::RGBA(85, 255, 255, 255);
    pub const BRIGHT_WHITE: Color = Color::RGBA(255, 255, 255, 255);
}
//Content to render
pub struct TextCells {
    visible_rows : Range<u32>, //lower number to higher number
    rows : u32, //number text rows in the terminal based on the size of the terminal window
    total_rows : u32, //total numbers of text rows rows in the terminal. 
    // The end of visibles rows should NEVER be greater than total_rows
    cols : u32, //number of text columns in the terminal
    //pub font_size : u16,
    font_width : u16, //font height in pixels
    font_height : u16, //font width in pixels
    current_x : u32, //x location of next character to insert.
    current_y : u32, //y location of next character to insert.
    cells : Vec<Cell>,
    current_forecolor : sdl2::pixels::Color, //foreground color of next cell unless it is changed by ANSI escape code.
    current_backcolor : sdl2::pixels::Color //background color of next cell unless it is changed by ANSI escape code.
}

impl TextCells {
    pub fn new(rows : u32, cols : u32, font_width : u16, font_height : u16) -> Self {
        Self {
            visible_rows : Range {
                start : 0,
                end : 1
            },
            rows,
            total_rows : 1,
            cols,
            font_width,
            font_height,
            current_x : 0,
            current_y : 0,
            cells : Vec::with_capacity((rows * cols) as usize),
            current_forecolor : Color::RGB(255, 255, 255),
            current_backcolor : Color::RGB(34, 34, 34)
        }
    }
    #[inline]
    pub fn rows(&self) -> u32 {
        self.rows
    }
    #[inline]
    pub fn cols(&self) -> u32 {
        self.cols
    }
    #[inline]
    pub fn size(&self) -> (u32, u32) {
        (self.rows, self.cols)
    }
    #[inline]
    pub fn font_height(&self) -> (u16, u16) {
        (self.font_height, self.font_width)
    }
    #[inline]
    pub fn cells(&self) -> &[Cell] {
        &self.cells
    }
    //get all of the visible cells in the terminal
    #[inline]
    pub fn visible_cells(&self) -> &[Cell] {
        let top_row = self.visible_rows.start;
        let bottom_row = self.visible_rows.end;
        &self.cells[((top_row * self.cols) as usize)..((bottom_row * self.cols) as usize)]
    }
    //Adds a  new line to the terminal
    fn new_line(&mut self) {
        self.current_y += 1;
        self.current_x = 0;
        self.total_rows += 1;
        //The first screen isn't full yet.
        if self.total_rows < self.rows {
            self.visible_rows.end += 1;
        } else {
            self.visible_rows.end += 1;
            self.visible_rows.start += 1;
        }
    }
    //Fill rest of line with blank characters and starts new line;
    pub fn early_new_line(&mut self){
        for _ in (self.current_x .. self.cols){
            self.cells.push(Cell::with_color(' ', self.current_forecolor, self.current_backcolor));
        }
        self.new_line();
    }
    //Adds a charcter to the terminal window
    pub fn add_character(&mut self, character : char){
        self.cells.push(Cell::with_color(character, self.current_forecolor, self.current_backcolor));
        self.current_x += 1;
        //check to see if we are at the end of the line, if so, start a new line
        if self.current_x == self.cols {
            self.new_line();
        }
    }
    
    //Do not use this if the string has Ansi escape codes or null chracters; 
    //TODO later check if string is normalized unicode character cell should be in unicode NFC format
    pub fn add_string(&mut self, string : &str){
        for (i, character) in string.chars().enumerate(){
            
        }
    }
    //Changes the foreground/text color
    #[inline]
    pub fn set_forecolor(&mut self, color : sdl2::pixels::Color){
        self.current_forecolor = color;
    }
    //Changes the background color
    #[inline]
    pub fn set_backcolor(&mut self, color : sdl2::pixels::Color){
        self.current_backcolor = color;
    }
}

pub struct RenderWindow {
    //Textture creator for rendering Text
    pub texture_creator : sdl2::render::TextureCreator<WindowContext>,
    //Canvas for window
    pub canvas : sdl2::render::Canvas<Window>,
    //window for the terminal
    //pub window : sdl2::video::Window,
    pub height : u32,
    pub width : u32
}

impl RenderWindow {
    pub fn new(sdl_context : &sdl2::Sdl, width : u32, height :  u32) -> RenderWindow {
        let video_system = sdl_context.video().unwrap();
        let window = video_system.window("myterm", width, height).position(0, 0).build().unwrap();
        let mut canvas : Canvas<Window> = window.into_canvas().present_vsync().build().unwrap();
        let texture_creator = canvas.texture_creator();
        canvas.set_draw_color(Color::RGB(34, 34, 34));
        canvas.clear(); 
        canvas.present();
        Self {
            texture_creator,
            canvas,
            //window,
            height,
            width
        }
    }
    
    pub fn create_text_cells(&self, font_width : u16, font_height : u16) -> TextCells {
        let rows = self.height / (font_height as u32);
        let cols = self.width / (font_width as u32);
        TextCells::new(rows, cols, font_height, font_width)
    }
    
    pub fn render_text_surface(&mut self, text_surface : Surface){
        let (width, height) = text_surface.size();
        println!("{}, {}", width, height);
        let rect = Rect::new(0, 0, width, height);
        let text_texture : Texture = text_surface.as_texture(&self.texture_creator).unwrap();
        self.canvas.copy(&text_texture, None, Some(rect));
        self.canvas.present();
    }
}
