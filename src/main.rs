use std::io;
use std::process::{Command, Stdio};

use actix_files::Files;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    let html = std::fs::read_to_string("public/index.html").unwrap();
    HttpResponse::Ok().content_type("text/html").body(html)
}

async fn execute_command() -> impl Responder {
    if !std::path::Path::new("dependency").exists() {
        std::fs::create_dir("dependency").expect("Failed to create dependency folder");
    }

    let check_output = Command::new("dependency/yt-dlp").arg("--version").output();

    if check_output.is_err() {
        if cfg!(target_os = "windows") {
            let _install_output = Command::new("powershell")
                .arg("-Command")
                .arg("Invoke-WebRequest -Uri https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe -OutFile dependency/yt-dlp.exe")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .expect("Failed to download yt-dlp");

            println!("yt-dlp installed successfully on Windows");
        } else if cfg!(target_os = "linux") {
            let _install_output = Command::new("curl")
                .arg("-L")
                .arg("https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp")
                .arg("-o")
                .arg("/usr/local/bin/yt-dlp")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .expect("Failed to download yt-dlp");

            Command::new("chmod")
                .arg("a+rx")
                .arg("/usr/local/bin/yt-dlp")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .expect("Failed to set executable permissions for yt-dlp");

            println!("yt-dlp installed successfully on Linux");
        } else {
            println!("Unsupported platform");
        }
    } else {
        println!("yt-dlp is already installed");
    }

    let check_ffmpeg_output = Command::new("dependency/ffmpeg.exe")
        .arg("--version")
        .output();

    if check_ffmpeg_output.is_err() {
        if cfg!(target_os = "windows") {
            /*let _install_7zip4powershell = Command::new("powershell")
            .arg("-Command")
            .arg("Install-Module -Name 7Zip4Powershell -Scope CurrentUser -Force")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .expect("Failed to install 7Zip4Powershell module");*/

            let _install_ffmpeg_output = Command::new("powershell")
                .arg("-Command")
                .arg("Invoke-WebRequest -Uri https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-full.7z -OutFile dependency/ffmpeg.7z")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .expect("Failed to download ffmpeg");

            Command::new("powershell")
                .arg("-Command")
                .arg("Expand-7Zip -ArchiveFileName dependency/ffmpeg.7z -TargetPath dependency/")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .expect("Failed to extract ffmpeg");

            Command::new("powershell")
                .arg("-Command")
                .arg("Move-Item -Path dependency/ffmpeg-6.0-full_build/bin/ffmpeg.exe -Destination dependency/ffmpeg.exe")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .expect("Failed to move ffmpeg to dependency/ffmpeg");

            println!("ffmpeg installed successfully on Windows");
        } else if cfg!(target_os = "linux") {
            let _install_ffmpeg_output = Command::new("wget")
                .arg("https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-amd64-static.tar.xz")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .expect("Failed to download ffmpeg");

            Command::new("tar")
                .arg("xf")
                .arg("ffmpeg-release-amd64-static.tar.xz")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .expect("Failed to extract ffmpeg");

            let ffmpeg_dir = "ffmpeg-*-amd64-static";
            Command::new("sudo")
                .arg("cp")
                .arg(format!("{}/ffmpeg", ffmpeg_dir))
                .arg(format!("{}/ffprobe", ffmpeg_dir))
                .arg("/usr/local/bin/")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .expect("Failed to copy ffmpeg and ffprobe to /usr/local/bin");

            println!("ffmpeg installed successfully on Linux");
        } else {
            println!("Unsupported platform");
        }
    } else {
        println!("ffmpeg is already installed");
    }
    let url = "https://youtu.be/qwxZU0VkWII";
    let output = Command::new("dependency/yt-dlp.exe")
        .arg("-x")
        .arg("--audio-format")
        .arg("mp3")
        .arg(url)
        .output()
        .expect("Failed to execute command");

    let output_str = String::from_utf8(output.stdout).unwrap();
    HttpResponse::Ok().body(output_str)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(Files::new("/public", "public/"))
            .service(Files::new("/images", "images/"))
            .route("/execute", web::get().to(execute_command))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
