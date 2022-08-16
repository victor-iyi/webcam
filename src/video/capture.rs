use opencv::{highgui, prelude::*, videoio, Result};

pub fn video_capture() -> Result<()> {
    let window = "Video capture";
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;

    #[cfg(ocvrs_opencv_branch_32)]
    let mut cam = videio::VideoCapture::new_default(0)?;
    #[cfg(not(ocvrs_opecv_branch_32))]
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;

    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open default camera!");
    }

    // Video loop.
    loop {
        let mut frame = Mat::default();
        cam.read(&mut frame)?;
        if frame.size()?.width > 0 {
            highgui::imshow(window, &mut frame)?;
        }

        let key = highgui::wait_key(10)?;
        if key > 0 && key != 255 {
            break;
        }
    }

    Ok(())
}
