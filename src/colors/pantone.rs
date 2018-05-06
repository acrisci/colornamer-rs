use color_names::ColorNames;
use color::{Color, color_from_hex};

pub struct PantoneColors {
}

impl ColorNames for PantoneColors {
    fn get_colors(&self) -> Vec<Color> {
        return vec![
            color_from_hex("Mahogany", "#CD4A4A"),
            color_from_hex("Fuzzy Wuzzy Brown", "#CC6666"),
            color_from_hex("Chestnut", "#BC5D58"),
            color_from_hex("Red Orange", "#FF5349"),
            color_from_hex("Sunset Orange", "#FD5E53"),
            color_from_hex("Bittersweet", "#FD7C6E"),
            color_from_hex("Melon", "#FDBCB4"),
            color_from_hex("Outrageous Orange", "#FF6E4A"),
            color_from_hex("Vivid Tangerine", "#FFA089"),
            color_from_hex("Burnt Sienna", "#EA7E5D"),
            color_from_hex("Brown", "#B4674D"),
            color_from_hex("Sepia", "#A5694F"),
            color_from_hex("Orange", "#FF7538"),
            color_from_hex("Burnt Orange", "#FF7F49"),
            color_from_hex("Copper", "#DD9475"),
            color_from_hex("Mango Tango", "#FF8243"),
            color_from_hex("Atomic Tangerine", "#FFA474"),
            color_from_hex("Beaver", "#9F8170"),
            color_from_hex("Antique Brass", "#CD9575"),
            color_from_hex("Desert Sand", "#EFCDB8"),
            color_from_hex("Raw Sienna", "#D68A59"),
            color_from_hex("Tumbleweed", "#DEAA88"),
            color_from_hex("Tan", "#FAA76C"),
            color_from_hex("Peach", "#FFCFAB"),
            color_from_hex("Macaroni and Cheese", "#FFBD88"),
            color_from_hex("Apricot", "#FDD9B5"),
            color_from_hex("Neon Carrot", "#FFA343"),
            color_from_hex("Almond", "#EFDBC5"),
            color_from_hex("Yellow Orange", "#FFB653"),
            color_from_hex("Gold", "#E7C697"),
            color_from_hex("Shadow", "#8A795D"),
            color_from_hex("Banana Mania", "#FAE7B5"),
            color_from_hex("Sunglow", "#FFCF48"),
            color_from_hex("Goldenrod", "#FCD975"),
            color_from_hex("Dandelion", "#FDDB6D"),
            color_from_hex("Yellow", "#FCE883"),
            color_from_hex("Green Yellow", "#F0E891"),
            color_from_hex("Spring Green", "#ECEABE"),
            color_from_hex("Olive Green", "#BAB86C"),
            color_from_hex("Laser Lemon", "#FDFC74"),
            color_from_hex("Unmellow Yellow", "#FDFC74"),
            color_from_hex("Canary", "#FFFF99"),
            color_from_hex("Yellow Green", "#C5E384"),
            color_from_hex("Inch Worm", "#B2EC5D"),
            color_from_hex("Asparagus", "#87A96B"),
            color_from_hex("Granny Smith Apple", "#A8E4A0"),
            color_from_hex("Electric Lime", "#1DF914"),
            color_from_hex("Screamin Green", "#76FF7A"),
            color_from_hex("Fern", "#71BC78"),
            color_from_hex("Forest Green", "#6DAE81"),
            color_from_hex("Sea Green", "#9FE2BF"),
            color_from_hex("Green", "#1CAC78"),
            color_from_hex("Mountain Meadow", "#30BA8F"),
            color_from_hex("Shamrock", "#45CEA2"),
            color_from_hex("Jungle Green", "#3BB08F"),
            color_from_hex("Caribbean Green", "#1CD3A2"),
            color_from_hex("Tropical Rain Forest", "#17806D"),
            color_from_hex("Pine Green", "#158078"),
            color_from_hex("Robin Egg Blue", "#1FCECB"),
            color_from_hex("Aquamarine", "#78DBE2"),
            color_from_hex("Turquoise Blue", "#77DDE7"),
            color_from_hex("Sky Blue", "#80DAEB"),
            color_from_hex("Outer Space", "#414A4C"),
            color_from_hex("Blue Green", "#199EBD"),
            color_from_hex("Pacific Blue", "#1CA9C9"),
            color_from_hex("Cerulean", "#1DACD6"),
            color_from_hex("Cornflower", "#9ACEEB"),
            color_from_hex("Midnight Blue", "#1A4876"),
            color_from_hex("Navy Blue", "#1974D2"),
            color_from_hex("Denim", "#2B6CC4"),
            color_from_hex("Blue", "#1F75FE"),
            color_from_hex("Periwinkle", "#C5D0E6"),
            color_from_hex("Cadet Blue", "#B0B7C6"),
            color_from_hex("Indigo", "#5D76CB"),
            color_from_hex("Wild Blue Yonder", "#A2ADD0"),
            color_from_hex("Manatee", "#979AAA"),
            color_from_hex("Blue Bell", "#ADADD6"),
            color_from_hex("Blue Violet", "#7366BD"),
            color_from_hex("Purple Heart", "#7442C8"),
            color_from_hex("Royal Purple", "#7851A9"),
            color_from_hex("Purple Mountains’ Majesty", "#9D81BA"),
            color_from_hex("Violet (Purple)", "#926EAE"),
            color_from_hex("Wisteria", "#CDA4DE"),
            color_from_hex("Vivid Violet", "#8F509D"),
            color_from_hex("Fuchsia", "#C364C5"),
            color_from_hex("Shocking Pink", "#FB7EFD"),
            color_from_hex("Pink Flamingo", "#FC74FD"),
            color_from_hex("Plum", "#8E4585"),
            color_from_hex("Hot Magenta", "#FF1DCE"),
            color_from_hex("Purple Pizzazz", "#FF1DCE"),
            color_from_hex("Razzle Dazzle Rose", "#FF48D0"),
            color_from_hex("Orchid", "#E6A8D7"),
            color_from_hex("Red Violet", "#C0448F"),
            color_from_hex("Eggplant", "#6E5160"),
            color_from_hex("Cerise", "#DD4492"),
            color_from_hex("Wild Strawberry", "#FF43A4"),
            color_from_hex("Magenta", "#F664AF"),
            color_from_hex("Lavender", "#FCB4D5"),
            color_from_hex("Cotton Candy", "#FFBCD9"),
            color_from_hex("Violet Red", "#F75394"),
            color_from_hex("Carnation Pink", "#FFAACC"),
            color_from_hex("Razzmatazz", "#E3256B"),
            color_from_hex("Piggy Pink", "#FDD7E4"),
            color_from_hex("Jazzberry Jam", "#CA3767"),
            color_from_hex("Blush", "#DE5D83"),
            color_from_hex("Tickle Me Pink", "#FC89AC"),
            color_from_hex("Pink Sherbet", "#F780A1"),
            color_from_hex("Maroon", "#C8385A"),
            color_from_hex("Red", "#EE204D"),
            color_from_hex("Radical Red", "#FF496C"),
            color_from_hex("Mauvelous", "#EF98AA"),
            color_from_hex("Wild Watermelon", "#FC6C85"),
            color_from_hex("Scarlet", "#FC2847"),
            color_from_hex("Salmon", "#FF9BAA"),
            color_from_hex("Brick Red", "#CB4154"),
            color_from_hex("White", "#EDEDED"),
            color_from_hex("Timberwolf", "#DBD7D2"),
            color_from_hex("Silver", "#CDC5C2"),
            color_from_hex("Gray", "#95918C"),
            color_from_hex("Black", "#232323")
        ]
    }
}
