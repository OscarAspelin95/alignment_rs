use dioxus::{document::eval, prelude::*};

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
    // Input from the user.
    let mut query: Signal<String> = use_signal(|| "".to_string());
    let mut subject: Signal<String> = use_signal(|| "".to_string());

    // Check if input is valid.
    let mut valid_query: Signal<bool> = use_signal(|| true);
    let mut valid_subject: Signal<bool> = use_signal(|| true);

    // Returned from the alignment function.
    let mut aligned_query: Signal<String> = use_signal(|| "".to_string());
    let mut aligned_subject: Signal<String> = use_signal(|| "".to_string());
    let mut aligned_matches: Signal<String> = use_signal(|| "".to_string());

    let reset = move || async move {
        [
            query,
            subject,
            aligned_query,
            aligned_subject,
            aligned_matches,
        ]
        .into_iter()
        .for_each(|mut string_signal| {
            string_signal.set("".to_string());
        });

        // Reset query validation.
        valid_query.set(true);
        eval(&format!(
            r#"document.getElementById('query-input').value = ''"#
        ));

        // Reset subject validation.
        valid_subject.set(true);
        eval(&format!(
            r#"document.getElementById('subject-input').value = ''"#
        ));
    };

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

        div { id: "form-container",
            form {


                div { id: "some-container",

                    div { id: "form-sequence",
                        label { r#for: "query-input", "Query:" }
                        input {
                            r#type: "text",
                            name: "query-input",
                            id: "query-input",
                            placeholder: "ATCG...",
                            maxlength: "80",
                            oninput: move |evt| {
                                let v = evt.value().to_uppercase();
                                v.chars()
                                    .all(|c| { matches!(c, 'A' | 'C' | 'G' | 'T') })
                                    .then(|| {
                                        query.set(v.to_uppercase());
                                        valid_query.set(true);
                                    })
                                    .unwrap_or_else(|| {
                                        valid_query.set(false);
                                    });
                            },
                        }
                    }

                    div { id: "form-sequence",
                        label { r#for: "subject-input", "Subject:" }
                        input {
                            r#type: "text",
                            id: "subject-input",
                            name: "subject-input",
                            placeholder: "ATCG...",
                            maxlength: "80",
                            oninput: move |evt| {
                                let v = evt.value().to_uppercase();
                                v.chars()
                                    .all(|c| { matches!(c, 'A' | 'C' | 'G' | 'T') })
                                    .then(|| {
                                        subject.set(v);
                                        valid_subject.set(true);
                                    })
                                    .unwrap_or_else(|| {
                                        valid_subject.set(false);
                                    });
                            },
                        }
                    }

                }
            } // END of input container

            // We should move this to a separate function.
            button {
                id: "clear-btn",
                onclick: move |_| async move { reset().await },
                "Clear"
            }
        }

        if !*valid_query.read() | !*valid_subject.read() {
            span { id: "invalid-input-message", "Only canonical bases [ATCGatcg] allowed." }
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
