fn calc_rollup_size(included: usize, queued: usize) -> usize {
    match () {
        _ if queued >= 16 && included % 16 == 0 => 16,
        _ if queued >= 8 && included % 8 == 0 => 8,
        _ if queued >= 4 && included % 4 == 0 => 4,
        _ if queued >= 2 && included % 2 == 0 => 2,
        _ => 1,
    }
}
pub fn calc_rollup_size_array(included: usize, queued: usize) -> Vec<usize> {
    let mut rollup_array = Vec::new();
    let mut included_count = included;
    let mut queued_count = queued;
    let mut rollup_size = 0;

    loop {
        let new_rollup_size = calc_rollup_size(included_count, queued_count);
        if new_rollup_size == rollup_size {
            break;
        }

        rollup_size = new_rollup_size;
        rollup_array.push(rollup_size);

        if rollup_size >= 16 || queued_count < 2 * rollup_size {
            break;
        }

        queued_count -= rollup_size;
        included_count += rollup_size;
    }

    rollup_array
}
