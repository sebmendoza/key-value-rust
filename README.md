## Log Compaction Strategy

The compaction process removes redundant entries from the log while preserving the latest state of the key-value store.

### How It Works

- Compaction Triggering: The compaction process is triggered based on two main criteria:

  - Log size reaching the COMPACTION_THRESHOLD (1MB)
  - Redundancy ratio exceeding 50% of total log size

- Space Management: KVStore keeps track of total bytes in log files, stale bytes (outdated or deleted entries), active bytes (bytes of most recent state)

- Compaction Process
  - Analysis Phase: Calculates actual redundency (using space management). Determines if compaction is necessary or prevents unnecessary compaction operations
- Compaction Phase
  - Creates a new log file
  - Sorts entries by log ID to optimize disk access patterns
  - Writes only the latest value for each key
  - Updates in-memory index to point to new locations
  - Once all operations are done, it removes old log files

### Performance Considerations

**Disk I/O**: Minimizes unnecessary disk operations by compacting only when needed
**Sequential Access**: Sorts entries before processing to improve read patterns

## Testing

- Put up the server: `RUST_LOG=debug cargo run --bin kvs-server`
- Put up the client: `cargo run --bin kvs-client get mykey`
