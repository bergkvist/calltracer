use std::io::{self, BufRead, Write};
use std::collections::HashMap;
use clap::clap_app;

#[derive(Default, Clone, Copy)]
struct FileTrace { depth: i64, enter_ns: i64, total_ns: i64 }

fn main() {
    let matches = clap_app!(calltracer =>
        (version: "1.0.0")
        (author: "Tobias Bergkvist <tobias@bergkv.ist>")
        (about: "Rank function-trace locations according to evaluation time")
        (@arg prefix: +takes_value -p --prefix "Filter by specific location prefix (like file or folder name)")
        (@arg coordinates: -c --coordinates "Include line and column numbers in trace output (function coordinates within a file)")
        (@arg maxcount: +takes_value -m --maxcount "Limit number of top results")
        (@arg ascending: -a --ascending "Ascending order of results")
    ).get_matches();

    let prefix = matches.value_of("prefix").unwrap_or("");
    let coordinates = matches.is_present("coordinates");
    let maxcount = matches.value_of("maxcount")
        .and_then(|v|Some(v.parse::<i64>().expect("maxcount must be a valid integer")))
        .unwrap_or(-1);
    let ascending = matches.is_present("ascending");


    let mut traces: HashMap<String, FileTrace> = HashMap::new();

    for maybe_line in io::stdin().lock().lines() {
        let line = maybe_line.expect("Failed to read line from stdin");
        if !line.starts_with("function-trace") {
            continue;
        }
        let space_split: Vec<&str> = line.split(" ").collect();
        if space_split.len() < 5 {
            panic!("Invalid function-trace line: {}", line);
        }
        let direction = space_split[1];
        let location = if coordinates { space_split[2] } else { space_split[2].split(":").collect::<Vec<_>>()[0] };
        let time_ns = space_split[4];

        if !location.starts_with(prefix) {
            continue;
        }


        let trace = traces.entry(location.into()).or_default();
        if direction == "entered" {
            if trace.depth == 0 {
                trace.enter_ns = time_ns.parse::<i64>().unwrap();
            }
            trace.depth += 1;
        } else if direction == "exited" {
            trace.depth -= 1;
            if trace.depth == 0 {
                let exit_ns = time_ns.parse::<i64>().unwrap();
                trace.total_ns += exit_ns - trace.enter_ns;
            }
        }
    }


    let mut sorted_traces = traces.into_iter().collect::<Vec<_>>();
    sorted_traces.sort_by_key(|(_, trace)| trace.total_ns);
    if !ascending { sorted_traces.reverse(); }
    let mut stdout = std::io::stdout().lock();
    let mut i = maxcount;
    for (location, trace) in sorted_traces {
        if i == 0 { break; };
        writeln!(stdout, "{} {}", trace.total_ns, location).unwrap();
        i -= 1;
    }

}
