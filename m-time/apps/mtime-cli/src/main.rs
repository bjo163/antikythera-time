use clap::{Parser, Subcommand};
use mtime_calendar::CalendarProfile;
use mtime_conformance::run_core_conformance;
use mtime_hijri::HijriAstronomicalState;

#[derive(Parser)]
struct Cli { #[command(subcommand)] cmd: Cmd }

#[derive(Subcommand)]
enum Cmd {
    Conformance,
    Mabims {
        #[arg(long)]
        altitude: f64,
        #[arg(long)]
        elongation: f64,
    },
}

fn main() {
    match Cli::parse().cmd {
        Cmd::Conformance => {
            let r = run_core_conformance();
            println!("pass={} {:?}", r.pass, r.checks);
            if !r.pass { std::process::exit(1); }
        }
        Cmd::Mabims { altitude, elongation } => {
            let p = CalendarProfile::mabims_2026();
            let o = p.evaluate(&HijriAstronomicalState::synthetic(altitude, elongation));
            println!("{} {} pass={} clauses={:?}", p.id, p.version, o.pass, o.clauses);
        }
    }
}
