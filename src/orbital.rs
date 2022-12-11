use glam::{vec3, Quat, Vec3};
use mint::Vector3;
use stardust_xr_molecules::{
    fusion::{
        client::{Client, LifeCycleHandler, LogicStepInfo},
        drawable::Model,
        fields::BoxField,
        input::{
            action::{BaseInputAction, InputAction, InputActionHandler},
            InputData, InputDataType, InputHandler,
        },
        resource::NamespacedResource,
        spatial::Spatial,
        HandlerWrapper,
    },
    SingleActorAction,
};

use lazy_static::lazy_static;

lazy_static! {
    static ref TESTSPHERE: NamespacedResource = NamespacedResource::new("orbital", "testsphere");
    static ref GRABCUBE: NamespacedResource = NamespacedResource::new("orbital", "grabcube");
}

pub struct Orbital {
    root: Spatial,
    window_field: BoxField,
    test_sphere: Model,
    grab_cube: Model,
    input_handler: HandlerWrapper<InputHandler, InputActionHandler<()>>,
    hover_input_action: BaseInputAction<()>,
    grab_input_action: SingleActorAction<()>,
}

impl Orbital {
    pub fn new(client: &Client) -> Self {
        println!("{:?}", &*GRABCUBE);
        let root = Spatial::builder()
            .spatial_parent(client.get_root())
            .zoneable(false)
            .build()
            .unwrap();
        let window_field = BoxField::builder()
            .spatial_parent(&root)
            .size(Vector3::from([0.1; 3]))
            .position(Vector3::from([0.0, 0.0, 0.0]))
            .build()
            .unwrap();
        let test_sphere = Model::builder()
            .spatial_parent(&root)
            .resource(&*TESTSPHERE)
            .build()
            .unwrap();
        let grab_cube = Model::builder()
            .spatial_parent(&window_field)
            .resource(&*GRABCUBE)
            .position(Vector3::from([0.0, 0.0, 0.0]))
            .build()
            .unwrap();
        let input_handler = InputHandler::create(&client.get_root(), None, None, &window_field)
            .unwrap()
            .wrap(InputActionHandler::new(()))
            .unwrap();
        let hover_input_action =
            BaseInputAction::new(false, |input_data, _| input_data.distance.abs() < 0.05);
        let grab_input_action = SingleActorAction::new(true, Orbital::grab_action, true);

        Orbital {
            root,
            window_field,
            test_sphere,
            grab_cube,
            input_handler,
            hover_input_action,
            grab_input_action,
        }
    }
    fn grab_action(input_data: &InputData, _: &()) -> bool {
        input_data
            .datamap
            .with_data(|data| match &input_data.input {
                InputDataType::Hand(_) => data.idx("grab_strength").as_f32() > 0.8,
                _ => data.idx("grab").as_f32() > 0.9,
            })
    }
}

impl LifeCycleHandler for Orbital {
    fn logic_step(&mut self, _info: LogicStepInfo) {
        self.input_handler.lock_wrapped().update_actions([
            self.hover_input_action.type_erase(),
            self.grab_input_action.type_erase(),
        ]);
        self.grab_input_action.update(&mut self.hover_input_action);
        if self.grab_input_action.actor_acting() {
            let pos_vector = position(self.grab_input_action.actor().unwrap());
            let look_at = Quat::from_rotation_arc(vec3(0.0, 1.0, -1.0), pos_vector);

            self.window_field
                .set_rotation(Some(&self.root), look_at)
                .unwrap();
            self.window_field
                .set_position(Some(&self.root), pos_vector)
                .unwrap();
            self.test_sphere
                .set_rotation(Some(&self.root), look_at)
                .unwrap();
            self.test_sphere
                .set_position(Some(&self.root), pos_vector)
                .unwrap();
            self.grab_cube
                .set_position(Some(&self.root), pos_vector)
                .unwrap();
            self.grab_cube
                .set_rotation(Some(&self.root), look_at)
                .unwrap();
        }
    }
}

fn position(data: &InputData) -> Vec3 {
    match &data.input {
        InputDataType::Pointer(p) => p.deepest_point.into(),
        InputDataType::Hand(h) => h.palm.position.into(),
        InputDataType::Tip(t) => t.origin.into(),
    }
}
