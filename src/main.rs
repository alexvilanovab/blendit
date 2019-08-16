extern crate image;
extern crate imageproc;
extern crate indicatif;
extern crate rusttype;

mod cli;

fn main() {
    let args = cli::get_arguments();

    let img_path = std::path::Path::new(args.value_of("image").unwrap());
    let img = match image::open(img_path) {
        Ok(img) => img.to_rgb(),
        Err(e) => {
            eprintln!("Could not open the given image: {}", e);
            std::process::exit(1);
        }
    };

    let txt_path = std::path::Path::new(args.value_of("text").unwrap());
    let txt = match std::fs::read_to_string(txt_path) {
        Ok(txt) => txt,
        Err(e) => {
            eprintln!("Could not read the given text file: {}", e);
            std::process::exit(1);
        }
    };
    let mut txt_it = txt.chars();

    let font_data: &[u8] = include_bytes!("../fonts/Bitter-Bold.ttf");
    let font = match rusttype::Font::from_bytes(font_data) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Could not load the given font file: {}", e);
            std::process::exit(1);
        }
    };
    let font_size: u32 = match args.value_of("font_size").unwrap().parse() {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Invalid value for 'font-size': {}", e);
            std::process::exit(1);
        }
    };
    let font_scale = rusttype::Scale {
        x: (font_size as f32) * 1.5,
        y: (font_size as f32) * 1.5,
    };

    let out_path = std::path::Path::new(args.value_of("output").unwrap());
    let mut out = image::RgbImage::new(img.width() * font_size, img.height() * font_size);

    let bar = indicatif::ProgressBar::hidden();
    bar.set_position(0);
    bar.set_length((img.width() * img.height()) as u64);
    bar.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:50} {pos}/{len}")
            .progress_chars("■ "),
    );
    bar.set_draw_target(indicatif::ProgressDrawTarget::stderr());

    for (x, y, rgb) in img.enumerate_pixels() {
        let c = match txt_it.next() {
            Some(val) => val.to_string(),
            None => {
                txt_it = txt.chars();
                String::from(" ")
            }
        };
        imageproc::drawing::draw_text_mut(
            &mut out,
            *rgb,
            x * font_size,
            y * font_size,
            font_scale,
            &font,
            &c,
        );
        bar.inc(1)
    }

    bar.finish();

    match out.save(out_path) {
        Ok(_) => {
            println!(
                "output image saved successfully: {}",
                out_path.to_str().unwrap()
            );
        }
        Err(e) => {
            eprintln!("Failed saving output file: {}", e);
            std::process::exit(1);
        }
    };
}
