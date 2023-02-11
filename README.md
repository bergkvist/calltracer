# calltracer
Find out where the bottlenecks in your call traces are.


```
calltracer 1.0.1
Tobias Bergkvist <tobias@bergkv.ist>
Rank function-trace locations according to evaluation time

USAGE:
    calltracer [FLAGS] [OPTIONS]

FLAGS:
    -a, --ascending      Ascending order of results
    -c, --coordinates    Include line and column numbers in output (coordinates within file)
    -h, --help           Prints help information
    -V, --version        Prints version information

OPTIONS:
    -m, --maxcount <maxcount>    Limit number of top results
    -p, --prefix <prefix>        Filter by specific location prefix (like file or folder name)
```


## Supported Trace Format
`function-trace <entered|exited> <file>:<coordinate> at <timestamp>`
### Example:
```
function-trace entered /my/file:2:10 at 1676000026006000443
function-trace entered /my/file:2:17 at 1676000026006041983
function-trace exited /my/file:2:17 at 1676000026006065614
function-trace entered /my/other file:11:30 at 1676000026134969573
function-trace exited /my/other file:11:30 at 1676000026134973013
function-trace exited /my/file:2:10 at 1676000026135148774
```

---
## Usage example

calltracer can be particulariry useful when working with function-traces that are too big for flamegraphs. (Like here: https://github.com/NixOS/nix/blob/master/contrib/stack-collapse.py)

```sh
# Generate function call trace from nix
$ nix-shell --run "exit 0" --trace-function-calls 2> calls.trace
```

```sh
# Find the 25 nix files that use the most evaluation time
$ <calls.trace calltracer --maxcount 10

15594969588 //builtin/derivation.nix
15220903531 /nix/store/5n402azp0s9vza4rziv4z5y88v2cv1mq-nixpkgs/lib/customisation.nix
15176936836 /nix/store/5n402azp0s9vza4rziv4z5y88v2cv1mq-nixpkgs/lib/lists.nix
15152031543 /nix/store/5n402azp0s9vza4rziv4z5y88v2cv1mq-nixpkgs/pkgs/build-support/buildenv/default.nix
14901704086 /nix/store/5n402azp0s9vza4rziv4z5y88v2cv1mq-nixpkgs/lib/fixed-points.nix
14862257810 /nix/store/5n402azp0s9vza4rziv4z5y88v2cv1mq-nixpkgs/pkgs/development/interpreters/python/default.nix
14849430547 /nix/store/n6hsk44fw5x8kqkff2mxyd5wd6663ai5-source/mach_nix/nix/lib.nix
14461407592 /nix/store/n6hsk44fw5x8kqkff2mxyd5wd6663ai5-source/mach_nix/nix/nixpkgs-json.nix
10121348064 /nix/store/5n402azp0s9vza4rziv4z5y88v2cv1mq-nixpkgs/pkgs/stdenv/generic/make-derivation.nix
9964507497 /nix/store/5n402azp0s9vza4rziv4z5y88v2cv1mq-nixpkgs/lib/attrsets.nix
```

```sh
# Find out which functions inside of a specific file we spend the most time in
$ <calls.trace calltracer --maxcount 10 --coordinates --prefix /nix/store/5n402azp0s9vza4rziv4z5y88v2cv1mq-nixpkgs/lib/customisation.nix

15218517306 /nix/store/5n402azp0s9vza4rziv4z5y88v2cv1mq-nixpkgs/lib/customisation.nix:81:10
15212528114 /nix/store/5n402azp0s9vza4rziv4z5y88v2cv1mq-nixpkgs/lib/customisation.nix:69:16
15192065810 /nix/store/5n402azp0s9vza4rziv4z5y88v2cv1mq-nixpkgs/lib/customisation.nix:121:8
14857585175 /nix/store/5n402azp0s9vza4rziv4z5y88v2cv1mq-nixpkgs/lib/customisation.nix:233:14
2754525532 /nix/store/5n402azp0s9vza4rziv4z5y88v2cv1mq-nixpkgs/lib/customisation.nix:77:41
1796832904 /nix/store/5n402azp0s9vza4rziv4z5y88v2cv1mq-nixpkgs/lib/customisation.nix:86:13
1767924827 /nix/store/5n402azp0s9vza4rziv4z5y88v2cv1mq-nixpkgs/lib/customisation.nix:79:27
1600962473 /nix/store/5n402azp0s9vza4rziv4z5y88v2cv1mq-nixpkgs/lib/customisation.nix:79:60
803762791 /nix/store/5n402azp0s9vza4rziv4z5y88v2cv1mq-nixpkgs/lib/customisation.nix:79:63
765426449 /nix/store/5n402azp0s9vza4rziv4z5y88v2cv1mq-nixpkgs/lib/customisation.nix:86:32
```
