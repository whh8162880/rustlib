windows下Rust安装太慢解决办法

1、打开powershell

2、分别执行下面两行代码：

$ENV:RUSTUP_DIST_SERVER='https://mirrors.ustc.edu.cn/rust-static'

$ENV:RUSTUP_UPDATE_ROOT='https://mirrors.ustc.edu.cn/rust-static/rustup'

3、继续在此命令行下执行 rustup-init.exe