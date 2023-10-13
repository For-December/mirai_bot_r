use std::process::Command;
const _MAX_LENGTH: usize = 256;
pub fn aitaffy(msg: &str) -> Vec<String> {
    let mut res = Vec::new();
    let temp = msg;
    if utf8_slice::len(temp) > _MAX_LENGTH {
        res.push(aitaffy_origin(utf8_slice::slice(temp, 0, _MAX_LENGTH)));
        // temp = utf8_slice::slice(temp, 128, utf8_slice::len(temp))
    } else {
        res.push(aitaffy_origin(msg));
    }
    // res.push(aitaffy_origin(temp));
    res
}
pub fn aitaffy_origin(msg: &str) -> String {
    let command = "aitaffy.py"; // 将要执行的 cmd 命令

    let output = Command::new("py")
        .args(&[command, msg])
        .output() // 执行命令，并获取输出结果
        .expect("执行命令失败");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("{}", stdout);
    println!("{}", stderr);
    let mut res = stdout.as_ref().split("\n");
    res.next();
    let res = res.next().unwrap();
    println!("{}", res);
    println!("{}", stderr);
    return String::from(res.replace("\r", ""));
}

#[cfg(test)]
mod test {
    // use super::aitaffy;

    #[test]
    pub fn test_api() {
        // aitaffy("关注塔菲喵");
    }
}
