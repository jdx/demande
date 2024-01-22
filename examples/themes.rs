use demand::{Confirm, DemandOption, Input, MultiSelect, Theme};

fn main() {
    let theme = Theme {
        selected_prefix: String::from(" •"),
        selected_prefix_fg: Theme::color_rgb(2, 191, 135),
        unselected_prefix: String::from("  "),
        ..Theme::default()
    };

    let i = Input::new("What's your e-mail?")
        .description("Please enter your e-mail address.")
        .placeholder("john.doe@acme.com")
        .theme(theme.clone());
    i.run().expect("error running input");

    let ms = MultiSelect::new("Interests")
        .description("Select your interests")
        .min(1)
        .max(4)
        .filterable(true)
        .option(DemandOption::new("Art"))
        .option(DemandOption::new("Books"))
        .option(DemandOption::new("Food"))
        .option(DemandOption::new("Music"))
        .option(DemandOption::new("Technology"))
        .option(DemandOption::new("Travel"))
        .option(DemandOption::new("Sports"))
        .theme(theme.clone());
    ms.run().expect("error running multi select");

    let c = Confirm::new("Confirm privacy policy")
        .description("Do you accept the privacy policy?")
        .affirmative("Yes")
        .negative("No")
        .theme(theme.clone());
    c.run().expect("error running confirm");
}
