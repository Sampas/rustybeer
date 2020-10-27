use clap::{value_t, App, Arg, ArgMatches};
use rustybeer_util::conversions::{VolumeBuilder};
pub use rustybeer::calculators::alcohol_volume_weight::{
    calculate_abv_abw, calculate_abv_abw_density, calculate_abw_abv, calculate_abw_abv_density,
    calculate_alc_vol, calculate_alc_weight,
};

pub fn add_subcommand<'a, 'b>() -> App<'a, 'b> {
    App::new("abv_abw")
        .version("0.1")
        .author("Joseph Russell (josephrussell123@gmail.com)")
        .about("Calculates Alcohol by Weight (ABW) from Alcohol By Volue (ABV) and vice versa")
        .arg(
            Arg::with_name("percent")
                .short("p")
                .long("percent")
                .value_name("PERCENT")
                .help("'From' alcohol percentage")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("total_volume")
                .short("v")
                .long("total_volume")
                .value_name("TOTAL_VOLUME")
                .help("Total beer volume")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("total_density")
                .short("d")
                .long("total_density")
                .value_name("TOTAL_DENSITY")
                .help("Total density of beer in g/cm³")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("reverse")
                .short("r")
                .long("reverse")
                .value_name("REVERSE")
                .help("Calculates ABW to ABV")
                .required(false)
                .takes_value(false),
        )
}

pub fn do_matches(matches: &ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("abv_abw") {
        if matches.is_present("percent") {
            let percent = value_t!(matches, "percent", f64).unwrap_or_else(|e| e.exit());
            let end_percentage: f64;

            // main ABV <-> ABW conversion
            if matches.is_present("reverse") {
                if matches.is_present("total_density") {
                    let total_density =
                        value_t!(matches, "total_density", f64).unwrap_or_else(|e| e.exit());
                    end_percentage = calculate_abw_abv_density(percent, total_density);
                } else {
                    end_percentage = calculate_abw_abv(percent);
                }
                println!("ABV: {:.3}%", end_percentage);
            } else {
                if matches.is_present("total_density") {
                    let total_density =
                        value_t!(matches, "total_density", f64).unwrap_or_else(|e| e.exit());
                    end_percentage = calculate_abv_abw_density(percent, total_density);
                } else {
                    end_percentage = calculate_abv_abw(percent);
                }
                println!("ABW: {:.3}%", end_percentage);
            }

            // Quantity of alcohol
            if matches.is_present("total_volume") {
                let total_volume = value_t!(matches, "total_volume", String).unwrap_or_else(|e| e.exit());
                let total_volume_millilitres = VolumeBuilder::new(&total_volume).unwrap().as_millilitres();
                if matches.is_present("reverse") {
                    println!(
                        "Alcohol: {:.3} ml",
                        calculate_alc_vol(total_volume_millilitres, end_percentage)
                    );
                } else {
                    println!(
                        "Alcohol: {:.3} g",
                        calculate_alc_weight(total_volume_millilitres, percent)
                    );
                }
            }
        } else {
            println!("The alcohol percentage has not been specified.");
            println!("{}", matches.usage());
        }
    }
}
