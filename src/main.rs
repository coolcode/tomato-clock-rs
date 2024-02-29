use std::env;
use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

const WORK_MINUTES: u64 = 25;
const BREAK_MINUTES: u64 = 5;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("🍅 tomato {WORK_MINUTES} minutes. Ctrl+C to exit");
        tomato(WORK_MINUTES, "It is time to take a break");
        println!("🛀 break {BREAK_MINUTES} minutes. Ctrl+C to exit");
        tomato(BREAK_MINUTES, "It is time to work");
    } else {
        match args.get(1).map(|s| s.as_str()) {
            Some("-t") => {
                let minutes = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(WORK_MINUTES);
                println!("🍅 tomato {minutes} minutes. Ctrl+C to exit");
                tomato(minutes, "It is time to take a break");
            }
            Some("-b") => {
                let minutes = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(BREAK_MINUTES);
                println!("🛀 break {minutes} minutes. Ctrl+C to exit");
                tomato(minutes, "It is time to work");
            }
            Some("-h") | None => help(),
            Some("-n") => {
                notify("hello world");
            }
            _ => help(),
        }
    }
}

fn tomato(minutes: u64, msg: &str) {
    let start_time = Instant::now();
    loop {
        let diff_seconds = start_time.elapsed().as_secs();
        let left_seconds = minutes * 60 - diff_seconds;
        let seconds_slot = left_seconds % 60;
        let countdown = format!("{}:{:0>2} ⏰", left_seconds / 60, seconds_slot);
        let duration = std::cmp::min(minutes, 25);
        progressbar(diff_seconds, minutes * 60, duration, &countdown);
        if left_seconds <= 0 {
            println!();
            break;
        }
        thread::sleep(Duration::from_secs(1));
    }
    notify(msg);
}

fn progressbar(curr: u64, total: u64, duration: u64, extra: &str) {
    let frac = curr as f64 / total as f64;
    let filled = (frac * duration as f64).round() as u64;
    print!("\r");
    for _ in 0..filled {
        print!("🍅");
    }
    for _ in 0..duration - filled {
        print!("__");
    }
    print!(" [{:.0}%]", frac * 100.0);
    print!(" {}", extra);
    io::stdout().flush().unwrap();
}

fn notify(msg: &str) {
    println!("{}", msg);
    let _ = notify_rust::Notification::new()
        .summary("🍅")
        .body(msg)
        //.icon("🍅")
        .appname("tomato")
        .show();
}

fn help() {
    let appname = env::args().next().unwrap_or_else(|| String::from("tomato"));
    println!("====== 🍅 Tomato Clock =======");
    println!(
        "{}         # start a {} minutes tomato clock + {} minutes break",
        appname, WORK_MINUTES, BREAK_MINUTES
    );
    println!("{} -t      # start a {} minutes tomato clock", appname, WORK_MINUTES);
    println!("{} -t <n>  # start a <n> minutes tomato clock", appname);
    println!("{} -b      # take a {} minutes break", appname, BREAK_MINUTES);
    println!("{} -b <n>  # take a <n> minutes break", appname);
    println!("{} -h      # help", appname);
}
