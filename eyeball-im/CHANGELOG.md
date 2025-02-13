# 0.4.1

- Add extra methods to `VectorSubscriber`: `values`, `into_values_and_stream`,
  `into_values_and_batched_stream`

# 0.4.0

- Add `truncate` method to `ObservableVector`, `ObservableVectorTransaction`
  - Add `VectorDiff::Truncate { length: usize }`

# 0.3.2

- Fix transaction commit not working when there are no subscribers

# 0.3.1

- Fix logging for rollback-on-drop

# 0.3.0

- Add `ObservableVectorTransaction` for making multiple updates as one atomic
  unit (created via `observable_vector.transaction()`)
- Remove `Stream` implementation from `VectorSubscriber` in favor of
  `.into_stream()` and `.into_batched_stream()` methods that return different
  stream types

# 0.2.6

This release only updates metadata for crates.io.

# 0.2.5

- Emit more tracing events when the `tracing` Cargo feature is enabled

# 0.2.4

- Add `ObservableVector::entries`

# 0.2.3

- Add `entry` and `for_each` methods to `ObservableVector`

# 0.2.2

- Update the lag handling in `VectorSubscriber` to yield a `VectorDiff::Reset`
  with the latest state, instead of the state N - 1 changes earlier (where N is
  the capacity of the internal buffer)

# 0.2.1

- Add `VectorDiff::map`

# 0.2.0

- Switch from the unmaintained `im` crate to the maintained fork `imbl`
- Re-export the non-observable `Vector` type from the crate root
