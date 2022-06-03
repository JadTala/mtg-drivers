use std::error::Error;
use kiss3d::light::Light;
use kiss3d::text::Font;
use kiss3d::window::Window;
use mtg_drivers::glove;
use mtg_drivers::hand::Hand;
use nalgebra::{Point3, Point2};
use std::sync::MutexGuard;
use uuid::Uuid;

// Communications specific macros
const GLOVE_NAME: &str = "MTG";
const DATA_UUID: Uuid = Uuid::from_u128(0xdc931335_7019_4096_b1e7_42a29e570f8f);

// Debugger specific macros
const DEBUGGER_NAME: &str = "MTG Debugger";
const BACKGROUND_COLOR: [f32; 3] = [0.0, 0.0, 0.6];

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>
{
    let hand_original = glove::connect(GLOVE_NAME, DATA_UUID);

    // Create window
    let mut window = Window::new(DEBUGGER_NAME);
    window.set_background_color(BACKGROUND_COLOR[0], BACKGROUND_COLOR[1], BACKGROUND_COLOR[2]);
    window.set_light(Light::StickToCamera);

    let hand = hand_original.clone();
    // Render hand model
    while window.render() {
        draw_hand_model(&mut window, hand.lock().unwrap());
        draw_data(&mut window, hand.lock().unwrap());
    }

    Ok(())
}

fn draw_hand_model(window: &mut Window, hand: MutexGuard<Hand>)
{
    // Connect palm to fingers
    window.draw_line(&Point3::from(hand.get_palm_coords()[0]), &Point3::from(hand.get_thumb_coords()[0]), &Point3::new(1.0, 1.0, 1.0));
    window.draw_line(&Point3::from(hand.get_palm_coords()[0]), &Point3::from(hand.get_index_coords()[0]), &Point3::new(1.0, 1.0, 1.0));
    window.draw_line(&Point3::from(hand.get_palm_coords()[0]), &Point3::from(hand.get_middle_coords()[0]), &Point3::new(1.0, 1.0, 1.0));
    window.draw_line(&Point3::from(hand.get_palm_coords()[0]), &Point3::from(hand.get_ring_coords()[0]), &Point3::new(1.0, 1.0, 1.0));
    window.draw_line(&Point3::from(hand.get_palm_coords()[0]), &Point3::from(hand.get_little_coords()[0]), &Point3::new(1.0, 1.0, 1.0));

    // Connect thumb joints
    window.draw_line(&Point3::from(hand.get_thumb_coords()[0]), &Point3::from(hand.get_thumb_coords()[1]), &Point3::new(1.0, 1.0, 1.0));
    window.draw_line(&Point3::from(hand.get_thumb_coords()[1]), &Point3::from(hand.get_thumb_coords()[2]), &Point3::new(1.0, 1.0, 1.0));

    // Connect index joints
    window.draw_line(&Point3::from(hand.get_index_coords()[0]), &Point3::from(hand.get_index_coords()[1]), &Point3::new(1.0, 1.0, 1.0));
    window.draw_line(&Point3::from(hand.get_index_coords()[1]), &Point3::from(hand.get_index_coords()[2]), &Point3::new(1.0, 1.0, 1.0));
    window.draw_line(&Point3::from(hand.get_index_coords()[2]), &Point3::from(hand.get_index_coords()[3]), &Point3::new(1.0, 1.0, 1.0));

    // Connect middle finger joints
    window.draw_line(&Point3::from(hand.get_middle_coords()[0]), &Point3::from(hand.get_middle_coords()[1]), &Point3::new(1.0, 1.0, 1.0));
    window.draw_line(&Point3::from(hand.get_middle_coords()[1]), &Point3::from(hand.get_middle_coords()[2]), &Point3::new(1.0, 1.0, 1.0));
    window.draw_line(&Point3::from(hand.get_middle_coords()[2]), &Point3::from(hand.get_middle_coords()[3]), &Point3::new(1.0, 1.0, 1.0));

    // Connect ring finger joints
    window.draw_line(&Point3::from(hand.get_ring_coords()[0]), &Point3::from(hand.get_ring_coords()[1]), &Point3::new(1.0, 1.0, 1.0));
    window.draw_line(&Point3::from(hand.get_ring_coords()[1]), &Point3::from(hand.get_ring_coords()[2]), &Point3::new(1.0, 1.0, 1.0));
    window.draw_line(&Point3::from(hand.get_ring_coords()[2]), &Point3::from(hand.get_ring_coords()[3]), &Point3::new(1.0, 1.0, 1.0));

    // Connect little finger joints
    window.draw_line(&Point3::from(hand.get_little_coords()[0]), &Point3::from(hand.get_little_coords()[1]), &Point3::new(1.0, 1.0, 1.0));
    window.draw_line(&Point3::from(hand.get_little_coords()[1]), &Point3::from(hand.get_little_coords()[2]), &Point3::new(1.0, 1.0, 1.0));
    window.draw_line(&Point3::from(hand.get_little_coords()[2]), &Point3::from(hand.get_little_coords()[3]), &Point3::new(1.0, 1.0, 1.0));
}

fn draw_data(window: &mut Window, hand: MutexGuard<Hand>)
{
    let font = Font::default();

    window.draw_text(
        format!("Palm:\n{:#.2?}", hand.get_palm_coords()).as_str(),
        &Point2::new(0.0, 0.0),
        50.0,
        &font,
        &Point3::new(1.0, 1.0, 0.0),
    );

    window.draw_text(
        format!("Thumb:\n{:#.2?}", hand.get_thumb_coords()).as_str(),
        &Point2::new(180.0, 0.0),
        50.0,
        &font,
        &Point3::new(1.0, 1.0, 0.0),
    );

    window.draw_text(
        format!("Index:\n{:#.2?}", hand.get_index_coords()).as_str(),
        &Point2::new(360.0, 0.0),
        50.0,
        &font,
        &Point3::new(1.0, 1.0, 0.0),
    );

    window.draw_text(
        format!("Middle:\n{:#.2?}", hand.get_middle_coords()).as_str(),
        &Point2::new(540.0, 0.0),
        50.0,
        &font,
        &Point3::new(1.0, 1.0, 0.0),
    );

    window.draw_text(
        format!("Ring:\n{:#.2?}", hand.get_ring_coords()).as_str(),
        &Point2::new(720.0, 0.0),
        50.0,
        &font,
        &Point3::new(1.0, 1.0, 0.0),
    );

    window.draw_text(
        format!("Little:\n{:#.2?}", hand.get_little_coords()).as_str(),
        &Point2::new(900.0, 0.0),
        50.0,
        &font,
        &Point3::new(1.0, 1.0, 0.0),
    );

    window.draw_text(
        format!("Palm\nEuler:\n{:#.2?}", hand.get_euler(mtg_drivers::hand::HandPart::Palm)).as_str(),
        &Point2::new(0.0, 1200.0),
        50.0,
        &font,
        &Point3::new(1.0, 1.0, 0.0),
    );

    window.draw_text(
        format!("Thumb\nEuler:\n{:#.2?}", hand.get_euler(mtg_drivers::hand::HandPart::Thumb)).as_str(),
        &Point2::new(180.0, 1200.0),
        50.0,
        &font,
        &Point3::new(1.0, 1.0, 0.0),
    );

    window.draw_text(
        format!("Index\nEuler:\n{:#.2?}", hand.get_euler(mtg_drivers::hand::HandPart::Index)).as_str(),
        &Point2::new(360.0, 1200.0),
        50.0,
        &font,
        &Point3::new(1.0, 1.0, 0.0),
    );

    window.draw_text(
        format!("Middle\nEuler:\n{:#.2?}", hand.get_euler(mtg_drivers::hand::HandPart::Middle)).as_str(),
        &Point2::new(540.0, 1200.0),
        50.0,
        &font,
        &Point3::new(1.0, 1.0, 0.0),
    );

    window.draw_text(
        format!("Ring\nEuler:\n{:#.2?}", hand.get_euler(mtg_drivers::hand::HandPart::Ring)).as_str(),
        &Point2::new(720.0, 1200.0),
        50.0,
        &font,
        &Point3::new(1.0, 1.0, 0.0),
    );

    window.draw_text(
        format!("Little\nEuler:\n{:#.2?}", hand.get_euler(mtg_drivers::hand::HandPart::Little)).as_str(),
        &Point2::new(900.0, 1200.0),
        50.0,
        &font,
        &Point3::new(1.0, 1.0, 0.0),
    );

    window.draw_text(
        format!("Acceleration:\n{:#.2?}", hand.get_acceleration()).as_str(),
        &Point2::new(1080.0, 1200.0),
        50.0,
        &font,
        &Point3::new(1.0, 1.0, 0.0),
    );

    window.draw_text(
        format!("Bent fingers:\n{:#.2?}", hand.get_bent_fingers()).as_str(),
        &Point2::new(0.0, 1600.0),
        50.0,
        &font,
        &Point3::new(1.0, 1.0, 0.0),
    );

    window.draw_text(
        format!("Finger touching thumb:\n{:#.2?}", hand.get_finger_touching_thumb()).as_str(),
        &Point2::new(360.0, 1600.0),
        50.0,
        &font,
        &Point3::new(1.0, 1.0, 0.0),
    );
}