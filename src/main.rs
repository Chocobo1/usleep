fn main() -> std::process::ExitCode {
    let start_time = std::time::Instant::now(); // hopefully this operation isn't time-consuming

    // https://www.freebsd.org/cgi/man.cgi?query=sysexits&apropos=0&sektion=0&manpath=FreeBSD+14.0-current&arch=default&format=html
    let usage_error_exit_code = std::process::ExitCode::from(64);

    let sleep_time_str = match std::env::args().nth(1) {
        Some(x) => x,
        _ => return usage_error_exit_code,
    };
    let sleep_time = match sleep_time_str.parse::<u64>() {
        Ok(x) => std::time::Duration::from_micros(x),
        _ => return usage_error_exit_code,
    };

    const SLEEP_OVERHEAD: std::time::Duration = std::time::Duration::from_nanos(100);
    let processing_time = start_time.elapsed() + SLEEP_OVERHEAD;

    if sleep_time > processing_time {
        std::thread::sleep(sleep_time - processing_time);
    }

    std::process::ExitCode::SUCCESS
}
