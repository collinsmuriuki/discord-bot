use rand::{self, Rng};

pub fn generate_random_name() -> String {
    let names = [
        "Baby Oil",
        "Bad News",
        "Big Burps",
        "Bill 'Beenie-Weenie'",
        "Bob 'Stinkbug'",
        "Bowel Noises",
        "Boxelder",
        "Bud 'Lite'",
        "Butterbean",
        "Buttermilk",
        "Buttocks",
        "Chad",
        "Chesterfield",
        "Chewy",
        "Chigger",
        "Cinnabuns",
        "Cleet",
        "Cornbread",
        "Crab Meat",
        "Crapps",
        "Dark Skies",
        "Dennis Clawhammer",
        "Dicman",
        "Elphonso",
        "Fancypants",
        "Figgs",
        "Foncy",
        "Gootsy",
        "Greasy Jim",
        "Huckleberry",
        "Huggy",
        "Ignatious",
        "Jimbo",
        "Joe 'Pottin Soil'",
        "Johnny",
        "Lemongrass",
        "Lil Debil",
        "Longbranch",
        "'Lunch Money'",
        "Mergatroid",
        "'Mr Peabody'",
        "Oil-Can",
        "Oinks",
        "Old Scratch",
        "Ovaltine",
        "Pennywhistle",
        "Pitchfork Ben",
        "Potato Bug",
        "Pushmeet",
        "Rock Candy",
        "Schlomo",
        "Scratchensniff",
        "Scut",
        "Sid 'The Squirts'",
        "Skidmark",
        "Slaps",
        "Snakes",
        "Snoobs",
        "Snorki",
        "Soupcan Sam",
        "Spitzitout",
        "Squids",
        "Stinky",
        "Storyboard",
        "Sweet Tea",
        "TeeTee",
        "Wheezy Joe",
        "Winston 'Jazz Hands'",
        "'Worms'",
    ];

    let last = [
        "Appleyard",
        "Bigmeat",
        "Bloominshine",
        "Boogerbottom",
        "Breedslovetrout",
        "Butterbaugh",
        "Clovenhoof",
        "Clutterbuck",
        "Cocktoasten",
        "Endicott",
        "Fewhairs",
        "Gooberdapple",
        "Goodensmith",
        "Goodpasture",
        "Guster",
        "Henderson",
        "Hooperbag",
        "Hoosenater",
        "Hootkins",
        "Jefferson",
        "Jenkins",
        "Jingley-Schmidt",
        "Johnson",
        "Kingfish",
        "Listenbee",
        "M'Bembo",
        "McFadden",
        "Moonshine",
        "Nettles",
        "Noseworthy",
        "Olivetti",
        "Outerbridge",
        "Overpeck",
        "Overturf",
        "Oxhandler",
        "Pealike",
        "Pennywhistle",
        "Peterson",
        "Pieplow",
        "Pinkerton",
        "Porkins",
        "Putney",
        "Quakenbush",
        "Rainwater",
        "Rosenthal",
        "Rubbins",
        "Sackrider",
        "Snuggleshine",
        "Splern",
        "Stevens",
        "Stroganoff",
        "Sugar-Gold",
        "Swackhamer",
        "Tippins",
        "Turnipseed",
        "Vinaigrette",
        "Ng'ombe",
        "Walkingstick",
        "Wallbanger",
        "Weewax",
        "Weiners",
        "Whipkey",
        "Wigglesworth",
        "Wimplesnatch",
        "Winterkorn",
        "Woolysocks",
    ];

    let mut rng = rand::thread_rng();
    let mut value = String::new();

    for (i, v) in names.iter().enumerate() {
        if i == rng.gen_range(1, names.len()) {
            value.push_str(*v);
            break;
        }
    }

    for (i, v) in last.iter().enumerate() {
        if i == rng.gen_range(1, last.len()) {
            value.push_str(*v);
            break;
        }
    }

    format!("You are from now on...{}", value)
}

pub fn parse_r6_args(message: &str) -> Option<(&str, &str)> {
    for (i, c) in message.chars().enumerate() {
        if c == ' ' {
            return Some((&message[..i], &message[i+1..]));
        }
    }
    None
}

pub fn get_r6_stats(platform: &str, username: &str) -> String {
    unimplemented!()
}