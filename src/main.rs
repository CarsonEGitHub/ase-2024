use std::{fs::File, io::Write};
use hound;

fn show_info() {
    eprintln!("MUSI-6106 Assignment Executable");
    eprintln!("(c) 2024 Stephen Garrett & Ian Clester");
}

fn main() {
    show_info();

    // Parse command line arguments
    // First argument is input .wav file, second argument is output text file.
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input_wav_file> <output_txt_file>", 
args[0]);
        std::process::exit(1);
    }

    // Open the input wave file and determine number of channels
    let input_file = &args[1];
    let mut reader = hound::WavReader::open(input_file)
        .expect("Failed to open input WAV file");

    let num_channels = reader.spec().channels;

    // Determine the maximum value for the bit depth
    let max_value = f32::powi(2.0, reader.spec().bits_per_sample as i32 - 
1) as f32;

    // Read audio data and write it to the output text file (two columns, 44100*3 rows)
    let output_file_name = &args[2];
    let mut output_file = File::create(output_file_name)
        .expect("Failed to create output text file");

    for _ in 0..44100 * 3 {
        for channel in 0..num_channels {
            let sample_value = reader.samples::<i16>().next()
                .expect("Failed to read sample")
                .expect("No more samples available") as f32;

            // Normalize the sample value to be between -1 and 1
            let normalized_value = sample_value / max_value;

            // TODO: Process normalized sample as needed

            // Write the normalized sample value to the output text file
            write!(output_file, "{} ", normalized_value)
                .expect("Failed to write to output text file");
        }
        writeln!(output_file).expect("Failed to write to output text 
file");
    }
}
