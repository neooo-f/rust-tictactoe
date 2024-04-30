use rand::seq::SliceRandom;

fn check_free_indexes(fields: &[u8]) -> Vec<usize> {
    let mut free_indexes: Vec<usize> = vec![];

    for i in 0..fields.len() {
        if fields[i] == 0 {
            free_indexes.push(i);
        }
    }

    return free_indexes;
}

pub fn make_move(fields: &mut [u8]) {
    let mut free_indexes = check_free_indexes(&fields);
    let mut rng = rand::thread_rng();

    if let Some(random_index) = free_indexes.choose_mut(&mut rng) {
        fields[*random_index] = 2; // uses unwrap to extract the value from the Option
    } else {
        println!("No free indexes available!");
    }
}

pub fn check_winner(fields: &[u8]) -> bool {
    // check horizontal rows
    for i in (0..9).step_by(3) {
        if fields[i] == fields[i + 1] && fields[i + 1] == fields[i + 2] && (fields[i] == 1 || fields[i] == 2) {
            return true;
        }
    }

    // check vertical rows
    for i in 0..3 {
        if fields[i] == fields[i + 3] && fields[i + 3] == fields[i + 6] && (fields[i] == 1 || fields[i] == 2) {
            return true;
        }
    }

    // check diagonal rows
    if fields[0] == fields[4] && fields[4] == fields[8] && (fields[0] == 1 || fields[0] == 2) {
        return true;
    }
    if fields[2] == fields[4] && fields[4] == fields[6] && (fields[2] == 1 || fields[2] == 2) {
        return true;
    }

    false
}