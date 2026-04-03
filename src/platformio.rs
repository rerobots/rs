// Copyright (C) 2026 rerobots, Inc.

use std::io::prelude::*;
use std::net::TcpStream;
use std::path::PathBuf;

pub const COMMAND_UPLOAD: u8 = 0;

fn usize_to_u8vec(x: usize) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Ignore bits beyond 32
    Ok(vec![
        (x & 0xff).try_into()?,
        ((x >> 8) & 0xff).try_into()?,
        ((x >> 16) & 0xff).try_into()?,
        ((x >> 24) & 0xff).try_into()?,
    ])
}

pub fn proxy_client(
    addr: std::net::SocketAddr,
    ini_file: PathBuf,
    exe_file: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let ini_data = std::fs::read(&ini_file)?;
    let exe_data = std::fs::read(&exe_file)?;
    let mut stream = TcpStream::connect(addr)?;

    let mut header: Vec<u8> = vec![0, COMMAND_UPLOAD];
    header.append(&mut usize_to_u8vec(ini_data.len())?);
    header.append(&mut usize_to_u8vec(exe_data.len())?);

    stream.write_all(&header)?;
    stream.write_all(&ini_data)?;
    stream.write_all(&exe_data)?;

    let mut buf = vec![];
    let nb = stream.read_to_end(&mut buf)?;
    let x = String::from_utf8_lossy(&buf[..nb]);
    println!("{x}");

    Ok(())
}
