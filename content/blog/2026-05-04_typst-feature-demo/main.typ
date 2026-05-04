#import "/template.typ": *

#show: template.with()

In this post, we'll walk through the design of a math library that abstracts
over these backends while maintaining zero-cost performance.

== The Problem

Consider something as simple as adding two arrays element-wise. In scalar code:

```rust
fn add(a: &[f32], b: &[f32], out: &mut [f32]) {
    for i in 0..a.len() {
        out[i] = a[i] + b[i];
    }
}
```

Hello?

For a polynomial of degree two with coefficients $a$, $b$ and $c$, the roots of
the polynomial are given by

$
  x = (-b plus.minus sqrt(b^2 - 4 a c)) / (2 a).
$

Goodbye?

#import "@preview/cetz:0.5.0"

#figure(cetz.canvas(length: 3cm, {
  import cetz.draw: *

  // Change the design for all elements after it
  set-style(
    // Design of arrow tips at the end of lines
    mark: (fill: black, scale: 2),
    // Design of lines
    stroke: (thickness: 0.4pt, cap: "round"),
    // Design of angles
    angle: (
      radius: 0.3,
      label-radius: .22,
      fill: green.lighten(80%),
      stroke: (paint: green.darken(50%)),
    ),
    // Design of all text elements with an anchor
    content: (padding: 1pt),
  )

  // Draws the grid behind the circle
  grid(
    (-1.5, -1.5),
    (1.4, 1.4),
    step: 0.5,
    stroke: gray + 0.2pt,
  )

  // Draw the unit circle
  circle((0, 0), radius: 1)

  // Draw the axis lines and axis labels
  line((-1.5, 0), (1.5, 0), mark: (end: "stealth"))
  content((), $ x $, anchor: "west")
  line((0, -1.5), (0, 1.5), mark: (end: "stealth"))
  content((), $ y $, anchor: "south")

  // Draw the number steps on the x-axis
  for (x, ct) in ((-1, $ -1 $), (-0.5, $ -1/2 $), (1, $ 1 $)) {
    line((x, 3pt), (x, -3pt))
    content((), anchor: "north", ct)
  }

  // Draw the number steps on the y-axis
  for (y, ct) in ((-1, $ -1 $), (-0.5, $ -1/2 $), (0.5, $ 1/2 $), (1, $ 1 $)) {
    line((3pt, y), (-3pt, y))
    content((), anchor: "east", ct)
  }

  // Draw the green angle
  cetz.angle.angle((0, 0), (1, 0), (1, calc.tan(30deg)), label: text(
    green,
    [#sym.alpha],
  ))

  // Draw the hypothenuse of the triangle
  line((0, 0), (1, calc.tan(30deg)))

  // Change the stroke for all upcoming elements
  set-style(stroke: (thickness: 1.2pt))

  // Draw the inner opposite leg of the triangle:
  // "The intersection of a vertical line (|-) through (30deg, 1) and a horizontal line through (0, 0)"
  line((30deg, 1), ((), "|-", (0, 0)), stroke: (paint: red), name: "sin")
  // Place the text halfway through on the opposite leg
  content(("sin.start", 50%, "sin.end"), text(red)[$ sin alpha $])

  // Draw the adjacent leg of the triangle
  line("sin.end", (0, 0), stroke: (paint: blue), name: "cos")
  // Place the text halfway and position it below the line
  content(
    ("cos.start", 50%, "cos.end"),
    text(blue)[$ cos alpha $],
    anchor: "north",
  )

  // Draw the outer opposite leg of the triangle
  line((1, 0), (1, calc.tan(30deg)), name: "tan", stroke: (paint: orange))
  // Draw the tangent equasion at the top and to the right of the line
  content(
    "tan.end",
    $
      text(#orange, tan alpha) = text(#red, sin alpha) / text(#blue, cos alpha)
    $,
    anchor: "west",
  )
}))

Figure?

This is clean and portable, but it leaves performance on the table. A modern CPU
can process 8 floats at once with AVX2, or 16 with AVX-512. To use that, you'd
write something like:

```c
// x86 AVX2 intrinsics
__m256 va = _mm256_loadu_ps(&a[i]);
__m256 vb = _mm256_loadu_ps(&b[i]);
__m256 vc = _mm256_add_ps(va, vb);
_mm256_storeu_ps(&out[i], vc);
```

Same semantics, different syntax. This is the core insight that makes
abstraction possible.

=== A Quick Aside on Auto-Vectorization

Before we dive in, it's worth noting that compilers _can_ auto-vectorize simple
loops. But for complex operations --- FMA chains, reductions, shuffles --- you
need explicit control.

== The Abstraction Layer

We define a trait that captures the shared behaviour:

```rust
pub trait SimdVector: Sized + Copy {
    const LANES: usize;

    fn splat(val: f32) -> Self;
    fn load(ptr: &[f32]) -> Self;
    fn store(self, ptr: &mut [f32]);

    fn add(self, rhs: Self) -> Self;
    fn mul(self, rhs: Self) -> Self;
    fn fmadd(self, b: Self, c: Self) -> Self;
}
```

Each backend implements this trait. The compiler monomorphizes everything ---
there's no dynamic dispatch, no vtable lookup, no runtime cost.

#note[
  All benchmarks in this post were run on an M2 MacBook Pro with NEON enabled.
  Your x86 results may differ significantly.
]

=== Backend Implementations

Each architecture gets its own zero-cost implementation. The scalar fallback
works everywhere:

```rust
impl SimdVector for f32 {
    const LANES: usize = 1;
    fn splat(val: f32) -> Self { val }
    fn add(self, rhs: Self) -> Self { self + rhs }
}
```

=== Writing Generic Code

With the trait in place, algorithms become portable:

```rust
fn dot_product<V: SimdVector>(a: &[f32], b: &[f32]) -> f32 {
    let mut acc = V::splat(0.0);
    for chunk in a.chunks_exact(V::LANES).zip(b.chunks_exact(V::LANES)) {
        let va = V::load(chunk.0);
        let vb = V::load(chunk.1);
        acc = va.fmadd(vb, acc);
    }
    0.0 // horizontal sum omitted
}
```

One implementation, three backends, zero overhead.

== Benchmarks

We benchmarked a 1024-element dot product across all backends:

#table(
  columns: 3,
  table.header[Backend][M2 MacBook Pro][Ryzen 9 7950X],
  [Scalar], [312 ns], [289 ns],
  [NEON], [48 ns], [---],
  [AVX2], [---], [38 ns],
  [AVX-512], [---], [21 ns],
)

The SIMD backends are 6--15× faster than scalar, with AVX-512 achieving the best
throughput.

#tip[
  Compile with `RUSTFLAGS="-C target-cpu=native"` to enable all available SIMD
  instructions on your machine.
]

=== Some Maths

The dot product of two vectors $bold(a)$ and $bold(b)$ is defined as:

$ bold(a) dot bold(b) = sum_(i=1)^n a_i b_i $

With SIMD, we compute this in chunks of $L$ lanes:

$
  bold(a) dot bold(b) = sum_(j=0)^(n\/L - 1) sum_(k=0)^(L-1) a_(j L + k) b_(j L + k)
$

== Lessons Learned

A few things we discovered along the way:

- *Alignment matters.* Aligned loads are faster on older hardware, but modern
  CPUs have largely closed this gap.
- *Don't mix instruction sets.* On Intel CPUs, mixing AVX and SSE code without
  `_mm256_zeroupper` causes severe performance penalties.
- *The compiler is smarter than you think.* With `-C target-cpu=native`, Rust's
  auto-vectorizer can often match hand-written SIMD for simple loops.

#warning[
  Mixing AVX and SSE code without `_mm256_zeroupper` causes severe performance
  penalties on Intel CPUs.
]

== Conclusion

SIMD abstraction layers can provide zero-cost portability when designed
carefully. The key is keeping the trait surface minimal and letting the compiler
do its job.

#success[All 847 tests passing across x86_64, aarch64, and scalar fallback.]
