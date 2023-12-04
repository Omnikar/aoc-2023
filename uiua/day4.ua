+@\0&ru0 0                 # Read input from stdin
⊜∘≠@\n.                    # Split lines
≡↘+2⊗@:⊢.                  # Remove "Card …: " prefixes
≡(/+∊∩(⊜□≠@ .)⊃↙(↘+1)⊗@|.) # Parse number groups and count matches

PartOne ← /+≡(⌊ⁿ:2-1)
PartTwo ← /+∧(⊂+1/+⬚0↙⊙.):[]⇌ # Iterate backwards, collecting card counts

(PartOne|PartTwo) -1 parse ⊔⊡1&args
