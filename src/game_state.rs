use wgpu::{Backend, Backends, Device, DeviceDescriptor, Features, Instance, InstanceDescriptor, Limits, PowerPreference, Queue, RequestAdapterOptions, Surface, SurfaceConfiguration, TextureUsages};
use winit::window::Window;

pub struct GameState<'a> {
    pub surface: Surface<'a>,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub(crate) window: &'a Window
}

impl<'a> GameState<'a> {
    //Creation de l'atat de rendu wgpu et lien avec la fenetre winit via un constructeur
    pub async fn new(window: &'a Window) -> Self {
        //Taille de la fenetre
        let size = window.inner_size();
        //Instance de Wgpu
        let instance = Instance::new(InstanceDescriptor{
            backends: Backends::PRIMARY,
            ..Default::default()
        });
        //Comme SLD la surface est le contenu de la fenetre
        let surface = instance.create_surface(window).unwrap();
        //Adaptateur graphique => GPU Nvidia Physique
        let adapter = instance
            .request_adapter(&RequestAdapterOptions{
                power_preference: PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        //Cible le PC
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor{
                    required_features: Features::empty(),
                    required_limits: Limits::default(),
                    label: None
                },
                None,
            )
            .await
            .unwrap();

        //Configuration de la surface => zone d'affichage dans la fenetre
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = SurfaceConfiguration{
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            desired_maximum_frame_latency: 2,
            view_formats: vec![surface_format.into()],
        };
        surface.configure(&device, &config);

        Self{
            window, surface, device, queue, config, size
        }
    }
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }
    //MAJ
    pub fn update(&mut self) {}

    //Le rendu
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor{
                label: Some("Render Encoder"),
            });
        {
            //Efface l'ecran avec une couleur unie
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor{
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment{
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations{
                        load: wgpu::LoadOp::Clear(wgpu::Color{
                            r: 0.5,
                            g: 0.12,
                            b: 0.2,
                            a: 1.0
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}