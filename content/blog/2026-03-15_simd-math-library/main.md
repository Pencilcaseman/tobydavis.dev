# This Is a Heading

Modern CPUs ship with powerful SIMD instruction sets --- AVX2, AVX-512, NEON --- but writing portable vectorized code remains painful. A single operation like a fused multiply-add might need three completely different implementations across x86, ARM, and fallback scalar paths.

In this post, we'll walk through the design of a math library that abstracts over these backends while maintaining zero-cost performance.

## The Problem

Consider something as simple as adding two arrays element-wise. In scalar code:

```rust
fn add(a: &[f32], b: &[f32], out: &mut [f32]) {
    for i in 0..a.len() {
        out[i] = a[i] + b[i];
    }
}
```

This is clean and portable, but it leaves performance on the table. A modern CPU can process 8 floats at once with AVX2, or 16 with AVX-512. To use that, you'd write something like:

```c
// x86 AVX2 intrinsics
__m256 va = _mm256_loadu_ps(&a[i]);
__m256 vb = _mm256_loadu_ps(&b[i]);
__m256 vc = _mm256_add_ps(va, vb);
_mm256_storeu_ps(&out[i], vc);
```

And for ARM NEON, it's a completely different API:

```c
// ARM NEON intrinsics
float32x4_t va = vld1q_f32(&a[i]);
float32x4_t vb = vld1q_f32(&b[i]);
float32x4_t vc = vaddq_f32(va, vb);
vst1q_f32(&out[i], vc);
```

Same semantics, different syntax. This is the core insight that makes abstraction possible.

### This Is a Smaller Heading

Hello!

## The Abstraction Layer

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

Each backend implements this trait. The compiler monomorphizes everything --- there's no dynamic dispatch, no vtable lookup, no runtime cost.

### Backend Implementations

For AVX2 on x86:

```rust
#[cfg(target_arch = "x86_64")]
impl SimdVector for __m256 {
    const LANES: usize = 8;

    fn splat(val: f32) -> Self {
        unsafe { _mm256_set1_ps(val) }
    }

    fn add(self, rhs: Self) -> Self {
        unsafe { _mm256_add_ps(self, rhs) }
    }

    // ... remaining methods
}
```

For NEON on ARM:

```rust
#[cfg(target_arch = "aarch64")]
impl SimdVector for float32x4_t {
    const LANES: usize = 4;

    fn splat(val: f32) -> Self {
        unsafe { vdupq_n_f32(val) }
    }

    fn add(self, rhs: Self) -> Self {
        unsafe { vaddq_f32(self, rhs) }
    }

    // ... remaining methods
}
```

And a scalar fallback that works everywhere:

```rust
impl SimdVector for f32 {
    const LANES: usize = 1;

    fn splat(val: f32) -> Self { val }
    fn add(self, rhs: Self) -> Self { self + rhs }

    // ... remaining methods
}
```

### Writing Generic Code

With the trait in place, algorithms become portable:

```rust
fn dot_product<V: SimdVector>(a: &[f32], b: &[f32]) -> f32 {
    let mut acc = V::splat(0.0);
    for chunk in a.chunks_exact(V::LANES).zip(b.chunks_exact(V::LANES)) {
        let va = V::load(chunk.0);
        let vb = V::load(chunk.1);
        acc = va.fmadd(vb, acc);
    }
    // horizontal sum omitted for brevity
    0.0
}
```

One implementation, three backends, zero overhead.

## Benchmarks

We benchmarked a 1024-element dot product across all three backends on two machines:

| Backend | M2 MacBook Pro | Ryzen 9 7950X |
|---------|---------------|---------------|
| Scalar  | 312 ns        | 289 ns        |
| NEON    | 48 ns         | ---           |
| AVX2    | ---           | 38 ns         |
| AVX-512 | ---           | 21 ns         |

The SIMD backends are 6--15x faster than scalar, with AVX-512 achieving the best throughput thanks to its wider registers.

### Methodology

All benchmarks were run using [criterion.rs](https://github.com/bheisler/criterion.rs) with 1000 iterations and 100 warm-up iterations. The input data was pre-allocated and cache-hot. Results represent the median of all iterations.

We compiled with `RUSTFLAGS="-C target-cpu=native"` to ensure the compiler could use all available instructions.

## Lessons Learned

A few things we discovered along the way:

- **Alignment matters.** Aligned loads (`_mm256_load_ps`) are faster than unaligned loads on older hardware, but modern CPUs have largely closed this gap.
- **Don't mix instruction sets.** On Intel CPUs, mixing AVX and SSE code without `_mm256_zeroupper` causes a performance cliff due to register state transitions.
- **The compiler is smarter than you think.** With `-C target-cpu=native`, Rust's auto-vectorizer can often match hand-written SIMD for simple loops. The abstraction layer pays off most for complex operations like FMA chains and reductions.
- **Test on real hardware.** Emulators and CI runners often don't have AVX-512 support, so you need access to real hardware for accurate benchmarks.

## Conclusion

SIMD abstraction layers can provide zero-cost portability when designed carefully. The key is keeping the trait surface minimal and letting the compiler do its job.

The full source code is available on [GitHub](https://github.com/example/simd-math). Contributions welcome.
