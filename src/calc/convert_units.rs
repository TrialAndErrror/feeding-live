const TBSP_MLS: u16 = 15;
const TSP_MLS: u16 = 5;

#[derive(Debug)]
pub struct Result {
    pub tablespoons: u16,
    pub teaspoons: u16,
    pub leftovers: f32,
    pub scoop_portions: f32,
}

pub fn convert_ml_to_tbsp(num_ml: u16) -> Result {
    let initial_powder_calculation = (f32::from(num_ml) * 50.0) / 120.0;
    println!("Total mLs of Formula: {value:.*}\n", 2, value=initial_powder_calculation);

    let remaining_powder = initial_powder_calculation as u16;
    let leftover_powder = initial_powder_calculation - f32::from(remaining_powder);

    let tablespoons_needed = remaining_powder / TBSP_MLS;
    let remaining_powder = remaining_powder - (TBSP_MLS * tablespoons_needed);

    let teaspoons_needed = remaining_powder / TSP_MLS;
    let remaining_powder = remaining_powder - (TSP_MLS * teaspoons_needed);

    let scoop_portions = initial_powder_calculation / 22.5;

    let leftover_powder = f32::from(remaining_powder) + leftover_powder;

    Result {
        tablespoons: tablespoons_needed,
        teaspoons: teaspoons_needed,
        leftovers: leftover_powder,
        scoop_portions,
    }
}
