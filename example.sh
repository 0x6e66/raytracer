for i in {1..3}; do
    cargo run --bin main -- --preset=$i --width=1200 --height=800 --anti-aliasing=2 --max-depth=4 --occlusion-offset=0.1 --output out --versionize
done;