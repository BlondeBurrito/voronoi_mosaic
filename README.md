[![crates.io](https://img.shields.io/crates/v/voronoi_mosaic)](https://crates.io/crates/voronoi_mosaic)
[![docs.rs](https://docs.rs/voronoi_mosaic/badge.svg)](https://docs.rs/voronoi_mosaic)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/blondeburrito/voronoi_mosaic#license)
![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/blondeburrito/voronoi_mosaic/ci.yml)
![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/blondeburrito/voronoi_mosaic/code-cov.yml?label=CodeCov>85%)

<img src="https://raw.githubusercontent.com/BlondeBurrito/voronoi_mosaic/main/docs/png/emblem.png" alt="e" width="300"/>

# voronoi_mosaic

Bevy mesh generation from a series of points in space using [Delaunay Triangulation](https://en.wikipedia.org/wiki/Delaunay_triangulation) and [Voronoi Tessellation](https://en.wikipedia.org/wiki/Voronoi_diagram).

| bevy | voronoi_mosaic |
|------|----------------|
| 0.16 | 0.1 |

# Table of Contents

1. [Intro](#intro)
1. [Delaunay Triangulation](#delaunay-triangulation)
1. [Voronoi Tessellation](#voronoi-tessellation)
1. [Design/Process](#designprocess)
1. [Usage](#usage)
1. [Features](#features)
1. [Performance](#performance)
1. [License](#license)

## Intro

TODO

## Delaunay Triangulation

todo
TODO note about extremely actue angles in a triangle getting dropped
TODO add acute angle detection into delaunay and vornoi, and warn if data points are close to each other?
TODO use seed for random points in examples?
TODO showcase of subdiving voronoi cells by running them back into delaunay then voronoi
if some points are too close together then acute angles lead to being ignored

3d - all regualr poluhedra (e.g cube, pyramid) have a circumsphere, most irregular polyhedra do not

``` txt
if super triangle is minimum size, i.e two of its sides touch the corners of the data plane then triangulation can leaves holes - suspect it's in relation to dimensions of data plane. exmaple: plane min (-51, -1) max (51, 51) is a flat wide rectangle

Adding point to triangulation: Vec2(0.0, 50.0)
Circumcircle from triangle [Vec2(0.0, -27.0), Vec2(-203.01923, 76.5), Vec2(203.01923, 76.5)] with centre [-0, 223.86499] and radius 250.86499
Point Vec2(0.0, 50.0) is within circumcircle
Bad triangles contains [Triangle2d { vertex_a: Vec2(0.0, -27.0), vertex_b: Vec2(-203.01923, 76.5), vertex_c: Vec2(203.01923, 76.5) }]
Triangles after removing bads contains []
Verts to use in new triangles [Vec2(0.0, -27.0), Vec2(203.01923, 76.5), Vec2(-203.01923, 76.5)]
Adding new triangle [Vec2(0.0, 50.0), Vec2(0.0, -27.0), Vec2(203.01923, 76.5)]
Adding new triangle [Vec2(0.0, 50.0), Vec2(203.01923, 76.5), Vec2(-203.01923, 76.5)]
Adding new triangle [Vec2(0.0, 50.0), Vec2(-203.01923, 76.5), Vec2(0.0, -27.0)]
Adding point to triangulation: Vec2(-50.0, 0.0)
Circumcircle from triangle [Vec2(0.0, 50.0), Vec2(0.0, -27.0), Vec2(203.01923, 76.5)] with centre [108.26451, 11.5] and radius 114.90628
Circumcircle from triangle [Vec2(0.0, 50.0), Vec2(203.01923, 76.5), Vec2(-203.01923, 76.5)] with centre [0, 840.9256] and radius 790.9256
Circumcircle from triangle [Vec2(0.0, 50.0), Vec2(-203.01923, 76.5), Vec2(0.0, -27.0)] with centre [-108.26451, 11.5] and radius 114.90628
Point Vec2(-50.0, 0.0) is within circumcircle
Bad triangles contains [Triangle2d { vertex_a: Vec2(0.0, 50.0), vertex_b: Vec2(-203.01923, 76.5), vertex_c: Vec2(0.0, -27.0) }]
Triangles after removing bads contains [Triangle2d { vertex_a: Vec2(0.0, 50.0), vertex_b: Vec2(0.0, -27.0), vertex_c: Vec2(203.01923, 76.5) }, Triangle2d { vertex_a: Vec2(0.0, 50.0), vertex_b: Vec2(203.01923, 76.5), vertex_c: Vec2(-203.01923, 76.5) }]
Verts to use in new triangles [Vec2(0.0, 50.0), Vec2(-203.01923, 76.5), Vec2(0.0, -27.0)]
Adding new triangle [Vec2(-50.0, 0.0), Vec2(0.0, 50.0), Vec2(-203.01923, 76.5)]
Adding new triangle [Vec2(-50.0, 0.0), Vec2(-203.01923, 76.5), Vec2(0.0, -27.0)]
Adding new triangle [Vec2(-50.0, 0.0), Vec2(0.0, -27.0), Vec2(0.0, 50.0)]
Adding point to triangulation: Vec2(50.0, 0.0)
Circumcircle from triangle [Vec2(0.0, 50.0), Vec2(0.0, -27.0), Vec2(203.01923, 76.5)] with centre [108.26451, 11.5] and radius 114.90628
Point Vec2(50.0, 0.0) is within circumcircle
Circumcircle from triangle [Vec2(0.0, 50.0), Vec2(203.01923, 76.5), Vec2(-203.01923, 76.5)] with centre [0, 840.9256] and radius 790.9256
Circumcircle from triangle [Vec2(-50.0, 0.0), Vec2(0.0, 50.0), Vec2(-203.01923, 76.5)] with centre [-97.09221, 97.09221] and radius 107.91003
Circumcircle from triangle [Vec2(-50.0, 0.0), Vec2(-203.01923, 76.5), Vec2(0.0, -27.0)] with centre [-1741.9521, -3193.041] and radius 3613.615
Circumcircle from triangle [Vec2(-50.0, 0.0), Vec2(0.0, -27.0), Vec2(0.0, 50.0)] with centre [-11.5, 11.5] and radius 40.18084
Bad triangles contains [Triangle2d { vertex_a: Vec2(0.0, 50.0), vertex_b: Vec2(0.0, -27.0), vertex_c: Vec2(203.01923, 76.5) }]
Triangles after removing bads contains [Triangle2d { vertex_a: Vec2(0.0, 50.0), vertex_b: Vec2(203.01923, 76.5), vertex_c: Vec2(-203.01923, 76.5) }, Triangle2d { vertex_a: Vec2(-50.0, 0.0), vertex_b: Vec2(0.0, 50.0), vertex_c: Vec2(-203.01923, 76.5) }, Triangle2d { vertex_a: Vec2(-50.0, 0.0), vertex_b: Vec2(-203.01923, 76.5), vertex_c: Vec2(0.0, -27.0) }, Triangle2d { vertex_a: Vec2(-50.0, 0.0), vertex_b: Vec2(0.0, -27.0), vertex_c: Vec2(0.0, 50.0) }]
Verts to use in new triangles [Vec2(0.0, -27.0), Vec2(203.01923, 76.5), Vec2(0.0, 50.0)]
Adding new triangle [Vec2(50.0, 0.0), Vec2(0.0, -27.0), Vec2(203.01923, 76.5)]
Adding new triangle [Vec2(50.0, 0.0), Vec2(203.01923, 76.5), Vec2(0.0, 50.0)]
Adding new triangle [Vec2(50.0, 0.0), Vec2(0.0, 50.0), Vec2(0.0, -27.0)]
Discarding triangle [Vec2(0.0, 50.0), Vec2(203.01923, 76.5), Vec2(-203.01923, 76.5)]
Discarding triangle [Vec2(-50.0, 0.0), Vec2(0.0, 50.0), Vec2(-203.01923, 76.5)]
Discarding triangle [Vec2(-50.0, 0.0), Vec2(-203.01923, 76.5), Vec2(0.0, -27.0)]
Discarding triangle [Vec2(-50.0, 0.0), Vec2(0.0, -27.0), Vec2(0.0, 50.0)]
Discarding triangle [Vec2(50.0, 0.0), Vec2(0.0, -27.0), Vec2(203.01923, 76.5)]
Discarding triangle [Vec2(50.0, 0.0), Vec2(203.01923, 76.5), Vec2(0.0, 50.0)]
Discarding triangle [Vec2(50.0, 0.0), Vec2(0.0, 50.0), Vec2(0.0, -27.0)]
Computed final triangles []

```

## Voronoi Tessellation

<details>
<summary>Click to expand!</summary>
</details>

todo
todo how many minimum points needed to generate

## Useage

raw delany
voronoi from raw delauny stream
voronoi from points (generates and discard delaunay)
meshes directly from points (delaunay and voronoi discarded)

# LICENSE

Dual license of MIT and Apache.
