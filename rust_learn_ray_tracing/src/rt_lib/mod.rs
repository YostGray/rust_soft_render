use std::io::{BufWriter, self, Write};

pub mod vector3;
pub mod geometry;
pub mod ray;
pub mod scene;
pub mod cam;
pub mod sphere;
pub mod material;

///progresss 0 - 1
pub fn show_progress(sw:&mut BufWriter<io::Stdout>,progresss:f64) {
    let num = progresss.ceil() as i64;
    sw.write_fmt(format_args!("\r{}",'[')).unwrap();
    for i in 0..100 {
        if i < num {
            sw.write_fmt(format_args!("{}", "#")).unwrap();
        }
        else if i == num {
            sw.write_fmt(format_args!("{}", ">")).unwrap();
        }
        else {
            sw.write_fmt(format_args!("{}", " ")).unwrap();
        }
    }
    sw.write_fmt(format_args!("{}", ']')).unwrap();
    sw.write_fmt(format_args!("\t({:3}%)", progresss)).unwrap();
    sw.flush().unwrap();
}