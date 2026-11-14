//! Tools for installing by hand and for repository maintainers; `ast` itself does not need them.

pub mod chroot;
pub mod fstab;
pub mod strap;

use ast::args::Args;
use ast::cli::{TermUi, escalate, report};
use ast::error::Result;

/// `astfstab` and `astchroot`.
pub fn run_tool(name: &str, args: Vec<String>) -> i32 {
    let help = match name {
        "astfstab" => fstab::HELP,
        _ => chroot::HELP,
    };
    if args.is_empty() || args.iter().any(|a| a == "-h" || a == "--help") {
        println!("{help}");
        return 0;
    }
    let go = || -> Result<i32> {
        if name == "astchroot" && !ast::util::is_root() {
            return Err(escalate());
        }
        match name {
            "astfstab" => fstab::run(args),
            _ => chroot::run(args),
        }
    };
    match go() {
        Ok(code) => code,
        Err(e) => {
            report(&e);
            e.exit_code()
        }
    }
}

/// `astrap`.
pub fn run_strap() -> i32 {
    let go = || -> Result<i32> {
        let mut a = Args::from_env();
        if a.is_empty() || a.flag(&["-h", "--help"]) {
            println!("{}", strap::HELP);
            return Ok(0);
        }
        if !ast::util::is_root() {
            return Err(escalate());
        }
        strap::run(a, &mut TermUi::new())
    };
    match go() {
        Ok(code) => code,
        Err(e) => {
            report(&e);
            e.exit_code()
        }
    }
}
