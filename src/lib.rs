use shura::*;

#[shura::main]
fn shura_main(config: ShuraConfig) {
    // Starting your game with a new scene
    config.init(NewScene::new(1, |ctx| {
        register!(ctx.components, [DummyComponent]);
        // Load the shared resources
        ctx.scene_states.insert(DummyResources {
            sprite: ctx.gpu.create_sprite(include_bytes!("../res/burger.png")),
        });
        // Set camera size
        ctx.world_camera.set_scaling(WorldCameraScale::Min(5.0));
        // Add your components to the default group
        ctx.components.add(DummyComponent::new(
            Vector::new(3.0, 0.0),
            Vector::new(0.5, 0.5),
            ctx,
        ));
        ctx.components.add(DummyComponent::new(
            Vector::new(-3.0, 1.0),
            Vector::new(1.0, 1.0),
            ctx,
        ));
    }))
}

#[derive(State)]
// Define a state that all components can access. In this case,
// this is used for comonent independent resources. A state is always
// a singleton.
struct DummyResources {
    // Sprite to render
    sprite: Sprite,
}

#[derive(Component)]
// Define a component
struct DummyComponent {
    // Model to render
    model: Model,
    // Base of a component. Used to base information like the position
    // of the component or the rigid body if it has one
    #[base]
    base: PositionComponent,
}

impl DummyComponent {
    pub fn new(translation: Vector<f32>, size: Vector<f32>, ctx: &Context) -> Self {
        DummyComponent {
            model: ctx.gpu.create_model(ModelBuilder::cuboid(size)),
            base: PositionComponent::new(PositionBuilder::default().translation(translation)),
        }
    }
}

impl ComponentController for DummyComponent {
    const CONFIG: ComponentConfig = ComponentConfig {
        // call `update` every frame
        update: UpdateOperation::EveryFrame,
        // Multiple of this component can be stored
        storage: ComponentStorage::Multiple,
        ..ComponentConfig::DEFAULT
    };
    
    fn update(ctx: &mut Context) {
        let frame_time = ctx.frame.frame_time();

        // Iterate over all active dummies
        ctx.components.for_each_mut::<Self>(|dummy| {
            // or ctx.component_manager.active_mut(active)
            let new_rot = dummy.base.rotation().angle() + frame_time * -1.0;
            dummy.base.set_rotation(Rotation::new(new_rot));
        });
    }

    fn render(ctx: &Context, encoder: &mut RenderEncoder) {
        // Render each component with it's own Model but use the shared sprite from the state
        let state = ctx.scene_states.get::<DummyResources>();
        ctx.components.render_each::<Self>(encoder, RenderConfig::WORLD, |renderer, dummy, instance| {
            renderer.render_sprite(instance, &dummy.model, &state.sprite)
        });
    }
}
