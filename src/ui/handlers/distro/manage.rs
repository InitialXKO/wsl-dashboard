// Updated the ModelRc conversion to use Rc instead of Arc
// This change affects the way the model is initialized in the app

// Line 256
slint::ModelRc::from(std::rc::Rc::new(slint::VecModel::from(containers)))