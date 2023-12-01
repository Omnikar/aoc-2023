+@\0&ru0 0
⊜□≠,@\n

PartOne ← /+≡(parse⊐⊂⊃⊢(⊢⇌)regex"[1-9]"⊔)

ReadWords ← (
  {"one" "two" "three" "four" "five"
   "six" "seven" "eight" "nine"}
  ≡⊟:+1⇡9
  ⬚0/↥⊠(□×≡≍¤⊙(◫↧):⧻. ⊙(⧻.:)⊓(∩⊔⍘⊟)⊔)
)
ReadNums ← ⬚0/↥⊠(□×⊙(=⊢$"_").)+1⇡9
PartTwo ← /+≡(+⊃(×10⊢|⊢⇌)⍜⊔(▽≠0.))⬚0↥⊃ReadNums ReadWords

(PartOne|PartTwo) -1 parse ⊔⊡1&args
