mod common;
mod demuxer;
mod muxer;

use clap::{App, AppSettings, Arg};
use rav1d::api::*;

use std::io;

pub struct CLISettings {
    pub demuxer: Box<dyn demuxer::Demuxer>,
    pub muxer: Box<dyn muxer::Muxer>,
    pub limit: usize,
    pub skip: usize,
    pub verbose: bool,
    pub threads: usize,
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

// Decode and write a frame, returns frame information.
fn process_frame(
    cli: &mut CLISettings,
    ctx: &mut Context<u8>,
    count: &mut usize,
) -> Option<Vec<common::FrameSummary>> {
    let mut frame_summaries = Vec::new();

    if cli.limit != 0 && *count == cli.limit {
        ctx.flush();
    } else {
        match cli.demuxer.read() {
            Ok(pkt) => {
                if cli.verbose {
                    eprintln!("{}", pkt);
                }
                *count += 1;
                let _ = ctx.send_packet(&mut Some(pkt));
            }
            _ => {
                ctx.flush();
            }
        };
    }

    loop {
        let frame_wrapped = ctx.receive_frame();
        match frame_wrapped {
            Ok(frame) => {
                //cli.muxer.write(&frame);
                frame_summaries.push(frame.into());
            }
            Err(CodecStatus::NeedMoreData) => {
                break;
            }
            Err(CodecStatus::EnoughData) => {}
            Err(CodecStatus::LimitReached) => {
                return None;
            }
            Err(CodecStatus::Failure) => {
                panic!("Failed to decode video");
            }
            //Err(CodecStatus::Decoded) => {}
        }
    }
    Some(frame_summaries)
}

fn main() -> io::Result<()> {
    let mut cli = parse_cli();
    let cfg = Config {
        threads: cli.threads,
        ..Default::default()
    };

    let video_info = cli.demuxer.open()?;
    if cli.verbose {
        eprintln!("{:?}", video_info);
    }
    cli.muxer.open(&video_info)?;

    let mut pkt: Packet;
    for _ in 0..cli.skip {
        pkt = cli.demuxer.read()?;
        if cli.verbose {
            eprintln!("{}", pkt);
        }
    }

    // TODO: use seq header probe to find out pixel type
    let mut ctx: Context<u8> = cfg.new_context();

    let mut progress = common::ProgressInfo::new(
        Rational {
            num: video_info.time_base.den,
            den: video_info.time_base.num,
        },
        if cli.limit == 0 {
            None
        } else {
            Some(cli.limit)
        },
    );

    let mut count = 0;
    while let Some(frame_info) = process_frame(&mut cli, &mut ctx, &mut count) {
        for frame in frame_info {
            progress.add_frame(frame);
            if cli.verbose {
                eprintln!("{} - {}", frame, progress);
            } else {
                eprint!("\r{}     ", progress);
            };
        }
    }
    eprint!("\n{}\n", progress.print_summary());

    cli.muxer.close();
    cli.demuxer.close();

    Ok(())
}
