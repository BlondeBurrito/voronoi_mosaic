[![crates.io](https://img.shields.io/crates/v/voronoi_mosaic)](https://crates.io/crates/voronoi_mosaic)
[![docs.rs](https://docs.rs/voronoi_mosaic/badge.svg)](https://docs.rs/voronoi_mosaic)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/blondeburrito/voronoi_mosaic#license)
![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/blondeburrito/voronoi_mosaic/ci.yml)
![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/blondeburrito/voronoi_mosaic/code-cov.yml?label=CodeCov>85%)

<img src="https://raw.githubusercontent.com/BlondeBurrito/voronoi_mosaic/refs/heads/main/docs/png/emblem.png" alt="e" width="300"/>

# voronoi_mosaic

Bevy mesh generation from a series of points in space using [Delaunay Triangulation](https://en.wikipedia.org/wiki/Delaunay_triangulation) and [Voronoi Tessellation](https://en.wikipedia.org/wiki/Voronoi_diagram).

| bevy | voronoi_mosaic |
|------|----------------|
| 0.16 | TBD |

## Table of Contents

1. [Intro](#intro)
1. [Delaunay Triangulation](#delaunay-triangulation)
1. [Voronoi Tessellation](#voronoi-tessellation)
1. [Usage](#usage)
1. [Performance](#performance)
1. [License](#license)

## Intro

This library is designed to generate Bevy meshes from a series of points in space.

## Delaunay Triangulation

Delaunay Triangulation describes a set of data points in space that form a series of triangles whereby each circumcircle of a triangle does not contain any of the data points.

A series of data points that have been triangulated:

<img src="https://raw.githubusercontent.com/BlondeBurrito/voronoi_mosaic/refs/heads/main/docs/png/delaunay_tri.png" alt="e" width="300"/>

<details>
<summary>To read through the triangulation process click to exapnd</summary>

### Process

abc

</details>

## Voronoi Tessellation

A Voronoi Tesselation (or Voronoi diagram) describes a number of regions (referred to here as Cells) for which all points in a plane belong to a particular Cell.

Here is an example showing each Cell as a different colour (some cells extend beyond the viewport):

<img src="https://raw.githubusercontent.com/BlondeBurrito/voronoi_mosaic/refs/heads/main/docs/png/2d_voronoi.png" alt="e" width="300"/>

<details>
<summary>For the details of converting Delanay into Voronoi click to expand</summary>

### Process

abc

</details>

## Usage

### 2d

#### Delaunay

Generating the Delaunay simply requires a series of points in space:

```rust
use bevy::prelude::*;
use voronoi_mosaic::prelude::*;

let points: Vec<Vec2> = vec![...];
if let Some(delaunay) = DelaunayData::compute_triangulation_2d(&points) {
	// do something with the data
}
```

For a full visualisation you can check out this example [2d_delaunay](https://github.com/BlondeBurrito/voronoi_mosaic/blob/main/examples/2d_delaunay.rs).

#### Voronoi

With some generated Delaunay data the Voronoi Cells can easily be generated:

```rust
use bevy::prelude::*;
use voronoi_mosaic::prelude::*;

let points = vec![...];
if let Some(delaunay) = DelaunayData::compute_triangulation_2d(&points) {
	if let Some(voronoi) = VoronoiData::from_delaunay_2d(&delaunay) {
		// do something with the generated cells
	}
}
```

For a full visualisation you can check out this example [2d_voronoi](https://github.com/BlondeBurrito/voronoi_mosaic/blob/main/examples/2d_voronoi.rs).

#### Meshes

The Voronoi data can be converted into Bevy meshes like so:

```rust
use bevy::prelude::*;
use voronoi_mosaic::prelude::*;

let points = vec![...];
if let Some(delaunay) = DelaunayData::compute_triangulation_2d(&points) {
	if let Some(voronoi) = VoronoiData::from_delaunay_2d(&delaunay) {
		// convert the cell data structures into bevy meshes
		let meshes = voronoi.as_bevy_meshes_2d();
	}
}
```

For a full visualisation you can check out this example [2d_meshes](https://github.com/BlondeBurrito/voronoi_mosaic/blob/main/examples/2d_meshes.rs).

#### Clipping

Voronoi Cells can be clipped to a boundary - this means that any Cells outside of a given boundary are dropped and any that overlap the boundary have their vertices clipped to the boundary edge.

It is important to note that clipping involves adding/removing vertices, this shatters the duality between Voronoi and Delaunay - once clipped you wouldn't be able to convert Voronoi to Delaunay and expect to get your original data set back.

```rust
use bevy::prelude::*;
use voronoi_mosaic::prelude::*;

let points = vec![...];
if let Some(delaunay) = DelaunayData::compute_triangulation_2d(&points) {
	if let Some(mut voronoi) = VoronoiData::from_delaunay_2d(&delaunay) {
		// define a series of boundary vertices that form a polygon
		// they must be in anti-clockwise order!
		let boundary = vec![...];
		voronoi.clip_cells_to_boundary(&boundary);
		// do something with the clipped cells like turning them into meshes
		let meshes = voronoi.as_bevy_meshes_2d();
	}
}
```

For a full visualisation you can check out this exmaple [2d_meshes_clipped](https://github.com/BlondeBurrito/voronoi_mosaic/blob/main/examples/2d_meshes_clipped.rs).

### 3d

#### Delaunay

#### Voronoi

#### Meshes

#### Clipping

## Performance

A number of [benchmarks](https://github.com/BlondeBurrito/voronoi_mosaic/tree/main/benches) are included to measure the different phases of calculation.

You can run all benchmarks with:

```bash
cargo bench -q --benches --workspace --all-features
```

Or target a benchmark specifically:

```bash
cargo bench -q --bench BENCH_NAME --workspace --all-features
```

Once executed a browser based report can be viewed at `[your_repo_root]/target/criterion/report/index.html`

# LICENSE

Dual license of MIT and Apache.

# TODO/ notes

todo
TODO note about extremely actue angles in a triangle getting dropped
TODO add acute angle detection into delaunay and vornoi, and warn if data points are close to each other?
TODO use seed for random points in examples?
TODO showcase of subdiving voronoi cells by running them back into delaunay then voronoi
if some points are too close together then acute angles lead to being ignored

3d - all regualr poluhedra (e.g cube, pyramid) have a circumsphere, most irregular polyhedra do not

raw delany
voronoi from raw delauny stream
voronoi from points (generates and discard delaunay)
meshes directly from points (delaunay and voronoi discarded)
todo
todo how many minimum points needed to generate
