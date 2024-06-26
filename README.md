# log graph

Log graph is a simple tool to create minimalistic graphs from log files in toml like format:
```
[Mon Apr  1 00:13:32 2024]
Iteration 14190000 / 334007599
Iteration 23160000 / 129088151
[Mon Apr  1 00:20:09 2024]
Iteration 23170000 / 129088151
Iteration 14200000 / 334007599
```
While not being scientifically accurate or even labeled for that matter, these graphs allow understanding simple trends in data at a glance.

This is a minor side project targeting simplicity and getting _finished_. This will likely not accept most patches and will have only limited maintenance.

## Usage
```
log-graph [OPTIONS] <INPUT_FILE> <OUTPUT_PNG>
```

### Arguments
```
  <INPUT_FILE>  Path of the input file
  <OUTPUT_PNG>  Path of the output image
```

### Options
```
  -r, --rate           Whether to generate a graph of the rate of change of the values
  -w, --width <WIDTH>  Width of the line in the graph [default: 8]
  -h, --help           Print help
  -V, --version        Print version
```