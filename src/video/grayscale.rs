use opencv::{highgui, imgproc, prelude::*, videoio, Result};

pub fn grayscale() -> Result<()> {
  let window = "Grayscale";
  highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;

  #[cfg(ocvrs_opencv_branch_32)]
  let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
  #[cfg(not(ocvrs_opencv_branch_32))]
  let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;

  let opened = videoio::VideoCapture::is_opened(&cam)?;
  if !opened {
    panic!("Unable to open default camera");
  }

  loop {
    let mut frame = Mat::default();
    cam.read(&mut frame)?;

    // If we read a frame.
    if frame.size()?.width > 0 {
      let mut gray = Mat::default();
      imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
      highgui::imshow(window, &gray)?;
    }

    // Press any key to quit...
    if highgui::wait_key(10)? > 0 {
      break;
    }
  }

  Ok(())
}
