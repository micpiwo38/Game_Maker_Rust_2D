use std::collections::HashMap;
use egui::TextureHandle;
use wgpu::{Backends, Device, DeviceDescriptor, Features, Instance, InstanceDescriptor, Limits, PowerPreference, Queue, RequestAdapterOptions, Surface, SurfaceConfiguration, TextureUsages};
use winit::window::Window;
use crate::views::asset_browser::AssetEntry;
use crate::views::sprite_editor::SpriteEditorState;

pub struct GameState<'a> {
    pub surface: Surface<'a>,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub(crate) window: &'a Window,
    //Champ egui
    pub egui_ctx: egui::Context,
    pub egui_state: egui_winit::State,
    pub egui_renderer: egui_wgpu::Renderer,

    //Le panneau Hierarchy
    pub selected_object: Option<String>,
    pub scene_object: Vec<String>,
    pub current_asset_path: String,
    pub assets: Vec<AssetEntry>,
    pub last_path: String,
    pub asset_texture_cache: HashMap<String, TextureHandle>,
    //Selectionné un asset (image, son, GameObject, etc...)
    pub selected_asset: Option<crate::views::asset_browser::AssetEntry>,
    pub sprite_editor_state: SpriteEditorState
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
        //Init d'egui
        let egui_ctx = egui::Context::default();
        let egui_state = egui_winit::State::new(
            egui_ctx.clone(),
            egui::ViewportId::ROOT,
            window,
            None,
            None
        );
        let egui_renderer = egui_wgpu::Renderer::new(&device, surface_format, None, 1);
        let current_path = "assets".to_string();
        let initial_assets = crate::views::asset_browser::load_directory(&current_path);

        Self{
            window, surface, device, queue, config, size, egui_ctx, egui_state, egui_renderer, selected_object: None,
            scene_object: vec![
                "MainCamera".to_string(),
                "Player".to_string(),
                "Enemy_One".to_string(),
                "TileMap".to_string(),
            ],
            current_asset_path: current_path,
            assets: initial_assets,
            last_path: "assets".to_string(),
            asset_texture_cache: HashMap::new(),
            selected_asset: None,
            sprite_editor_state: Default::default()
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
    //Gestion des evenements
    pub fn handle_event(&mut self, event: &winit::event::WindowEvent) -> bool {
        let response = self.egui_state.on_window_event(self.window, event);
        response.consumed
    }

    //MAJ
    pub fn update(&mut self) {}

    //Le rendu
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        //1. Demarrage de la frame egui et appel du module menu
        let raw_input = self.egui_state.take_egui_input(self.window);
        //-------------------------------------------------------------------------------//
        let full_output = self.egui_ctx.run(raw_input, |ctx| {
            //1. Le menu
            crate::views::menu::render(ctx);
            //2. Le panneau Hierarchy
            let object_refs: Vec<&str> = self.scene_object.iter().map(|s| s.as_str()).collect();
            crate::views::hierarchy::render(ctx, &object_refs, &mut self.selected_object);
            //3. Le panneau Inspector a droite
            crate::views::inspector::render(ctx, &mut self.selected_object, &mut self.selected_asset, &mut self.sprite_editor_state);
            //4. Explorateur de fichier
            // Si le chemin a changé (ex: on a cliqué sur un dossier ou sur "Back"), on recharge les assets !
            if self.current_asset_path != self.last_path {
                self.assets = crate::views::asset_browser::load_directory(&self.current_asset_path);
                self.last_path = self.current_asset_path.clone();
            }

            crate::views::asset_browser::render(
                ctx,
                &mut self.current_asset_path,
                &mut self.assets,
                &mut self.asset_texture_cache,
                &mut self.selected_asset,
                &mut self.selected_object
            );

            //5. Ouvrir l'editeur de Sprite
            let active_texture = self.selected_asset.as_ref().and_then(|asset|{
                self.asset_texture_cache.get(&asset.path)
            });
            //6. Ouvrir l'editeur de sprite et vérifier si on vient de valider une découpe
            let slice_applied = crate::views::sprite_editor::render(
                ctx, &mut self.sprite_editor_state, active_texture, &mut self.selected_asset
            );

            if slice_applied {
                // On recharge les assets du dossier pour prendre en compte le nouveau .meta et afficher les sous-sprites
                self.assets = crate::views::asset_browser::load_directory(&self.current_asset_path);
            }
            //7. Viewport = Zone centrale
            crate::views::viewport::render(ctx);
        });
        //-----------------------------------------------------------------------------------//
        self.egui_state.handle_platform_output(self.window, full_output.platform_output);
        let tris = self.egui_ctx.tessellate(full_output.shapes, self.window.scale_factor() as f32);
        let screen_descriptor = egui_wgpu::ScreenDescriptor{
            size_in_pixels: [self.config.width, self.config.height],
            pixels_per_point: self.window.scale_factor() as f32,
        };
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor{
                label: Some("Render Encoder"),
            });
        //MAJ des textures + buffers egui
        for(id, image_delta) in &full_output.textures_delta.set{
            self.egui_renderer.update_texture(&self.device, &self.queue, *id, image_delta);
        }

        self.egui_renderer.update_buffers(
            &self.device,
            &self.queue,
            &mut encoder,
            &tris,
            &screen_descriptor,
        );
        {
            //Efface l'ecran avec une couleur unie
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor{
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
            //Rendu de l'interface egui par dessus -> wgpu
            self.egui_renderer.render(&mut render_pass, &tris, &screen_descriptor);
        }
        //Nettoyage des textures egui et liberation memoire
        for id in &full_output.textures_delta.free{
            self.egui_renderer.free_texture(id);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}