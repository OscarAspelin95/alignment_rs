use dioxus::prelude::*;

use crate::utils::local_alignment;

/// NOTE - we can rewrite this to return a String
/// to make the matching a bit more streamlined.
fn get_id<'a>(c: char) -> &'a str {
    match c {
        'A' => "aligned-char adenosine",
        'C' => "aligned-char cytosine",
        'G' => "aligned-char guanine",
        'T' => "aligned-char thymine",
        'a' | 'c' | 'g' | 't' => "aligned-char softmask",
        _ => "aligned-char",
    }
}

#[component]
pub fn Alignment() -> Element {
    // We need signals and use effects for rows and columns
    let mut query: Signal<String> = use_signal(|| "".to_string());
    let mut subject: Signal<String> = use_signal(|| "".to_string());

    let mut aligned_query: Signal<String> = use_signal(|| "".to_string());
    let mut aligned_subject: Signal<String> = use_signal(|| "".to_string());
    let mut aligned_matches: Signal<String> = use_signal(|| "".to_string());

    use_effect(move || {
        let q = query.read();
        let s = subject.read();

        if q.len() > 0 && s.len() > 0 {
            let (aln_query, aln_matches, aln_subject) = local_alignment(q.as_str(), s.as_str());

            aligned_query.set(aln_query);
            aligned_matches.set(aln_matches);
            aligned_subject.set(aln_subject);
        }
    });

    rsx! {
        h1 { "Local alignment visualizer" }

        div { id: "some-container",

            div { id: "form-container",
                form {
                    label { r#for: "query-input", "Query:" }
                    input {
                        r#type: "text",
                        name: "query-input",
                        placeholder: "ATCG...",
                        maxlength: "80",
                        oninput: move |evt| {
                            let v = evt.value();
                            v.chars()
                                .all(|c| {
                                    match c {
                                        'A' | 'C' | 'G' | 'T' => true,
                                        _ => false,
                                    }
                                })
                                .then(|| query.set(v));
                        },
                    }
                    label { r#for: "subject-input", "Subject:" }
                    input {
                        r#type: "text",
                        name: "subject-input",
                        placeholder: "ATCG...",
                        maxlength: "80",
                        oninput: move |evt| {
                            let v = evt.value();
                            v.chars()
                                .all(|c| {
                                    match c {
                                        'A' | 'C' | 'G' | 'T' => true,
                                        _ => false,
                                    }
                                })
                                .then(|| subject.set(v));
                        },
                    }
                }
            } // END of input container


            // Here, we also need to clear input values.
            button {
                onclick: move |_| {
                    query.set("".to_string());
                    subject.set("".to_string());
                    aligned_query.set("".to_string());
                    aligned_subject.set("".to_string());
                    aligned_matches.set("".to_string());
                },
                "Clear"
            }


        }








        div { id: "mega-align",
            div { id: "aligned-segment",
                for c in aligned_query.read().chars() {

                    div { class: get_id(c), "{c}" }
                }
            }


            div { id: "aligned-segment",
                for c in aligned_matches.read().chars() {
                    div { class: get_id(c), "{c}" }
                }
            }

            div { id: "aligned-segment",
                for c in aligned_subject.read().chars() {
                    div { class: get_id(c), "{c}" }
                }
            }
        }
    } // END of RSX
}
