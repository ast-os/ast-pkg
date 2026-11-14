fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    std::process::exit(ast_tools::run_tool("astfstab", a));
}
