// To print a character to the screen in VGA text mode, 
// one has to write it to the text buffer of the VGA hardware. 
// The VGA text buffer is a two-dimensional array with typically 25 rows 
// and 80 columns, which is directly rendered to the screen. 
// Each array entry describes a single screen character.

// Colors representation 
// C-ike enum 
#[allow(dead_code)] // allow unused variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)] // each enum variant is stored as a u8
pub enum Color {
  Black = 0,
  Blue = 1,
  Green = 2,
  Cyan = 3,
  Red = 4,
  Magenta = 5,
  Brown = 6,
  LightGray = 7,
  DarkGray = 8,
  LightBlue = 9,
  LightGreen = 10,
  LightCyan = 11,
  LightRed = 12,
  Pink = 13,
  Yellow = 14,
  White = 15,
}

// Represent foreground and background color 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// To ensure that the ColorCode has the exact same data layout as a u8
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
  fn new(foreground: Color, background: Color) -> ColorCode {
    ColorCode((background as u8 ) << 4 | (foreground as u8))
  }
}

// represent screen char
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
  ascii_character: u8,
  color_code: ColorCode
}

const BUFFER_HEIGHT : usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
  chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

//To actually write to screen, we now create a writer type
//The writer will always write to the last line and shift lines 
//up when a line is full (or on \n). The column_position field keeps 
//track of the current position in the last row.
pub struct Writer {
  column_position: usize,
  color_code: ColorCode,
  buffer: &'static mut Buffer
}

// Use the Writer to modify the buffer’s characters
impl Writer {
  pub fn write_byte(&mut self, byte: u8) {
    match byte {
      b'\n' => self.new_line(),
      byte => {
        if self.column_position >= BUFFER_WIDTH {
          self.new_line();
        }
        let row = BUFFER_HEIGHT -1;
        let col = self.column_position;

        let color_code = self.color_code;
        self.buffer.chars[row][col] = ScreenChar {
          ascii_character: byte,
          color_code,
        };
        self.column_position+=1;
      }
    }
  }
  fn new_line(&mut self) {
    /*TODO */
  }
}

