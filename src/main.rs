fn bytes_to_binary(s: &[u8]) -> Vec<char> {
    s.into_iter()
        .flat_map(|x| {
            (0..8)
                .into_iter()
                .rev()
                .map(|i| char::from(48 + ((*x >> i) & 1)))
        })
        .collect()
}

fn binary_to_bytes(s: &[char]) -> Vec<u8> {
    s.chunks(8)
        .map(|x| {
            let x: String = x.into_iter().collect();
            u8::from_str_radix(&x, 2).unwrap()
        })
        .collect()
}

#[rustfmt::skip]
fn pc1<T: Copy>(s: &[T]) -> (Vec<T>, Vec<T>) {
    let c0 = vec![
        s[56], s[48], s[40], s[32], s[24], s[16], s[8],
        s[0], s[57], s[49], s[41], s[33], s[25], s[17],
        s[9], s[1], s[58], s[50], s[42], s[34], s[26],
        s[18], s[10], s[2], s[59], s[51], s[43], s[35]
    ];
    let d0 = vec![
        s[62], s[54], s[46], s[38], s[30], s[22], s[14],
        s[6], s[61], s[53], s[45], s[37], s[29], s[21],
        s[13], s[5], s[60], s[52], s[44], s[36], s[28],
        s[20], s[12], s[4], s[27], s[19], s[11], s[3]
    ];
    (c0, d0)
}

#[rustfmt::skip]
fn pc2(s: Vec<char>) -> Vec<char> {
    vec![
        s[13], s[16], s[10], s[23], s[0], s[4], s[2], s[27],
		s[14], s[5], s[20], s[9], s[22], s[18], s[11], s[3],
		s[25], s[7], s[15], s[6], s[26], s[19], s[12], s[1],
		s[40], s[51], s[30], s[36], s[46], s[54], s[29], s[39],
		s[50], s[44], s[32], s[47], s[43], s[48], s[38], s[55],
		s[33], s[52], s[45], s[41], s[49], s[35], s[28], s[31]
    ]
}

const SHIFTS: [usize; 16] = [1, 1, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1];

fn generate_keys(s: &str) -> Vec<Vec<char>> {
    let initial_key = bytes_to_binary(s.as_bytes());
    let (mut c, mut d) = pc1(&initial_key);
    let mut keys = Vec::with_capacity(16);

    for i in 0..16 {
        c.rotate_left(SHIFTS[i]);
        d.rotate_left(SHIFTS[i]);
        let key = c.iter().chain(d.iter()).cloned().collect();

        keys.push(pc2(key));
    }

    return keys;
}

#[rustfmt::skip]
fn ip<T: Copy>(s: &[T]) -> Vec<T> {
	vec![
        s[57], s[49], s[41], s[33], s[25], s[17], s[9], s[1],
		s[59], s[51], s[43], s[35], s[27], s[19], s[11], s[3],
		s[61], s[53], s[45], s[37], s[29], s[21], s[13], s[5],
		s[63], s[55], s[47], s[39], s[31], s[23], s[15], s[7],
		s[56], s[48], s[40], s[32], s[24], s[16], s[8], s[0],
		s[58], s[50], s[42], s[34], s[26], s[18], s[10], s[2],
		s[60], s[52], s[44], s[36], s[28], s[20], s[12], s[4],
		s[62], s[54], s[46], s[38], s[30], s[22], s[14], s[6],
    ]
}

#[rustfmt::skip]
fn ipl1<T: Copy>(s: &[T]) -> Vec<T> {
    vec![s[39], s[7], s[47], s[15], s[55], s[23], s[63], s[31],
		s[38], s[6], s[46], s[14], s[54], s[22], s[62], s[30],
		s[37], s[5], s[45], s[13], s[53], s[21], s[61], s[29],
		s[36], s[4], s[44], s[12], s[52], s[20], s[60], s[28],
		s[35], s[3], s[43], s[11], s[51], s[19], s[59], s[27],
		s[34], s[2], s[42], s[10], s[50], s[18], s[58], s[26],
		s[33], s[1], s[41], s[9], s[49], s[17], s[57], s[25],
		s[32], s[0], s[40], s[8], s[48], s[16], s[56], s[24],
    ]
}

#[rustfmt::skip]
fn expansion<T: Copy>(s: &[T]) -> Vec<T> {
	vec![s[31], s[0], s[1], s[2], s[3], s[4], s[3], s[4],
		s[5], s[6], s[7], s[8], s[7], s[8], s[9], s[10],
		s[11], s[12], s[11], s[12], s[13], s[14], s[15], s[16],
		s[15], s[16], s[17], s[18], s[19], s[20], s[19], s[20],
		s[21], s[22], s[23], s[24], s[23], s[24], s[25], s[26],
		s[27], s[28], s[27], s[28], s[29], s[30], s[31], s[0]]
}

#[rustfmt::skip]
const SBLOCKS : [[[u8; 4]; 16]; 8] = [
[
    [14, 0, 4, 15],
    [4, 15, 1, 12],
    [13, 7, 14, 8],
    [1, 4, 8, 2],
	[2, 14, 13, 4],
    [15, 2, 6, 9],
    [11, 13, 2, 1],
    [8, 1, 11, 7],
	[3, 10, 15, 5],
    [10, 6, 12, 11],
    [6, 12, 9, 3],
    [12, 11, 7, 14],
	[5, 9, 3, 10],
    [9, 5, 10, 0],
    [0, 3, 5, 6],
    [7, 8, 0, 13]
],
[
    [15, 3, 0, 13],
    [1, 13, 14, 8],
    [8, 4, 7, 10],
    [14, 7, 11, 1],
    [6, 15, 10, 3],
    [11, 2, 4, 15],
    [3, 8, 13, 4],
    [4, 14, 1, 2],
	[9, 12, 5, 11],
    [7, 0, 8, 6],
    [2, 1, 12, 7],
    [13, 10, 6, 12],
    [12, 6, 9, 0],
    [0, 9, 3, 5],
    [5, 11, 2, 14],
    [10, 5, 15, 9]
],
[
    [10, 13, 13, 1],
    [0, 7, 6, 10],
    [9, 0, 4, 13],
    [14, 9, 9, 0],
    [6, 3, 8, 6],
    [3, 4, 15, 9],
    [15, 6, 3, 8],
    [5, 10, 0, 7],
	[1, 2, 11, 4],
    [13, 8, 1, 15],
    [12, 5, 2, 14],
    [7, 14, 12, 3],
    [11, 12, 5, 11],
    [4, 11, 10, 5],
    [2, 15, 14, 2],
    [8, 1, 7, 12]
],
[
    [7, 13, 10, 3],
    [13, 8, 6, 15],
    [14, 11, 9, 0],
    [3, 5, 0, 6],
    [0, 6, 12, 10],
    [6, 15, 11, 1],
    [9, 0, 7, 13],
    [10, 3, 13, 8],
	[1, 4, 15, 9],
    [2, 7, 1, 4],
    [8, 2, 3, 5],
    [5, 12, 14, 11],
    [11, 1, 5, 12],
    [12, 10, 2, 7],
    [4, 14, 8, 2],
    [15, 9, 4, 14]
],
[
    [2, 14, 4, 11],
    [12, 11, 2, 8],
    [4, 2, 1, 12],
    [1, 12, 11, 7],
    [7, 4, 10, 1],
    [10, 7, 13, 14],
    [11, 13, 7, 2],
    [6, 1, 8, 13],
	[8, 5, 15, 6],
    [5, 0, 9, 15],
    [3, 15, 12, 0],
    [15, 10, 5, 9],
    [13, 3, 6, 10],
    [0, 9, 3, 4],
    [14, 8, 0, 5],
    [9, 6, 14, 3]
],
[
    [12, 10, 9, 4],
    [1, 15, 14, 3],
    [10, 4, 15, 2],
    [15, 2, 5, 12],
    [9, 7, 2, 9],
    [2, 12, 8, 5],
    [6, 9, 12, 15],
    [8, 5, 3, 10],
	[0, 6, 7, 11],
    [13, 1, 0, 14],
    [3, 13, 4, 1],
    [4, 14, 10, 7],
    [14, 0, 1, 6],
    [7, 11, 13, 0],
    [5, 3, 11, 8],
    [11, 8, 6, 13]
],
[
    [4, 13, 1, 6],
    [11, 0, 4, 11],
    [2, 11, 11, 13],
    [14, 7, 13, 8],
    [15, 4, 12, 1],
    [0, 9, 3, 4],
    [8, 1, 7, 10],
    [13, 10, 14, 7],
	[3, 14, 10, 9],
    [12, 3, 15, 5],
    [9, 5, 6, 0],
    [7, 12, 8, 15],
    [5, 2, 0, 14],
    [10, 15, 5, 2],
    [6, 8, 9, 3],
    [1, 6, 2, 12]
],
[
    [13, 1, 7, 2],
    [2, 15, 11, 1],
    [8, 13, 4, 14],
    [4, 8, 1, 7],
    [6, 10, 9, 4],
    [15, 3, 12, 10],
    [11, 7, 14, 8],
    [1, 4, 2, 13],
	[10, 12, 0, 15],
    [9, 5, 6, 12],
    [3, 6, 10, 9],
    [14, 11, 13, 0],
    [5, 0, 15, 3],
    [0, 14, 3, 5],
    [12, 9, 5, 6],
    [7, 2, 8, 11]
],
];

fn substitution(s: &[char]) -> Vec<char> {
    let mut res = Vec::with_capacity(32);

    for (i, chunk) in s.chunks(6).enumerate() {
        let x: String = chunk[1..5].into_iter().collect();
        let x = usize::from_str_radix(&x, 2).unwrap();

        let y: String = format!("{}{}", chunk[0], chunk[5]);
        let y = usize::from_str_radix(&y, 2).unwrap();

        let r = SBLOCKS[i][x][y];

        res.extend(format!("{r:04b}").chars());
    }

    return res;
}

#[rustfmt::skip]
fn permutation_p<T: Copy>(s: &[T]) -> Vec<T> {
	vec![s[15], s[6], s[19], s[20], s[28], s[11], s[27], s[16],
		s[0], s[14], s[22], s[25], s[4], s[17], s[30], s[9],
		s[1], s[7], s[23], s[13], s[31], s[26], s[2], s[8],
		s[18], s[12], s[29], s[5], s[21], s[10], s[3], s[24]]
}

fn rounds(binary_ip: &[char], keys: &[Vec<char>], decrypt: bool) -> (Vec<char>, Vec<char>) {
    let mut left_block: Vec<char> = binary_ip[..32].into_iter().cloned().collect();
    let mut right_block: Vec<char> = binary_ip[32..].into_iter().cloned().collect();

    for i in 0..16 {
        let right_block_expanded: String = expansion(&right_block).into_iter().collect();
        let right_block_expanded = u64::from_str_radix(&right_block_expanded, 2).unwrap();

        let key = if decrypt { &keys[15 - i] } else { &keys[i] };
        let key: String = key.into_iter().collect();
        let key = u64::from_str_radix(&key, 2).unwrap();

        let tmp = right_block_expanded ^ key;
        let tmp_string = format!("{tmp:048b}");

        let right_block_expanded: Vec<_> = tmp_string.chars().into_iter().collect();
        let tmp = substitution(&right_block_expanded);
        let tmp = permutation_p(&tmp);
        let tmp: String = tmp.into_iter().collect();
        let tmp = u64::from_str_radix(&tmp, 2).unwrap();

        let left_block_s: String = left_block.into_iter().collect();
        let left_block_u = u64::from_str_radix(&left_block_s, 2).unwrap();

        let tmp = tmp ^ left_block_u;
        let tmp = format!("{tmp:032b}");

        left_block = right_block;
        right_block = tmp.chars().into_iter().collect();
    }

    (left_block, right_block)
}

fn encrypt(data: &[u8], keys: &[Vec<char>]) -> Vec<u8> {
    let chunks = data.chunks(8);
    let mut res = Vec::new();
    let mut buffer = vec![0u8; 8];

    for chunk in chunks {
        buffer[..chunk.len()].copy_from_slice(chunk);
        if chunk.len() < buffer.len() {
            buffer[chunk.len()..].fill(46);
        }
        let message = bytes_to_binary(&buffer);
        let binary_ip = ip(&message);
        let (l, r) = rounds(&binary_ip, keys, false);
        let lr: Vec<_> = r.into_iter().chain(l.into_iter()).collect();
        let lr = ipl1(&lr);
        res.extend(binary_to_bytes(&lr));
    }

    res
}

fn decrypt(data: &[u8], keys: &[Vec<char>]) -> Vec<u8> {
    let mut res = Vec::new();

    for chunk in data.chunks(8) {
        let chunk: Vec<char> = bytes_to_binary(chunk);
        let chunk = ip(&chunk);
        let (l, r) = rounds(&chunk, keys, true);
        let lr: Vec<_> = r.into_iter().chain(l.into_iter()).collect();
        let lr = ipl1(&lr);
        res.extend(binary_to_bytes(&lr));
    }

    res
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let keys = generate_keys(&args[1]);
    let inf = &args[2];
    let outf = &args[3];

    let input = std::fs::read(inf).unwrap();

    // let data: String = encrypt(&input, &keys).into_iter().collect();
    let data = decrypt(&input, &keys);

    std::fs::write(outf, &data).unwrap();
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let message = "hello world!\n".as_bytes();
        let key = "12345678";
        let keys = generate_keys(key);
        let enc_data = encrypt(message, &keys);
        let dec_data = decrypt(&enc_data, &keys);
        assert_eq!(&dec_data, "hello world!\n...".as_bytes());
    }
}
