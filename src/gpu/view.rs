use crate::core::objects::objects::Scene;
use crate::gpu::context::GPUContext;
use crate::gpu::renderer::{self, Renderer};
use std::sync::Arc;
use winit::{application::ApplicationHandler, event_loop::ActiveEventLoop, window::Window};

pub(crate) struct State<'a> {
    window: Arc<Window>,
    context: GPUContext<'a>,
    renderer: Renderer,
    scene: Scene,
}

impl<'a> State<'a> {
    #[allow(dead_code)]
    pub fn update(&mut self) {
        // We don't have anything to update yet
    }

    async fn new(window: Arc<Window>, scene: Scene) -> Self {
        // Adapter
        // let adapter = instance
        //     .request_adapter(&wgpu::RequestAdapterOptions::default())
        //     .await
        //     .unwrap();

        let context = GPUContext::new(window.clone()).await;
        context
            .surface
            .configure(&context.device, &context.surface_config);

        let renderer = renderer::Renderer::new(&context);

        Self {
            window,
            context,
            renderer,
            scene,
        }
    }

    pub fn render(&mut self) {
        let rend = self.renderer.render(&mut self.context, &self.scene);
        match rend {
            Err(err) => eprintln!("Render error: {}", err),
            Ok(_) => {}
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.context.surface_config.width = width;
            self.context.surface_config.height = height;
            self.context
                .surface
                .configure(&self.context.device, &self.context.surface_config);
        }
    }
}

#[derive(Default)]
pub(crate) struct App<'a> {
    state: Option<State<'a>>,
    scene: Option<Scene>,
}

impl<'a> App<'a> {
    pub fn new(scene: Scene) -> Self {
        Self {
            state: None,
            scene: Some(scene),
        }
    }
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes = Window::default_attributes().with_title("Animotion");
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        let scene = self.scene.take().unwrap();

        self.state = Some(pollster::block_on(async {
            // On passe la scène récupérée à l'état WGPU
            State::new(window.into(), scene).await
        }));
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let state = match &mut self.state {
            Some(state) => state,
            None => return,
        };

        match event {
            winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            winit::event::WindowEvent::Resized(size) => {
                state.resize(size.width, size.height);
            }
            winit::event::WindowEvent::RedrawRequested => {
                let _ = state.render();
                // Emits a new redraw requested event.
                state.window.request_redraw();
                state.update();
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(state) = &self.state {
            state.window.request_redraw();
        }
    }
}
