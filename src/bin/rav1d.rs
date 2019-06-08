mod common;
mod demuxer;
mod muxer;

use clap::{App, AppSettings, Arg};
use rav1d::api::*;
use std::io;

pub struct CLISettings {
    //pub input: Box<dyn Read>,
    //pub output: Box<dyn Write>,
    pub demuxer: Box<dyn demuxer::Demuxer>,
    pub muxer: Box<dyn muxer::Muxer>,
    //const char *frametimes;
    //const char *verify;
    pub limit: usize,
    pub skip: usize,
    pub verbose: bool,
    pub threads: usize,
    /*enum {
        REALTIME_DISABLE = 0,
        REALTIME_INPUT,
        REALTIME_CUSTOM,
    } realtime;
    double realtime_fps;
    unsigned realtime_cache;*/
}

pub fn parse_cli() -> CLISettings {
    let mut app = App::new("rav1d")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Rust AV1 Decoder")
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::SubcommandsNegateReqs)
        .arg(
            Arg::with_name("FULLHELP")
                .help("Prints more detailed help information")
                .long("fullhelp"),
        )
        // THREADS
        .arg(
            Arg::with_name("THREADS")
                .help("Set the threadpool size")
                .long("threads")
                .takes_value(true)
                .default_value("0"),
        )
        // INPUT/OUTPUT
        .arg(
            Arg::with_name("INPUT")
                .help("Compressed AV1 in IVF video output")
                .required_unless("FULLHELP")
                .index(1),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("Uncompressed YUV4MPEG2 video input")
                .short("o")
                .long("output")
                .required_unless("FULLHELP")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("LIMIT")
                .help("Maximum number of frames to decode")
                .short("l")
                .long("limit")
                .takes_value(true)
                .default_value("0"),
        )
        .arg(
            Arg::with_name("SKIP")
                .help("Skip n number of frames and decode")
                .long("skip")
                .takes_value(true)
                .default_value("0"),
        )
        // DEBUGGING
        .arg(
            Arg::with_name("VERBOSE")
                .help("Verbose logging; outputs info for every frame")
                .long("verbose")
                .short("v"),
        );

    let matches = app.clone().get_matches();

    if matches.is_present("FULLHELP") {
        app.print_long_help().unwrap();
        std::process::exit(0);
    }

    CLISettings {
        demuxer: demuxer::new(matches.value_of("INPUT").unwrap()),
        muxer: muxer::new(matches.value_of("OUTPUT").unwrap()),
        limit: matches.value_of("LIMIT").unwrap().parse().unwrap(),
        skip: matches.value_of("SKIP").unwrap().parse().unwrap(),
        verbose: matches.is_present("VERBOSE"),
        threads: matches
            .value_of("THREADS")
            .map(|v| v.parse().expect("Threads must be an integer"))
            .unwrap(),
    }
}

fn main() -> io::Result<()> {
    let mut cli_settings = parse_cli();
    let lib_settings = Rav1dSettings::new();
    let video_info = cli_settings.demuxer.open()?;
    if !cli_settings.verbose {
        eprintln!("rav1d {}", rav1d_version());
        eprintln!("{:?}", video_info);
    }
    let total = if cli_settings.limit != 0 && cli_settings.limit < video_info.num_frames {
        cli_settings.limit
    } else {
        video_info.num_frames
    };

    let mut data: Rav1dData;
    for _i in 0..cli_settings.skip {
        data = cli_settings.demuxer.read()?;
        eprintln!("{:?}", data.m);
    }

    let mut ctx = rav1d_open(&lib_settings).unwrap();

    while let Ok(data) = cli_settings.demuxer.read() {
        eprintln!("{:?}", data.m);

        let res = rav1d_send_data(&mut ctx, &data);
        if res < 0 {
            eprintln!("Error decoding frame: {}", res);
        }

        //let res = rav1d_get_picture(&ctx, )
    }

    cli_settings.muxer.close();

    Ok(())
}
