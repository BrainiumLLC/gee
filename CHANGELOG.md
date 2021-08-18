# Unreleased

- Added `round` convenience method to all types that have a `map` method.
- **Breaking:** Switched back to using a top-left origin.
- Added `Vector::one`.
- **Breaking:** Rename `Vector::normalized` to `Vector::normalize`.
- **Breaking:** Removed redundant method `Vector::unit_from_angle`.
- Added `Transform3d`.
- **Breaking:** `Transform::post_mul` no longer takes a reference.
- Added `Circle::radius_squared` and `Circle::contains`.
- `T` defaults to `f32`.
- Added cast methods to `Transform`.
- Added methods for decomposing and recomposing `Transform`.
- Added methods for skewing `Transform`.
- Added `degrees`, `map_radians`, and `map_degrees` to `Angle`.
- Added missing `Vector` math ops for `Vector`.
- Added missing `Vector` math ops for `Rect`.
- **Breaking:** Renamed `Rect::new` to `Rect::from_top_right_bottom_left`.
- Added `Vector::map_dx` and `Vector::map_dy`.
- Implemented `Vector` math ops for `Point`.
- Fixed `Rect::has_area` being completely wrong.
- Fixed left and right being swapped in `Rect::intersection`.
- Implemented `Vector` math ops for `Size`.
- Added methods for building a `Transform` from a `Vector`.
- Added `Transform::from_rotation_with_fixed_point`.
- **Breaking:** Switched the order of arguments for `Rect::map_width` and `Rect::map_height` to be more consistent with other `Rect` methods.
- Added `Rect::split_at_ratio_width` and `Rect::split_at_ratio_height`.

# 0.2.0 (2020-08-13)

- **Breaking:** Changed almost literally everything.

# 0.1.0 (2019-01-28)

- Initial release! 🎉
