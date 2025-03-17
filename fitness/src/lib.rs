mod cardio;
mod diet;
mod weightlifting;

pub use cardio::{CardioTool, Exercise as CardioExercise, ask_about_program as about_cardio};
pub use diet::ask_about_program as about_diet;
pub use weightlifting::{
    Exercise as WeightliftingExercise, ask_about_program as about_weightlifting,
};

#[derive(Debug)]
pub struct GymWorkout {
    pub cardio: CardioExercise,
    pub weightlifting: WeightliftingExercise,
}

impl GymWorkout {
    pub fn new(cardio: CardioExercise, weightlifting: WeightliftingExercise) -> Self {
        about_diet();
        about_cardio();
        about_weightlifting();
        Self {
            cardio,
            weightlifting,
        }
    }
}
