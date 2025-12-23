echo "Python timing Hello" && \
time python python/main.py && \
echo "" && \
echo "Rust timing Hello" &&\
time ./rust/hello/target/release/hello && \
echo "" && \
echo "Python timing compute" && \
time python python/compute.py && \
echo "" && \
echo "Rust timing compute" && \
time ./rust/hello/target/release/compute && \
echo "" && \
echo "Python timing compute parrallel multiprocessing" && \
time python python/compute_parrallel.py && \
echo "" && \
echo "Rust timing compute parrallel multiprocessing" && \
time ./rust/hello/target/release/compute_parrallel 
