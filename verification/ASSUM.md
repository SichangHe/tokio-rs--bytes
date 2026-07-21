# underlying_buf_advance

For each underlying buffer, `Buf::advance(n)` reduces its remaining count by exactly `n` when `n <= remaining()`.

An underlying buffer's remaining count is stable until that buffer is mutably accessed.

Mutably advancing one underlying buffer does not change the other buffer's remaining count.
