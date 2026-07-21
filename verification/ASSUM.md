# underlying_buf_advance

For each underlying buffer, `Buf::advance(n)` reduces its remaining count by exactly `n` when `n <= remaining()`.

An underlying buffer's remaining count is stable until that buffer is mutably accessed.

Mutably advancing one underlying buffer does not change the other buffer's remaining count.

# underlying_chunks_vectored

`Buf::chunks_vectored(dst)` returns no more slices than the supplied destination can hold, and every returned slice describes readable bytes in order. The production `Take` method supplies at most 16 slots.

The count-level `Take` shadow model receives the lengths of the slices actually returned by that call, with their count bounded by the caller capacity and 16. It does not verify generic-trait refinement, byte identity, slice lifetimes, or the production transmute.
