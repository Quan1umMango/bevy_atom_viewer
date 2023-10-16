/// Contains enums and other predefinedthings dfor 118 atoms
use std::fmt;
use crate::atoms::*;

 
#[derive(Clone,PartialEq)]
pub enum AtomType {
    Hydrogen,
    Helium,
    Lithium,
    Beryllium,
    Boron,
    Carbon,
    Nitrogen,
    Oxygen,
    Fluorine,
    Neon,
    Sodium,
    Magnesium,
    Aluminum,
    Silicon,
    Phosphorus,
    Sulfur,
    Chlorine,
    Argon,
    Potassium,
    Calcium,
    Scandium,
    Titanium,
    Vanadium,
    Chromium,
    Manganese,
    Iron,
    Cobalt,
    Nickel,
    Copper,
    Zinc,
    Gallium,
    Germanium,
    Arsenic,
    Selenium,
    Bromine,
    Krypton,
    Rubidium,
    Strontium,
    Yttrium,
    Zirconium,
    Niobium,
    Molybdenum,
    Technetium,
    Ruthenium,
    Rhodium,
    Palladium,
    Silver,
    Cadmium,
    Indium,
    Tin,
    Antimony,
    Tellurium,
    Iodine,
    Xenon,
    Cesium,
    Barium,
    Lanthanum,
    Cerium,
    Praseodymium,
    Neodymium,
    Promethium,
    Samarium,
    Europium,
    Gadolinium,
    Terbium,
    Dysprosium,
    Holmium,
    Erbium,
    Thulium,
    Ytterbium,
    Lutetium,
    Hafnium,
    Tantalum,
    Tungsten,
    Rhenium,
    Osmium,
    Iridium,
    Platinum,
    Gold,
    Mercury,
    Thallium,
    Lead,
    Bismuth,
    Polonium,
    Astatine,
    Radon,
    Francium,
    Radium,
    Actinium,
    Thorium,
    Protactinium,
    Uranium,
    Neptunium,
    Plutonium,
    Americium,
    Curium,
    Berkelium,
    Californium,
    Einsteinium,
    Fermium,
    Mendelevium,
    Nobelium,
    Lawrencium,
    Rutherfordium,
    Dubnium,
    Seaborgium,
    Bohrium,
    Hassium,
    Meitnerium,
    Darmstadtium,
    Roentgenium,
    Copernicium,
    Nihonium,
    Flerovium,
    Moscovium,
    Livermorium,
    Tennessine,
    Oganesson,
}


impl Atom {
    pub fn new(atom_type: AtomType, atomic_number: u32, protons: u32, electrons: u32, neutrons: u32) -> Self {
        Self {
            atom_type,
            atomic_number,
            protons,
            electrons,
            neutrons,
        }
    }

    pub fn hydrogen() -> Atom {
        Atom::new(AtomType::Hydrogen, 1, 1, 1, 0)
    }

    pub fn helium() -> Atom {
        Atom::new(AtomType::Helium, 2, 2, 2, 2)
    }

    pub fn lithium() -> Atom {
        Atom::new(AtomType::Lithium, 3, 3, 3, 4)
    }

    pub fn beryllium() -> Atom {
        Atom::new(AtomType::Beryllium, 4, 4, 4, 5)
    }

    pub fn boron() -> Atom {
        Atom::new(AtomType::Boron, 5, 5, 5, 6)
    }

    pub fn carbon() -> Atom {
        Atom::new(AtomType::Carbon, 6, 6, 6, 6)
    }

    pub fn nitrogen() -> Atom {
        Atom::new(AtomType::Nitrogen, 7, 7, 7, 7)
    }

    pub fn oxygen() -> Atom {
        Atom::new(AtomType::Oxygen, 8, 8, 8, 8)
    }

    pub fn fluorine() -> Atom {
        Atom::new(AtomType::Fluorine, 9, 9, 9, 10)
    }

    pub fn neon() -> Atom {
        Atom::new(AtomType::Neon, 10, 10, 10, 10)
    }

    pub fn sodium() -> Atom {
        Atom::new(AtomType::Sodium, 11, 11, 11, 12)
    }

    pub fn magnesium() -> Atom {
        Atom::new(AtomType::Magnesium, 12, 12, 12, 12)
    }

    pub fn aluminum() -> Atom {
        Atom::new(AtomType::Aluminum, 13, 13, 13, 14)
    }

    pub fn silicon() -> Atom {
        Atom::new(AtomType::Silicon, 14, 14, 14, 14)
    }

    pub fn phosphorus() -> Atom {
        Atom::new(AtomType::Phosphorus, 15, 15, 15, 16)
    }

    pub fn sulfur() -> Atom {
        Atom::new(AtomType::Sulfur, 16, 16, 16, 16)
    }

    pub fn chlorine() -> Atom {
        Atom::new(AtomType::Chlorine, 17, 17, 17, 18)
    }

    pub fn argon() -> Atom {
        Atom::new(AtomType::Argon, 18, 18, 18, 22)
    }

    pub fn potassium() -> Atom {
        Atom::new(AtomType::Potassium, 19, 19, 19, 20)
    }

    pub fn calcium() -> Atom {
        Atom::new(AtomType::Calcium, 20, 20, 20, 20)
    }

    pub fn scandium() -> Atom {
        Atom::new(AtomType::Scandium, 21, 21, 21, 24)
    }

    pub fn titanium() -> Atom {
        Atom::new(AtomType::Titanium, 22, 22, 22, 26)
    }

    pub fn vanadium() -> Atom {
        Atom::new(AtomType::Vanadium, 23, 23, 23, 28)
    }

    pub fn chromium() -> Atom {
        Atom::new(AtomType::Chromium, 24, 24, 24, 28)
    }

    pub fn manganese() -> Atom {
        Atom::new(AtomType::Manganese, 25, 25, 25, 30)
    }

    pub fn iron() -> Atom {
        Atom::new(AtomType::Iron, 26, 26, 26, 30)
    }

    pub fn cobalt() -> Atom {
        Atom::new(AtomType::Cobalt, 27, 27, 27, 32)
    }

    pub fn nickel() -> Atom {
        Atom::new(AtomType::Nickel, 28, 28, 28, 31)
    }

    pub fn copper() -> Atom {
        Atom::new(AtomType::Copper, 29, 29, 29, 35)
    }

    pub fn zinc() -> Atom {
        Atom::new(AtomType::Zinc, 30, 30, 30, 35)
    }

    pub fn gallium() -> Atom {
        Atom::new(AtomType::Gallium, 31, 31, 31, 39)
    }

    pub fn germanium() -> Atom {
        Atom::new(AtomType::Germanium, 32, 32, 32, 42)
    }

    pub fn arsenic() -> Atom {
        Atom::new(AtomType::Arsenic, 33, 33, 33, 42)
    }

    pub fn selenium() -> Atom {
        Atom::new(AtomType::Selenium, 34, 34, 34, 45)
    }

    pub fn bromine() -> Atom {
        Atom::new(AtomType::Bromine, 35, 35, 35, 45)
    }

    pub fn krypton() -> Atom {
        Atom::new(AtomType::Krypton, 36, 36, 36, 48)
    }

    pub fn rubidium() -> Atom {
        Atom::new(AtomType::Rubidium, 37, 37, 37, 48)
    }

    pub fn strontium() -> Atom {
        Atom::new(AtomType::Strontium, 38, 38, 38, 49)
    }

    pub fn yttrium() -> Atom {
        Atom::new(AtomType::Yttrium, 39, 39, 39, 50)
    }

    pub fn zirconium() -> Atom {
        Atom::new(AtomType::Zirconium, 40, 40, 40, 51)
    }

    pub fn niobium() -> Atom {
        Atom::new(AtomType::Niobium, 41, 41, 41, 52)
    }

    pub fn molybdenum() -> Atom {
        Atom::new(AtomType::Molybdenum, 42, 42, 42, 54)
    }

    pub fn technetium() -> Atom {
        Atom::new(AtomType::Technetium, 43, 43, 43, 55)
    }

    pub fn ruthenium() -> Atom {
        Atom::new(AtomType::Ruthenium, 44, 44, 44, 57)
    }

    pub fn rhodium() -> Atom {
        Atom::new(AtomType::Rhodium, 45, 45, 45, 58)
    }

    pub fn palladium() -> Atom {
        Atom::new(AtomType::Palladium, 46, 46, 46, 60)
    }

    pub fn silver() -> Atom {
        Atom::new(AtomType::Silver, 47, 47, 47, 61)
    }

    pub fn cadmium() -> Atom {
        Atom::new(AtomType::Cadmium, 48, 48, 48, 64)
    }

    pub fn indium() -> Atom {
        Atom::new(AtomType::Indium, 49, 49, 49, 66)
    }

    pub fn tin() -> Atom {
        Atom::new(AtomType::Tin, 50, 50, 50, 70)
    }

    pub fn antimony() -> Atom {
        Atom::new(AtomType::Antimony, 51, 51, 51, 71)
    }

    pub fn tellurium() -> Atom {
        Atom::new(AtomType::Tellurium, 52, 52, 52, 76)
    }

    pub fn iodine() -> Atom {
        Atom::new(AtomType::Iodine, 53, 53, 53, 74)
    }

    pub fn xenon() -> Atom {
        Atom::new(AtomType::Xenon, 54, 54, 54, 77)
    }

    pub fn cesium() -> Atom {
        Atom::new(AtomType::Cesium, 55, 55, 55, 78)
    }

    pub fn barium() -> Atom {
        Atom::new(AtomType::Barium, 56, 56, 56, 81)
    }

    pub fn lanthanum() -> Atom {
        Atom::new(AtomType::Lanthanum, 57, 57, 57, 82)
    }

    pub fn cerium() -> Atom {
        Atom::new(AtomType::Cerium, 58, 58, 58, 82)
    }

    pub fn praseodymium() -> Atom {
        Atom::new(AtomType::Praseodymium, 59, 59, 59, 82)
    }

    pub fn neodymium() -> Atom {
        Atom::new(AtomType::Neodymium, 60, 60, 60, 83)
    }

    pub fn promethium() -> Atom {
        Atom::new(AtomType::Promethium, 61, 61, 61, 84)
    }

    pub fn samarium() -> Atom {
        Atom::new(AtomType::Samarium, 62, 62, 62, 88)
    }

    pub fn europium() -> Atom {
        Atom::new(AtomType::Europium, 63, 63, 63, 89)
    }

    pub fn gadolinium() -> Atom {
        Atom::new(AtomType::Gadolinium, 64, 64, 64, 93)
    }

    pub fn terbium() -> Atom {
        Atom::new(AtomType::Terbium, 65, 65, 65, 94)
    }

    pub fn dysprosium() -> Atom {
        Atom::new(AtomType::Dysprosium, 66, 66, 66, 97)
    }

    pub fn holmium() -> Atom {
        Atom::new(AtomType::Holmium, 67, 67, 67, 98)
    }

    pub fn erbium() -> Atom {
        Atom::new(AtomType::Erbium, 68, 68, 68, 99)
    }

    pub fn thulium() -> Atom {
        Atom::new(AtomType::Thulium, 69, 69, 69, 100)
    }

    pub fn ytterbium() -> Atom {
        Atom::new(AtomType::Ytterbium, 70, 70, 70, 103)
    }

    pub fn lutetium() -> Atom {
        Atom::new(AtomType::Lutetium, 71, 71, 71, 104)
    }

    pub fn hafnium() -> Atom {
        Atom::new(AtomType::Hafnium, 72, 72, 72, 107)
    }

    pub fn tantalum() -> Atom {
        Atom::new(AtomType::Tantalum, 73, 73, 73, 108)
    }

    pub fn tungsten() -> Atom {
        Atom::new(AtomType::Tungsten, 74, 74, 74, 110)
    }

    pub fn rhenium() -> Atom {
        Atom::new(AtomType::Rhenium, 75, 75, 75, 111)
    }

    pub fn osmium() -> Atom {
        Atom::new(AtomType::Osmium, 76, 76, 76, 114)
    }

    pub fn iridium() -> Atom {
        Atom::new(AtomType::Iridium, 77, 77, 77, 115)
    }

    pub fn platinum() -> Atom {
        Atom::new(AtomType::Platinum, 78, 78, 78, 117)
    }

    pub fn gold() -> Atom {
        Atom::new(AtomType::Gold, 79, 79, 79, 118)
    }

    pub fn mercury() -> Atom {
        Atom::new(AtomType::Mercury, 80, 80, 80, 121)
    }

    pub fn thallium() -> Atom {
        Atom::new(AtomType::Thallium, 81, 81, 81, 123)
    }

    pub fn lead() -> Atom {
        Atom::new(AtomType::Lead, 82, 82, 82, 125)
    }

    pub fn bismuth() -> Atom {
        Atom::new(AtomType::Bismuth, 83, 83, 83, 126)
    }

    pub fn polonium() -> Atom {
        Atom::new(AtomType::Polonium, 84, 84, 84, 125)
    }

    pub fn astatine() -> Atom {
        Atom::new(AtomType::Astatine, 85, 85, 85, 125)
    }

    pub fn radon() -> Atom {
        Atom::new(AtomType::Radon, 86, 86, 86, 136)
    }

    pub fn francium() -> Atom {
        Atom::new(AtomType::Francium, 87, 87, 87, 136)
    }

    pub fn radium() -> Atom {
        Atom::new(AtomType::Radium, 88, 88, 88, 138)
    }

    pub fn actinium() -> Atom {
        Atom::new(AtomType::Actinium, 89, 89, 89, 138)
    }

    pub fn thorium() -> Atom {
        Atom::new(AtomType::Thorium, 90, 90, 90, 142)
    }

    pub fn protactinium() -> Atom {
        Atom::new(AtomType::Protactinium, 91, 91, 91, 140)
    }

    pub fn uranium() -> Atom {
        Atom::new(AtomType::Uranium, 92, 92, 92, 146)
    }

    pub fn neptunium() -> Atom {
        Atom::new(AtomType::Neptunium, 93, 93, 93, 144)
    }

    pub fn plutonium() -> Atom {
        Atom::new(AtomType::Plutonium, 94, 94, 94, 150)
    }

    pub fn americium() -> Atom {
        Atom::new(AtomType::Americium, 95, 95, 95, 148)
    }

    pub fn curium() -> Atom {
        Atom::new(AtomType::Curium, 96, 96, 96, 151)
    }

    pub fn berkelium() -> Atom {
        Atom::new(AtomType::Berkelium, 97, 97, 97, 150)
    }

    pub fn californium() -> Atom {
        Atom::new(AtomType::Californium, 98, 98, 98, 153)
    }

    pub fn einsteinium() -> Atom {
        Atom::new(AtomType::Einsteinium, 99, 99, 99, 152)
    }

    pub fn fermium() -> Atom {
        Atom::new(AtomType::Fermium, 100, 100, 100, 157)
    }

    pub fn mendelevium() -> Atom {
        Atom::new(AtomType::Mendelevium, 101, 101, 101, 157)
    }

    pub fn nobelium() -> Atom {
        Atom::new(AtomType::Nobelium, 102, 102, 102, 157)
    }

    pub fn lawrencium() -> Atom {
        Atom::new(AtomType::Lawrencium, 103, 103, 103, 159)
    }

    pub fn rutherfordium() -> Atom {
        Atom::new(AtomType::Rutherfordium, 104, 104, 104, 160)
    }

    pub fn dubnium() -> Atom {
        Atom::new(AtomType::Dubnium, 105, 105, 105, 161)
    }

    pub fn seaborgium() -> Atom {
        Atom::new(AtomType::Seaborgium, 106, 106, 106, 163)
    }

    pub fn bohrium() -> Atom {
        Atom::new(AtomType::Bohrium, 107, 107, 107, 163)
    }

    pub fn hassium() -> Atom {
        Atom::new(AtomType::Hassium, 108, 108, 108, 163)
    }

    pub fn meitnerium() -> Atom {
        Atom::new(AtomType::Meitnerium, 109, 109, 109, 164)
    }

    pub fn darmstadtium() -> Atom {
        Atom::new(AtomType::Darmstadtium, 110, 110, 110, 171)
        }

        pub fn roentgenium() -> Atom {
            Atom::new(AtomType::Roentgenium, 111, 111, 111, 171)
        }

        pub fn copernicium() -> Atom {
            Atom::new(AtomType::Copernicium, 112, 112, 112, 173)
        }

        pub fn nihonium() -> Atom {
            Atom::new(AtomType::Nihonium, 113, 113, 113, 173)
        }

        pub fn flerovium() -> Atom {
            Atom::new(AtomType::Flerovium, 114, 114, 114, 175)
        }

        pub fn moscovium() -> Atom {
            Atom::new(AtomType::Moscovium, 115, 115, 115, 175)
        }

        pub fn livermorium() -> Atom {
            Atom::new(AtomType::Livermorium, 116, 116, 116, 177)
        }

        pub fn tennessine() -> Atom {
            Atom::new(AtomType::Tennessine, 117, 117, 117, 177)
        }

        pub fn oganesson() -> Atom {
            Atom::new(AtomType::Oganesson, 118, 118, 118, 176)
        }
    }



impl fmt::Display for AtomType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let atom_name = match self {
            AtomType::Hydrogen => "Hydrogen",
            AtomType::Helium => "Helium",
            AtomType::Lithium => "Lithium",
            AtomType::Beryllium => "Beryllium",
            AtomType::Boron => "Boron",
            AtomType::Carbon => "Carbon",
            AtomType::Nitrogen => "Nitrogen",
            AtomType::Oxygen => "Oxygen",
            AtomType::Fluorine => "Fluorine",
            AtomType::Neon => "Neon",
            AtomType::Sodium => "Sodium",
            AtomType::Magnesium => "Magnesium",
            AtomType::Aluminum => "Aluminum",
            AtomType::Silicon => "Silicon",
            AtomType::Phosphorus => "Phosphorus",
            AtomType::Sulfur => "Sulfur",
            AtomType::Chlorine => "Chlorine",
            AtomType::Argon => "Argon",
            AtomType::Potassium => "Potassium",
            AtomType::Calcium => "Calcium",
            AtomType::Scandium => "Scandium",
            AtomType::Titanium => "Titanium",
            AtomType::Vanadium => "Vanadium",
            AtomType::Chromium => "Chromium",
            AtomType::Manganese => "Manganese",
            AtomType::Iron => "Iron",
            AtomType::Cobalt => "Cobalt",
            AtomType::Nickel => "Nickel",
            AtomType::Copper => "Copper",
            AtomType::Zinc => "Zinc",
            AtomType::Gallium => "Gallium",
            AtomType::Germanium => "Germanium",
            AtomType::Arsenic => "Arsenic",
            AtomType::Selenium => "Selenium",
            AtomType::Bromine => "Bromine",
            AtomType::Krypton => "Krypton",
            AtomType::Rubidium => "Rubidium",
            AtomType::Strontium => "Strontium",
            AtomType::Yttrium => "Yttrium",
            AtomType::Zirconium => "Zirconium",
            AtomType::Niobium => "Niobium",
            AtomType::Molybdenum => "Molybdenum",
            AtomType::Technetium => "Technetium",
            AtomType::Ruthenium => "Ruthenium",
            AtomType::Rhodium => "Rhodium",
            AtomType::Palladium => "Palladium",
            AtomType::Silver => "Silver",
            AtomType::Cadmium => "Cadmium",
            AtomType::Indium => "Indium",
            AtomType::Tin => "Tin",
            AtomType::Antimony => "Antimony",
            AtomType::Tellurium => "Tellurium",
            AtomType::Iodine => "Iodine",
            AtomType::Xenon => "Xenon",
            AtomType::Cesium => "Cesium",
            AtomType::Barium => "Barium",
            AtomType::Lanthanum => "Lanthanum",
            AtomType::Cerium => "Cerium",
            AtomType::Praseodymium => "Praseodymium",
            AtomType::Neodymium => "Neodymium",
            AtomType::Promethium => "Promethium",
            AtomType::Samarium => "Samarium",
            AtomType::Europium => "Europium",
            AtomType::Gadolinium => "Gadolinium",
            AtomType::Terbium => "Terbium",
            AtomType::Dysprosium => "Dysprosium",
            AtomType::Holmium => "Holmium",
            AtomType::Erbium => "Erbium",
            AtomType::Thulium => "Thulium",
            AtomType::Ytterbium => "Ytterbium",
            AtomType::Lutetium => "Lutetium",
            AtomType::Hafnium => "Hafnium",
            AtomType::Tantalum => "Tantalum",
            AtomType::Tungsten => "Tungsten",
            AtomType::Rhenium => "Rhenium",
            AtomType::Osmium => "Osmium",
            AtomType::Iridium => "Iridium",
            AtomType::Platinum => "Platinum",
            AtomType::Gold => "Gold",
            AtomType::Mercury => "Mercury",
            AtomType::Thallium => "Thallium",
            AtomType::Lead => "Lead",
            AtomType::Bismuth => "Bismuth",
            AtomType::Polonium => "Polonium",
            AtomType::Astatine => "Astatine",
            AtomType::Radon => "Radon",
            AtomType::Francium => "Francium",
            AtomType::Radium => "Radium",
            AtomType::Actinium => "Actinium",
            AtomType::Thorium => "Thorium",
            AtomType::Protactinium => "Protactinium",
            AtomType::Uranium => "Uranium",
            AtomType::Neptunium => "Neptunium",
            AtomType::Plutonium => "Plutonium",
            AtomType::Americium => "Americium",
            AtomType::Curium => "Curium",
            AtomType::Berkelium => "Berkelium",
            AtomType::Californium => "Californium",
            AtomType::Einsteinium => "Einsteinium",
            AtomType::Fermium => "Fermium",
            AtomType::Mendelevium => "Mendelevium",
            AtomType::Nobelium => "Nobelium",
            AtomType::Lawrencium => "Lawrencium",
            AtomType::Rutherfordium => "Rutherfordium",
            AtomType::Dubnium => "Dubnium",
            AtomType::Seaborgium => "Seaborgium",
            AtomType::Bohrium => "Bohrium",
            AtomType::Hassium => "Hassium",
            AtomType::Meitnerium => "Meitnerium",
            AtomType::Darmstadtium => "Darmstadtium",
            AtomType::Roentgenium => "Roentgenium",
            AtomType::Copernicium => "Copernicium",
            AtomType::Nihonium => "Nihonium",
            AtomType::Flerovium => "Flerovium",
            AtomType::Moscovium => "Moscovium",
            AtomType::Livermorium => "Livermorium",
            AtomType::Tennessine => "Tennessine",
            AtomType::Oganesson => "Oganesson",
        };
        write!(f, "{}", atom_name)
    }
}
