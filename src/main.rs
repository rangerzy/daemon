use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    loop {
        let args = std::env::args();
        for arg in args {
            let query_cmd = format!("sc query {}", arg);
            let start_cmd = format!("sc start {}", arg);
            let output = Command::new("cmd")
                // .creation_flags(0x08000000)
                .arg("/c")
                .arg(query_cmd)
                .stdout(Stdio::piped()).output().expect("cmd exec error!");

            let stdout = String::from_utf8_lossy(&output.stdout);
            let stdout_str = stdout.to_string();
            if output.status.success() {
                println!("{}->执行成功，执行结果：{}", format!("sc query {}", arg), stdout_str);
            } else {
                println!("{}->执行失败，执行结果：{}", format!("sc query {}", arg), stdout_str);
            }


            if stdout_str.contains("STOPPED") {
                println!("进程状态：停止");
                let output = Command::new("cmd")
                    // .creation_flags(0x08000000)
                    .arg("/c")
                    .arg(start_cmd)
                    .stdout(Stdio::piped()).output().expect("cmd exec error!");
                if output.status.success() {
                    println!("{}->执行成功", format!("sc start {}", arg));
                } else {
                    println!("{}->执行失败", format!("sc start {}", arg));
                }
            } else {
                println!("进程状态：正常");
            }
        }

        // 休眠
        println!("休眠10秒……");
        sleep(Duration::from_secs(10))
    }
}
