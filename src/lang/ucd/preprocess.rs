use super::{UcdScript, BY_NAME};
use ::core::cmp::Ordering;
use debug_unsafe::slice::SliceGetter;

const fn char_ranges_count() -> usize {
    let mut i = 0;
    let mut cnt = 0;
    while i < BY_NAME.len() {
        cnt += BY_NAME[i].1.len();
        i += 1;
    }

    cnt
}
const LEN: usize = char_ranges_count();

#[derive(Clone, Copy, Debug)]
pub(super) struct RangeScript {
    range_start: char,
    range_end: char,
    script: UcdScript,
}
const RANGE_SCRIPT_DEFAULT: RangeScript = RangeScript {
    range_start: char::MAX,
    range_end: char::MAX,
    script: unsafe { ::core::mem::transmute::<u8, UcdScript>(0) },
};

/* #[const_trait]
trait ConstDefault {
    fn default<const RUNTIME: bool>() -> Self;
}
impl const ConstDefault for RangeScript {
    fn default() -> Self {
        RangeScript {
            range_start: char::MAX,
            range_end: char::MAX,
            script: Script::Latin,
        }
    }
} */

const fn char_ranges_array_sorted() -> [RangeScript; LEN] {
    let mut res: [RangeScript; LEN] = [RANGE_SCRIPT_DEFAULT; LEN];

    // foreach BY_NAME
    let mut i = 0;
    while i < BY_NAME.len() {
        let (script, ranges) = BY_NAME[i];
        // foreach charset
        let mut j = 0;
        while j < ranges.len() {
            let range = ranges[j];
            // looking for insertion
            let mut ins = 0;
            while ins < LEN {
                let mut prev = res[ins];
                if range.0 < prev.range_start {
                    res[ins] = RangeScript {
                        range_start: range.0,
                        range_end: range.1,
                        script,
                    };
                    if prev.range_start == char::MAX {
                        break;
                    }
                    // shifts all elements right
                    let mut next_ins = ins + 1;
                    while next_ins < LEN {
                        let next = res[next_ins];
                        res[next_ins] = prev;
                        if next.range_start == char::MAX {
                            break;
                        }
                        prev = next;
                        next_ins += 1;
                    }
                    break;
                }
                ins += 1;
            }
            j += 1;
        }
        i += 1;
    }

    res
}
pub(super) const CHAR_RANGES_SORTED: [RangeScript; LEN] = char_ranges_array_sorted();

/* #[test]
fn print_char_ranges_sorted() {
    panic!("{:?}", CHAR_RANGES_SORTED);
} */

/* const fn char_ranges_sorted_merged_count() -> usize {
    let mut i = 0;
    let mut cnt = 1;
    while i < CHAR_RANGES_SORTED.len() - 1 {
        let curr = CHAR_RANGES_SORTED[i];
        let next = CHAR_RANGES_SORTED[i + 1];
        if curr.script.into_code() != next.script.into_code()
            || (curr.range_end as u32) + 1 < next.range_start as u32
        {
            cnt += 1;
        }
        i += 1;
    }

    cnt
}
const LEN_NEW: usize = char_ranges_sorted_merged_count();

const fn char_ranges_array_sorted_merged() -> [RangeScript; LEN_NEW] {
    let mut res: [RangeScript; LEN_NEW] = [RANGE_SCRIPT_DEFAULT; LEN_NEW];

    let mut i = 1;
    let mut res_i = 0;
    let mut prev = CHAR_RANGES_SORTED[0];
    while i < CHAR_RANGES_SORTED.len() {
        let next = CHAR_RANGES_SORTED[i];
        if prev.script.into_code() != next.script.into_code()
            || (prev.range_end as u32) + 1 < next.range_start as u32
        {
            res[res_i] = prev;
            prev = next;
            res_i += 1;
        } else {
            prev.range_end = next.range_end;
        }
        i += 1;
    }
    res[res_i] = prev;

    res
}
pub(super) const CHAR_RANGES_SORTED_MERGED: [RangeScript; LEN_NEW] = char_ranges_array_sorted_merged(); */

#[inline(always)]
fn compare(ra: &RangeScript, ch: char) -> Ordering {
    if ch < ra.range_start {
        Ordering::Greater
    } else if ch > ra.range_end {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

pub(super) trait BSearch<T> {
    fn binary_search_by_char(&self, ch: char) -> UcdScript;
}
// better than native `binary_search_by` because of `match` with an early exit,
// which gives better optimization with branches instead of `cmovae`.
impl BSearch<RangeScript> for [RangeScript] {
    #[inline(always)]
    fn binary_search_by_char(&self, ch: char) -> UcdScript {
        let mut size = self.len();
        let mut base = 0_usize;

        while size > 1 {
            let half = size >> 1;
            let mid = base + half;

            let res = self.get_safe_unchecked(mid);
            let cmp = compare(res, ch);

            base = match cmp {
                Ordering::Less => mid,
                Ordering::Equal => return res.script,
                Ordering::Greater => base,
            };
            // base = ::core::hint::select_unpredictable(cmp == Ordering::Greater, base, mid);
            size -= half;
        }

        let res = self.get_safe_unchecked(base);
        let cmp = compare(res, ch);
        if cmp == Ordering::Equal {
            res.script
        } else {
            // Some unused `Common` ranges in `ucd` are commented out, so it defaults to `Common`.
            UcdScript::Common
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BSearch, RangeScript, UcdScript, CHAR_RANGES_SORTED};

    #[test]
    fn test_char_ranges_sorted() {
        let mut prev = *CHAR_RANGES_SORTED.first().unwrap();
        for range in CHAR_RANGES_SORTED.into_iter().skip(1) {
            assert_ne!(range.range_start, char::MAX, "range_start is default");
            assert!(
                range.range_start <= range.range_end,
                "range_start > range_end\n{range:?}"
            );
            assert!(
                prev.range_end < range.range_start,
                "prev.range_end >= range.range_start\nprev: {prev:?}\nrange: {range:?}"
            );
            prev = range;
        }
    }

    #[test]
    fn test_bsearch() {
        let input = [RangeScript {
            range_start: 'a',
            range_end: 'z',
            script: UcdScript::Latin,
        }];
        assert_eq!(input.binary_search_by_char('b'), UcdScript::Latin);

        let input = [
            RangeScript {
                range_start: 'a',
                range_end: 'b',
                script: UcdScript::Adlam,
            },
            RangeScript {
                range_start: 'c',
                range_end: 'd',
                script: UcdScript::Ahom,
            },
            RangeScript {
                range_start: 'e',
                range_end: 'f',
                script: UcdScript::AnatolianHieroglyphs,
            },
        ];
        assert_eq!(input.binary_search_by_char('a'), UcdScript::Adlam);
        assert_eq!(input.binary_search_by_char('d'), UcdScript::Ahom);
        assert_eq!(
            input.binary_search_by_char('e'),
            UcdScript::AnatolianHieroglyphs
        );
        assert_eq!(input.binary_search_by_char('x'), UcdScript::Common);

        let input = [
            RangeScript {
                range_start: 'a',
                range_end: 'b',
                script: UcdScript::Adlam,
            },
            RangeScript {
                range_start: 'c',
                range_end: 'd',
                script: UcdScript::Ahom,
            },
            RangeScript {
                range_start: 'e',
                range_end: 'f',
                script: UcdScript::AnatolianHieroglyphs,
            },
            RangeScript {
                range_start: 'g',
                range_end: 'h',
                script: UcdScript::Arabic,
            },
        ];
        assert_eq!(input.binary_search_by_char('a'), UcdScript::Adlam);
        assert_eq!(input.binary_search_by_char('b'), UcdScript::Adlam);
        assert_eq!(input.binary_search_by_char('c'), UcdScript::Ahom);
        assert_eq!(input.binary_search_by_char('d'), UcdScript::Ahom);
        assert_eq!(
            input.binary_search_by_char('e'),
            UcdScript::AnatolianHieroglyphs
        );
        assert_eq!(
            input.binary_search_by_char('f'),
            UcdScript::AnatolianHieroglyphs
        );
        assert_eq!(input.binary_search_by_char('g'), UcdScript::Arabic);
        assert_eq!(input.binary_search_by_char('h'), UcdScript::Arabic);
        assert_eq!(input.binary_search_by_char('x'), UcdScript::Common);

        let input = [
            RangeScript {
                range_start: 'a',
                range_end: 'b',
                script: UcdScript::Adlam,
            },
            RangeScript {
                range_start: 'c',
                range_end: 'd',
                script: UcdScript::Ahom,
            },
            RangeScript {
                range_start: 'e',
                range_end: 'f',
                script: UcdScript::AnatolianHieroglyphs,
            },
            RangeScript {
                range_start: 'g',
                range_end: 'h',
                script: UcdScript::Arabic,
            },
            RangeScript {
                range_start: 'i',
                range_end: 'j',
                script: UcdScript::Armenian,
            },
        ];
        assert_eq!(input.binary_search_by_char('a'), UcdScript::Adlam);
        assert_eq!(input.binary_search_by_char('b'), UcdScript::Adlam);
        assert_eq!(input.binary_search_by_char('c'), UcdScript::Ahom);
        assert_eq!(input.binary_search_by_char('d'), UcdScript::Ahom);
        assert_eq!(
            input.binary_search_by_char('e'),
            UcdScript::AnatolianHieroglyphs
        );
        assert_eq!(
            input.binary_search_by_char('f'),
            UcdScript::AnatolianHieroglyphs
        );
        assert_eq!(input.binary_search_by_char('g'), UcdScript::Arabic);
        assert_eq!(input.binary_search_by_char('h'), UcdScript::Arabic);
        assert_eq!(input.binary_search_by_char('i'), UcdScript::Armenian);
        assert_eq!(input.binary_search_by_char('j'), UcdScript::Armenian);
        assert_eq!(input.binary_search_by_char('x'), UcdScript::Common);

        assert_eq!(
            CHAR_RANGES_SORTED.binary_search_by_char('Ύ'),
            UcdScript::Greek
        );
        assert_eq!(
            CHAR_RANGES_SORTED.binary_search_by_char('Σ'),
            UcdScript::Greek
        );
        assert_eq!(
            CHAR_RANGES_SORTED.binary_search_by_char(char::MAX),
            UcdScript::Common
        );
    }
}
