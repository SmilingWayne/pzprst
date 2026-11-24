# Segment reference investigation

Early generator outputs can be retained here locally and are gitignored. The
canonical compact inputs are in `fixtures/angleloop.jsonl` and
`fixtures/kouchoku.jsonl`; optional full inputs are generated under the
gitignored `fixtures/full/` directory. The reference accepts rectangular boards with segments,
but some quarter turns throw while rebuilding the line graph. The pinned source
also applies the post-turn rectangle to segment coordinates; for non-square
boards this can differ from the usual D4 coordinate map even when no error is
thrown. The infallible `Board::transform` cannot express that behavior.

The fixture generator preserves the inputs, including cases that produce
reference errors. The ports now live in `src/genres` and `Board::try_transform`
models reference operations; the reference checker records and compares both
successful and rejected cases. Do not drop rectangular inputs merely to make a
check succeed.
