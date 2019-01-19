mod mfhash;
mod similarity;

use self::mfhash::{ MFHash };
use self::similarity::similarity;

pub fn mkfs(string1: &str, string2: &str, k: usize, max_distance: Option<u32>) -> u32 {
    let distance = similarity(&MFHash::new(string1, k), &MFHash::new(string2, k));

    match max_distance {
        Some(limit) => limit - distance,
        None => distance
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn mkfs_test() {
        let string1 = "significant";
        let string2 = "capabilities";
        let similarity: u32 = mkfs(string1, string2, 2, Some(10));
        assert_eq!(similarity, 7);

        let string3 = "research";
        let string4 = "seeking";
        let similarity: u32 = mkfs(string3, string4, 2, None);
        assert_eq!(similarity, 2);

        let string5 = "research";
        let string6 = "research";
        let similarity: u32 = mkfs(string5, string6, 2, Some(10));
        assert_eq!(similarity, 6);

        let string7 = "nicht";
        let string8 = "nacht";
        let similarity: u32 = mkfs(string7, string8, 2, Some(10));
        assert_eq!(similarity, 9);

        let string9 = "my";
        let string10 = "a";
        let similarity: u32 = mkfs(string9, string10, 2, Some(10));
        assert_eq!(similarity, 10);

    }

    #[test]
    fn mkfs_test_fasta() {
        let string: &str = "LCLYTHIGRNIYYGSYLYSETWNTGIMLLLITMATAFMGYVLPWGQMSFWGATVITNLFSAIPYIGTNLV";
        let string2: &str = "EWIWGGFSVDKATLNRFFAFHFILPFTMVALAGVHLTFLHETGSNNPLGLTSDSDKIPFHPYYTIKDFLG";
        let similarity: u32 = mkfs(string, string2, 2, Some(100));
        assert_eq!(similarity, 83);
    }

    #[bench]
    fn mkfs_bench(b: &mut Bencher)  {
        b.iter(|| {
            mkfs("significant", "capabilities", 2, Some(10))
        })
    }

    #[bench]
    fn mkfs_benc_fasta(b: &mut Bencher)  {
        b.iter(|| {
            mkfs(
                "LCLYTHIGRNIYYGSYLYSETWNTGIMLLLITMATAFMGYVLPWGQMSFWGATVITNLFSAIPYIGTNLV",
                "EWIWGGFSVDKATLNRFFAFHFILPFTMVALAGVHLTFLHETGSNNPLGLTSDSDKIPFHPYYTIKDFLG",
                2,
                Some(100))
        })
    }
}
