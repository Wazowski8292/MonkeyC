//! Whole-program tests for the Monkey C compiler.
//!
//! Put this file at `tests/programs.rs`. Every `*.MC` file under
//! `tests/programs/` becomes its own test.
//!
//! What a test does (each one in its own folder under target/tmp, so tests can
//! run in parallel and never litter `tests/programs/` with .asm files):
//!
//!   1. copy the program to `<workdir>/prog.MC`
//!   2. run the compiler on it:        monkey_c prog.MC        -> writes prog.asm
//!   3. assemble + link:               nasm ... && gcc ...     -> executable `prog`
//!      (skipped when the compiler already built `prog` itself, which yours does)
//!   4. run `prog` and compare its stdout with the expectations
//!
//! Markers inside a program:
//!
//!   ## expect: <line>          one line of expected program stdout (in order)
//!   ## expect-error: <text>    the COMPILER must report <text> (stdout or stderr)
//!                              and must not produce prog.asm
//!   ## timeout: <seconds>      override the default timeout for this program
//!   ## ignore: <reason>        skip this test (check/feature not implemented yet)
//!
//! (`##` is the Monkey C comment syntax; change `COMMENT` below if it changes.)
//!
//! Things you may need to edit: `compiler()`, `assemble_cmd()` and `link_cmd()`.

use libtest_mimic::{Arguments, Failed, Trial};
use std::{
    fs,
    io::{self, Read},
    path::{Path, PathBuf},
    process::{Command, ExitStatus, Stdio},
    thread,
    time::{Duration, Instant},
};

// ---------------------------------------------------------------- settings --

const PROGRAMS_DIR: &str = "tests/programs";
const EXTENSION: &str = "MC"; // matched case-insensitively here, but the compiler itself only accepts ".MC"
const DEFAULT_TIMEOUT_SECS: u64 = 5; // applies to the compiler, and separately to the compiled program

/// How a comment starts in Monkey C. The test markers are written as comments,
/// e.g. `## expect: 11`.
const COMMENT: &str = "##";

/// false: compare stdout ignoring ALL whitespace, so it does not matter whether
///        print_int & co. end their output with a newline (safe default).
/// true:  compare line by line (each `// expect:` must be exactly one output
///        line). Switch to this once you know your print functions add "\n".
const STRICT_LINES: bool = false;

/// false: ignore the exit code of the compiled program. Your `main` may not
///        zero `eax` before returning, so the exit code can be garbage.
/// true:  the compiled program must exit with code 0.
const REQUIRE_ZERO_EXIT: bool = false;

/// The compiler binary. `monkey_c` must match the binary name in Cargo.toml.
fn compiler() -> Command {
    Command::new(env!("CARGO_BIN_EXE_monkey_c"))
}

/// Turns prog.asm into prog.o. The library code in the docs uses `[rel ...]`,
/// which is NASM syntax. Change this if you use another assembler.
fn assemble_cmd() -> Command {
    let mut c = Command::new("nasm");
    c.args(["-f", "elf64", "prog.asm", "-o", "prog.o"]);
    c
}

/// Turns prog.o into the executable `prog`. gcc links libc (for printf) and
/// provides the entry point that calls `main`. `-no-pie` avoids relocation
/// errors for a plain `call printf`.
fn link_cmd() -> Command {
    let mut c = Command::new("gcc");
    c.args(["prog.o", "-o", "prog", "-no-pie"]);
    c
}

// ------------------------------------------------------------ entry point --

fn main() {
    let args = Arguments::from_args(); // handles filters: `cargo test --test programs loops`
    let mut files = Vec::new();
    find_programs(Path::new(PROGRAMS_DIR), &mut files);
    files.sort();
    assert!(
        !files.is_empty(),
        "no .{EXTENSION} files found under {:?} (cwd: {:?})",
        Path::new(PROGRAMS_DIR).canonicalize(),
        std::env::current_dir()
    );

    let tests: Vec<Trial> = files
        .into_iter()
        .map(|path| {
            let name = path.strip_prefix(PROGRAMS_DIR).unwrap().with_extension("").display().to_string().replace('\\', "/");
            // `// ignore: reason` marks a test for a check that is not implemented yet.
            // It shows as "ignored"; run those with `cargo test --test programs -- --ignored`.
            let ignored = fs::read_to_string(&path).map(|s| s.lines().any(|l| marker(l, "ignore").is_some())).unwrap_or(false);
            let test_name = name.clone();
            Trial::test(name, move || run_case(&path, &test_name).map_err(Failed::from)).with_ignored_flag(ignored)
        })
        .collect();

    libtest_mimic::run(&args, tests).exit();
}

/// Recursively collect every program file under `dir`.
fn find_programs(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) => panic!("cannot read {}: {e}", dir.display()),
    };
    for entry in entries {
        let path = entry.unwrap().path();
        if path.is_dir() {
            find_programs(&path, out);
        } else if path.extension().is_some_and(|e| e.eq_ignore_ascii_case(EXTENSION)) {
            out.push(path);
        }
    }
}

// ------------------------------------------------------------ expectations --

struct Expectations {
    stdout_lines: Vec<String>,
    error: Option<String>,
    timeout: Duration,
}

/// If `line` is a marker comment like `## expect: 11`, return the text after
/// the colon (one leading space removed). `## expect:` alone gives "".
fn marker<'a>(line: &'a str, name: &str) -> Option<&'a str> {
    let rest = line.trim().strip_prefix(COMMENT)?.trim_start();
    let rest = rest.strip_prefix(name)?.strip_prefix(':')?;
    Some(rest.strip_prefix(' ').unwrap_or(rest))
}

fn parse_expectations(src: &str) -> Result<Expectations, String> {
    let mut stdout_lines = Vec::new();
    let mut error = None;
    let mut timeout = Duration::from_secs(DEFAULT_TIMEOUT_SECS);

    // Environment override, handy on slow CI machines: TEST_TIMEOUT_SECS=30
    if let Ok(v) = std::env::var("TEST_TIMEOUT_SECS") {
        if let Ok(secs) = v.parse::<f64>() {
            timeout = Duration::from_secs_f64(secs);
        }
    }

    for line in src.lines() {
        if let Some(rest) = marker(line, "expect") {
            stdout_lines.push(rest.to_string()); // `## expect:` alone expects an empty output line
        } else if let Some(rest) = marker(line, "expect-error") {
            if error.is_none() {
                error = Some(rest.trim().to_string());
            }
        } else if let Some(rest) = marker(line, "timeout") {
            let secs: f64 = rest.trim().parse().map_err(|_| format!("bad timeout marker: {line:?}"))?;
            timeout = Duration::from_secs_f64(secs);
        }
    }

    Ok(Expectations {
        stdout_lines,
        error,
        timeout,
    })
}

// ------------------------------------------------------- running with timeout --

struct RunResult {
    status: Option<ExitStatus>, // None if we had to kill it
    stdout: String,
    stderr: String,
    timed_out: bool,
}

/// Run `cmd`, capture stdout/stderr, and kill it if it exceeds `timeout`.
///
/// The pipes are drained on separate threads while we wait. If we only read
/// them after the process exits, a program that prints a lot would fill the
/// pipe buffer, block on write, and look like a hang.
fn run_with_timeout(mut cmd: Command, timeout: Duration) -> io::Result<RunResult> {
    let mut child = cmd.stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;

    let mut out_pipe = child.stdout.take().unwrap();
    let mut err_pipe = child.stderr.take().unwrap();
    let out_thread = thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = out_pipe.read_to_end(&mut buf);
        buf
    });
    let err_thread = thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = err_pipe.read_to_end(&mut buf);
        buf
    });

    let start = Instant::now();
    let (status, timed_out) = loop {
        if let Some(status) = child.try_wait()? {
            break (Some(status), false);
        }
        if start.elapsed() >= timeout {
            let _ = child.kill();
            let _ = child.wait(); // reap it so it doesn't linger as a zombie
            break (None, true);
        }
        thread::sleep(Duration::from_millis(10));
    };

    // Once the process is dead its pipes close, so these joins return.
    let stdout = out_thread.join().unwrap_or_default();
    let stderr = err_thread.join().unwrap_or_default();

    Ok(RunResult {
        status,
        stdout: String::from_utf8_lossy(&stdout).into_owned(),
        stderr: String::from_utf8_lossy(&stderr).into_owned(),
        timed_out,
    })
}

/// Run one build step (assembler, linker). Fails on spawn error, timeout or a
/// non-zero exit code, and includes the tool's output in the message.
fn run_step(label: &str, cmd: Command, timeout: Duration) -> Result<(), String> {
    let r = run_with_timeout(cmd, timeout).map_err(|e| format!("could not start {label}: {e} (is it installed and on PATH?)"))?;
    if r.timed_out {
        return Err(format!("{label} TIMED OUT after {timeout:?}"));
    }
    if !r.status.is_some_and(|s| s.success()) {
        return Err(format!("{label} failed ({:?})\n--- stdout ---\n{}\n--- stderr ---\n{}", r.status, r.stdout, r.stderr));
    }
    Ok(())
}

// ------------------------------------------------------------ working folder --

/// A fresh folder per test under target/tmp. It is wiped at the start of the
/// next run, not at the end, so after a failure you can open it and look at
/// prog.asm, prog.o and prog yourself.
fn work_dir(test_name: &str) -> PathBuf {
    let safe: String = test_name.chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '_' }).collect();
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("monkeyc_tests").join(safe);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("cannot create work dir");
    dir
}

// -------------------------------------------------------------- one test --

fn run_case(path: &Path, name: &str) -> Result<(), String> {
    let src = fs::read_to_string(path).map_err(|e| format!("cannot read file: {e}"))?;
    let expect = parse_expectations(&src)?;

    let dir = work_dir(name);
    // Copy the program: the compiler writes prog.asm right next to its input.
    fs::write(dir.join("prog.MC"), &src).map_err(|e| format!("cannot copy source: {e}"))?;

    // ---- 1. compile ----
    // The compiler prints its errors with println! and exits with code 0 even
    // when there are errors, so the exit code says nothing: judge by the
    // message and by whether prog.asm was produced.
    let mut cmd = compiler();
    cmd.arg("prog.MC").current_dir(&dir).env("NO_COLOR", "1");
    let compiled = run_with_timeout(cmd, expect.timeout).map_err(|e| format!("could not start the compiler: {e}"))?;
    if compiled.timed_out {
        return Err(format!(
            "COMPILER TIMEOUT after {:?}\n--- stdout so far ---\n{}\n--- stderr so far ---\n{}",
            expect.timeout, compiled.stdout, compiled.stderr
        ));
    }
    let compiler_output = format!("{}\n{}", compiled.stdout, compiled.stderr);
    let asm_path = dir.join("prog.asm");

    // Case 1: this program is supposed to be rejected by the compiler.
    if let Some(msg) = &expect.error {
        if !compiler_output.contains(msg.as_str()) {
            return Err(format!(
                "compiler output does not contain {msg:?}\n--- compiler stdout ---\n{}\n--- compiler stderr ---\n{}",
                compiled.stdout, compiled.stderr
            ));
        }
        if asm_path.exists() || dir.join("prog").exists() {
            return Err("the compiler reported the error but still produced prog.asm or prog".into());
        }
        return Ok(());
    }

    // Case 2: this program is supposed to compile, run and print these lines.
    // The compiler prints "Done! Binary Compiled to: prog", i.e. it already
    // assembles and links by itself. Use that executable when it exists, so the
    // tests exercise your real toolchain. Only if just prog.asm exists do we
    // assemble and link it ourselves.
    if !dir.join("prog").exists() {
        if !asm_path.exists() {
            return Err(format!("the compiler produced neither prog nor prog.asm\n--- compiler output ---\n{compiler_output}"));
        }

        // ---- 2. assemble + link ----
        let mut asm = assemble_cmd();
        asm.current_dir(&dir);
        run_step("assembler", asm, expect.timeout)?;

        let mut link = link_cmd();
        link.current_dir(&dir);
        run_step("linker", link, expect.timeout)?;
    }

    // ---- 3. run the compiled program ----
    let mut run = Command::new(dir.join("prog"));
    run.current_dir(&dir);
    let result = run_with_timeout(run, expect.timeout).map_err(|e| format!("could not start the compiled program: {e}"))?;
    if result.timed_out {
        return Err(format!(
            "PROGRAM TIMEOUT after {:?} (infinite loop in the program?)\n--- stdout so far ---\n{}\n--- stderr so far ---\n{}",
            expect.timeout, result.stdout, result.stderr
        ));
    }
    if REQUIRE_ZERO_EXIT && !result.status.is_some_and(|s| s.success()) {
        return Err(format!(
            "program exited with {:?}\n--- stdout ---\n{}\n--- stderr ---\n{}",
            result.status, result.stdout, result.stderr
        ));
    }
    if let Some(status) = result.status {
        // A crash (e.g. segfault) is never acceptable, whatever REQUIRE_ZERO_EXIT says.
        #[cfg(unix)]
        {
            use std::os::unix::process::ExitStatusExt;
            if let Some(sig) = status.signal() {
                return Err(format!(
                    "program was killed by signal {sig}\n--- stdout so far ---\n{}\n--- stderr ---\n{}",
                    result.stdout, result.stderr
                ));
            }
        }
        let _ = status;
    }

    // ---- 4. compare output ----
    if STRICT_LINES {
        let actual: Vec<&str> = result.stdout.lines().collect();
        if actual != expect.stdout_lines {
            return Err(format!("output mismatch\n  expected: {:?}\n       got: {:?}", expect.stdout_lines, actual));
        }
    } else {
        let squash = |s: &str| s.split_whitespace().collect::<String>();
        let actual = squash(&result.stdout);
        let wanted = squash(&expect.stdout_lines.concat());
        if actual != wanted {
            return Err(format!("output mismatch (whitespace ignored)\n  expected: {:?}\n       got: {:?}", wanted, actual));
        }
    }
    Ok(())
}
