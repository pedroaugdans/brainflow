use std::{thread, time::Duration};

use brainflow::{
    board_shim, brainflow_input_params::BrainFlowInputParamsBuilder, data_filter, BoardIds,
    WindowFunctions,
};

fn main() {
    brainflow::board_shim::enable_dev_board_logger().unwrap();

    let board_id = BoardIds::SyntheticBoard as i32;
    let eeg_channels = board_shim::get_eeg_channels(board_id).unwrap();
    println!("{:?}", eeg_channels);

    let params = BrainFlowInputParamsBuilder::default().build();
    let board_id = BoardIds::SyntheticBoard as i32;
    let board = board_shim::BoardShim::new(board_id, params).unwrap();

    board.prepare_session().unwrap();
    board.start_stream(45000, "").unwrap();
    thread::sleep(Duration::from_secs(5));
    board.stop_stream().unwrap();
    let mut data = board.get_board_data(Some(64)).unwrap();
    board.release_session().unwrap();

    let data_len = data[0].len();

    let fft_data = data_filter::perform_fft(
        &mut data[eeg_channels[0]],
        WindowFunctions::BlackmanHarris as i32,
    )
    .unwrap();
    let restored_fft = data_filter::perform_ifft(&fft_data, data_len).unwrap();
    println!("{:?}", restored_fft);

    println!("{:?}", data[eeg_channels[1]]);
    let wavelet_data =
        data_filter::perform_wavelet_transform(&mut data[eeg_channels[1]], "db3", 3).unwrap();
    let restored_wavelet = data_filter::perform_inverse_wavelet_transform(wavelet_data).unwrap();
    println!("{:?}", restored_wavelet);
}
