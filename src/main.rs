extern crate image;
extern crate imageproc;
extern crate indicatif;
extern crate rusttype;

mod cli;

use blendit::*;

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

    let font_size: u32 = match args.value_of("font_size").unwrap().parse() {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Invalid value for 'font-size': {}", e);
            std::process::exit(1);
        }
    };
    let font = Font::new(font_size);

    let out_path = std::path::Path::new(args.value_of("output").unwrap());

    let bar = indicatif::ProgressBar::hidden();
    bar.set_position(0);
    bar.set_length((img.width() * img.height()) as u64);
    bar.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:50} {pos}/{len}")
            .progress_chars("■ "),
    );
    bar.set_draw_target(indicatif::ProgressDrawTarget::stderr());

    let out = process(img, &txt, &font, || bar.inc(1));

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
