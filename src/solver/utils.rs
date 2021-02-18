pub(super) fn get_transform(
    tubes: &Vec<u8>,
    height: usize,
    tube_count: usize,
) -> (Vec<usize>, Vec<u8>) {
    let mut transform: Vec<usize> = (0..tube_count).collect();
    transform.sort_unstable_by_key(|index| &tubes[index * height..(index + 1) * height]);
    let mut sorted_tubes = vec![0; tube_count * height];
    for i in 0..tube_count {
        sorted_tubes[i * height..(i + 1) * height]
            .clone_from_slice(&tubes[transform[i] * height..(transform[i] + 1) * height])
    }
    (transform, sorted_tubes)
}

pub(super) fn is_solved(state: &Vec<u8>, height: usize) -> bool {
    state.chunks_exact(height).all(|tube| {
        if tube[0] == 0 {
            return true;
        }
        for i in 1..height {
            if tube[i] != tube[0] {
                return false;
            }
        }
        true
    })
}

pub(super) struct TubeStats {
    pub(super) size: usize,
    pub(super) color_height: usize,
    pub(super) color: u8,
    pub(super) simple: bool,
}

pub(super) fn get_tube_stat(tube: &[u8], height: usize) -> TubeStats {
    if tube[0] == 0 {
        return TubeStats {
            size: 0,
            color_height: 0,
            color: 0,
            simple: false,
        };
    }
    let mut size = height;
    while size > 0 && tube[size - 1] == 0 {
        size -= 1;
    }
    let mut color_height = 1;
    let color = tube[size - 1];
    while color_height < size && tube[size - color_height - 1] == tube[size - 1] {
        color_height += 1;
    }
    TubeStats {
        size,
        color_height,
        color,
        simple: color_height == size,
    }
}

pub(super) fn pour(
    state: &mut Vec<u8>,
    height: usize,
    tube_stats: &Vec<TubeStats>,
    from: usize,
    to: usize,
    amount: usize,
) {
    let from_offset = from * height + tube_stats[from].size;
    let to_offset = to * height + tube_stats[to].size;
    state[from_offset - amount..from_offset].fill(0);
    state[to_offset..to_offset + amount].fill(tube_stats[from].color);
}

pub(super) fn pour_back(state: &mut Vec<u8>, height: usize, from: usize, to: usize, amount: usize) {
    let from_stat = get_tube_stat(&state[from * height..(from + 1) * height], height);
    let to_stat = get_tube_stat(&state[to * height..(to + 1) * height], height);
    let from_offset = from * height + from_stat.size;
    let to_offset = to * height + to_stat.size;
    state[from_offset..from_offset + amount].fill(to_stat.color);
    state[to_offset - amount..to_offset].fill(0);
}
