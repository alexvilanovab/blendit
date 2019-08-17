extern crate clap;

pub fn get_arguments() -> clap::ArgMatches<'static> {
    clap::App::new("blendit")
        .version("0.1.0")
        .author("alexvilanovab <alexvilanovab@gmail.com>")
        .about("Blend images with text and generate amazing looking posters")
        .arg(
            clap::Arg::with_name("image")
                .value_name("IMG")
                .help("Image you want to blend")
                .index(1)
                .required(true),
        )
        .arg(
            clap::Arg::with_name("text")
                .value_name("TXT")
                .help("Text file you want to blend")
                .index(2)
                .required(true),
        )
        .arg(
            clap::Arg::with_name("font_size")
                .long("font-size")
                .short("fs")
                .value_name("UINT")
                .default_value("24")
                .help("Font size used when generating the poster"),
        )
        .arg(
            clap::Arg::with_name("output")
                .long("output")
                .short("o")
                .value_name("IMG")
                .default_value("./output.jpg")
                .help("Where you want to save the JPEG with the generated poster"),
        )
        .get_matches()
}
