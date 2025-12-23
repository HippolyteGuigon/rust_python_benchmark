echo "Python timing Hello" && \
time python python/main.py && \
echo "" && \
echo "Rust timing Hello" &&\
time ./rust/hello/target/release/hello && \
echo "" && \
echo "Python timing Hello" && \
time python python/compute.py && \
echo "" && \
echo "Rust timing Hello" && \
time ./rust/hello/target/release/compute 
