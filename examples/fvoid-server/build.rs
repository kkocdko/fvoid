use std::fs;
use std::path::PathBuf;
use std::process::Command;
fn main() {
    let dir = PathBuf::from("./target/_temp");
    if dir.exists() {
        return;
    }
    fs::remove_dir_all(&dir).ok(); // ignore error when dir not exists
    fs::create_dir_all(&dir).unwrap();
    let ops = [
        "-y -f lavfi -i nullsrc=size=4x4:rate=1:duration=3600,lutrgb=0:0:0 -hls_time 7200 void.m3u8",
        "-y -f lavfi -i nullsrc=size=4x4:rate=1:duration=3600,lutrgb=0:0:0 void.mp4",
    ];
    for op in ops {
        let success = Command::new("ffmpeg") // or /home/kkocdko/misc/tools/ffmpeg
            .args(op.split(' ').collect::<Vec<&str>>())
            .current_dir(&dir)
            .status()
            .unwrap()
            .success();
        assert!(success, "build.rs exec failed");
    }
}
