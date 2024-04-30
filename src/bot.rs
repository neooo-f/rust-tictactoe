use rand::Rng;

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
    let free_indexes = check_free_indexes(&fields);

    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..free_indexes.len());

    fields[random_index] = 2;
}