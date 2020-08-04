windows下Rust安装太慢解决办法

1、打开powershell

2、分别执行下面两行代码：

$ENV:RUSTUP_DIST_SERVER='https://mirrors.ustc.edu.cn/rust-static'

$ENV:RUSTUP_UPDATE_ROOT='https://mirrors.ustc.edu.cn/rust-static/rustup'

3、继续在此命令行下执行 rustup-init.exe

RUSTUP_UPDATE_ROOT=http://mirrors.ustc.edu.cn/rust-static/rustup
RUSTUP_DIST_SERVER=http://mirrors.ustc.edu.cn/rust-static

在 C:\Users\用户名\.cargo 下创建 config 文件
内容为

[http]
proxy = "127.0.0.1:1080"

[https]
proxy = "127.0.0.1:1080"