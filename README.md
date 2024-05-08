# Graph pathfinding visualization

## Overview
Visualization of several well-known graph pathfinding algorithms using [Rust programming language](https://www.rust-lang.org) with [pixels crate](https://github.com/parasyte/pixels) for graphics

The graph is presented as a grid with start-end points and obstacles. Each move (if it moves to a free cell) costs 1 point. 4 types of difficulties of obstacles are supported. Both Dijkstra and A* support moving through obstacles with moving's cost `obstacle's difficulty * Graph::OBSTACLE_DIFFICULTY_K`

<img src="https://github.com/Cheshulko/Graph-pathfinding-visualization-rs/blob/main/assets/pre1/dijkstra-pre1.gif" width="700">

## Algorithms
- [x] [Breadth first search](https://en.wikipedia.org/wiki/Breadth-first_search)
- [x] [Dijkstra's algorithm](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm)
- [x] [Heuristic algorithm](https://en.wikipedia.org/wiki/Admissible_heuristic) (heuristic function - [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry))
- [x] [A* algorithm](https://en.wikipedia.org/wiki/A*_search_algorithm) (heuristic function - [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry))

|  |  |  |
| -------  |---| --- |
| <img src="https://github.com/Cheshulko/Graph-pathfinding-visualization-rs/blob/main/assets/pre2/bfs-pre2.gif?raw=true" width="400" /> | Breadth first search |Seen points: 73, Length: 15|
| <img src="https://github.com/Cheshulko/Graph-pathfinding-visualization-rs/blob/main/assets/pre2/dijkstra-pre2.gif?raw=true" width="400" /> | Dijkstra's algorithm |Seen points:    76, Length: 15|
| <img src="https://github.com/Cheshulko/Graph-pathfinding-visualization-rs/blob/main/assets/pre2/h-pre2.gif?raw=true" width="400" /> | Heuristic algorithm |Seen points: 16, Length: 17|
| <img src="https://github.com/Cheshulko/Graph-pathfinding-visualization-rs/blob/main/assets/pre2/a-pre2.gif?raw=true" width="400" /> | A* algorithm |Seen points: 68, Length: 15|

## Navigation
Primitive navigation:
```
`s` - make algorithm's step
`r` - reset graph to initial state

`d` - set dijksta's algorithm
`b` - set bfs algorithm
`h` - set heuristic algorithm
`a` - set a-star algorithm

`1` - set 1' predefined graph
`2` - set 2' predefined graph
`-` - generate ramdom graph
```

## References
https://www.redblobgames.com/pathfinding/a-star/introduction.html

## License
MIT  
