fn main() -> std::process::ExitCode {
    // https://www.freebsd.org/cgi/man.cgi?query=sysexits&apropos=0&sektion=0&manpath=FreeBSD+14.0-current&arch=default&format=html
    let usage_error_exit_code = std::process::ExitCode::from(64);

    let sleep_time_str = match std::env::args().nth(1) {
        Some(x) => x,
        _ => return usage_error_exit_code,
    };
    let sleep_time = match sleep_time_str.parse::<i64>() {
        Ok(x) => {
            if x < 0 {
                return usage_error_exit_code;
            }
            x
        }
        _ => return usage_error_exit_code,
    };
    std::thread::sleep(std::time::Duration::from_micros(sleep_time as u64));

    std::process::ExitCode::SUCCESS
}
