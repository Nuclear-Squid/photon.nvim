// Some useless import to show what they look like
use std::string::String;
use std::vec::Vec;

#[derive(Debug)]
struct Catalog {
    entries: Vec<(String, String)>,
}

impl Catalog {
    fn show(&self) {
        for (theme_name, description) in self.entries.iter() {
            println!("{theme_name} is cool : {description}");
        }
    }

    // Ok, this might be a bit mean on my end ^^'
    fn keep_themes_with_great_semantic_coloring(&mut self) {
        let mut buffer = Vec::new();

        for (theme_name, description) in self.entries.iter() {
            if theme_name == "photon.nvim" {
                buffer.push((theme_name.clone(), description.clone()));
            }
        }

        self.entries = buffer;
    }
}

fn main() {
    let mut themes_i_like = Catalog {
        entries: [
            ("photon.nvim",  "The one you’re seeing right now!"),
            ("oneokai.nvim", "The one I forked to make photon"),
            ("duskfox",      "The one I took the bg and fg colors from"),
            ("gruvbox",      "Is just plain cool, I reused some of it’s ideas"),
            ("kanagawa",     "see gruvbox"),
            ("everforest",   "Is pretty unique and deserves a shoutout!"),
        ].map(|(s1, s2)| (s1.into(), s2.into()))
         .to_vec(),
    };

    Catalog::show(&themes_i_like);

    themes_i_like.show();
    themes_i_like.keep_themes_with_great_semantic_coloring();
    assert_eq!(
        themes_i_like.entries,
        vec![("photon.nvim".into(), "The one you’re seeing right now!".into())]
    );
}
