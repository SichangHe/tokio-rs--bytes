# chain cursor verification

- target
  - `Chain<T, U>::remaining` and `Chain<T, U>::advance` in `src/buf/chain.rs`
- model
  - `ChainCursor` is a count-level shadow model of the two underlying buffers
  - `remaining` uses the production `saturating_add`
  - `advance` follows the production branches and subtraction order
  - there is no refinement proof connecting the model to generic `Chain<T, U>` implementations
- guarantees
  - `remaining` is the mathematical total capped at `usize::MAX`
  - a valid advance consumes exactly the requested count
  - if the count fits in the old first count, the new first count is the difference and the second is unchanged
  - otherwise, the new first count is zero and the new second count is the old total minus the count
- verifier
  - campaign-pinned Verus `0.2026.03.17.a96bad0`, release profile, Rust toolchain `1.94.0-x86_64-unknown-linux-gnu`
  - the complete verifier closure is 783 MiB and already resides on the same local filesystem, so the campaign records its exact path, version, and checksum instead of duplicating it
  - SHA-256 `9a675a675ad2e37e9d51d1fadcf3e2d8377e413f7d915665d629e30e5d92c6ed`
  - invocation:

    ```console
    /ssd1/sichangheagent/work_logs/private_artifacts/midas_lex_12096/toolchain/verus-2026-03-17/verus verification/chain_cursor.rs
    ```

# take vectored-chunk verification

- target
  - `Take<T>::chunks_vectored` in `src/buf/take.rs`
- model
  - `take_chunks_vectored_lengths` is a count-level shadow of the production prefix/truncation loop
  - source entries are lengths actually returned by the underlying `Buf`; a precondition records that their count is bounded by destination capacity and the production internal cap of 16
  - `@ASSUME:underlying_chunks_vectored` records the underlying-buffer boundary
  - production tests check byte-level correspondence separately
- guarantees
  - returned count is bounded by source count, destination capacity, and 16
  - every complete returned slice preserves its source length; only the final slice may be shorter
  - returned length total is the smaller of the `Take` limit and available source total
  - zero limit or destination capacity returns no slices
- boundary
  - there is no generic-trait refinement, byte-identity, slice-lifetime, transmute-safety, or constant-time proof
- verifier
  - campaign-pinned Verus `0.2026.03.17.a96bad0`, release profile, Rust toolchain `1.94.0-x86_64-unknown-linux-gnu`
  - the complete verifier closure is 783 MiB and already resides on the same local filesystem, so the campaign records its exact path, version, and checksum instead of duplicating it
  - SHA-256 `9a675a675ad2e37e9d51d1fadcf3e2d8377e413f7d915665d629e30e5d92c6ed`
  - invocation:

    ```console
    /ssd1/sichangheagent/work_logs/private_artifacts/midas_lex_12096/toolchain/verus-2026-03-17/verus verification/take_chunks_vectored.rs
    ```
