use std::collections::{ BTreeMap };
use super::mfhash::{ MFHash };

const RADIX: u32 = 10;

pub fn similarity(hash1: &MFHash, hash2: &MFHash) -> u32 {
    let map: BTreeMap<char, u32> = (*hash2)
        .iter()
        .fold(BTreeMap::<char, u32>::new(), |mut hmap, (k, v)| {
            hmap.entry(*k).or_insert(*v); hmap
        });

    (*hash1)
        .iter()
        .fold(0u32, |cnt, (ch, val)| {
            match map.contains_key(&ch) {
                true => {
                    let freq1 = val;
                    let freq2 = map.get(&ch).unwrap();
                    if freq1 == freq2 {
                        cnt + freq1
                    } else {
                        cnt + freq1 + freq2
                    }
                }
                _ => cnt
            }
        })
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn similarity_test() {
        let hash1: MFHash = MFHash::from("n1i1");
        let hash2: MFHash = MFHash::from("n1a1");

        let result: u32 = similarity(&hash1, &hash2);
        assert_eq!(result, 1);

        let hash3: MFHash = MFHash::from("L9T8");
        let hash4: MFHash = MFHash::from("F9L8");
        let result2 = similarity(&hash3, &hash4);
        assert_eq!(result2, 17);

        let hash5: MFHash = MFHash::from("r2e2");
        let hash6: MFHash = MFHash::from("r2e2");

        let result: u32 = similarity(&hash5, &hash6);
        assert_eq!(result, 4);

        let hash7: MFHash = MFHash::from("r2e2");
        let hash8: MFHash = MFHash::from("e2s1");

        let result: u32 = similarity(&hash7, &hash8);
        assert_eq!(result, 2);
    }

    #[bench]
    fn similarity_bench(b: &mut Bencher)  {
        let hash1 = MFHash::from("i3n2");
        let hash2 = MFHash::from("i3g2");
        b.iter(|| {
            similarity(&hash1, &hash2)
        })
    }
}
