use wgpu::{Backend, Backends, Device, DeviceDescriptor, Features, Instance, InstanceDescriptor, Limits, PowerPreference, Queue, RequestAdapterOptions, Surface, SurfaceConfiguration, TextureUsages};
use winit::window::Window;

pub struct GameState<'a> {
    surface: Surface<'a>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: &'a Window
}

impl<'a> GameState<'a> {
    //Creation de l'atat de rendu wgpu et lien avec la fenetre winit via un constructeur
    pub fn new(window: &'a Window) -> Self {
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
                }
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
            desired_maximum_frame_latency: 2
        };
        surface.configure(&device, &config);

        Self{
            window, surface, device, queue, config, size
        }


    }
}