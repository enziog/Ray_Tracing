use std::fmt::Write as fmtWrite;
use std::fs::File;
//use std::io::BufWriter;
use std::io::Write as ioWrite;

fn main() {
    let nx = 1280;
    let ny = 800;
    let mut image = String::new();
    let mut file = File::create("auto.ppm").unwrap();

    writeln!(&mut image, "P3\n{} {}\n255", &nx, &ny).unwrap();
    //let mut fout = BufWriter::new(image);
    file.write_all(image.as_bytes()).unwrap();
    //println!("P3\n{} {}\n255", nx, ny);
    let mut j = ny - 1;
    while j >= 0 {
        let mut i = 0;

        while i < nx {
            let r = i as f32 / nx as f32;
            let g = j as f32 / ny as f32;
            let b = 0.2;
            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;
            let mut point = String::new();
            writeln!(point, "{} {} {}", ir, ig, ib).unwrap();
            //println!("{} {} {}", ir, ig, ib);
            file.write_all(point.as_bytes()).unwrap();
            //println!("{} {} {}", ir, ig, ib);
            i += 1;
        }
        j -= 1;
    }
}
