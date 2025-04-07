use sdl2::pixels::Color;
pub struct Cell {
    pub character : char, //unicode character of the cell
    pub forecolor : Color, //foreground color of the cell
    pub backcolor : Color //background color of the cell
}
impl Cell {
    pub const fn with_default_colors(character : char) -> Self {
        Self {
            character : char,
            forecolor : Color::White,
            backcolor : Color::Black
       }
    }
}
pub struct Renderer {
    rows : u32,
    cols : u32,
    font_size : u16,
    visible_rows : core::ops::Range<u32>, //lower number to higher number
    cells : Vec<Cell>
}
