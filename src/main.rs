use webcam::cuda::process_gpu;
// use webcam::video::face_detection;

fn main() {
  //   File: webcam::video::face_detection
  //   let path = "data/haarcascades/haarcascade_frontalface_alt.xml";
  //   match face_detection(path) {
  //     Ok(_) => println!("Successfully exit window"),
  //     Err(e) => panic!("ERR: {}", e),
  //   }

  // File: webcam::cuda::process_gpu
  let path = "data/images/inspiration4/Hayley Arceneaux.JPG";
  match process_gpu(path) {
    Ok(_) => println!("Successful!"),
    Err(e) => panic!("ERR: {}", e),
  }
}
