# Finite Field Implementation

## Understanding Finite Field

A finite field is a set of numbers and two operations: `+` and `*` that satisfies the following properties:

1. It is **closed** such that `a + b` and `a * b` are in the set.
   An example:

```bash
Given the set {0,1,2}
The addition 1 + 2 = 3 is not closed in the set above,also the multipcation of 1 * 3 = 3 is not closed in the set above.  On the other
hand, the set {–1,0,1} is closed under normal multiplication
```

2.  **Additive Identity**: 0 exist and has the property `a + 0 = a`.

```bash
Such that field have the additive identity which  means 0
exist in the set.
```

3. **Multiplicative Identity** : 1 exist and has the property `a * 1 = a`.

```bash
Such that field have the multiplicative identity which  means 1
exist in the set.
```

4. **Additive Inverse** : If a is in the set, `–a` is in the set, which is defined as the value that makes `a + (–a) = 0`.

```bash
That is, if a is in the set, –a is in the set. Using the additive inverse, we can define subtraction.
```

5. **Multiplicative Inverse** :
   If a is in the set and is not `0`, `a–1` is in the set, which is defined as the value that makes `a ⋅ a–1 = 1`.

```bash
That is a ⋅ a–1 = 1. Using the multiplicative inverse, we can define division.
```

## Representation of a Finite field

F<sub>p</sub> = `{0,1,2 .... p -1}`.

To define a finite field of order 19 we have;
F<sub>p</sub> = `{0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18}`

## Modular Arithemetic

To make a finite field `closed` under addition, subtraction,
multiplication, and division, we use `modulo arithmetic`.

[Modular arithmetic](https://www.youtube.com/results?search_query=modular+arithmetic) is crucial to ensure closure under addition, subtraction, multiplication, and division. Here's how modular arithmetic is important for maintaining closure in a field:

- Addition : `a + b` = `a + b mod p`

In a field, addition is defined as usual, but the result is taken modulo a prime number (the characteristic of the field). This ensures that the result is within the field and maintains closure under addition.

Subtraction: `a - b` = `a - b` `mod p`

- Subtraction is defined similarly to addition, but taking the remainder after the subtraction operation is performed modulo the prime. This ensures closure under subtraction.

- Multiplication: `a * b` = `a * b` `mod p`

Multiplication in a field involves regular multiplication followed by taking the result modulo the prime. This ensures closure under multiplication.

- Division: `a / b` = `a * b`<sup>-1</sup> `mod p`

Division is a bit more complex and involves finding the modular multiplicative inverse.

## Finite Field Addition and Subtraction

**Addition**: a + <sub>f</sub> b ∈ F <sub>19</sub>

finite field addition is defined with <sub>+f</sub> to avoid confusion with normal integer addition.


Example:

* 7+<sub>f</sub> 8 = (7+8) % 19 = 15
* 11+ <sub>f</sub> 17 = (11+17) % 19 = 9

**Subtraction**: a - <sub>f</sub> b ∈ F <sub>19</sub>

- 11 – <sub>f</sub>9 = (11-9) % 19 = 2
- 6 – <sub>f</sub>13 = (6-13) % 19 = 12


The module [field_impl.rs](src/field_impl/field_impl.rs) contains implementation of a finite field and defines several operations that can be carried out on it.