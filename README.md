# Polyomino Puzzle

A polyomino puzzle solver was implemented. It runs fast for puzzles up to Pentominoes. While it is slow, it can still handle Hexominoes. However, for Heptominoes and larger, it is too slow to be practical. For Pentomino puzzles, the code outputs the total number of solutions for a given board. For Hexominoes and larger polyominoes, the solver stops and outputs as soon as it finds the first solution, as counting all solutions is impractical.

**How to use**: 
1. In the main() function, create a board object as a Vec<Vec\<usize\>>. Use 1 to mark holes (places where pieces cannot be placed) and 0 for open spaces.
2. To use the Dancing Links solver, call solve_polyomino_dlx(board, size). For the Backtracking solver, call solve_polyomino_bt(board, size).
3. Both solve_polyomino_XXX functions return a Vec<Vec<Vec\<usize\>>>, which is a vector of boards. Each board represents a solution.

**Backtracking**: According to [WikiPedia's Backtracking page](https://en.wikipedia.org/wiki/Backtracking) 
> Backtracking is a class of algorithms for finding solutions to some computational problems, notably constraint satisfaction problems, that incrementally builds candidates to the solutions, and abandons a candidate ("backtracks") as soon as it determines that the candidate cannot possibly be completed to a valid solution.

I used [Ethnum](https://crates.io/crates/ethnum)'s U256 for the bitmap, so the current backtracking code does not work on large boards with more than 256 cells. If you use u64 for the bitmap, the current code runs a little faster.

**Dancing Links**: To solve polyomino puzzles as exact cover problems, [Knuth's Algorithm X](https://en.wikipedia.org/wiki/Knuth%27s_Algorithm_X) is used. According to [Dancing Links](https://doi.org/10.48550/arXiv.cs/0011047), 
> My purpose is to discuss an extremely simple technique that deserves to be better known. Suppose x points to an element of a doubly linked list; let L[x] and R[x] point to the predecessor and successor of that element. Then the operations
> 
> (1) L[R[x]] ← L[x], R[L[x]] ← R[x]
> 
> remove x from the list; every programmer knows this. But comparatively few programmers have realized that the subsequent operations
> 
> (2) L[R[x]] ← x, R[L[x]] ← x
> 
> will put x back into the list again. ...The idea of (2) was introduced in 1979 by Hitotumatu and Noshita, who showed that it makes Dijkstra’s well-known program for the N queens problem run nearly twice as fast without making the program significantly more complicated.

To implement this efficiently in Rust, I used the great implementation of [Dancing Links In Rust](https://ferrous-systems.com/blog/dlx-in-rust/). They use indices instead of references to implement Dancing Links, which is super cool.

**Redelmeier's algorithm**: To generate polyomino pieces automatically for a given size n, Redelmeier's algorithm is used. It is described in [Counting polyominoes: Yet another attack](https://doi.org/10.1016/0012-365X(81)90237-5). My current implementation lists free polyominoes only. Free polyominoes are distinct if no rigid transformation (such as moving, rotating, or flipping) can make one match the other. A rigid transformation keeps the shape and size of an object the same. Non-rigid transformations, like scaling, shearing, or stretching, change the size or proportions of the shape.

**Performance**: I ran tests using boards with n = 4 to 7 (each having exactly one solution) in the Dancing Links code and measured how long it took to find the solution. The results were as follows:

| name      | # of pieces | time(sec) |
|-----------|-------------|-----------|
| tetromino | 5           | 0.0056    |
| pentomino | 12          | 0.4239    |
| hexomino  | 35          | 33.711    |
| heptomino | 108         | 2828.89   |

If the goal is to find just a few solutions in a large board, backtracking with proper pruning might be faster than Dancing Links.


