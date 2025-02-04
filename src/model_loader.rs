use std::fs::File;
use std::io::{ Read, Error};

use surrealml_core::storage::surml_file::SurMlFile;
use surrealml_core::storage::header::Header;
use surrealml_core::storage::header::normalisers::{
    wrapper::NormaliserType,
    linear_scaling::LinearScaling
};

pub fn load_model() -> Result<SurMlFile, Error> {
    // Load the model from the file
    let mut file = File::open("./models/R1_dist_fp16.onnx")?;
    let mut model_bytes = Vec::new();

    file.read_to_end(&mut model_bytes)?;
    //let mut surml_file = SurMlFile::from_bytes(&model_bytes).unwrap();

    // Create headers for the model
    let mut header = Header::fresh();
    header.add_column(String::from("scene_context"));
    header.add_column(String::from("instruction"));
    header.add_output(String::from("decision"), None);
    // Define header and add normalisers for the columns
    header.add_normaliser(
        "scene_context".to_string(),
        NormaliserType::LinearScaling(LinearScaling { min: 0.0, max: 1.0 })
    ).expect("Failed to add normaliser");
    header.add_normaliser(
        "instruction".to_string(),
        NormaliserType::LinearScaling(LinearScaling { min: 0.0, max: 1.0 })
    ).expect("Failed to add normaliser");

    // Package the Surml file and write it to disk
    let surml_file = SurMlFile::new(header, model_bytes);
    surml_file.write("./stash/test.surml").unwrap();

    // Load the model from the file
    let new_file = SurMlFile::from_file("./stash/test.surml").unwrap();
    Ok(new_file)
}
