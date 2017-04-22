use nom;
use nom::IResult;
use printer::Alignment;
use std::str;

#[derive(Debug)]
pub enum Field {
    Qr,
    Link,
    Name,
    Email,
    Password
}

named!(field<&[u8], Field>, alt_complete!(
        tag!("qr") => {|_| Field::Qr } |
        tag!("link") => {|_| Field::Link } |
        tag!("name") => {|_| Field::Name } |
        tag!("email") => {|_| Field::Email } |
        tag!("password") => {|_| Field::Password }));

#[derive(Debug)]
pub enum Command {
    Str(String),
    Image(String),
    Newline,
    Align(Alignment),
    Emphasis(bool),
    Underline(bool),
    Qr(Vec<u8>),
    Field(Field)
}

named!(field_cmd<&[u8], Command>, map!(preceded!(tag!("\\"), field), { |x| Command::Field(x) }));

named!(emphasis<&[u8], Command>, alt_complete!(
        tag!("\\emphasis") => { |_| Command::Emphasis(true) } |
        tag!("\\noemphasis") => { |_| Command::Emphasis(false) })
    );

named!(underline<&[u8], Command>, alt_complete!(
        tag!("\\underline") => { |_| Command::Underline(true) } |
        tag!("\\nounderline") => { |_| Command::Underline(false) })
    );

named!(align<&[u8], Command>, preceded!(tag!("\\align "), alt_complete!(
            tag!("left") => { |_| Command::Align(Alignment::Left) } |
            tag!("center") => { |_| Command::Align(Alignment::Center) } |
            tag!("right") => { |_| Command::Align(Alignment::Right) })
        )
    );

named!(text<&[u8], Command>, map!(many1!(none_of!("\\\n\r")), {|x:Vec<char>| Command::Str(x.into_iter().collect())}));

named!(image<&[u8], Command>, map!(preceded!(tag!("\\image "), many1!(none_of!(" \t\r\n"))), {|x:Vec<char>| Command::Image(x.into_iter().collect())}));

named!(newline<&[u8], Command>, map!(one_of!("\n\r"), { |_| Command::Newline}));

named!(command<&[u8], Command>, alt_complete!(field_cmd | align | emphasis | underline | image | newline | text));

named!(manycommand<&[u8], Vec<Command>>, many1!(command));

fn parse_command(bytes: &[u8]) -> IResult<&[u8], Command> {
    unimplemented!();
}


pub fn parse(bytes: &[u8]) -> Result<Vec<Command>, String> {
    match manycommand(bytes) {
        IResult::Done(i, o) => {
            if i.len() == 0 {
                Ok(o)
            } else {
                Err(String::from(format!("unable to finish parsing, failed to read:\n{}", str::from_utf8(i).unwrap_or("UNABLE TO CONVERT TO UTF8"))))
            }
        },
        IResult::Error(e) => {
            Err(format!("parsing error: {:?}", e))
        },
        IResult::Incomplete(i) => {
            Err(format!("data incomplete: {:?}", i))
        }
    }
}
