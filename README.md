[![crates.io](https://img.shields.io/crates/v/voronoi_mosaic)](https://crates.io/crates/voronoi_mosaic)
[![docs.rs](https://docs.rs/voronoi_mosaic/badge.svg)](https://docs.rs/voronoi_mosaic)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/blondeburrito/voronoi_mosaic#license)
![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/blondeburrito/voronoi_mosaic/ci.yml)
![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/blondeburrito/voronoi_mosaic/code-cov.yml?label=CodeCov>85%)

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
if some points are too close together then acute angles lead to being ignored

3d - all regualr poluhedra (e.g cube, pyramid) have a circumsphere, most irregular polyhedra do not

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
