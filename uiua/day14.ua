+@\0&ru0 0
⊜∘≠@\n.

ShiftLeft ← (
  ☇1⇡⊟⊃(-1⧻⊢)⧻.
  ∧(⍜(⊏⊙⊡|↻≍".O".)+0_1°⊟)
)

Left ← ;⍢(ShiftLeft.⊙;|¬≍):[]
Up ← ⍜⍉Left
Down ← ⍜⇌Up
Right ← ⍜⍉Down

NorthLoad ← /+♭×+1⇡⧻.=@O⇌

Spin ← Right Down Left Up

PartOne ← NorthLoad Up

PartTwo ← (
  ⍢(⊃Spin⊂|¬∊)⊃Spin¤
  ⊃(-:1e9)↘⊗⊙.⊙⇌
  NorthLoad⊡◿⧻,
)

(PartOne|PartTwo) -1⋕⊡1&args
