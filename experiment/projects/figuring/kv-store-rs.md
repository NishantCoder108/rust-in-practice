## KV Store 
A log-structured key-value store written in Rust, inspired by the Bitcask storage model. All writes are appended to a log file on disk. An in-memory hash index maps each key to its position and length in the log, making reads a single seek. Compaction rewrites the log keeping only the latest value per key, dropping stale entries and tombstones.
