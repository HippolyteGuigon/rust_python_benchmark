echo "Python timing" && \
time python python/main.py && \
echo "Rust timing" &&\
time ./rust/hello/target/release/hello