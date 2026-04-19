use std::os::unix::net::UnixDatagram;

fn main() -> std::io::Result<()> {
    let sock_path = "/tmp/scry.sock";
    let socket = UnixDatagram::bind(sock_path)?;
    let mut buf = vec![0; 100];

    lemt mut signal = Sign

    println!("Start recv");
    loop {
        let size = socket
            .recv(buf.as_mut_slice())
            .expect("recv function failed");
        let msg = std::str::from_utf8(&buf[..size]).expect("Failed to parse message");
        println!("{}", msg);
    }
}
