use std::char;
use std::string::ToString;
use std::ops::Deref;
use std::convert::From;
use indexmap::IndexMap;

const RADIX: u32 = 10;

#[derive(Debug, Clone)]
pub struct MFHash(Vec<(char, u32)>, usize);

impl MFHash {
    pub fn new(source: &str, k: usize) -> Self {
        let len = source.len();
        let scores = source
            .chars()
            .into_iter()
            .fold(IndexMap::<char, u32>::with_capacity(len), |mut hmap, char| {
                *hmap.entry(char).or_insert(0) += 1; hmap
            });

        let mut vals = scores.into_iter().collect::<Vec<(char, u32)>>();
        vals.sort_by(|a, b| { b.1.cmp(&a.1) });
        let hash: Vec<(char, u32)> = vals.into_iter().take(k).collect();

        Self(hash, k)
    }
}

impl From<&str> for MFHash {
    fn from(string: &str) -> Self {
        // Check value validity

        let len = string.len();
        let tmp: Vec<char> = string
            .chars()
            .into_iter()
            .collect();

        let val = &tmp
            .as_slice()
            .chunks(2)
            .map(|chunk| {
                (chunk[0], chunk[1].to_digit(RADIX).unwrap())
            })
            .collect::<Vec<(char, u32)>>();

        Self(val.clone().to_owned(), len / 2)
    }
 }

impl Deref for MFHash{
    type Target = Vec<(char, u32)>;

    fn deref(&self) -> &Vec<(char, u32)> {
        &self.0
    }
}


impl ToString for MFHash {
    fn to_string(&self) -> String {
        self.0.clone()
            .into_iter()
            .fold(Vec::with_capacity(self.1), |mut vec, val| {
                let string = [char::from_digit(val.1, 10).unwrap(), val.0]
                    .into_iter()
                    .collect::<String>();

                vec.push(val.0);
                vec.push(char::from_digit(val.1, 10).unwrap());
                vec
            })
            .iter()
            .map(|c| *c)
            .collect::<String>()
    }
}


// Tests for NFHash

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn mfhash_test() {
        let string: &str = "research";
        let hash: String = MFHash::new(string, 2).to_string();
        assert_eq!(hash, "r2e2");

        let string1 = "significant";
        let hash2: String = MFHash::new(string1, 2).to_string();
        assert_eq!(hash2, "i3n2");

        let string2 = "capabilities";
        let hash2: String = MFHash::new(string2, 2).to_string();
        assert_eq!(hash2, "i3a2");

        let string3 = "aaaaabbbb";
        let hash3: String = MFHash::new(string3, 2).to_string();
        assert_eq!(hash3, "a5b4");

        let string4 = "a";
        let hash3: String = MFHash::new(string4, 2).to_string();
        assert_eq!(hash3, "a1");
    }

    #[test]
    fn mfhash_test_fasta() {
        let string: &str = "LCLYTHIGRNIYYGSYLYSETWNTGIMLLLITMATAFMGYVLPWGQMSFWGATVITNLFSAIPYIGTNLV";
        let hash: String = MFHash::new(string, 2).to_string();
        assert_eq!(hash, "L9T8");

        let string2: &str = "EWIWGGFSVDKATLNRFFAFHFILPFTMVALAGVHLTFLHETGSNNPLGLTSDSDKIPFHPYYTIKDFLG";
        let hash2: String = MFHash::new(string2, 2).to_string();
        assert_eq!(hash2, "F9L8");
    }


    #[bench]
    fn hashing_bench(b: &mut Bencher) {
        b.iter(|| {
            MFHash::new("research", 2)
        })
    }

    #[bench]
    fn hashing_bench_fasta(b: &mut Bencher) {
        b.iter(|| {
            MFHash::new("LCLYTHIGRNIYYGSYLYSETWNTGIMLLLITMATAFMGYVLPWGQMSFWGATVITNLFSAIPYIGTNLV", 2)
        })
    }

    #[bench]
    fn to_string_bench(b: &mut Bencher) {
        let hash: MFHash = MFHash::new("research", 2);
        b.iter(|| {
            hash.to_string()
        })
    }
}
