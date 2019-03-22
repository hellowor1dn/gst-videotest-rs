use gstreamer as gst;
use gst::prelude::*;

use std::error::Error as StdError;

use failure::Error;

use test_lib::*;

fn run() -> Result<(), Error> {
    // init gstreamer
    gst::init()?;

    // init loop
    let main_loop = glib::MainLoop::new(None, false);

    // region set up pipeline and add elements
    let pipeline = gst::Pipeline::new(None);
    let src = make_element("videotestsrc", None)?;
    let f1 = make_element("capsfilter", None)?;
    let q1 = make_element("queue", None)?;
    let enc = make_element("x264enc", None)?;
    let q2 = make_element("queue", None)?;
    let dec = make_element("avdec_h264", None)?;
    let q3 = make_element("queue", None)?;
    let conv = make_element("videoconvert", None)?;
    let scale = make_element("videoscale", None)?;
    let rate = make_element("videorate", None)?;
    let f2 = make_element("capsfilter", None)?;
    let sink = make_element("autovideosink", None)?;

    // add elements
    pipeline.add_many(&[
        &src,
        &f1,
        &q1,
        &enc,
        &q2,
        &dec,
        &q3,
        &conv,
        &scale,
        &rate,
        &f2,
        &sink
    ])?;

    // link elements
    gst::Element::link_many(&[
        &src,
        &f1,
        &q1,
        &enc,
        &q2,
        &dec,
        &q3,
        &conv,
        &scale,
        &rate,
        &f2,
        &sink
    ])?;

    // set source pattern
    src.set_property("is-live", &true)?;
    src.set_property_from_str("pattern", "ball");

    // set source caps
    let framerate = gst::Fraction::new(60, 1);
    let video_caps = gst::Caps::builder("video/x-raw")
        .field("width", &1920i32)
        .field("height", &1080i32)
        .field("framerate", &framerate)
        .build();
    f1.set_property("caps", &video_caps)?;

    // encoder settings
    enc.set_property("key-int-max", &10u32.to_value())?;

    // set sink caps
    let framerate = gst::Fraction::new(15, 1);
    let video_caps = gst::Caps::builder("video/x-raw")
        .field("width", &1920i32)
        .field("height", &1080i32)
        .field("framerate", &framerate)
        .build();
    f2.set_property("caps", &video_caps)?;
    // endregion

    // region add message handler
    let bus: gst::Bus = pipeline.get_bus()
        .expect("Pipeline doesn't have a bus (shouldn't happen)!");
    let loop_clone = main_loop.clone();
    bus.add_watch(move |_, msg| {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => {
                println!("End of stream.");
                loop_clone.quit();
            }
            MessageView::Error(err) => {
                let error_msg = ErrorMessage {
                    src: msg
                        .get_src()
                        .map(|s| String::from(s.get_path_string()))
                        .unwrap_or_else(|| String::from("None")),
                    error: err.get_error().description().into(),
                    debug: Some(err.get_debug().unwrap().to_string()),
                    cause: err.get_error(),
                };

                eprintln!("Error: {}", error_msg);
                loop_clone.quit();
            }
            MessageView::Warning(w) => {
                let error_msg = ErrorMessage {
                    src: msg
                        .get_src()
                        .map(|s| String::from(s.get_path_string()))
                        .unwrap_or_else(|| String::from("None")),
                    error: w.get_error().description().into(),
                    debug: Some(w.get_debug().unwrap().to_string()),
                    cause: w.get_error(),
                };

                eprintln!("Warning: {}", error_msg);
            }
            MessageView::Qos(q) => {
//                let (live, running_time, stream_time, timestamp, duration) = q.get();
                let (processed, dropped) = q.get_stats();
                println!("QoS warning: p={:?} d={:?}", processed, dropped);
            }
//            MessageView::StateChanged(s) => {
//                println!(
//                    "State changed from {:?}: {:?} -> {:?} ({:?})",
//                    s.get_src().map(|s| s.get_path_string()),
//                    s.get_old(),
//                    s.get_current(),
//                    s.get_pending()
//                );
//            }
            MessageView::StreamStart(_) => {
                println!("Stream started!");
            }
            _ => (),
        }

        glib::Continue(true)
    }).ok_or(WatchError)?;
    // endregion

    // start playing
    println!("Now playing");
    pipeline.set_state(gst::State::Playing)?;

    // main loop
    println!("Running...");
    main_loop.run();

    // clean up
    println!("Stopping...");
    pipeline.set_state(gst::State::Null)?;
    bus.remove_watch()?;

    Ok(())
}

fn main() {
    match run() {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {}", e)
    }
}
