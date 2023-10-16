use bevy_egui::{egui, EguiContexts};
use crate::*; 
pub fn update_ui(mut contexts: EguiContexts,mut selected_atom: ResMut<SelectedAtom>,mut ev_change_selected_atom:EventWriter<ChangeSelectedAtom>) {

    egui::Window::new("Choose Atoms").vscroll(true).show(contexts.ctx_mut(), |ui| {
        if ui.add(egui::Button::new("Hydrogen")).clicked() && selected_atom.0.atom_type != AtomType::Hydrogen {
            selected_atom.0 = Atom::hydrogen();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Hydrogen));
        }

        // Handling selection for Helium
        if ui.add(egui::Button::new("Helium")).clicked() && selected_atom.0.atom_type != AtomType::Helium {
            selected_atom.0 = Atom::helium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Helium));
        }

        // Handling selection for Lithium
        if ui.add(egui::Button::new("Lithium")).clicked() && selected_atom.0.atom_type != AtomType::Lithium {
            selected_atom.0 = Atom::lithium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Lithium));
        }

        // Handling selection for Beryllium
        if ui.add(egui::Button::new("Beryllium")).clicked() && selected_atom.0.atom_type != AtomType::Beryllium {
            selected_atom.0 = Atom::beryllium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Beryllium));
        }

        // Handling selection for Boron
        if ui.add(egui::Button::new("Boron")).clicked() && selected_atom.0.atom_type != AtomType::Boron {
            selected_atom.0 = Atom::boron();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Boron));
        }

        // Handling selection for Carbon
        if ui.add(egui::Button::new("Carbon")).clicked() && selected_atom.0.atom_type != AtomType::Carbon {
            selected_atom.0 = Atom::carbon();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Carbon));
        }

        // Handling selection for Nitrogen
        if ui.add(egui::Button::new("Nitrogen")).clicked() && selected_atom.0.atom_type != AtomType::Nitrogen {
            selected_atom.0 = Atom::nitrogen();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Nitrogen));
        }

        // Handling selection for Oxygen
        if ui.add(egui::Button::new("Oxygen")).clicked() && selected_atom.0.atom_type != AtomType::Oxygen {
            selected_atom.0 = Atom::oxygen();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Oxygen));
        }

        // Handling selection for Fluorine
        if ui.add(egui::Button::new("Fluorine")).clicked() && selected_atom.0.atom_type != AtomType::Fluorine {
            selected_atom.0 = Atom::fluorine();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Fluorine));
        }

        // Handling selection for Neon
        if ui.add(egui::Button::new("Neon")).clicked() && selected_atom.0.atom_type != AtomType::Neon {
            selected_atom.0 = Atom::neon();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Neon));
        }

        // Handling selection for Sodium
        if ui.add(egui::Button::new("Sodium")).clicked() && selected_atom.0.atom_type != AtomType::Sodium {
            selected_atom.0 = Atom::sodium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Sodium));
        }

        // Handling selection for Magnesium
        if ui.add(egui::Button::new("Magnesium")).clicked() && selected_atom.0.atom_type != AtomType::Magnesium {
            selected_atom.0 = Atom::magnesium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Magnesium));
        }

        // Handling selection for Aluminum
        if ui.add(egui::Button::new("Aluminum")).clicked() && selected_atom.0.atom_type != AtomType::Aluminum {
            selected_atom.0 = Atom::aluminum();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Aluminum));
        }

        // Handling selection for Silicon
        if ui.add(egui::Button::new("Silicon")).clicked() && selected_atom.0.atom_type != AtomType::Silicon {
            selected_atom.0 = Atom::silicon();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Silicon));
        }

        // Handling selection for Phosphorus
        if ui.add(egui::Button::new("Phosphorus")).clicked() && selected_atom.0.atom_type != AtomType::Phosphorus {
            selected_atom.0 = Atom::phosphorus();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Phosphorus));
        }

        // Handling selection for Sulfur
        if ui.add(egui::Button::new("Sulfur")).clicked() && selected_atom.0.atom_type != AtomType::Sulfur {
            selected_atom.0 = Atom::sulfur();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Sulfur));
        }

        // Handling selection for Chlorine
        if ui.add(egui::Button::new("Chlorine")).clicked() && selected_atom.0.atom_type != AtomType::Chlorine {
            selected_atom.0 = Atom::chlorine();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Chlorine));
        }

        // Handling selection for Argon
        if ui.add(egui::Button::new("Argon")).clicked() && selected_atom.0.atom_type != AtomType::Argon {
            selected_atom.0 = Atom::argon();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Argon));
        }

        // Handling selection for Potassium
        if ui.add(egui::Button::new("Potassium")).clicked() && selected_atom.0.atom_type != AtomType::Potassium {
            selected_atom.0 = Atom::potassium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Potassium));
        }

        // Handling selection for Calcium
        if ui.add(egui::Button::new("Calcium")).clicked() && selected_atom.0.atom_type != AtomType::Calcium {
            selected_atom.0 = Atom::calcium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Calcium));
        }

        // Handling selection for Scandium
        if ui.add(egui::Button::new("Scandium")).clicked() && selected_atom.0.atom_type != AtomType::Scandium {
            selected_atom.0 = Atom::scandium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Scandium));
        }

        // Handling selection for Titanium
        if ui.add(egui::Button::new("Titanium")).clicked() && selected_atom.0.atom_type != AtomType::Titanium {
            selected_atom.0 = Atom::titanium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Titanium));
        } 

        if ui.add(egui::Button::new("Vanadium")).clicked() && selected_atom.0.atom_type != AtomType::Vanadium {
            selected_atom.0 = Atom::vanadium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Vanadium));
        }

        // Handling selection for Chromium
        if ui.add(egui::Button::new("Chromium")).clicked() && selected_atom.0.atom_type != AtomType::Chromium {
            selected_atom.0 = Atom::chromium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Chromium));
        }

        // Handling selection for Manganese
        if ui.add(egui::Button::new("Manganese")).clicked() && selected_atom.0.atom_type != AtomType::Manganese {
            selected_atom.0 = Atom::manganese();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Manganese));
        }

        // Handling selection for Iron
        if ui.add(egui::Button::new("Iron")).clicked() && selected_atom.0.atom_type != AtomType::Iron {
            selected_atom.0 = Atom::iron();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Iron));
        }

        // Handling selection for Cobalt
        if ui.add(egui::Button::new("Cobalt")).clicked() && selected_atom.0.atom_type != AtomType::Cobalt {
            selected_atom.0 = Atom::cobalt();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Cobalt));
        }

        // Handling selection for Nickel
        if ui.add(egui::Button::new("Nickel")).clicked() && selected_atom.0.atom_type != AtomType::Nickel {
            selected_atom.0 = Atom::nickel();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Nickel));
        }

        // Handling selection for Copper
        if ui.add(egui::Button::new("Copper")).clicked() && selected_atom.0.atom_type != AtomType::Copper {
            selected_atom.0 = Atom::copper();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Copper));
        }

        // Handling selection for Zinc
        if ui.add(egui::Button::new("Zinc")).clicked() && selected_atom.0.atom_type != AtomType::Zinc {
            selected_atom.0 = Atom::zinc();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Zinc));
        }

        // Handling selection for Gallium
        if ui.add(egui::Button::new("Gallium")).clicked() && selected_atom.0.atom_type != AtomType::Gallium {
            selected_atom.0 = Atom::gallium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Gallium));
        }

        // Handling selection for Germanium
        if ui.add(egui::Button::new("Germanium")).clicked() && selected_atom.0.atom_type != AtomType::Germanium {
            selected_atom.0 = Atom::germanium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Germanium));
        }

        // Handling selection for Arsenic
        if ui.add(egui::Button::new("Arsenic")).clicked() && selected_atom.0.atom_type != AtomType::Arsenic {
            selected_atom.0 = Atom::arsenic();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Arsenic));
        }

        // Handling selection for Selenium
        if ui.add(egui::Button::new("Selenium")).clicked() && selected_atom.0.atom_type != AtomType::Selenium {
            selected_atom.0 = Atom::selenium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Selenium));
        }

        // Handling selection for Bromine
        if ui.add(egui::Button::new("Bromine")).clicked() && selected_atom.0.atom_type != AtomType::Bromine {
            selected_atom.0 = Atom::bromine();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Bromine));
        }

        // Handling selection for Krypton
        if ui.add(egui::Button::new("Krypton")).clicked() && selected_atom.0.atom_type != AtomType::Krypton {
            selected_atom.0 = Atom::krypton();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Krypton));
        }

        // Handling selection for Rubidium
        if ui.add(egui::Button::new("Rubidium")).clicked() && selected_atom.0.atom_type != AtomType::Rubidium {
            selected_atom.0 = Atom::rubidium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Rubidium));
        }

        // Handling selection for Strontium
        if ui.add(egui::Button::new("Strontium")).clicked() && selected_atom.0.atom_type != AtomType::Strontium {
            selected_atom.0 = Atom::strontium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Strontium));
        }

        // Handling selection for Yttrium
        if ui.add(egui::Button::new("Yttrium")).clicked() && selected_atom.0.atom_type != AtomType::Yttrium {
            selected_atom.0 = Atom::yttrium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Yttrium));
        }

        // Handling selection for Zirconium
        if ui.add(egui::Button::new("Zirconium")).clicked() && selected_atom.0.atom_type != AtomType::Zirconium {
            selected_atom.0 = Atom::zirconium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Zirconium));
        }

        // Handling selection for Niobium
        if ui.add(egui::Button::new("Niobium")).clicked() && selected_atom.0.atom_type != AtomType::Niobium {
            selected_atom.0 = Atom::niobium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Niobium));
        }

        // Handling selection for Molybdenum
        if ui.add(egui::Button::new("Molybdenum")).clicked() && selected_atom.0.atom_type != AtomType::Molybdenum {
            selected_atom.0 = Atom::molybdenum();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Molybdenum));
        }

        // Handling selection for Technetium
        if ui.add(egui::Button::new("Technetium")).clicked() && selected_atom.0.atom_type != AtomType::Technetium {
            selected_atom.0 = Atom::technetium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Technetium));
        }

        // Handling selection for Ruthenium
        if ui.add(egui::Button::new("Ruthenium")).clicked() && selected_atom.0.atom_type != AtomType::Ruthenium {
            selected_atom.0 = Atom::ruthenium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Ruthenium));
        }

        // Handling selection for Rhodium
        if ui.add(egui::Button::new("Rhodium")).clicked() && selected_atom.0.atom_type != AtomType::Rhodium {
            selected_atom.0 = Atom::rhodium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Rhodium));
        }

        // Handling selection for Palladium
        if ui.add(egui::Button::new("Palladium")).clicked() && selected_atom.0.atom_type != AtomType::Palladium {
            selected_atom.0 = Atom::palladium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Palladium));
        }

        // Handling selection for Silver
        if ui.add(egui::Button::new("Silver")).clicked() && selected_atom.0.atom_type != AtomType::Silver {
            selected_atom.0 = Atom::silver();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Silver));
        }

        // Handling selection for Cadmium
        if ui.add(egui::Button::new("Cadmium")).clicked() && selected_atom.0.atom_type != AtomType::Cadmium {
            selected_atom.0 = Atom::cadmium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Cadmium));
        }

        // Handling selection for Indium
        if ui.add(egui::Button::new("Indium")).clicked() && selected_atom.0.atom_type != AtomType::Indium {
            selected_atom.0 = Atom::indium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Indium));
        }

        // Handling selection for Tin
        if ui.add(egui::Button::new("Tin")).clicked() && selected_atom.0.atom_type != AtomType::Tin {
            selected_atom.0 = Atom::tin();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Tin));
        }

        // Handling selection for Antimony
        if ui.add(egui::Button::new("Antimony")).clicked() && selected_atom.0.atom_type != AtomType::Antimony {
            selected_atom.0 = Atom::antimony();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Antimony));
        }

        // Handling selection for Tellurium
        if ui.add(egui::Button::new("Tellurium")).clicked() && selected_atom.0.atom_type != AtomType::Tellurium {
            selected_atom.0 = Atom::tellurium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Tellurium));
        }

        // Handling selection for Iodine
        if ui.add(egui::Button::new("Iodine")).clicked() && selected_atom.0.atom_type != AtomType::Iodine {
            selected_atom.0 = Atom::iodine();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Iodine));
        }

        // Handling selection for Xenon
        if ui.add(egui::Button::new("Xenon")).clicked() && selected_atom.0.atom_type != AtomType::Xenon {
            selected_atom.0 = Atom::xenon();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Xenon));
        }

        // Handling selection for Cesium
        if ui.add(egui::Button::new("Cesium")).clicked() && selected_atom.0.atom_type != AtomType::Cesium {
            selected_atom.0 = Atom::cesium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Cesium));
        }

        // Handling selection for Barium
        if ui.add(egui::Button::new("Barium")).clicked() && selected_atom.0.atom_type != AtomType::Barium {
            selected_atom.0 = Atom::barium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Barium));
        }

        // Handling selection for Lanthanum
        if ui.add(egui::Button::new("Lanthanum")).clicked() && selected_atom.0.atom_type != AtomType::Lanthanum {
            selected_atom.0 = Atom::lanthanum();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Lanthanum));
        }

        // Handling selection for Cerium
        if ui.add(egui::Button::new("Cerium")).clicked() && selected_atom.0.atom_type != AtomType::Cerium {
            selected_atom.0 = Atom::cerium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Cerium));
        }

        // Handling selection for Praseodymium
        if ui.add(egui::Button::new("Praseodymium")).clicked() && selected_atom.0.atom_type != AtomType::Praseodymium {
            selected_atom.0 = Atom::praseodymium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Praseodymium));
        }

        // Handling selection for Neodymium
        if ui.add(egui::Button::new("Neodymium")).clicked() && selected_atom.0.atom_type != AtomType::Neodymium {
            selected_atom.0 = Atom::neodymium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Neodymium));
        }

        // Handling selection for Promethium
        if ui.add(egui::Button::new("Promethium")).clicked() && selected_atom.0.atom_type != AtomType::Promethium {
            selected_atom.0 = Atom::promethium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Promethium));
        }

        // Handling selection for Samarium
        if ui.add(egui::Button::new("Samarium")).clicked() && selected_atom.0.atom_type != AtomType::Samarium {
            selected_atom.0 = Atom::samarium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Samarium));
        }

        // Handling selection for Europium
        if ui.add(egui::Button::new("Europium")).clicked() && selected_atom.0.atom_type != AtomType::Europium {
            selected_atom.0 = Atom::europium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Europium));
        }

        // Handling selection for Gadolinium
        if ui.add(egui::Button::new("Gadolinium")).clicked() && selected_atom.0.atom_type != AtomType::Gadolinium {
            selected_atom.0 = Atom::gadolinium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Gadolinium));
        }

        // Handling selection for Terbium
        if ui.add(egui::Button::new("Terbium")).clicked() && selected_atom.0.atom_type != AtomType::Terbium {
            selected_atom.0 = Atom::terbium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Terbium));
        }

        // Handling selection for Dysprosium
        if ui.add(egui::Button::new("Dysprosium")).clicked() && selected_atom.0.atom_type != AtomType::Dysprosium {
            selected_atom.0 = Atom::dysprosium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Dysprosium));
        }

        // Handling selection for Holmium
        if ui.add(egui::Button::new("Holmium")).clicked() && selected_atom.0.atom_type != AtomType::Holmium {
            selected_atom.0 = Atom::holmium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Holmium));
        }

        // Handling selection for Erbium
        if ui.add(egui::Button::new("Erbium")).clicked() && selected_atom.0.atom_type != AtomType::Erbium {
            selected_atom.0 = Atom::erbium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Erbium));
        }

        // Handling selection for Thulium
        if ui.add(egui::Button::new("Thulium")).clicked() && selected_atom.0.atom_type != AtomType::Thulium {
            selected_atom.0 = Atom::thulium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Thulium));
        }

        // Handling selection for Ytterbium
        if ui.add(egui::Button::new("Ytterbium")).clicked() && selected_atom.0.atom_type != AtomType::Ytterbium {
            selected_atom.0 = Atom::ytterbium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Ytterbium));
        }

        // Handling selection for Lutetium
        if ui.add(egui::Button::new("Lutetium")).clicked() && selected_atom.0.atom_type != AtomType::Lutetium {
            selected_atom.0 = Atom::lutetium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Lutetium));
        }

        // Handling selection for Hafnium
        if ui.add(egui::Button::new("Hafnium")).clicked() && selected_atom.0.atom_type != AtomType::Hafnium {
            selected_atom.0 = Atom::hafnium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Hafnium));
        }

        // Handling selection for Tantalum
        if ui.add(egui::Button::new("Tantalum")).clicked() && selected_atom.0.atom_type != AtomType::Tantalum {
            selected_atom.0 = Atom::tantalum();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Tantalum));
        }

        // Handling selection for Tungsten
        if ui.add(egui::Button::new("Tungsten")).clicked() && selected_atom.0.atom_type != AtomType::Tungsten {
            selected_atom.0 = Atom::tungsten();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Tungsten));
        }

        // Handling selection for Rhenium
        if ui.add(egui::Button::new("Rhenium")).clicked() && selected_atom.0.atom_type != AtomType::Rhenium {
            selected_atom.0 = Atom::rhenium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Rhenium));
        }

        // Handling selection for Osmium
        if ui.add(egui::Button::new("Osmium")).clicked() && selected_atom.0.atom_type != AtomType::Osmium {
            selected_atom.0 = Atom::osmium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Osmium));
        }

        // Handling selection for Iridium
        if ui.add(egui::Button::new("Iridium")).clicked() && selected_atom.0.atom_type != AtomType::Iridium {
            selected_atom.0 = Atom::iridium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Iridium));
        }

        // Handling selection for Platinum
        if ui.add(egui::Button::new("Platinum")).clicked() && selected_atom.0.atom_type != AtomType::Platinum {
            selected_atom.0 = Atom::platinum();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Platinum));
        }

        // Handling selection for Gold
        if ui.add(egui::Button::new("Gold")).clicked() && selected_atom.0.atom_type != AtomType::Gold {
            selected_atom.0 = Atom::gold();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Gold));
        }

        // Handling selection for Mercury
        if ui.add(egui::Button::new("Mercury")).clicked() && selected_atom.0.atom_type != AtomType::Mercury {
            selected_atom.0 = Atom::mercury();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Mercury));
        }

        // Handling selection for Thallium
        if ui.add(egui::Button::new("Thallium")).clicked() && selected_atom.0.atom_type != AtomType::Thallium {
            selected_atom.0 = Atom::thallium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Thallium));
        }

        // Handling selection for Lead
        if ui.add(egui::Button::new("Lead")).clicked() && selected_atom.0.atom_type != AtomType::Lead {
            selected_atom.0 = Atom::lead();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Lead));
        }

        // Handling selection for Bismuth
        if ui.add(egui::Button::new("Bismuth")).clicked() && selected_atom.0.atom_type != AtomType::Bismuth {
            selected_atom.0 = Atom::bismuth();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Bismuth));
        }

        // Handling selection for Polonium
        if ui.add(egui::Button::new("Polonium")).clicked() && selected_atom.0.atom_type != AtomType::Polonium {
            selected_atom.0 = Atom::polonium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Polonium));
        }

        // Handling selection for Astatine
        if ui.add(egui::Button::new("Astatine")).clicked() && selected_atom.0.atom_type != AtomType::Astatine {
            selected_atom.0 = Atom::astatine();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Astatine));
        }

        // Handling selection for Radon
        if ui.add(egui::Button::new("Radon")).clicked() && selected_atom.0.atom_type != AtomType::Radon {
            selected_atom.0 = Atom::radon();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Radon));
        }

        // Handling selection for Francium
        if ui.add(egui::Button::new("Francium")).clicked() && selected_atom.0.atom_type != AtomType::Francium {
            selected_atom.0 = Atom::francium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Francium));
        }

        // Handling selection for Radium
        if ui.add(egui::Button::new("Radium")).clicked() && selected_atom.0.atom_type != AtomType::Radium {
            selected_atom.0 = Atom::radium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Radium));
        }

        // Handling selection for Actinium
        if ui.add(egui::Button::new("Actinium")).clicked() && selected_atom.0.atom_type != AtomType::Actinium {
            selected_atom.0 = Atom::actinium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Actinium));
        }

        // Handling selection for Thorium
        if ui.add(egui::Button::new("Thorium")).clicked() && selected_atom.0.atom_type != AtomType::Thorium {
            selected_atom.0 = Atom::thorium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Thorium));
        }

        // Handling selection for Protactinium
        if ui.add(egui::Button::new("Protactinium")).clicked() && selected_atom.0.atom_type != AtomType::Protactinium {
            selected_atom.0 = Atom::protactinium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Protactinium));
        }

        // Handling selection for Uranium
        if ui.add(egui::Button::new("Uranium")).clicked() && selected_atom.0.atom_type != AtomType::Uranium {
            selected_atom.0 = Atom::uranium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Uranium));
        }

        // Handling selection for Neptunium
        if ui.add(egui::Button::new("Neptunium")).clicked() && selected_atom.0.atom_type != AtomType::Neptunium {
            selected_atom.0 = Atom::neptunium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Neptunium));
        }

        // Handling selection for Plutonium
        if ui.add(egui::Button::new("Plutonium")).clicked() && selected_atom.0.atom_type != AtomType::Plutonium {
            selected_atom.0 = Atom::plutonium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Plutonium));
        }

        // Handling selection for Americium
        if ui.add(egui::Button::new("Americium")).clicked() && selected_atom.0.atom_type != AtomType::Americium {
            selected_atom.0 = Atom::americium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Americium));
        }

        // Handling selection for Curium
        if ui.add(egui::Button::new("Curium")).clicked() && selected_atom.0.atom_type != AtomType::Curium {
            selected_atom.0 = Atom::curium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Curium));
        }

        // Handling selection for Berkelium
        if ui.add(egui::Button::new("Berkelium")).clicked() && selected_atom.0.atom_type != AtomType::Berkelium {
            selected_atom.0 = Atom::berkelium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Berkelium));
        }

        // Handling selection for Californium
        if ui.add(egui::Button::new("Californium")).clicked() && selected_atom.0.atom_type != AtomType::Californium {
            selected_atom.0 = Atom::californium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Californium));
        }

        // Handling selection for Einsteinium
        if ui.add(egui::Button::new("Einsteinium")).clicked() && selected_atom.0.atom_type != AtomType::Einsteinium {
            selected_atom.0 = Atom::einsteinium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Einsteinium));
        }

        // Handling selection for Fermium
        if ui.add(egui::Button::new("Fermium")).clicked() && selected_atom.0.atom_type != AtomType::Fermium {
            selected_atom.0 = Atom::fermium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Fermium));
        }

        // Handling selection for Mendelevium
        if ui.add(egui::Button::new("Mendelevium")).clicked() && selected_atom.0.atom_type != AtomType::Mendelevium {
            selected_atom.0 = Atom::mendelevium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Mendelevium));
        }

        // Handling selection for Nobelium
        if ui.add(egui::Button::new("Nobelium")).clicked() && selected_atom.0.atom_type != AtomType::Nobelium {
            selected_atom.0 = Atom::nobelium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Nobelium));
        }

        // Handling selection for Lawrencium
        if ui.add(egui::Button::new("Lawrencium")).clicked() && selected_atom.0.atom_type != AtomType::Lawrencium {
            selected_atom.0 = Atom::lawrencium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Lawrencium));
        }

        // Handling selection for Rutherfordium
        if ui.add(egui::Button::new("Rutherfordium")).clicked() && selected_atom.0.atom_type != AtomType::Rutherfordium {
            selected_atom.0 = Atom::rutherfordium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Rutherfordium));
        }

        // Handling selection for Dubnium
        if ui.add(egui::Button::new("Dubnium")).clicked() && selected_atom.0.atom_type != AtomType::Dubnium {
            selected_atom.0 = Atom::dubnium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Dubnium));
        }

        // Handling selection for Seaborgium
        if ui.add(egui::Button::new("Seaborgium")).clicked() && selected_atom.0.atom_type != AtomType::Seaborgium {
            selected_atom.0 = Atom::seaborgium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Seaborgium));
        }

        // Handling selection for Bohrium
        if ui.add(egui::Button::new("Bohrium")).clicked() && selected_atom.0.atom_type != AtomType::Bohrium {
            selected_atom.0 = Atom::bohrium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Bohrium));
        }

        // Handling selection for Hassium
        if ui.add(egui::Button::new("Hassium")).clicked() && selected_atom.0.atom_type != AtomType::Hassium {
            selected_atom.0 = Atom::hassium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Hassium));
        }

        // Handling selection for Meitnerium
        if ui.add(egui::Button::new("Meitnerium")).clicked() && selected_atom.0.atom_type != AtomType::Meitnerium {
            selected_atom.0 = Atom::meitnerium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Meitnerium));
        }

        // Handling selection for Darmstadtium
        if ui.add(egui::Button::new("Darmstadtium")).clicked() && selected_atom.0.atom_type != AtomType::Darmstadtium {
            selected_atom.0 = Atom::darmstadtium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Darmstadtium));
        }

        // Handling selection for Roentgenium
        if ui.add(egui::Button::new("Roentgenium")).clicked() && selected_atom.0.atom_type != AtomType::Roentgenium {
            selected_atom.0 = Atom::roentgenium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Roentgenium));
        }

        // Handling selection for Copernicium
        if ui.add(egui::Button::new("Copernicium")).clicked() && selected_atom.0.atom_type != AtomType::Copernicium {
            selected_atom.0 = Atom::copernicium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Copernicium));
        }

        // Handling selection for Nihonium
        if ui.add(egui::Button::new("Nihonium")).clicked() && selected_atom.0.atom_type != AtomType::Nihonium {
            selected_atom.0 = Atom::nihonium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Nihonium));
        }

        // Handling selection for Flerovium
        if ui.add(egui::Button::new("Flerovium")).clicked() && selected_atom.0.atom_type != AtomType::Flerovium {
            selected_atom.0 = Atom::flerovium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Flerovium));
        }

        // Handling selection for Moscovium
        if ui.add(egui::Button::new("Moscovium")).clicked() && selected_atom.0.atom_type != AtomType::Moscovium {
            selected_atom.0 = Atom::moscovium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Moscovium));
        }

        // Handling selection for Livermorium
        if ui.add(egui::Button::new("Livermorium")).clicked() && selected_atom.0.atom_type != AtomType::Livermorium {
            selected_atom.0 = Atom::livermorium();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Livermorium));
        }

        // Handling selection for Tennessine
        if ui.add(egui::Button::new("Tennessine")).clicked() && selected_atom.0.atom_type != AtomType::Tennessine {
            selected_atom.0 = Atom::tennessine();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Tennessine));
        }

        // Handling selection for Oganesson
        if ui.add(egui::Button::new("Oganesson")).clicked() && selected_atom.0.atom_type != AtomType::Oganesson {
            selected_atom.0 = Atom::oganesson();
            ev_change_selected_atom.send(ChangeSelectedAtom(AtomType::Oganesson));
        }
    });
}

