extern crate image;
extern crate url;
extern crate qrcode;

#[macro_use]
extern crate nom;

mod printer;
mod parser;

use url::Url;

use parser::{Field,Command};
use printer::Printer;

use io::prelude::*;
use std::fs::File;
use std::env;
use std::io;
use std::error;

fn print_commands<I:Iterator<Item=Command>, W: Write>(it: I, p: &mut Printer<W>) -> Result<(), Box<error::Error>> {
    for command in it {
        match command {
            Command::Str(s) => p.write_text(&s)?,
            Command::Newline => p.write_text("\n")?,
            Command::Align(a) => p.align(a)?,
            Command::Emphasis(e) => p.emphasis(e)?,
            Command::Underline(u) => p.underline(u)?,
            Command::Image(im) => {
                let im = image::open(im)?;
                p.write_bit_image(&im)?
            },
            Command::Qr(data) => {
                let code = qrcode::QrCode::new(&data).unwrap();
                let im: image::GrayImage = image::imageops::resize(&code.render().to_image(), 150, 150, image::FilterType::Nearest);
                p.write_bit_image(&im)?;
            },
            _ => p.write_text("ERROR")?
        }
    }
    Ok(())
}

fn print_thing<W:Write>(p: &mut Printer<W>, base: &str, name: &str, email: &str, password: &str) -> Result<(), Box<error::Error>> {
    
    let mut url = Url::parse(&base)?;
    url = url.join("/api/signout")?;
    //url = url.join(&format!("{}",url::percent_encoding::percent_encode(&email.as_bytes(), url::percent_encoding::SIMPLE_ENCODE_SET))).unwrap();
    url = url.join(&email)?;

    println!("{}", url.as_str());

    let commands = match parser::parse(include_str!("format.conf").as_bytes()) {
        Ok(val) => val,
        Err(s) => {println!("ERROR: {}", s); return Ok(())}
    };

    print_commands(commands.into_iter().map(|c| {
        match c {
            Command::Field(Field::Name) => Command::Str(String::from(name)),
            Command::Field(Field::Link) => Command::Str(String::from(base)),
            Command::Field(Field::Email) => Command::Str(String::from(email)),
            Command::Field(Field::Password) => Command::Str(String::from(password)),
            Command::Field(Field::Qr) => Command::Qr(url.as_str().bytes().collect()),
            x => x
        }
    }), p)?;
    Ok(())
}

fn main() {
    let mut args = env::args();
    args.next().unwrap();
    let ploc = args.next().unwrap();
    let baseurl = args.next().unwrap();
    let name = args.next().unwrap();
    let email = args.next().unwrap();
    let password = args.next().unwrap();

    let mut printer = File::create(ploc).unwrap();
    let mut printer = Printer::new(printer);

    print_thing(&mut printer, &baseurl, &name, &email, &password);

    /*for command in commands.into_iter() {
        match command {
            Command::Str(s) => print!("{}", s),
            Command::Newline => println!(),
            Command::Field(f) => match f {
                Field::Name => print!("{}", name),
                Field::Email => print!("{}", email),
                Field::Password => print!("{}", password),
                _ => print!("ERROR")
            },
            _ => print!("ERROR")
        }
    }*/

    /*let outstr = format!(include_str!("format.txt"), name=name, email=email, password=password);
    println!("{}", outstr);*/
}
