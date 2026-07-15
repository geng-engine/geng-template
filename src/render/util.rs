#![allow(dead_code)]

use super::*;

#[derive(Debug, Clone, Copy)]
pub struct TextRenderOptions {
    pub size: f32,
    pub align: vec2<f32>,
    pub color: Color,
    pub rotation: Angle,
}

impl TextRenderOptions {
    pub fn new(size: f32) -> Self {
        Self { size, ..default() }
    }

    pub fn size(self, size: f32) -> Self {
        Self { size, ..self }
    }

    pub fn align(self, align: vec2<f32>) -> Self {
        Self { align, ..self }
    }

    pub fn color(self, color: Color) -> Self {
        Self { color, ..self }
    }
}

impl Default for TextRenderOptions {
    fn default() -> Self {
        Self {
            size: 1.0,
            align: vec2::splat(0.5),
            color: Color::WHITE,
            rotation: Angle::ZERO,
        }
    }
}

pub struct UtilRender {
    context: Context,
    pub unit_quad: ugli::VertexBuffer<draw2d::TexturedVertex>,
}

impl UtilRender {
    pub fn new(context: Context) -> Self {
        Self {
            unit_quad: geng_utils::geometry::unit_quad_geometry(context.geng.ugli()),
            context,
        }
    }

    pub fn draw_quad(
        &self,
        quad: Aabb2<f32>,
        color: Color,
        camera: &impl geng::AbstractCamera2d,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        self.context
            .geng
            .draw2d()
            .quad(framebuffer, camera, quad, color);
    }

    // pub fn draw_nine_slice_pp(
    //     &self,
    //     pos: Aabb2<f32>,
    //     color: Color,
    //     texture: &ugli::Texture,
    //     scale: f32,
    //     camera: &impl geng::AbstractCamera2d,
    //     framebuffer: &mut ugli::Framebuffer,
    // ) {
    //     let pos = geng_utils::pixel::pixel_perfect_aabb(
    //         pos.center(),
    //         vec2(0.5, 0.5),
    //         texture.size().map(|x| (x as f32 * scale).round() as usize),
    //         camera,
    //         framebuffer.size().as_f32(),
    //     );
    //     self.draw_nine_slice(pos, color, texture, scale, camera, framebuffer)
    // }

    pub fn draw_nine_slice(
        &self,
        pos: Aabb2<f32>,
        color: Color,
        texture: &ugli::Texture,
        scale: f32,
        camera: &impl geng::AbstractCamera2d,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        let whole = Aabb2::ZERO.extend_positive(vec2::splat(1.0));

        // TODO: configurable
        let mid = Aabb2 {
            min: vec2(0.3, 0.3),
            max: vec2(0.7, 0.7),
        };

        let size = mid.min * texture.size().as_f32() * scale;
        let size = vec2(size.x.min(pos.width()), size.y.min(pos.height()));

        let tl = Aabb2::from_corners(mid.top_left(), whole.top_left());
        let tm = Aabb2::from_corners(mid.top_left(), vec2(mid.max.x, whole.max.y));
        let tr = Aabb2::from_corners(mid.top_right(), whole.top_right());
        let rm = Aabb2::from_corners(mid.top_right(), vec2(whole.max.x, mid.min.y));
        let br = Aabb2::from_corners(mid.bottom_right(), whole.bottom_right());
        let bm = Aabb2::from_corners(mid.bottom_right(), vec2(mid.min.x, whole.min.y));
        let bl = Aabb2::from_corners(mid.bottom_left(), whole.bottom_left());
        let lm = Aabb2::from_corners(mid.bottom_left(), vec2(whole.min.x, mid.max.y));

        let slices: Vec<draw2d::TexturedVertex> = [tl, tm, tr, rm, br, bm, bl, lm, mid]
            .into_iter()
            .flat_map(|slice| {
                let [a, b, c, d] = slice.corners().map(|a_vt| {
                    let a_pos = vec2(
                        if a_vt.x == mid.min.x {
                            pos.min.x + size.x
                        } else if a_vt.x == mid.max.x {
                            pos.max.x - size.x
                        } else {
                            pos.min.x + pos.width() * a_vt.x
                        },
                        if a_vt.y == mid.min.y {
                            pos.min.y + size.y
                        } else if a_vt.y == mid.max.y {
                            pos.max.y - size.y
                        } else {
                            pos.min.y + pos.height() * a_vt.y
                        },
                    );
                    draw2d::TexturedVertex {
                        a_pos,
                        a_color: Color::WHITE,
                        a_vt,
                    }
                });
                [a, b, c, a, c, d]
            })
            .collect();
        let slices = ugli::VertexBuffer::new_dynamic(self.context.geng.ugli(), slices);

        ugli::draw(
            framebuffer,
            &self.context.assets.shaders.texture,
            ugli::DrawMode::Triangles,
            &slices,
            (
                ugli::uniforms! {
                    u_model_matrix: mat3::identity(),
                    u_color: color,
                    u_texture: texture,
                },
                camera.uniforms(framebuffer.size().as_f32()),
            ),
            ugli::DrawParameters {
                blend_mode: Some(ugli::BlendMode::straight_alpha()),
                ..default()
            },
        );

        // self.geng
        //     .draw2d()
        //     .textured_quad(framebuffer, camera, pos, texture, color);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw_nine_slice_splitcut(
        &self,
        pos: Aabb2<f32>,
        color: Color,
        cut_x: f32,
        texture_left: &ugli::Texture,
        texture_right: &ugli::Texture,
        scale: f32,
        camera: &impl geng::AbstractCamera2d,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        let whole = Aabb2::ZERO.extend_positive(vec2::splat(1.0));

        // TODO: configurable
        let mid = Aabb2 {
            min: vec2(0.3, 0.3),
            max: vec2(0.7, 0.7),
        };

        let size = mid.min * texture_left.size().as_f32() * scale;
        let size = vec2(size.x.min(pos.width()), size.y.min(pos.height()));

        let tl = Aabb2::from_corners(mid.top_left(), whole.top_left());
        let tm = Aabb2::from_corners(mid.top_left(), vec2(mid.max.x, whole.max.y));
        let tr = Aabb2::from_corners(mid.top_right(), whole.top_right());
        let rm = Aabb2::from_corners(mid.top_right(), vec2(whole.max.x, mid.min.y));
        let br = Aabb2::from_corners(mid.bottom_right(), whole.bottom_right());
        let bm = Aabb2::from_corners(mid.bottom_right(), vec2(mid.min.x, whole.min.y));
        let bl = Aabb2::from_corners(mid.bottom_left(), whole.bottom_left());
        let lm = Aabb2::from_corners(mid.bottom_left(), vec2(whole.min.x, mid.max.y));

        let slices: Vec<draw2d::TexturedVertex> = [tl, tm, tr, rm, br, bm, bl, lm, mid]
            .into_iter()
            .flat_map(|slice| {
                let [a, b, c, d] = slice.corners().map(|a_vt| {
                    let a_pos = vec2(
                        if a_vt.x == mid.min.x {
                            pos.min.x + size.x
                        } else if a_vt.x == mid.max.x {
                            pos.max.x - size.x
                        } else {
                            pos.min.x + pos.width() * a_vt.x
                        },
                        if a_vt.y == mid.min.y {
                            pos.min.y + size.y
                        } else if a_vt.y == mid.max.y {
                            pos.max.y - size.y
                        } else {
                            pos.min.y + pos.height() * a_vt.y
                        },
                    );
                    draw2d::TexturedVertex {
                        a_pos,
                        a_color: Color::WHITE,
                        a_vt,
                    }
                });
                [a, b, c, a, c, d]
            })
            .collect();
        let slices = ugli::VertexBuffer::new_dynamic(self.context.geng.ugli(), slices);

        ugli::draw(
            framebuffer,
            &self.context.assets.shaders.splitcut,
            ugli::DrawMode::Triangles,
            &slices,
            (
                ugli::uniforms! {
                    u_model_matrix: mat3::identity(),
                    u_color: color,
                    u_cut_x: cut_x,
                    u_texture_left: texture_left,
                    u_texture_right: texture_right,
                },
                camera.uniforms(framebuffer.size().as_f32()),
            ),
            ugli::DrawParameters {
                blend_mode: Some(ugli::BlendMode::straight_alpha()),
                ..default()
            },
        );

        // self.geng
        //     .draw2d()
        //     .textured_quad(framebuffer, camera, pos, texture, color);
    }

    // pub fn draw_text(
    //     &self,
    //     text: impl AsRef<str>,
    //     position: vec2<impl Float>,
    //     font: &Font,
    //     options: TextRenderOptions,
    //     camera: &impl geng::AbstractCamera2d,
    //     framebuffer: &mut ugli::Framebuffer,
    // ) {
    //     self.draw_text_with(
    //         text,
    //         position,
    //         0.0,
    //         font,
    //         options,
    //         ugli::DrawParameters {
    //             blend_mode: Some(ugli::BlendMode::straight_alpha()),
    //             ..default()
    //         },
    //         camera,
    //         framebuffer,
    //     )
    // }

    // #[allow(clippy::too_many_arguments)]
    // pub fn draw_text_with(
    //     &self,
    //     text: impl AsRef<str>,
    //     position: vec2<impl Float>,
    //     z_index: f32,
    //     font: &Font,
    //     mut options: TextRenderOptions,
    //     params: ugli::DrawParameters,
    //     camera: &impl geng::AbstractCamera2d,
    //     framebuffer: &mut ugli::Framebuffer,
    // ) {
    //     let text = text.as_ref();
    //     let framebuffer_size = framebuffer.size().as_f32();

    //     let position = position.map(Float::as_f32);
    //     let position = crate::util::world_to_screen(camera, framebuffer_size, position);

    //     let scale = crate::util::world_to_screen(
    //         camera,
    //         framebuffer_size,
    //         vec2::splat(std::f32::consts::FRAC_1_SQRT_2),
    //     ) - crate::util::world_to_screen(camera, framebuffer_size, vec2::ZERO);
    //     options.size *= scale.len();
    //     let font_size = options.size;

    //     let mut position = position;
    //     for line in text.lines() {
    //         let measure = font.measure(line, font_size);
    //         let size = measure.size();
    //         let align = size * (options.align - vec2::splat(0.5)); // Centered by default
    //         let descent = -font.descent() * font_size;
    //         let align = vec2(
    //             measure.center().x + align.x,
    //             descent + (measure.max.y - descent) * options.align.y,
    //         );

    //         let transform = mat3::translate(position)
    //             * mat3::rotate(options.rotation)
    //             * mat3::translate(-align);

    //         font.draw_with(
    //             framebuffer,
    //             line,
    //             z_index,
    //             font_size,
    //             options.color,
    //             transform,
    //             params.clone(),
    //         );
    //         position.y -= options.size; // NOTE: larger than text size to space out better
    //     }
    // }

    // pub fn draw_text_fit(
    //     &self,
    //     text: impl AsRef<str>,
    //     target: Aabb2<f32>,
    //     font: &Font,
    //     mut options: TextRenderOptions,
    //     camera: &impl geng::AbstractCamera2d,
    //     framebuffer: &mut ugli::Framebuffer,
    // ) {
    //     let text = text.as_ref();
    //     let measure = font.measure(text, 1.0);

    //     let max_size = target.size();
    //     let right = vec2(max_size.x, 0.0).rotate(options.rotation).x;
    //     let left = vec2(0.0, max_size.y).rotate(options.rotation).x;
    //     let width = if left.signum() != right.signum() {
    //         left.abs() + right.abs()
    //     } else {
    //         left.abs().max(right.abs())
    //     };

    //     let max_height = max_size.y * 0.9;
    //     let max_width = width * 0.85; // Leave some space TODO: move into a parameter or smth
    //     let max_size = max_width / measure.width();
    //     let size = options.size.min(max_size).min(max_height);

    //     options.size = size;

    //     self.draw_text_with(
    //         text,
    //         target.align_pos(options.align),
    //         0.0,
    //         font,
    //         options,
    //         ugli::DrawParameters {
    //             blend_mode: Some(ugli::BlendMode::straight_alpha()),
    //             ..default()
    //         },
    //         camera,
    //         framebuffer,
    //     );
    // }
}
