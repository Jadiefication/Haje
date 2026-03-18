# TODO.md — Quantum Atom Simulation Project

## Phase 0 — Prep & Math Refactor
- [x] Audit all `complex.rs`, `reals.rs`, `vec2/3/4.rs` operations.
- [x] Rework math operations to avoid `Copy`/`Clone` where unnecessary.
    - Use higher-order trait bounds and references (`&self`, `&rhs`) for arithmetic.
    - Ensure operators return new values without unnecessary cloning.
- [x] Add unit tests for basic operations (addition, multiplication, norm, dot product).

## Phase 1 — 2D Wavefunction Core
- [x] Define 2D grid structure to store `Complex` amplitudes.
- [x] Implement simple 2D Laplacian finite difference operator.
- [x] Implement Gaussian initialization for ψ(x,y).
- [x] Implement boundary conditions (e.g., Dirichlet or periodic).
- [x] Verify norm preservation for initial ψ. Verify the probability.

## Phase 2 — Time Evolution
- [x] Implement basic explicit time-stepping for ψ(t+Δt). Done through the schrodinger's(Computing the time derivative in the 1D SE, by doing that we get the change over time).
- [x] Optionally implement Crank–Nicolson for stability. (Used an approximation)
- [ ] Ensure unitary-like evolution: norm of ψ should remain ~1.
- [ ] Test with simple potentials (harmonic oscillator).

## Phase 3 — Visualization (OpenGL)
- [ ] Map |ψ(x,y)|² to colors or brightness.
- [ ] Render 2D grid as points or heatmap.
- [ ] Implement frame update loop and animate ψ over time.
- [ ] Add user controls to pause/reset/adjust Δt.

## Phase 4 — Physics Enhancements
- [ ] Introduce multiple wave packets for interference patterns.
- [ ] Introduce time-dependent superpositions.
- [ ] Implement more complex potentials (e.g., wells, barriers).
- [ ] Add probability measurement visualizations (summing |ψ|² over regions).

## Phase 5 — Prep for 3D & Hydrogen Atom
- [ ] Generalize grid to 3D using `vec3.rs`.
- [ ] Extend Laplacian operator to 3D.
- [ ] Prepare radial/angular decomposition functions for hydrogen-like potentials.
- [ ] Plan visualization strategy for 3D (slices, isosurfaces, projections).

## Misc / Future
- [ ] Consider sparse matrix representations for large grids.
- [ ] Benchmark performance of math primitives and grid updates.
- [ ] Document math engine clearly for future quantum simulation extensions.
- [ ] Ensure modular design: math layer separated from visualization.

---

This TODO.md gives a clear path from math refactoring → 2D wavefunction → time evolution → visualization → physics enhancements → prep for 3D.