use shura::*;

#[shura::main]
fn shura_main(config: ShuraConfig) {
    // Starting your game with a new scene
    config.init(NewScene::new(1, |ctx| {
        // Load the shared resources
        ctx.insert_scene_state(DummyResources {
            sprite: ctx.create_sprite(include_bytes!("../res/burger.png")),
        });
        // Set camera size
        ctx.set_camera_scale(WorldCameraScale::Min(5.0));
        // Add your components to the default group
        ctx.add_component(DummyComponent::new(Vector::new(3.0, 0.0), Vector::new(0.5, 0.5),  ctx));
        ctx.add_component(DummyComponent::new(Vector::new(-3.0, 1.0), Vector::new(1.0, 1.0),  ctx));
    }))
}

#[derive(State)]
// Define a state that all components can access. In this case,
// this is used for component independent resources. A state is always
// a singleton.
struct DummyResources {
    // Sprite to render
    sprite: Sprite,
}
impl SceneStateController for DummyResources {}

#[derive(Component)]
// Define a component
struct DummyComponent {
    // Model to render
    model: Model,
    // Base of a component. Used to base information like the position
    // of the component or the rigid body if it has one
    #[base]
    base: BaseComponent,
}

impl DummyComponent {
    pub fn new(translation: Vector<f32>, size: Vector<f32>, ctx: &Context) -> Self {
        DummyComponent {
            model: ctx.create_model(ModelBuilder::cuboid(size)),
            base: BaseComponent::new(PositionBuilder::default().translation(translation)),
        }
    }
}

impl ComponentController for DummyComponent {
    const CONFIG: ComponentConfig = ComponentConfig {
        // call `update` every frame
        update: UpdateOperation::EveryFrame,
        ..DEFAULT_CONFIG
    };
    fn update(active: &ActiveComponents<Self>, ctx: &mut Context) {
        let frame_time = ctx.frame_time(); // or ctx.frame_manager.frame_time()

        // Iterate over all active dummies
        for dummy in ctx.active_mut(active) {
            // or ctx.component_manager.active_mut(active)
            let new_rot = dummy.rotation().angle() + frame_time * -1.0;
            dummy.set_rotation(Rotation::new(new_rot));
        }
    }
    fn render(active: &ActiveComponents<Self>, ctx: &Context, encoder: &mut RenderEncoder) {
        // Render each component with it's own Model but use the shared sprite from the state
        let state = ctx.scene_state::<DummyResources>();
        ctx.render_each(
            active,
            encoder,
            RenderConfig::WORLD,
            |renderer, dummy, instance| {
                renderer.render_sprite(instance, &dummy.model, &state.sprite)
            },
        )
    }
}
