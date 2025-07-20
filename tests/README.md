Tests must be run from the test directory diretly!


For next-level testing, consider:
Property tests (using proptest crate): Generate random inputs, verify invariants hold
Invariant tests: After any mutation, canonical status should be correctly updated
Round-trip tests: set → get → set should be identity
Edge cases: Empty strings, unicode, very long inputs

Acceptance
Integration
Invarient
Proptest
Regression