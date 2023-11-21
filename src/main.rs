use clap::Parser;
use is_empty::IsEmpty;
use std::process;
use text_io::read;

// #[derive(Debug, Parser, IsEmpty)]
// struct Options {
//     ///Macros and body compositions
//     #[clap(short = 'd', long = "default")]
//     default: Option<bool>,
//     /// Macros
//     #[clap(short = 'm', long = "macros")]
//     macros: Option<bool>,
//     /// Body composition
//     #[clap(short = 'b', long = "bodycomp")]
//     bodycomp: Option<bool>,
// }

#[derive(Debug, IsEmpty)]
struct Macros {
    calories: Option<f32>,
    protein: Option<f32>,
    carbohydrate: Option<f32>,
    fat: Option<f32>,
}

#[derive(Debug, IsEmpty)]
struct BodyWeight {
    kilograms: Option<f32>,
    pounds: Option<f32>,
}

fn calculate_bmr_harris_benedict(
    body_weight: f32,
    age: i32,
    gender: &str,
    body_fat: f32,
) -> Result<f32, ()> {
    let fat_mass = body_weight * (body_fat / 100.0);
    let lean_body_mass = body_weight - (body_weight * (body_fat / 100.0));
    let basal_metabolic_rate = match gender {
        "m" => {
            (13.587 * lean_body_mass) + (9.613 * fat_mass) + (198.0 * 1.0) - (3.351 * age as f32)
                + 674.0
        }
        "f" => {
            (13.587 * lean_body_mass) + (9.613 * fat_mass) + (198.0 * 0.0) - (3.351 * age as f32)
                + 674.0
        }
        _ => 10.0,
    };
    Ok(basal_metabolic_rate)
}

fn calculate_macros(
    bmr: f32,
    body_weight: f32,
    body_fat: f32,
    activity_level: f32,
    deficit: f32,
) -> Macros {
    let maintenance = bmr * activity_level;
    let with_deficit = maintenance - deficit;
    let lean_body_mass = body_weight - (body_weight * (body_fat / 100.0));
    let protein = 2.2 * lean_body_mass;
    let protein_calories = protein * 4.0;
    let remaining_calories = with_deficit - protein_calories;
    let carbohydrate = (remaining_calories * 0.6) / 4.0;
    let fat = (remaining_calories * 0.4) / 9.0;
    Macros {
        calories: Some(with_deficit),
        protein: Some(protein),
        carbohydrate: Some(carbohydrate),
        fat: Some(fat),
    }
}

fn calculate_bodycomp(body_weight: f32, body_fat: f32, desired_percent: f32) -> BodyWeight {
    let percent_to_lose = body_fat - desired_percent;
    let fat_mass_to_lose = body_weight * (percent_to_lose / 100.0);
    let body_weight_loss = fat_mass_to_lose / 0.713;
    let needed_body_weight = body_weight - body_weight_loss;
    BodyWeight {
        kilograms: Some(needed_body_weight),
        pounds: Some(needed_body_weight * 2.2046226218),
    }
}

fn main() {
    // let mut options = Options {
    //     default: Some(false),
    //     macros: Some(false),
    //     bodycomp: Some(false),
    // };
    // options = Options::parse();

    let mut selection: char = 'f';
    while selection != 'q' {
        println!("Macros (m), Body Weight (b), or both (d)? (quit q)");
        selection = read!();
        if selection == 'q' {
            process::exit(0);
        }
        let kg_multiplier = 0.45359237;
        let mut macros = Macros {
            calories: None,
            protein: None,
            carbohydrate: None,
            fat: None,
        };
        let mut bodyweight = BodyWeight {
            kilograms: None,
            pounds: None,
        };
        print!("Enter body weight in lbs: ");
        let body_weight: f32 = read!();
        print!("Enter your body fat percentage: ");
        let body_fat: f32 = read!();
        let weight_kg: f32 = body_weight * kg_multiplier;
        if selection == 'm' || selection == 'd'
        //|| options.default.is_some() || options.macros.is_some()
        {
            print!("Enter your age in years: ");
            let age: i32 = read!();
            print!("Enter an activity level (1.2 Sedentary, 1.375 Light Activity, 1.55 Moderate Activity, 1.725 Very Active, 1.9 Extra Active): ");
            let activity_level: f32 = read!();
            print!("Enter your gender (m/f): ");
            let gender: String = read!();
            print!("How much of a daily deficit do you want? ");
            let deficit: f32 = read!();
            let basal_metabolic_rate =
                calculate_bmr_harris_benedict(weight_kg, age, &gender, body_fat).unwrap();
            macros = calculate_macros(
                basal_metabolic_rate,
                weight_kg,
                body_fat,
                activity_level,
                deficit,
            );
        }
        if selection == 'b' || selection == 'd'
        // || options.default.is_some() || options.macros.is_some()
        {
            print!("Enter desired body fat %: ");
            let desired_percentage: f32 = read!();
            bodyweight = calculate_bodycomp(weight_kg, body_fat, desired_percentage);
        }
        if !macros.is_empty() {
            println!();
            println!("Your estimated starting macros are: ");
            println!("Calories: {:.0}", macros.calories.unwrap());
            println!("Protein: {:.0}", macros.protein.unwrap());
            println!("Carbohydrates: {:.0}", macros.carbohydrate.unwrap());
            println!("Fat: {:.0}", macros.fat.unwrap());
        }
        if !bodyweight.is_empty() {
            println!();
            println!("Your estimated needed body weight is: ");
            println!("Pounds: {:.2}", bodyweight.pounds.unwrap());
            println!("Kilograms: {:.2}", bodyweight.kilograms.unwrap());
        }
        println!();
    }
}
