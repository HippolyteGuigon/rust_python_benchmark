# rust_python_benchmark
The goal of this repository is to benchmark Rust as a potential tool to be used for better MLOPS workflows.

This project aims to **progressively compare Python and Rust** through a set of simple, controlled experiments in order to:
- understand the differences in execution models,
- identify performance bottlenecks (CPU vs I/O),
- and evaluate where Rust can bring real value in production MLOps systems.

The focus is **not** to replace Python, but to explore how Rust can complement it in performance-critical or production-oriented components.

---

## Build Status

At the moment, the repository contains a set of **local benchmarks** comparing Python and Rust on identical tasks.

The experiments implemented (or in progress) include:
- program startup time (Hello World),
- CPU-bound computations (sum of squares),
- analysis of I/O impact on performance,
- multi-binary Rust builds for structured benchmarking.

The next steps of the project are:
- adding benchmarks involving I/O (file reading, network),
- testing parallelism and concurrency,
- integrating Rust binaries into Python-driven workflows,
- and drawing concrete conclusions for MLOps use cases (feature engineering, drift detection, batch jobs, APIs).

This repository is intentionally **simple and local-first**, in order to keep benchmarks understandable and reproducible.

If you have suggestions, ideas, or improvements, feel free to reach out at  
**hippolyte.auguste.guigon@gmail.com** — I will be delighted to discuss and get feedback.

---

## Code style

- Python code follows **PEP-8** guidelines  
  https://peps.python.org/pep-0008/

- Rust code follows standard **Rust formatting and conventions**, enforced by:
  ```bash
  cargo fmt

The goal is to keep the code:
- minimal,
- readable,
- focused on benchmarking rather than abstraction.

---

## Installation

### Python

This project can be run using a Python virtual environment or conda.

#### Using conda (optional)
```bash
conda create -n rust-python-benchmark python=3.10
conda activate rust-python-benchmark
```

#### Or using venv

```bash
python -m venv .venv
source .venv/bin/activate
```

Install Python dependencies:

```pip install -r requirements.txt```

### Rust

Rust is managed independently using `cargo`.

Install Rust (once per machine):
```bash
curl https://sh.rustup.rs -sSf | sh
```

Make sure Rust is available:
```rustc --version```
```cargo --version```

No virtual environment is needed for Rust: dependencies and builds are handled entirely by Cargo.

## Project structure

```text
rust_python_benchmark/
│
├── python/
│   ├── hello.py
│   └── compute.py
│
├── rust/
│   └── hello/
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs
│           └── bin/
│               └── compute.rs
│
└── README.md
```

Python scripts are executed directly.
Rust programs are compiled into native binaries before execution

## How to use ?

The benchmarks can be run locally in both languages.

### Python

```python python/hello.py
python python/compute.py
```

Optional timing:

```time python python/compute.py``` 

### Rust

Compile the Rust binaries (optimized mode):

```cd rust/hello
cargo build --release
```

Run the binaries:
```./target/release/hello
./target/release/hello
```

Optional timing:
```time ./target/release/compute```

### All in one 

To have all the benchmark results computed in one step, run the following command:
```bash benchmark.sh```

## Key takeaway
This repository is not about proving that one language is better than the other, but about understanding when and why 
- Rust can outperform Python, and how this difference matters in real-world MLOps pipelines.
- Python thinks fast.
- Rust executes fast
