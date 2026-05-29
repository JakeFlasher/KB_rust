# Round Summary (test fixture)

This round cited a card that does not exist on disk. The dispatcher should
still call verify_one_card (preserving journal cardinality) and then emit
a MISSING verdict with the "file not found" detail.

## Knowledge Consulted

- cards/reading_99/does_not_exist.md
