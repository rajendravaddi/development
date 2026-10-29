# String Patterns

## 1. Anagrams

Two strings are anagrams if they contain the same characters with the same frequencies, but possibly in a different order.

**Examples**
```text
"listen"  → "silent"   ✓ Anagrams
"eat"     → "tea"      ✓ Anagrams
"race"    → "care"     ✓ Anagrams
```

**Not anagrams:**
```text
"hello"   → "world"    ✗
"apple"   → "apply"    ✗
"abc"     → "abb"      ✗
```

The key condition is: `Same characters + same frequency = anagrams`

### Notes
- we can use `frequency tuple` as unique hash for all anagrams of a string.