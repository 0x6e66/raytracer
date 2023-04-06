for i in {1..4}; do
    cargo run --bin raytracer -- img --preset=$i --width=1200 --height=800 --anti-aliasing=2 --max-depth=4 --occlusion-offset=0.1 --output out --versionize
done;