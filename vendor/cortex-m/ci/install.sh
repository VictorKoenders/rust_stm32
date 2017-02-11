set -ex

main() {
    curl https://sh.rustup.rs -sSf | \
        sh -s -- -y --default-toolchain $TRAVIS_RUST_VERSION

    curl -LSfs http://japaric.github.io/trust/install.sh | \
        sh -s -- \
           --force \
           --git japaric/cross \
           --tag v0.1.2 \
           --target x86_64-unknown-linux-musl \
           --to ~/.cargo/bin
}

# NOTE(TRAVIS_BRANCH) Travis is configured to only build *pushes* (not PRs)
if [ $TRAVIS_BRANCH != master ] || [ $TRAVIS_EVENT_TYPE = cron ]; then
    main
fi
