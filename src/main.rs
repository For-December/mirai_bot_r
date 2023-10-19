// use std::collections::HashMap;
use mirai_bot::run;
use tokio::sync::mpsc;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {



    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    tokio::spawn(async move {
        tx.send("sending from first handle").await;
    });

    tokio::spawn(async move {
        tx2.send("sending from second handle").await;
    });

    while let Some(message) = rx.recv().await {
        println!("GOT = {}", message);
    }

    run().await?;
    Ok(())
}

#[cfg(test)]
mod test {

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
