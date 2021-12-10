use webcam::video::face_detection;

fn main() {
  let path = "haarcascades/haarcascade_frontalface_alt.xml";
  match face_detection(path) {
    Ok(_) => println!("Successfully exit window"),
    Err(e) => panic!("ERR: {}", e),
  }
}
