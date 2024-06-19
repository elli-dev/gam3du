use camera::Camera;
use floor::Floor;
use projection::Projection;
use robot::Robot;
use std::time::Instant;

mod camera;
mod floor;
mod projection;
mod robot;
pub(crate) use robot::Command;

pub(crate) struct Scene {
    depth_map: DepthTexture,
    start_time: Instant,
    projection: Projection,
    camera: Camera,
    robot: Robot,
    floor: Floor,
}

fn elapsed_as_vec(start_time: Instant) -> [u32; 2] {
    let elapsed = start_time.elapsed();
    let seconds = u32::try_from(elapsed.as_secs()).unwrap();
    let subsec_nanos = u64::from(elapsed.subsec_nanos());
    // map range of nanoseconds to value range of u32 with rounding
    let subseconds = ((subsec_nanos << u32::BITS) + 500_000_000) / 1_000_000_000;

    [seconds, u32::try_from(subseconds).unwrap()]
}

impl Scene {
    pub(crate) fn init(
        surface: &wgpu::SurfaceConfiguration,
        _adapter: &wgpu::Adapter,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Self {
        let cube = Robot::new(device, queue, surface.view_formats[0]);
        let floor = Floor::new(device, queue, surface.view_formats[0]);

        let projection = Projection::new_perspective(
            (surface.width, surface.height),
            45_f32.to_radians(),
            1.0..10.0,
        );

        let camera = Camera::new(glam::Vec3::new(1.5f32, -5.0, 3.0), glam::Vec3::ZERO);

        let start_time = Instant::now();

        let depth_map = DepthTexture::create_depth_texture(device, surface, "depth_map");

        Scene {
            depth_map,
            start_time,
            projection,
            camera,
            robot: cube,
            floor,
        }
    }

    pub(crate) fn resize(
        &mut self,
        surface: &wgpu::SurfaceConfiguration,
        device: &wgpu::Device,
        _queue: &wgpu::Queue,
    ) {
        self.projection
            .set_surface_dimensions((surface.width, surface.height));
        self.depth_map = DepthTexture::create_depth_texture(device, surface, "depth_map");
    }

    pub(crate) fn render(
        &mut self,
        texture_view: &wgpu::TextureView,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        self.render_cube(texture_view, &mut encoder, queue);

        self.render_floor(texture_view, &mut encoder, queue);

        queue.submit(Some(encoder.finish()));
    }

    fn render_cube(
        &mut self,
        texture_view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        queue: &wgpu::Queue,
    ) {
        let clear_color = wgpu::Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        };
        let render_pass_color_attachment = wgpu::RenderPassColorAttachment {
            view: texture_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(clear_color),
                store: wgpu::StoreOp::Store,
            },
        };
        let color_attachments = [Some(render_pass_color_attachment)];
        let render_pass_depth_stencil_attachment = wgpu::RenderPassDepthStencilAttachment {
            view: &self.depth_map.view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        };
        let render_pass_descriptor = wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &color_attachments,
            depth_stencil_attachment: Some(render_pass_depth_stencil_attachment.clone()),
            timestamp_writes: None,
            occlusion_query_set: None,
        };
        {
            let mut render_pass = encoder.begin_render_pass(&render_pass_descriptor);

            self.robot.render(
                queue,
                &mut render_pass,
                &self.camera,
                &self.projection,
                self.start_time,
            );
        }
    }

    fn render_floor(
        &mut self,
        texture_view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        queue: &wgpu::Queue,
    ) {
        let render_pass_color_attachment = wgpu::RenderPassColorAttachment {
            view: texture_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: wgpu::StoreOp::Store,
            },
        };
        let color_attachments = [Some(render_pass_color_attachment)];

        let render_pass_depth_stencil_attachment = wgpu::RenderPassDepthStencilAttachment {
            view: &self.depth_map.view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        };

        let render_pass_descriptor = wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &color_attachments,
            depth_stencil_attachment: Some(render_pass_depth_stencil_attachment),
            timestamp_writes: None,
            occlusion_query_set: None,
        };
        {
            let mut render_pass = encoder.begin_render_pass(&render_pass_descriptor);

            self.floor.render(
                queue,
                &mut render_pass,
                &self.camera,
                &self.projection,
                self.start_time,
            );
        }
    }

    pub(crate) fn is_idle(&mut self) -> bool {
        self.robot.is_idle()
    }

    pub(crate) fn process_command(&mut self, command: &Command) {
        self.robot.process_command(command);
    }
}

struct DepthTexture {
    _texture: wgpu::Texture,
    view: wgpu::TextureView,
    _sampler: wgpu::Sampler,
}

impl DepthTexture {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float; // 1.

    pub fn create_depth_texture(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        label: &str,
    ) -> Self {
        let size = wgpu::Extent3d {
            // 2.
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT // 3.
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };
        let texture = device.create_texture(&desc);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            // 4.
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual), // 5.
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        });

        Self {
            _texture: texture,
            view,
            _sampler: sampler,
        }
    }

    fn depth_stencil_state() -> wgpu::DepthStencilState {
        wgpu::DepthStencilState {
            format: DepthTexture::DEPTH_FORMAT,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less, // 1.
            stencil: wgpu::StencilState::default(),     // 2.
            bias: wgpu::DepthBiasState::default(),
        }
    }
}
