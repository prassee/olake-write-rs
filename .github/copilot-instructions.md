# Copilot Instructions: Olake Iceberg Rust Writer

## Project Context
This project is a high-performance, resource-efficient rewrite of the Olake Iceberg Java Writer into **Rust**. It must handle **PhonePe-scale** data ingestion (millions of events/sec) with strict p99 latency SLAs and minimal memory footprint.

---

## 1. Rust Syntax & Patterns Guide
* **Ownership:** Prefer borrowing (`&`) over cloning (`.clone()`). Avoid unnecessary heap allocations.
* **Async:** Use `tokio` for the async runtime. Always `.await` I/O-bound operations (S3, Catalog).
* **Error Handling:** Use `anyhow` for application-level errors and `thiserror` for library-level errors. Always use the `?` operator for propagation.
* **Type Safety:** Use Enums for state management and `Option<T>` / `Result<T, E>` instead of null checks.
* **Performance:** Use `Arc<T>` for shared read-only metadata and `sync.Pool` equivalents (like `lockfree` or `crossbeam`) if high-frequency allocations are needed.

---

## 2. Requirement Specification (V1 - V4)

### Version 1: The "Native Bridge" (Foundations)
* **Goal:** Establish connectivity and basic write capability.
* **Spec:** Connect to an Iceberg Catalog (Glue/REST), create a table with **custom properties**, and write a simple Parquet data file to S3.

### Version 2: The "CDC Specialist" (Equality Deletes)
* **Goal:** Handle row-level mutations for CDC.
* **Spec:** Implement **Equality Deletes** (Iceberg V2 spec). Support `RowDeltaAction` to commit both data and delete files in a single atomic transaction.

### Version 3: The "Scale Optimizer" (V3 & Performance)
* **Goal:** Optimize for massive throughput and Iceberg V3 features.
* **Spec:** Implement **Deletion Vectors** (V3) to minimize Merge-on-Read overhead. Support vectorized writes using `arrow-rs`.

### Version 4: The "Olake Core Integration"
* **Goal:** Full upstream readiness.
* **Spec:** Replace the gRPC Java Sidecar with the Rust binary. Implement observability (Prometheus metrics) and end-to-end integration tests.

---

## 3. Technical Specification & Dependencies

### Library Stack
* `iceberg`: Core SDK for metadata and catalog interactions.
* `arrow`: For memory-efficient, columnar data handling.
* `parquet`: For encoding data files.
* `tokio`: Multi-threaded async runtime.
* `opendal`: Unified data access layer for S3/GCS.

### Test Strategy
* **Unit Tests:** Every module must have internal tests using `cargo test`.
* **Integration Tests:** Located in `/tests`, simulating S3 using **LocalStack** or **MinIO**.
* **Property-Based Testing:** Use `proptest` for schema mapping logic to ensure no edge-case crashes.

### Code Coverage & Quality
* **Coverage:** Target **>85%** coverage. Use `cargo-tarpaulin` for reporting.
* **Linting:** Strictly follow `cargo clippy` recommendations.
* **Benchmarking:** Use `criterion` to measure Parquet encoding nanoseconds per row.

---

## 4. Implementation Rules for Copilot
* **Rule 1:** NEVER suggest using `unsafe` blocks unless explicitly requested for FFI.
* **Rule 2:** When writing metadata, always use **Transactions** to ensure ACID compliance.
* **Rule 3:** For S3 uploads, always use **Streaming Multipart Uploads** to cap memory usage.
* **Rule 4:** If a concept is non-idiomatic (e.g., Go-style pointers), suggest the Rust equivalent (e.g., `Box` or `Arc`).

***