+@\0&ru0 0
⊜□≠,@\n

PartOne ← /+≡(parse⊐⊂⊃⊢(⊢⇌)regex"[1-9]"⊔)

ReadWords ← (
  {"one" "two" "three" "four" "five"
   "six" "seven" "eight" "nine"}
  ≡⊟:+1⇡9
  # Note: This line relies on commit 304f2af in the Uiua repository,
  # which as of writing this is not contained in a release.
  /↥⬚0⊠(×⌕⊙:⊓(∩⊔⍘⊟)⊔)
)
ReadNums ← ×⬚0≡⊃(-@0|≤@9)
PartTwo ← /+≡(+⊃(×10⊢|⊢⇌)▽≠0.)↥⊃ReadNums ReadWords

(PartOne|PartTwo) -1 parse ⊔⊡1&args
