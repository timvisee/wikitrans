extern crate skim;
extern crate wikipedia;

use std::io::Cursor;

use skim::{Skim, SkimOptions};
use wikipedia::{http::default::Client, Wikipedia};

type WikiClient = Wikipedia<Client>;

fn main() {
    // The Wikipedia client
    let mut client = WikiClient::default();
    client.links_results = "max".into();

    // Fetch all available languages
    let langs = client.get_languages().expect("failed to get languages");

    // Select the search language
    let search_lang = select_lang(&langs, "Search language: ").unwrap();
    client.language = search_lang.into();

    // Search for the term, select the page
    let titles = client.search("rust").expect("search failed");
    let title = show_select(titles, "Select term: ").expect("failed to select page title");
    let page = client.page_from_title(title.clone());

    // Collect all available translations
    let langlinks = page
        .get_langlinks()
        .expect("failed to fetch langlinks")
        .collect::<Vec<_>>();
    let langlinks_tags = langlinks.iter().map(|l| &l.lang).collect::<Vec<_>>();
    let langlinks_names = langlinks
        .iter()
        .map(|l| l.title.clone().unwrap_or("".into()))
        .collect::<Vec<String>>();

    // Filter the target lang list
    let target_langs = langs
        .clone()
        .into_iter()
        .filter(|l| langlinks_tags.contains(&&l.0))
        .collect::<Vec<_>>();

    // Show an error if no translations are available
    if langlinks.is_empty() {
        println!("No translations available for: {}", title);
        return;
    }

    // Select the filtered target language and target language link
    let target_lang =
        select_lang_with(&target_langs, Some(&langlinks_names), "Translate to: ").unwrap();
    let target_langlink = langlinks
        .iter()
        .filter(|l| l.lang == target_lang)
        .next()
        .expect("failed to select find langlink");

    // Report the result
    println!("{}", target_langlink.title.clone().unwrap_or("".into()));
}

/// Let the user select a language
///
/// From the given list of `languages` (which may be fetched through `Wikipedia::get_languages()`),
/// a user selects their preferred language.
/// The tag of the selected language is returned. If nothing was selected, `None` is returned.
fn select_lang(languages: &Vec<(String, String)>, prompt: &str) -> Option<String> {
    select_lang_with(languages, None, prompt)
}

/// Let the user select a language
///
/// From the given list of `languages` (which may be fetched through `Wikipedia::get_languages()`),
/// a user selects their preferred language.
/// If a `with` list is given, it is zipped together with the list of languages, and shown after
/// each language item in the interactive selection view.
/// The tag of the selected language is returned. If nothing was selected, `None` is returned.
fn select_lang_with(
    languages: &Vec<(String, String)>,
    with: Option<&Vec<String>>,
    prompt: &str,
) -> Option<String> {
    // Fetch the list of languages
    let langs = if let Some(with) = with {
        languages
            .into_iter()
            .zip(with)
            .map(|(lang, with)| format!("{} - {}: {}", lang.0, lang.1, with))
            .collect::<Vec<String>>()
    } else {
        languages
            .into_iter()
            .map(|lang| format!("{} - {}", lang.0, lang.1))
            .collect::<Vec<String>>()
    };

    // Select the language
    show_select(langs, prompt).map(|l| l.split(" - ").next().unwrap().to_owned())
}

/// Show an interactive selection view for the given list of `items`.
/// The selected item is returned.  If no item is selected, `None` is returned instead.
fn show_select(items: Vec<String>, prompt: &str) -> Option<String> {
    // Configure the skim options
    let options = SkimOptions::default().prompt(prompt).height("50%");

    // Build the items string
    let items: String = items.join("\n");

    // Show the skim select view
    let selected = Skim::run_with(&options, Some(Box::new(Cursor::new(items))))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    // Get the first selected, and return
    selected
        .iter()
        .next()
        .map(|i| i.get_output_text().to_string())
}
