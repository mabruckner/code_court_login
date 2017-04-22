use std::cmp;
use std::thread;
use std::time::Duration;

use std::io;
use std::io::prelude::*;

use image::{
    GenericImage,
    Pixel
};

#[derive(Debug)]
pub enum Alignment {
    Left,
    Center,
    Right
}


pub struct Printer<W:Write> {
    writer: W
}

impl <W:Write> Printer<W> {
    pub fn new(writer: W) -> Printer<W> {
        Printer {
            writer: writer
        }
    }
    pub fn write_text(&mut self, text: &str) -> Result<(), io::Error> {
        self.writer.write(text.as_bytes())?;
        self.writer.flush()
    }

    pub fn write_pix_image<P:Pixel<Subpixel=u8>, I:GenericImage<Pixel=P>>(&mut self, img: &I) -> Result<(), io::Error> {
        let (ix, iy, ax, ay) = img.bounds();
        self.write_pix_array((iy..ay).map(|y| (ix..ax).map(move |x| img.get_pixel(x, y).to_luma().data[0] < 128)))
    }

    pub fn write_bit_image<P:Pixel<Subpixel=u8>, I:GenericImage<Pixel=P>>(&mut self, img: &I) -> Result<(), io::Error> {
        let (ix, iy, ax, ay) = img.bounds();
        self.write_bit_array((iy..ay).map(|y| (ix..ax).map(move |x| img.get_pixel(x, y).to_luma().data[0] < 128)))
    }

    pub fn write_pix_array<P:Iterator<Item=bool>, R:Iterator<Item=P>>(&mut self, mut it: R) -> Result<(), io::Error> {
        self.writer.write(&[0x1B, 0x33, 0x01])?;
        while self.write_pix_chunk(&mut it)? { }
        self.writer.write(&[0x1B, 0x32])?;
        Ok(())
    }

    pub fn write_bit_array<P:Iterator<Item=bool>, R:Iterator<Item=P>>(&mut self, mut it: R) -> Result<(), io::Error> {
        let mut array = Vec::new();
        for row in it {
            let mut rarray = Vec::new();
            for x in row {
                rarray.push(x);
            }
            array.push(rarray);
        }
        let height = cmp::min(array.len(), 255);
        let width = cmp::min((array[0].len()+7) / 8, 48);
        let mut data = Vec::new();
        for y in 0..height {
            for i in 0..width {
                let mut thing = 0;
                for j in 0..8 {
                    let x = i*8 + j;
                    thing = thing << 1;
                    if let Some(v) = array.get(y) {
                        if Some(&true) == v.get(x) {
                            thing = thing | 1;
                        }
                    }
                }
                data.push(thing);
            }
        }
        self.writer.write(&[0x1D, 0x76, 0x30, 0x00, width as u8, 0x00, height as u8, 0x00])?;
        self.writer.write(&data)?;
        Ok(()) 
    }
    
    pub fn print_saved_image(&mut self) -> Result<(), io::Error> {
        thread::sleep(Duration::from_millis(500));
        self.writer.write(&[0x1D, 0x2F, 0x00])?;
        self.writer.flush()?;
        Ok(())
    }

    pub fn write_pix_chunk<P:Iterator<Item=bool>, R:Iterator<Item=P>>(&mut self, it: &mut R) -> Result<bool, io::Error> {
        let mut data = Vec::new();
        for i in 0..8 {
            if let Some(row) = it.next() {
                for (j, val) in row.enumerate() {
                    if j == data.len() {
                        data.push(0);
                    }
                    if val {
                        data[j] = data[j] | (0x80 >> i);
                    }
                }
            } else {
                if data.len() > 0 {
                    self.write_chunk_vec(&data)?;
                }
                return Ok(false)
            }
        }
        self.write_chunk_vec(&data)?;
        Ok(true)
    }
    fn write_chunk_vec(&mut self, data: &Vec<u8>) -> Result<(), io::Error> {
        println!("{:?}", data);
        let width = cmp::min(data.len(), 32*8);
        self.writer.write(&[0x1B, 0x2A, 0, (width%256) as u8, (width/256) as u8])?;
        self.writer.write(&data[0..width])?;
        self.writer.flush()?;
        self.writer.write(&['\n' as u8])?;
        thread::sleep(Duration::from_millis(150));
        Ok(())
    }

    pub fn write_test_image(&mut self) -> Result<(), io::Error> {
        self.write_pix_chunk(&mut (0..9).map(|_| { (0..20).map(|x| x%2 == 0)}))?;
        Ok(())
    }

    pub fn align(&mut self, alignment: Alignment) -> Result<(), io::Error> {
        self.writer.write(&[0x1B, 0x61, match alignment {
            Alignment::Left => 0,
            Alignment::Center => 1,
            Alignment::Right => 2
        }])?;
        Ok(())
    }

    fn setting(&mut self, setting: u8, set: bool) -> Result<(), io::Error> {
        self.writer.write(&[0x1B, setting, match set {
            true => 0x01,
            false=> 0x00
        }])?;
        Ok(())
    }

    pub fn emphasis(&mut self, set: bool) -> Result<(), io::Error> {
        self.setting(0x45, set)
    }

    pub fn underline(&mut self, set: bool) -> Result<(), io::Error> {
        self.setting(0x2D, set)
    }

    pub fn write_test_image_b(&mut self) -> Result<(), io::Error> {
        self.writer.write(&[0x1D, 0x76, 0x30, 0x00, 0x03, 0x00, 0x08, 0x00, 0x3f])?;
        self.writer.write(&[0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f])?;
        self.writer.write(&[0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f])?;
        self.writer.write(&[0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f])?;
        /*self.writer.write(&[0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f])?;
        self.writer.write(&[0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f])?;
        self.writer.write(&[0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f])?;
        self.writer.write(&[0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f])?;
        self.writer.write(&[0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f])?;*/
        self.write_text("\n");
        self.print_saved_image()?;
        Ok(())
    }
}
