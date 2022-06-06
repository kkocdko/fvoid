use fvoid::*;
use tiny_http::{Header, Request, Response, Server};

fn respond(req: Request, data: &Vec<u8>, mime: &str) {
    let response = Response::from_data(data.clone())
        .with_header(Header::from_bytes("content-type", mime).unwrap())
        .with_header(Header::from_bytes("access-control-allow-origin", "*").unwrap());
    req.respond(response).unwrap();
}

fn main() {
    let void_bmp = VoidBMP {
        width: 10,
        height: 10,
    }
    .data();
    let void_flv = VoidFLV {
        duration: 3600.0,
        fps: 2.0,
    }
    .data();
    let void_pdf = VoidPDF {
        width: 10,
        height: 10,
        page_count: 1,
    }
    .data();
    let void_wav = VoidWAV {
        duration: 3600.0,
        ..Default::default()
    }
    .data();
    let void_m3u = include_bytes!("../target/_temp/void.m3u8").to_vec();
    let void_mp4 = include_bytes!("../target/_temp/void.mp4").to_vec();
    let void_mts = include_bytes!("../target/_temp/void0.ts").to_vec();
    println!("address = 0.0.0.0:9902");
    for req in Server::http("0.0.0.0:9902").unwrap().incoming_requests() {
        match req.url() {
            "/void.bmp" => respond(req, &void_bmp, "image/bmp"),
            "/void.pdf" => respond(req, &void_pdf, "application/pdf"),
            "/void.flv" => respond(req, &void_flv, "video/x-flv"),
            "/void.wav" => respond(req, &void_wav, "audio/wav"),
            // by build.rs
            "/void.m3u" | "/void.m3u8" => respond(req, &void_m3u, "application/mpegurl"),
            "/void.ts" | "/void0.ts" | "/void.mts" => respond(req, &void_mts, "video/MP2T"),
            "/void.mp4" => respond(req, &void_mp4, "video/mp4"),
            _ => {
                req.respond(Response::empty(404)).unwrap();
            }
        }
    }
}
