use std::collections::HashMap;

#[derive(Clone, Copy)]
enum AlignmentType {
    Match,
    Mismatch,
    DeletionQuery,
    DeletionSubject,
}

/// We could modify these if we want.
fn get_alignment_cost(aln: AlignmentType) -> i32 {
    match aln {
        AlignmentType::Match => 1,
        AlignmentType::Mismatch => -1,
        AlignmentType::DeletionQuery => -1,
        AlignmentType::DeletionSubject => -1,
    }
}

fn levenshtein_distance(
    s1: &str,
    s2: &str,
) -> (
    Vec<Vec<i32>>,
    HashMap<(usize, usize), ((usize, usize), AlignmentType)>,
    (usize, usize),
) {
    // We take the number of rows from the subject.
    let m = s2.len();

    // We take the number of columns from the query.
    let n = s1.len();

    assert!(m > 0);
    assert!(n > 0);

    // Store array as a vector of vectors.
    let mut array: Vec<Vec<i32>> = Vec::new();

    // Initialize array.
    for _ in 0..m + 1 {
        array.push(vec![0; n + 1]);
    }

    assert!(array[0].len() == s1.len() + 1);
    assert!(array.len() == s2.len() + 1);

    let mut max_score: (i32, (usize, usize)) = (0, (0, 0));

    // We store the origin of each element in the array.
    let mut traceback: HashMap<(usize, usize), ((usize, usize), AlignmentType)> = HashMap::new();

    // We move in the i direction (down), subject is consumed and query is deleted.
    for i in 1..m + 1 {
        array[i][0] = 0;
        // Remeber to add trace.
        traceback.insert((i, 0), ((i - 1, 0), AlignmentType::DeletionQuery));
    }
    // We move in the j direction (right), query is consumed and subject is deleted.
    for j in 1..n + 1 {
        array[0][j] = 0;
        // Remember to add trace.
        traceback.insert((0, j), ((0, j - 1), AlignmentType::DeletionSubject));
    }

    for i in 1..m + 1 {
        for j in 1..n + 1 {
            // For a diagonal move, we need to check if we have a match or mismatch.
            let match_or_mismatch = match s1.chars().nth(j - 1) == s2.chars().nth(i - 1) {
                true => (
                    (i - 1, j - 1),
                    std::cmp::max(
                        0,
                        array[i - 1][j - 1] + get_alignment_cost(AlignmentType::Match),
                    ),
                    AlignmentType::Match,
                ),
                false => (
                    (i - 1, j - 1),
                    std::cmp::max(
                        0,
                        array[i - 1][j - 1] + get_alignment_cost(AlignmentType::Mismatch),
                    ),
                    AlignmentType::Mismatch,
                ),
            };

            // We have moved in the j direction so query is consumed and subject is deleted
            let deletion_subject = (
                (i, j - 1),
                std::cmp::max(
                    0,
                    array[i][j - 1] + get_alignment_cost(AlignmentType::DeletionSubject),
                ),
                AlignmentType::DeletionSubject,
            );

            // We have moved in the j direction so subject is consumed and query is deleted
            let deletion_query = (
                (i - 1, j),
                std::cmp::max(
                    0,
                    array[i - 1][j] + get_alignment_cost(AlignmentType::DeletionQuery),
                ),
                AlignmentType::DeletionQuery,
            );

            // EDIT ME! Try switching the order of the
            // elements and see if this changes the traceback.
            let previous_values: Vec<((usize, usize), i32, AlignmentType)> =
                vec![match_or_mismatch, deletion_query, deletion_subject];

            let (previous_index, previous_value, alignment_type) =
                previous_values.iter().max_by_key(|x| x.1).unwrap();

            // Add trace for current element.
            traceback.insert((i, j), (*previous_index, *alignment_type));

            // Update array for current value.
            array[i][j] = *previous_value;

            // Update max array value and its index
            max_score = *vec![(array[i][j], (i, j)), max_score]
                .iter()
                .max_by_key(|x| x.0)
                .unwrap();
        }
    }

    return (array, traceback, max_score.1);
}

fn to_lowercase(nt: char) -> char {
    match nt {
        'A' => 'a',
        'C' => 'c',
        'G' => 'g',
        'T' => 't',
        _ => panic!(),
    }
}
fn get_traceback(
    array: &Vec<Vec<i32>>,
    traceback: HashMap<(usize, usize), ((usize, usize), AlignmentType)>,
    max_index: (usize, usize),
    s1: &str,
    s2: &str,
) -> (String, String, String) {
    let (mut m, mut n) = max_index;

    // Aligned part of s1 and s2 (including deletions).
    let mut s1_aln: Vec<char> = Vec::new();
    let mut s2_aln: Vec<char> = Vec::new();

    // We'll use "|" for match, "*" for mismatch and " " for deletion.
    let mut matches_aln: Vec<char> = Vec::new();

    let mut m_c = m.clone();
    let mut n_c = n.clone();

    // Fill the left unaligned, we do this first because we iterate the alignment backwards.
    while m_c <= s2.len() - 1 || n_c <= s1.len() - 1 {
        match s1.chars().nth(n_c) {
            // We are still within s1, so we push the soft masked base.
            Some(nt) => s1_aln.push(to_lowercase(nt)),
            // We have reached the end of s1, so we push a placeholder.
            // Must be empty, otherwise the end of the alignment looks weird.
            None => s1_aln.push('\0'),
        }

        match s2.chars().nth(m_c) {
            // We are still within s2, so we push the soft masked base.
            Some(nt) => s2_aln.push(to_lowercase(nt)),
            // We have reached the end of s2, so we push a placeholder.
            // Must be empty, otherwise the end of the alignment looks weird.
            None => s2_aln.push('\0'),
        }

        matches_aln.push(' ');
        m_c += 1;
        n_c += 1;
    }

    s1_aln.reverse();
    s2_aln.reverse();

    loop {
        if array[m][n] == 0 {
            break;
        }

        let ((i, j), aln_type) = traceback.get(&(m, n)).unwrap();

        match aln_type {
            AlignmentType::Match => {
                let s1_char = s1.chars().nth(*j).unwrap();
                let s2_char = s2.chars().nth(*i).unwrap();

                s1_aln.push(s1_char);
                s2_aln.push(s2_char);
                matches_aln.push('|');
            }
            AlignmentType::Mismatch => {
                let s1_char = s1.chars().nth(*j).unwrap();
                let s2_char = s2.chars().nth(*i).unwrap();
                s1_aln.push(s1_char);
                s2_aln.push(s2_char);
                matches_aln.push('*');
            }
            AlignmentType::DeletionQuery => {
                s1_aln.push('-');
                s2_aln.push(s2.chars().nth(*i).unwrap());
                matches_aln.push(' ');
            }
            AlignmentType::DeletionSubject => {
                s1_aln.push(s1.chars().nth(*j).unwrap());
                s2_aln.push('-');
                matches_aln.push(' ');
            }
        }
        m = *i;
        n = *j;
    }

    // Fill the right unaligned part, we do this last because we iterate the alignment backwards.
    let mut m = m as i32;
    let mut n = n as i32;

    // We iterate until we have reached the end of both s1 and s2.
    while m >= 1 || n >= 1 {
        // We are still within s1, so we push the soft masked base.
        if n >= 1 {
            match s1.chars().nth((n - 1) as usize) {
                Some(nt) => s1_aln.push(to_lowercase(nt)),
                None => panic!("Position {n} is invalid."),
            }
        }
        // We have reached the end of s1, so we push a placeholder.
        else {
            s1_aln.push(' ');
        }

        // We are still within s2, so we push the soft masked base.
        if m >= 1 {
            match s2.chars().nth((m - 1) as usize) {
                Some(nt) => s2_aln.push(to_lowercase(nt)),
                None => panic!("Position {m} is invalid."),
            }
        }
        // We have reached the end of s2, so we push a placeholder.
        else {
            s2_aln.push(' ');
        }

        matches_aln.push(' ');
        m -= 1;
        n -= 1;
    }

    let s1_aln_fwd: String = s1_aln.iter().rev().collect();
    let s2_aln_fwd: String = s2_aln.iter().rev().collect();
    let matches_aln_fwd: String = matches_aln.iter().rev().collect();

    return (s1_aln_fwd, matches_aln_fwd, s2_aln_fwd);
}

pub fn local_alignment(query: &str, subject: &str) -> (String, String, String) {
    let (array, traceback, max_index) = levenshtein_distance(query, subject);

    return get_traceback(&array, traceback, max_index, query, subject);
}
