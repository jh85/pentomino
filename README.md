# Polyomino Puzzle

**Backtracking**: According to [WikiPedia's Backtracking page](https://en.wikipedia.org/wiki/Backtracking) 
> Backtracking is a class of algorithms for finding solutions to some computational problems, notably constraint satisfaction problems, that incrementally builds candidates to the solutions, and abandons a candidate ("backtracks") as soon as it determines that the candidate cannot possibly be completed to a valid solution.

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

To implement this efficiently in Rust, I used the great implementation of [Dancing Links In Rust](https://ferrous-systems.com/blog/dlx-in-rust/)


**Redelmeier's algorithm**: To generate polyomino pieces automatically for a given size n, Redelmeier's algorithm is used. It is described in [Counting polyominoes: Yet another attack](https://doi.org/10.1016/0012-365X(81)90237-5). Only free polyominoes are used. Free polyominoes are distinct if no rigid transformation (such as moving, rotating, or flipping) can make one match the other. A rigid transformation keeps the shape and size of an object the same. Non-rigid transformations, like scaling, shearing, or stretching, change the size or proportions of the shape.




