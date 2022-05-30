use fvoid::*;

macro_rules! error {
    ($($arg:tt)*) => {{
        println!($($arg)*);
        return;
    }};
}

macro_rules! accept {
    ($builder:ident, [$( $param:ident ),*], $opts:expr) => {{
        let mut cfg = $builder::default();
        for (k, v) in $opts {
            match k {
                $(
                    stringify!($param) => { cfg.$param = v.parse().unwrap() }
                )*
                _ => error!("unknown option {k}"),
            }
        }
        cfg.data()
    }};
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        error!("{USAGE}");
    }
    let (kind, opts) = args[1].split_once(':').unwrap_or((args[1].as_str(), ""));
    let opts: Vec<(&str, &str)> = opts.split(',').filter_map(|i| i.split_once('=')).collect();
    let path = args[2].as_str();
    let data = match kind {
        "bin" => accept!(VoidBIN, [content, size], opts),
        "flv" => accept!(VoidFLV, [duration, fps], opts),
        "m3u" => accept!(VoidM3U, [duration, filename], opts),
        "mp3" => accept!(VoidMP3, [duration], opts),
        // "mp4" => accept!(VoidMP4, [duration, fps], opts),
        // "mts" => accept!(VoidMTS, [duration, fps], opts),
        "pdf" => accept!(VoidPDF, [width, height, page_count], opts),
        "svg" => accept!(VoidSVG, [width, height, color], opts),
        "wav" => accept!(VoidWAV, [duration, sampling], opts),
        k => error!("unknown format {k}"),
    };
    std::fs::write(path, data).unwrap();
}

const USAGE: &str = "\
usage:
    fvoid <format>[:options] <output>

examples:
    fvoid pdf:width=200,height=300 void.pdf
    fvoid mp3 void.mp3
";

// .vscode/run.sh mp4:duration=20 target/void.mp4
