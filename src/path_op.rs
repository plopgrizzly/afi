// Copyright Jeron Lau 2017 - 2018.
// Dual-licensed under either the MIT License or the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at https://www.boost.org/LICENSE_1_0.txt)

/// 3D Path operation for vector graphics.
#[derive(Copy, Clone)]
pub enum PathOp {
    /// Move somewhere else / start drawing new shape (x, y, z).
    Move(f32, f32, f32),
    /// Next point in edge (x, y, z).
    Line(f32, f32, f32),
    /// Qaudratic curve (cx, cy, cz, x, y, z).
    Quad(f32, f32, f32, f32, f32, f32),
    /// Cubic curve (c1x, c1y, c1z, c2x, c2y, c2z, x, y, z).
    Cubic(f32, f32, f32, f32, f32, f32, f32, f32, f32),
    /// Change the Pen / Stroke Width (width).
    Width(f32),
}
