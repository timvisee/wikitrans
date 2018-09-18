#[macro_use]
extern crate clap;
extern crate skim;
extern crate wikipedia;

use std::io::Cursor;

use clap::{App, Arg, ArgMatches};
use skim::{Skim, SkimOptions};
use wikipedia::{http::default::Client, Wikipedia};

/// The Wikipedia client being used.
type WikiClient = Wikipedia<Client>;

/// The main application entrypoint.
fn main() {
    // Get the clap matches
    let matches = build_app().get_matches();

    // Build the wiki client
    let mut client = build_wiki_client();

    // Fetch all available Wikipedia languages
    let langs = client.get_languages().expect("failed to get languages");

    // Run the translation logic, obtain the result and report
    let result = wikitrans(&matches, &mut client, &langs);
    println!("{}", result.unwrap_or("".into()));
}

/// Build the clap app definition.
fn build_app<'a>() -> App<'a, 'a> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("TERM")
                .help("The term to search and translate")
                .required(true)
                .multiple(true)
                .index(1),
        ).arg(
            Arg::with_name("language")
                .long("language")
                .short("l")
                .visible_alias("search")
                .alias("lang")
                .takes_value(true)
                .help("The search language tag"),
        ).arg(
            Arg::with_name("translate")
                .long("translate")
                .short("t")
                .alias("trans")
                .takes_value(true)
                .help("The translate language tag"),
        )
}

/// Build a WikiClient.
fn build_wiki_client() -> WikiClient {
    let mut client = WikiClient::default();
    client.links_results = "max".into();
    client
}

/// Run the wikitrans logic
fn wikitrans(
    matches: &ArgMatches,
    client: &mut WikiClient,
    langs: &Vec<(String, String)>,
) -> Option<String> {
    // Select the search language
    let search_lang = select_lang(&langs, matches.value_of("language"), "Search language: ")
        .expect("failed to select search language");
    let original_lang = client.language.clone();
    client.language = search_lang.into();

    // Get the search term and search for page titles
    let term = matches
        .values_of("TERM")
        .unwrap()
        .fold(String::new(), |a, b| a + " " + b);
    let titles = client
        .search(&term)
        .expect("failed to search for specified term");

    // Interactively select the proper title and get the page
    // TODO: do not show interactive select if none or one items are found
    let title = select(titles, "Select term: ").expect("failed to select page title");
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

    // Revert changed client properties
    client.language = original_lang;

    // Show an error if no translations are available
    if langlinks.is_empty() {
        println!("No translations available for: {}", title);
        return None;
    }

    // Select the filtered target language and target language link
    let target_lang = select_lang_with(
        &target_langs,
        Some(&langlinks_names),
        matches.value_of("translate"),
        "Translate to: ",
    ).unwrap();
    let target_langlink = langlinks
        .iter()
        .filter(|l| l.lang == target_lang)
        .next()
        .expect("failed to select find langlink");

    // Report the result
    target_langlink.title.clone()
}

/// Let the user select a language
///
/// From the given list of `langs` (which may be fetched through `Wikipedia::get_languages()`),
/// a user selects their preferred language.
/// A language tag `pref` may be given, to automatically select the language.
/// The tag of the selected language is returned. If nothing was selected, `None` is returned.
fn select_lang(langs: &Vec<(String, String)>, pref: Option<&str>, prompt: &str) -> Option<String> {
    select_lang_with(langs, None, pref, prompt)
}

/// Let the user select a language
///
/// From the given list of `langs` (which may be fetched through `Wikipedia::get_languages()`),
/// a user selects their preferred language.
/// If a `with` list is given, it is zipped together with the list of languages, and shown after
/// each language item in the interactive selection view.
/// A language tag `pref` may be given, to automatically select the language.
/// The tag of the selected language is returned. If nothing was selected, `None` is returned.
fn select_lang_with(
    langs: &Vec<(String, String)>,
    with: Option<&Vec<String>>,
    pref: Option<&str>,
    prompt: &str,
) -> Option<String> {
    // Attempt to select the language based on the preference
    if let Some(preference) = pref {
        if let Some((tag, _)) = langs.iter().filter(|l| l.0 == preference).next() {
            return Some(tag.to_owned());
        }

        // If it could not be selected automatically show an error
        eprintln!("Unknown preference language: {}", preference);
    }

    // Build a list of selectable language items
    let items = if let Some(with) = with {
        langs
            .into_iter()
            .zip(with)
            .map(|(lang, with)| format!("{} ({}): {}", lang.0, lang.1, with))
            .collect::<Vec<String>>()
    } else {
        langs
            .into_iter()
            .map(|lang| format!("{} ({})", lang.0, lang.1))
            .collect::<Vec<String>>()
    };

    // Show an interactive language selection view
    select(items, prompt).map(|l| l.split(" (").next().unwrap().to_owned())
}

/// Show an interactive selection view for the given list of `items`.
/// The selected item is returned.  If no item is selected, `None` is returned instead.
fn select(items: Vec<String>, prompt: &str) -> Option<String> {
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
