## AoC 2024 Day 13: Claw Contraption - writeup

> **Note**: In this writeup, I refer to the first coordinate as `i` and the second as `j` to avoid confusion with the `x` and `y` solutions of the Diophantine equations.

### Problem Summary
We need to operate claw machines with two buttons (A and B):
- Button A costs 3 tokens and moves the claw by (`ai`, `aj`)
- Button B costs 1 token and moves the claw by (`bi`, `bj`)
- Each prize is at a specific location (`target_i`, `target_j`)
- Goal: Find the minimum number of tokens needed to position the claw exactly over each prize

[Full problem statement](https://adventofcode.com/2024/day/13)

### Solution Approach
This is a mathematical optimization problem that can be solved using Diophantine equations. For each prize:

1. We need to find how many times to press button `A` (let's call it `x`) and button `B` (let's call it `y`)
2. The equations must satisfy both `i` and `j` coordinates:
   - For `i` coordinate: `x * ai + y * bi = target_i`
   - For `j` coordinate: `x * aj + y * bj = target_j`
   - Both equations must use the same `x` and `y` values! (The number of times we press each button is the same for both coordinates)

### Solving Using Diophantine Equations
A linear Diophantine equation has the form: ax + by = c
- It has solutions only if c is divisible by the GCD of a and b
- If (x, y) is one solution, all solutions are of the form:
  - `x = x₀ + k*v`
  - `y = y₀ - k*u`

  where `k` is any integer, and `u`, `v` are quotients of `a`, `b` by their GCD

For our problem:
1. First, we find solutions for `i` coordinate: `x_i + k_i*v_i`
2. Then for `j` coordinate: `x_j + k_j*v_j`
3. Now we need to ensure that the `x` and `y` values are the same for both coordinates:
   ```
   x_i + k_i*v_i = x_j + k_j*v_j
   y_i - k_i*u_i = y_j - k_j*u_j
   ```
4. We can solve the equation system above for `k_i` and `k_j` to determine the number of times we need to press each button so that the claw is positioned over the prize:
   ```
   k_i = (u_j*(x_j - x_i) + v_j*(y_j - y_i)) / (u_j*v_i - u_i*v_j)
   k_j = (u_i*(x_j - x_i) + v_i*(y_j - y_i)) / (u_j*v_i - u_i*v_j)
   ```

5. If k_i and k_j are integers, a solution exists for this prize!!

References:
- [Wikipedia: Diophantine equation](https://en.wikipedia.org/wiki/Diophantine_equation)
- [Video tutorial on solving Diophantine equations](https://www.youtube.com/watch?v=FjliV5u2IVw)

