use mtime_clock::digital_packet_from_utc;
use mtime_timescales::UtcInstant;

fn main() {
    let unix_seconds = std::env::args().nth(1)
        .map(|v| v.parse::<i64>().expect("unix seconds"))
        .unwrap_or(1_767_225_600);
    let packet = digital_packet_from_utc(UtcInstant { unix_seconds, nanoseconds: 0 })
        .expect("M-Time clock state");
    println!("{}", packet.render_text());
    println!("{}", packet.to_json().expect("packet JSON"));
}
