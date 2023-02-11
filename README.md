# calltracer
Aggregate call trace data

```
$ calltracer --help

calltracer 1.0
Tobias Bergkvist <tobias@bergkv.ist>
Rank function-trace locations according to evaluation time

USAGE:
    calltracer [FLAGS] [OPTIONS]

FLAGS:
    -a, --ascending      Ascending order of results
    -c, --coordinates    Include line and column numbers in trace output (function coordinates within a file)
    -h, --help           Prints help information
    -V, --version        Prints version information

OPTIONS:
    -m, --maxcount <maxcount>    Limit number of top results
    -p, --prefix <prefix>        Filter by specific location prefix (like file or folder name)
```

## Usage example
```sh
# Generate function call trace from nix
$ nix-shell --run "exit 0" --trace-function-calls 2> calls.trace

# Find the 25 nix files that use the most evaluation time
$ <calls.trace calltracer --maxcount 25

# Find out which functions inside of a specific file we spend the most time in
$ <calls.trace ./target/release/calltracer --coordinates --prefix /nix/store/5n402azp0s9vza4rziv4z5y88v2cv1mq-nixpkgs/lib/customisation.nix
```