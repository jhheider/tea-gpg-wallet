# Show all scripts
default:
  just -l

# For getting ptrace as html on macos
coverage:
  docker run \
    --name semverator \
    --rm \
    --volume .:/volume \
    --security-opt seccomp=unconfined \
    --platform linux/amd64 \
    xd009642/tarpaulin \
    cargo tarpaulin --engine llvm -o html --all --all-targets --all-features --output-dir /volume/coverage
