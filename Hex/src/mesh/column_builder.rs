use glam::{Quat, Vec3};

use crate::{Hex, HexLayout, PlaneMeshBuilder, UVOptions};

use super::{BASE_FACING, MeshInfo};

/// Builder struct to customize hex column mesh generation.
///
/// By default this builder will create a full hexagonal column with all faces
/// and no side subdivisions.
/// The mesh will be anchored at the center of the *bottom face*, use offsets to
/// cutomize anchor/pivot position.
///
/// # Example
///
/// ```rust
/// # use hexx::*;
///
/// let layout = HexLayout::default();
/// let mesh = ColumnMeshBuilder::new(&layout, 15.0)
///     .at(hex(2, 3))
///     .facing(Vec3::Z)
///     .with_subdivisions(5)
///     .with_offset(Vec3::new(1.2, 3.45, 6.7))
///     .without_bottom_face()
///     .without_top_face()
///     .build();
/// ```
///
/// # Note
///
/// Transform operations (Scale, Rotate, Translate) through the methods
///
/// - Scale: [`Self::with_scale`]
/// - Rotate: [`Self::with_rotation`], [`Self::facing`]
/// - Translate: [`Self::with_offset`], [`Self::at`]
///
/// Are executed in that order, or **SRT**
#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct ColumnMeshBuilder<'l> {
    /// The hexagonal layout, used to compute vertex positions
    pub layout: &'l HexLayout,
    /// The column height
    pub height: f32,
    /// Custom hex position, will apply an offset if not [`Hex::ZERO`]
    pub pos: Hex,
    /// Optional custom offset for the mesh vertex positions
    pub offset: Option<Vec3>,
    /// Optional custom scale factor for the mesh vertex positions
    pub scale: Option<Vec3>,
    /// Optional rotation quaternion, useful to have the mesh already
    /// rotated
    ///
    /// By default the mesh is *facing* up (**Y** axis)
    pub rotation: Option<Quat>,
    /// Amount of quads to be generated on the sides of the column
    pub subdivisions: Option<usize>,
    /// Should the top hexagonal face be present
    pub top_face: bool,
    /// Should the bottom hexagonal face be present
    pub bottom_face: bool,
    /// UV mapping options for the column sides
    pub sides_uv_options: [UVOptions; 6],
    /// UV mapping options for top and bottom faces
    pub caps_uv_options: UVOptions,
    /// If set to `true`, the mesh will ignore [`HexLayout::origin`]
    pub center_aligned: bool,
}

impl<'l> ColumnMeshBuilder<'l> {
    /// Setup a new builder using the given `layout` and `height`
    #[must_use]
    pub const fn new(layout: &'l HexLayout, height: f32) -> Self {
        Self {
            layout,
            height,
            pos: Hex::ZERO,
            rotation: None,
            subdivisions: None,
            offset: None,
            scale: None,
            top_face: true,
            bottom_face: true,
            sides_uv_options: [UVOptions::new(); 6],
            caps_uv_options: UVOptions::new(),
            center_aligned: false,
        }
    }

    /// Specifies a custom `pos`, which will apply an offset to the whole mesh.
    ///
    /// ## Note
    ///
    /// It might be more optimal to generate only one mesh at [`Hex::ZERO`] and
    /// offset it later than have one mesh per hex position
    #[must_use]
    #[inline]
    pub const fn at(mut self, pos: Hex) -> Self {
        self.pos = pos;
        self
    }

    /// Specify a custom *facing* direction for the mesh, by default the column
    /// is vertical (facing up)
    ///
    /// # Panics
    ///
    /// Will panic if `facing` is zero length
    #[must_use]
    #[inline]
    pub fn facing(mut self, facing: Vec3) -> Self {
        self.rotation = Some(Quat::from_rotation_arc(BASE_FACING, facing));
        self
    }

    /// Specify a custom rotation for the whole mesh
    #[must_use]
    pub const fn with_rotation(mut self, rotation: Quat) -> Self {
        self.rotation = Some(rotation);
        self
    }

    /// Specify a cusom offset for the whole mesh. This can be used to customize
    /// the anchor/pivot of the mesh.
    ///
    /// # Example
    ///
    /// To center the pivot you can offset the mesh by half its height:
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// let layout = HexLayout::default();
    /// let height = 10.0;
    /// let mesh = ColumnMeshBuilder::new(&layout, height)
    ///     .with_offset(Vec3::new(0.0, -height / 2.0, 0.0))
    ///     .build();
    /// ```
    #[must_use]
    #[inline]
    pub const fn with_offset(mut self, offset: Vec3) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Specify a custom scale factor for the whole mesh
    #[must_use]
    pub const fn with_scale(mut self, scale: Vec3) -> Self {
        self.scale = Some(scale);
        self
    }

    /// Defines the column side quads amount
    #[must_use]
    #[inline]
    pub const fn with_subdivisions(mut self, subdivisions: usize) -> Self {
        self.subdivisions = Some(subdivisions);
        self
    }

    /// The mesh will not include a *bottom* hexagon face
    #[must_use]
    #[inline]
    pub const fn without_bottom_face(mut self) -> Self {
        self.bottom_face = false;
        self
    }

    /// The mesh will not include a *top* hexagon face
    #[must_use]
    #[inline]
    pub const fn without_top_face(mut self) -> Self {
        self.top_face = false;
        self
    }

    #[must_use]
    #[inline]
    /// Specify custom uv options for the top/bottom caps triangles
    ///
    /// Note:
    /// this won't have any effect if `top_cap` and `bottom_cap` are disabled
    pub const fn with_caps_uv_options(mut self, uv_options: UVOptions) -> Self {
        self.caps_uv_options = uv_options;
        self
    }

    #[must_use]
    #[inline]
    /// Specify custom global uv options for the side quad triangles.
    ///
    /// To customize each side quad, prefer
    /// [`Self::with_multi_sides_uv_options`]
    pub const fn with_sides_uv_options(mut self, uv_options: UVOptions) -> Self {
        self.sides_uv_options = [uv_options; 6];
        self
    }

    #[must_use]
    #[inline]
    /// Specify custom uv options for each of the side quad triangles.
    ///
    /// For a global setting prefer [`Self::with_sides_uv_options`]
    pub const fn with_multi_sides_uv_options(mut self, uv_options: [UVOptions; 6]) -> Self {
        self.sides_uv_options = uv_options;
        self
    }

    #[must_use]
    #[inline]
    /// Ignores the [`HexLayout::origin`] offset, generating a mesh centered
    /// around `(0.0, 0.0)`.
    pub const fn center_aligned(mut self) -> Self {
        self.center_aligned = true;
        self
    }

    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::many_single_char_names)]
    /// Comsumes the builder to return the computed mesh data
    pub fn build(self) -> MeshInfo {
        // We compute the mesh at the origin to allow scaling
        let cap_mesh = PlaneMeshBuilder::new(self.layout)
            .with_uv_options(self.caps_uv_options)
            .center_aligned()
            .build();
        // We store the offset to match the `self.pos`
        let pos = if self.center_aligned {
            self.layout.hex_to_center_aligned_world_pos(self.pos)
        } else {
            self.layout.hex_to_world_pos(self.pos)
        };
        let mut offset = Vec3::new(pos.x, 0.0, pos.y);
        // We create the final mesh
        let mut mesh = MeshInfo::default();
        // Column sides
        let subidivisions = self.subdivisions.unwrap_or(0).max(1);
        let delta = self.height / subidivisions as f32;
        let [a, b, c, d, e, f] = self.layout.hex_corners(Hex::ZERO);
        let corners = [[a, b], [b, c], [c, d], [d, e], [e, f], [f, a]];
        (0..6).for_each(|side| {
            let [left, right] = corners[side];
            let normal = (left + right).normalize();
            for div in 0..subidivisions {
                let height = delta * div as f32;
                let left = Vec3::new(left.x, height, left.y);
                let right = Vec3::new(right.x, height, right.y);
                let mut quad =
                    MeshInfo::quad([left, right], Vec3::new(normal.x, 0.0, normal.y), delta);
                self.sides_uv_options[side].alter_uvs(&mut quad.uvs);
                mesh.merge_with(quad);
            }
        });
        if self.top_face {
            mesh.merge_with(cap_mesh.clone().with_offset(Vec3::Y * self.height));
        }
        if self.bottom_face {
            let rotation = Quat::from_rotation_arc(BASE_FACING, -BASE_FACING);
            let bottom_face = cap_mesh.rotated(rotation);
            mesh.merge_with(bottom_face);
        }
        // **S** - We apply optional scale
        if let Some(scale) = self.scale {
            mesh.vertices.iter_mut().for_each(|p| *p *= scale);
        }
        // **R** - We rotate the mesh to face the given direction
        if let Some(rotation) = self.rotation {
            mesh = mesh.rotated(rotation);
        }
        // **T** - We offset the vertex positions after scaling and rotating
        if let Some(custom_offset) = self.offset {
            offset += custom_offset;
        }
        mesh = mesh.with_offset(offset);
        mesh
    }
}
