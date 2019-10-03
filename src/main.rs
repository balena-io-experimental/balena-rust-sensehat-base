use sensehat_screen::{ PixelColor, Screen, FrameLine};
// use std::thread; 
// use std::time::Duration;

fn main() {
    println!("Hello, world!");

    let mut screen = Screen::open("/dev/fb1").unwrap();

    // balena logo has pixel colours
    let blue = PixelColor::new(0,207,255);
    let orange = PixelColor::new(251,133,0);
    let pale_orange = PixelColor::new(255,194,0);
    let green = PixelColor::new(0,118,77);
    let yellow = PixelColor::new(246,240,71);
    let dark_yellow = PixelColor::new(255,218,58);

    let logo_raw: [PixelColor;64] = [
        PixelColor::BLACK, PixelColor::BLACK, PixelColor::BLACK,blue,blue, PixelColor::BLACK, PixelColor::BLACK, PixelColor::BLACK,
         PixelColor::BLACK, PixelColor::BLACK,blue,green,green,blue, PixelColor::BLACK, PixelColor::BLACK,
          PixelColor::BLACK,blue,green,green,green,green,blue,PixelColor::BLACK,
          blue,orange,green,green,green,green,pale_orange,blue,
          dark_yellow,orange,orange,green,green,pale_orange,pale_orange,yellow,
          PixelColor::BLACK,dark_yellow,orange,orange,pale_orange,pale_orange,yellow,PixelColor::BLACK,
          PixelColor::BLACK,PixelColor::BLACK,dark_yellow,orange,dark_yellow,yellow,PixelColor::BLACK,PixelColor::BLACK,
          PixelColor::BLACK,PixelColor::BLACK,PixelColor::BLACK,dark_yellow,yellow,PixelColor::BLACK,PixelColor::BLACK,PixelColor::BLACK
    ];


    let logo_frame = FrameLine::from_pixels(&logo_raw);
    screen.write_frame(&logo_frame);
}
