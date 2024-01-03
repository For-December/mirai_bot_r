use chrono::Local;
use env_logger::{fmt::Color, Env};
use std::io::Write;
pub fn init_log() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format(|buf, record| {
            let level_color = match record.level() {
                log::Level::Error => Color::Red,
                log::Level::Warn => Color::Yellow,
                log::Level::Info => Color::Green,
                log::Level::Debug | log::Level::Trace => Color::Cyan,
            };

            let mut level_style = buf.style();
            level_style.set_color(level_color).set_bold(true);

            let mut style = buf.style();
            style.set_color(Color::White).set_dimmed(true);

            writeln!(
                buf,
                "{} {} [ {} ] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                level_style.value(record.level()),
                style.value(record.module_path().unwrap_or("<unnamed>")),
                record.args()
            )
        })
        .init();
}

#[cfg(test)]
mod test {
    use chrono::Local;
    use env_logger::{fmt::Color, Env};
    use std::io::Write;

    #[test]
    pub fn test_log() {
        env_logger::Builder::from_env(Env::default().default_filter_or("debug"))
            .format(|buf, record| {
                let level_color = match record.level() {
                    log::Level::Error => Color::Red,
                    log::Level::Warn => Color::Yellow,
                    log::Level::Info => Color::Green,
                    log::Level::Debug | log::Level::Trace => Color::Cyan,
                };

                let mut level_style = buf.style();
                level_style.set_color(level_color).set_bold(true);

                let mut style = buf.style();
                style.set_color(Color::White).set_dimmed(true);

                writeln!(
                    buf,
                    "{} {} [ {} ] {}",
                    Local::now().format("%Y-%m-%d %H:%M:%S"),
                    level_style.value(record.level()),
                    style.value(record.module_path().unwrap_or("<unnamed>")),
                    record.args()
                )
            })
            .init();

        log::info!("好好好");
        log::error!("错！");
    }
    #[test]
    pub fn test() {
        use std::process::Command;
        let command = "aitaffy.py"; // 将要执行的 cmd 命令

        let output = Command::new("py")
            .args(&[command])
            .output() // 执行命令，并获取输出结果
            .expect("执行命令失败");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let mut res = stdout.as_ref().split("\n");
        res.next();
        let res = res.next().unwrap();
        println!("{}", res);
        println!("{}", stderr);
    }
}
