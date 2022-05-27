use fvoid::*;

macro_rules! accept_kind {
    ($builder:ident, [$( $param:ident ),*], $opts:expr) => {{
        let mut cfg = $builder::default();
        for (k, v) in $opts {
            match k {
                $(
                    stringify!($param) => { cfg.$param = v.parse().unwrap() }
                )*
                _ => panic!("unknown option {k}"),
            }
        }
        cfg.data()
    }};
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("{USAGE}");
        return;
    }
    let (kind, opts) = args[1].split_once(':').unwrap_or((args[1].as_str(), ""));
    let opts: Vec<(&str, &str)> = opts.split(',').filter_map(|i| i.split_once('=')).collect();
    let path = args[2].as_str();
    let data = match kind {
        "flv" => accept_kind!(VoidFLV, [duration, fps], opts),
        "mp3" => accept_kind!(VoidMP3, [duration], opts),
        "pdf" => accept_kind!(VoidPDF, [width, height, page_count], opts),
        "svg" => accept_kind!(VoidSVG, [width, height, color], opts),
        "txt" => accept_kind!(VoidTXT, [content, size], opts),
        k => panic!("unknown format {k}"),
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

// .vscode/run.sh flv:duration=20 target/void.flv
