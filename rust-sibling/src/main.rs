use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Fast,
    Exact,
}

#[derive(Clone)]
struct Args {
    project: String,
    mode: Mode,
    pretty: bool,
    trace_performance: bool,
    cwd: PathBuf,
    subset_inputs: Vec<String>,
}

#[derive(Clone, Debug)]
struct Diagnostic {
    file: Option<String>,
    line: Option<usize>,
    column: Option<usize>,
    code: u32,
    category: String,
    message: String,
}

#[derive(Clone)]
struct RunInput {
    cwd: PathBuf,
    project_path: PathBuf,
    files: Vec<PathBuf>,
    strict_enabled: bool,
    pretty: bool,
}

struct Timings(Vec<(String, u128)>);

impl Timings {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn record(&mut self, label: &str, dur_ms: u128) {
        self.0.push((label.to_string(), dur_ms));
    }

    fn print(&self) {
        eprintln!("Performance timings (ms):");
        for (label, ms) in &self.0 {
            eprintln!("  {}: {}", label, ms);
        }
    }
}

fn main() {
    let code = run();
    std::process::exit(code);
}

fn run() -> i32 {
    let args = match parse_args() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("tsgo-strict-rs error: {e}");
            print_help();
            return 2;
        }
    };

    let mut timings = Timings::new();

    let t0 = Instant::now();
    let project_path = abs_path(&args.cwd, &args.project);
    let subset_files = resolve_subset_inputs(&args.cwd, &args.subset_inputs);
    timings.record("file-resolution", t0.elapsed().as_millis());

    let mut diagnostics: Vec<Diagnostic> = Vec::new();

    match args.mode {
        Mode::Fast => {
            let t = Instant::now();
            let strict = run_tsgo(RunInput {
                cwd: args.cwd.clone(),
                project_path: project_path.clone(),
                files: subset_files.clone(),
                strict_enabled: true,
                pretty: args.pretty,
            });
            timings.record("strict-run", t.elapsed().as_millis());

            let strict = match strict {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("tsgo-strict-rs error: {e}");
                    return 2;
                }
            };

            diagnostics = filter_to_targets(strict, &subset_files);
        }
        Mode::Exact => {
            let parallel = env::var("TSGO_STRICT_PARALLEL")
                .map(|v| v != "0")
                .unwrap_or(true);

            if parallel {
                let base_input = RunInput {
                    cwd: args.cwd.clone(),
                    project_path: project_path.clone(),
                    files: subset_files.clone(),
                    strict_enabled: false,
                    pretty: args.pretty,
                };
                let strict_input = RunInput {
                    cwd: args.cwd.clone(),
                    project_path: project_path.clone(),
                    files: subset_files.clone(),
                    strict_enabled: true,
                    pretty: args.pretty,
                };

                let tb = Instant::now();
                let ts = Instant::now();
                let h1 = std::thread::spawn(move || run_tsgo(base_input));
                let h2 = std::thread::spawn(move || run_tsgo(strict_input));

                let base = match h1.join() {
                    Ok(v) => v,
                    Err(_) => {
                        eprintln!("tsgo-strict-rs error: baseline thread panicked");
                        return 2;
                    }
                };
                let strict = match h2.join() {
                    Ok(v) => v,
                    Err(_) => {
                        eprintln!("tsgo-strict-rs error: strict thread panicked");
                        return 2;
                    }
                };
                timings.record("baseline-run", tb.elapsed().as_millis());
                timings.record("strict-run", ts.elapsed().as_millis());

                let base = match base {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("tsgo-strict-rs error: {e}");
                        return 2;
                    }
                };
                let strict = match strict {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("tsgo-strict-rs error: {e}");
                        return 2;
                    }
                };

                let td = Instant::now();
                diagnostics = diff_diagnostics(
                    &filter_to_targets(strict, &subset_files),
                    &filter_to_targets(base, &subset_files),
                );
                timings.record("diff", td.elapsed().as_millis());
            } else {
                let tb = Instant::now();
                let base = run_tsgo(RunInput {
                    cwd: args.cwd.clone(),
                    project_path: project_path.clone(),
                    files: subset_files.clone(),
                    strict_enabled: false,
                    pretty: args.pretty,
                });
                timings.record("baseline-run", tb.elapsed().as_millis());

                let ts = Instant::now();
                let strict = run_tsgo(RunInput {
                    cwd: args.cwd.clone(),
                    project_path: project_path.clone(),
                    files: subset_files.clone(),
                    strict_enabled: true,
                    pretty: args.pretty,
                });
                timings.record("strict-run", ts.elapsed().as_millis());

                let base = match base {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("tsgo-strict-rs error: {e}");
                        return 2;
                    }
                };
                let strict = match strict {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("tsgo-strict-rs error: {e}");
                        return 2;
                    }
                };

                let td = Instant::now();
                diagnostics = diff_diagnostics(
                    &filter_to_targets(strict, &subset_files),
                    &filter_to_targets(base, &subset_files),
                );
                timings.record("diff", td.elapsed().as_millis());
            }
        }
    }

    let t_format = Instant::now();
    diagnostics.retain(|d| d.category == "error");
    diagnostics.sort_by(|a, b| diag_key(a).cmp(&diag_key(b)));

    for d in &diagnostics {
        match (&d.file, d.line, d.column) {
            (Some(file), Some(line), Some(column)) => {
                println!("{}({},{}) : {} TS{}: {}", file, line, column, d.category, d.code, d.message);
            }
            _ => {
                println!("{} TS{}: {}", d.category, d.code, d.message);
            }
        }
    }
    println!("Found {} strict errors.", diagnostics.len());
    timings.record("formatting", t_format.elapsed().as_millis());

    if args.trace_performance {
        timings.print();
    }

    if diagnostics.is_empty() { 0 } else { 1 }
}

fn parse_args() -> Result<Args, String> {
    let mut project = "tsconfig.json".to_string();
    let mut mode = Mode::Exact;
    let mut pretty = true;
    let mut trace_performance = false;
    let mut cwd = env::current_dir().map_err(|e| format!("cwd: {e}"))?;
    let mut subset_inputs = Vec::new();

    let mut it = env::args().skip(1).peekable();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "-p" | "--project" => {
                project = it
                    .next()
                    .ok_or_else(|| "--project requires a value".to_string())?;
            }
            "--mode" => {
                let v = it
                    .next()
                    .ok_or_else(|| "--mode requires a value".to_string())?;
                mode = match v.as_str() {
                    "fast" => Mode::Fast,
                    "exact" => Mode::Exact,
                    _ => return Err("--mode must be fast|exact".to_string()),
                };
            }
            "--pretty" => pretty = true,
            "--no-pretty" => pretty = false,
            "--trace-performance" => trace_performance = true,
            "--cwd" => {
                let v = it
                    .next()
                    .ok_or_else(|| "--cwd requires a value".to_string())?;
                cwd = abs_path(&cwd, &v);
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            "--version" => {
                println!("0.1.0-rs");
                std::process::exit(0);
            }
            _ => subset_inputs.push(arg),
        }
    }

    Ok(Args {
        project,
        mode,
        pretty,
        trace_performance,
        cwd,
        subset_inputs,
    })
}

fn print_help() {
    println!("tsgo-strict-rs [fileOrGlob ...]");
    println!("  -p, --project <path>       tsconfig path (default tsconfig.json)");
    println!("      --mode <exact|fast>    mode (default exact)");
    println!("      --pretty|--no-pretty   pretty tsgo output");
    println!("      --trace-performance    print timing breakdown");
    println!("      --cwd <path>           working directory");
}

fn run_tsgo(input: RunInput) -> Result<Vec<Diagnostic>, String> {
    let temp_config = write_temp_config(
        &input.cwd,
        &input.project_path,
        &input.files,
        input.strict_enabled,
    )?;

    let binary = resolve_tsgo_binary(&input.cwd);
    let output = Command::new(binary)
        .arg("--noEmit")
        .arg("--pretty")
        .arg(if input.pretty { "true" } else { "false" })
        .arg("-p")
        .arg(&temp_config)
        .current_dir(&input.cwd)
        .output()
        .map_err(|e| format!("spawn tsgo: {e}"))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    let _ = fs::remove_file(&temp_config);

    Ok(parse_diagnostics(&stdout, &stderr, &input.cwd))
}

fn write_temp_config(
    cwd: &Path,
    project_path: &Path,
    files: &[PathBuf],
    strict_enabled: bool,
) -> Result<PathBuf, String> {
    let tmp_dir = cwd.join(".tsgo-strict-tmp");
    fs::create_dir_all(&tmp_dir).map_err(|e| format!("create tmp dir: {e}"))?;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("clock error: {e}"))?
        .as_nanos();

    let cfg = tmp_dir.join(format!(
        "rs-{}-{}-{}.json",
        std::process::id(),
        if strict_enabled { "strict" } else { "base" },
        now
    ));

    let mut json = String::new();
    json.push_str("{\n");
    json.push_str(&format!(
        "  \"extends\": \"{}\",\n",
        json_escape(&project_path.to_string_lossy())
    ));
    json.push_str("  \"compilerOptions\": {\n");
    json.push_str("    \"noEmit\": true,\n");

    let flags = [
        "strict",
        "strictBindCallApply",
        "strictBuiltinIteratorReturn",
        "strictFunctionTypes",
        "strictNullChecks",
        "strictPropertyInitialization",
        "useUnknownInCatchVariables",
        "noImplicitAny",
        "noImplicitThis",
        "noImplicitOverride",
        "noPropertyAccessFromIndexSignature",
        "noUncheckedIndexedAccess",
        "noUncheckedSideEffectImports",
        "exactOptionalPropertyTypes",
    ];

    for (i, flag) in flags.iter().enumerate() {
        let comma = if i + 1 == flags.len() { "" } else { "," };
        json.push_str(&format!(
            "    \"{}\": {}{}\n",
            flag,
            if strict_enabled { "true" } else { "false" },
            comma
        ));
    }

    json.push_str("  }");

    if !files.is_empty() {
        json.push_str(",\n  \"files\": [\n");
        for (i, file) in files.iter().enumerate() {
            let comma = if i + 1 == files.len() { "" } else { "," };
            json.push_str(&format!(
                "    \"{}\"{}\n",
                json_escape(&file.to_string_lossy()),
                comma
            ));
        }
        json.push_str("  ]\n");
    } else {
        json.push('\n');
    }

    json.push_str("}\n");

    fs::write(&cfg, json).map_err(|e| format!("write temp config: {e}"))?;
    Ok(cfg)
}

fn parse_diagnostics(stdout: &str, stderr: &str, cwd: &Path) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    let mut current: Option<Diagnostic> = None;

    for raw in format!("{}\n{}", stdout, stderr).lines() {
        let line = strip_ansi(raw).trim_end().to_string();
        if line.is_empty() {
            continue;
        }

        if let Some(parsed) = parse_primary_diagnostic(&line, cwd) {
            if let Some(prev) = current.take() {
                out.push(prev);
            }
            current = Some(parsed);
            continue;
        }

        if let Some(ref mut d) = current {
            d.message.push('\n');
            d.message.push_str(line.trim());
        }
    }

    if let Some(prev) = current.take() {
        out.push(prev);
    }

    out
}

fn parse_primary_diagnostic(line: &str, cwd: &Path) -> Option<Diagnostic> {
    if let Some(idx) = line.find(" TS") {
        let head = &line[..idx];
        let tail = &line[idx + 3..];

        let mut tail_parts = tail.splitn(2, ':');
        let code = tail_parts.next()?.trim().parse::<u32>().ok()?;
        let message = tail_parts.next().unwrap_or("").trim().to_string();

        if let Some(pos) = head.find(": error") {
            let loc = &head[..pos];
            return parse_loc_diagnostic(loc, "error", code, message, cwd);
        }
        if let Some(pos) = head.find(": warning") {
            let loc = &head[..pos];
            return parse_loc_diagnostic(loc, "warning", code, message, cwd);
        }
        if head.starts_with("error") {
            return Some(Diagnostic {
                file: None,
                line: None,
                column: None,
                code,
                category: "error".to_string(),
                message,
            });
        }
        if head.starts_with("warning") {
            return Some(Diagnostic {
                file: None,
                line: None,
                column: None,
                code,
                category: "warning".to_string(),
                message,
            });
        }
    }

    None
}

fn parse_loc_diagnostic(
    loc: &str,
    category: &str,
    code: u32,
    message: String,
    cwd: &Path,
) -> Option<Diagnostic> {
    if let Some(open) = loc.rfind('(') {
        if loc.ends_with(')') {
            let file = &loc[..open];
            let inner = &loc[open + 1..loc.len() - 1];
            let mut parts = inner.split(',');
            let line = parts.next()?.trim().parse::<usize>().ok()?;
            let column = parts.next()?.trim().parse::<usize>().ok()?;
            return Some(Diagnostic {
                file: Some(abs_path(cwd, file).to_string_lossy().to_string()),
                line: Some(line),
                column: Some(column),
                code,
                category: category.to_string(),
                message,
            });
        }
    }

    if let Some((file, l, c)) = parse_colon_loc(loc) {
        return Some(Diagnostic {
            file: Some(abs_path(cwd, &file).to_string_lossy().to_string()),
            line: Some(l),
            column: Some(c),
            code,
            category: category.to_string(),
            message,
        });
    }

    None
}

fn parse_colon_loc(s: &str) -> Option<(String, usize, usize)> {
    let mut parts = s.rsplitn(3, ':');
    let c = parts.next()?.trim().parse::<usize>().ok()?;
    let l = parts.next()?.trim().parse::<usize>().ok()?;
    let f = parts.next()?.to_string();
    Some((f, l, c))
}

fn diff_diagnostics(strict: &[Diagnostic], baseline: &[Diagnostic]) -> Vec<Diagnostic> {
    let base: HashSet<String> = baseline.iter().map(diag_key).collect();
    strict
        .iter()
        .filter(|d| !base.contains(&diag_key(d)))
        .cloned()
        .collect()
}

fn filter_to_targets(diags: Vec<Diagnostic>, targets: &[PathBuf]) -> Vec<Diagnostic> {
    if targets.is_empty() {
        return diags;
    }
    let set: HashSet<String> = targets.iter().map(|f| normalize(f.as_path())).collect();
    diags
        .into_iter()
        .filter(|d| {
            if let Some(file) = &d.file {
                set.contains(&normalize(Path::new(file)))
            } else {
                true
            }
        })
        .collect()
}

fn diag_key(d: &Diagnostic) -> String {
    let msg = d.message.split_whitespace().collect::<Vec<_>>().join(" ");
    format!(
        "{}|{}|{}|{}|{}|{}",
        d.file.as_deref().unwrap_or("").to_lowercase(),
        d.line.unwrap_or(0),
        d.column.unwrap_or(0),
        d.code,
        d.category,
        msg
    )
}

fn resolve_subset_inputs(cwd: &Path, inputs: &[String]) -> Vec<PathBuf> {
    if inputs.is_empty() {
        return Vec::new();
    }

    let mut out = Vec::new();
    for input in inputs {
        let abs = abs_path(cwd, input);
        if abs.is_file() {
            if is_ts_file(&abs) {
                out.push(abs);
            }
            continue;
        }

        if abs.is_dir() {
            collect_ts_files(&abs, &mut out);
            continue;
        }

        if input.contains('*') || input.contains('?') {
            // Minimal glob fallback: walk cwd and match suffix around '*'.
            let pattern = input.replace('\\', "/");
            let mut all = Vec::new();
            collect_ts_files(cwd, &mut all);
            for p in all {
                let rel = match p.strip_prefix(cwd) {
                    Ok(r) => r.to_string_lossy().replace('\\', "/"),
                    Err(_) => p.to_string_lossy().replace('\\', "/"),
                };
                if simple_glob_match(&pattern, &rel) {
                    out.push(p);
                }
            }
        }
    }

    dedupe_paths(out)
}

fn collect_ts_files(root: &Path, out: &mut Vec<PathBuf>) {
    if let Ok(read_dir) = fs::read_dir(root) {
        for entry in read_dir.flatten() {
            let p = entry.path();
            if p.is_dir() {
                collect_ts_files(&p, out);
            } else if is_ts_file(&p) {
                out.push(p);
            }
        }
    }
}

fn is_ts_file(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|s| s.to_str()),
        Some("ts") | Some("tsx") | Some("mts") | Some("cts")
    )
}

fn simple_glob_match(pattern: &str, candidate: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    if !pattern.contains('*') {
        return pattern == candidate;
    }
    let parts: Vec<&str> = pattern.split('*').collect();
    if parts.len() == 2 {
        return candidate.starts_with(parts[0]) && candidate.ends_with(parts[1]);
    }
    candidate.contains(parts[0])
}

fn dedupe_paths(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for p in paths {
        let key = normalize(&p);
        if seen.insert(key) {
            out.push(p);
        }
    }
    out
}

fn resolve_tsgo_binary(cwd: &Path) -> String {
    if let Ok(v) = env::var("TSGO_BINARY") {
        if !v.trim().is_empty() {
            return v;
        }
    }

    let local = cwd.join("node_modules/.bin/tsgo");
    if local.exists() {
        return local.to_string_lossy().to_string();
    }

    "tsgo".to_string()
}

fn strip_ansi(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out = String::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == 27 && i + 1 < bytes.len() && bytes[i + 1] == b'[' {
            i += 2;
            while i < bytes.len() && bytes[i] != b'm' {
                i += 1;
            }
            if i < bytes.len() {
                i += 1;
            }
            continue;
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}

fn json_escape(input: &str) -> String {
    input.replace('\\', "\\\\").replace('"', "\\\"")
}

fn abs_path(cwd: &Path, p: &str) -> PathBuf {
    let pb = PathBuf::from(p);
    if pb.is_absolute() {
        pb
    } else {
        cwd.join(pb)
    }
}

fn normalize(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/").to_lowercase()
}
