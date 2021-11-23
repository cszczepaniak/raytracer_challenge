set -ex

cargo build --release --bin $1
perf record ./target/release/$1
perf script | inferno-collapse-perf > stacks.folded
cat stacks.folded | inferno-flamegraph > profile.svg
rm stacks.folded
rm perf.*