#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![allow(non_snake_case)]

use std::cmp;

const NONE: u8 = 0;
const REMOVED: u8 = 1;
const COMMON: u8 = 2;
const ADDED: u8 = 3;

#[derive(Debug)]
struct Ctx<'a, T: 'a> {
    k: isize,
    base: isize,
    A: &'a [T],
    B: &'a [T],
    ptr: &'a mut usize,
    routes: &'a mut [usize],
    diff_types: &'a mut [u8],
}

#[derive(Debug, PartialEq)]
pub enum DiffResult {
    Removed(DiffElement),
    Common(DiffElement),
    Added(DiffElement),
}

#[derive(Debug, PartialEq)]
pub struct DiffElement {
    pub old_index: Option<usize>,
    pub new_index: Option<usize>,
}

#[derive(Clone)]
struct FarthestPoint {
    y: isize,
    id: usize,
}

fn back_trace<T: PartialEq + Clone>(
    A: &[T],
    B: &[T],
    current: &FarthestPoint,
    swapped: bool,
    routes: &[usize],
    diff_types: &[u8],
) -> Vec<DiffResult> {
    let M = A.len();
    let N = B.len();
    let mut result: Vec<DiffResult> = vec![];
    let mut a = M - 1;
    let mut b = N - 1;
    let mut j = routes[current.id];
    let mut diff_type = diff_types[current.id];
    loop {
        if j == 0 && diff_type == NONE {
            break;
        }
        let prev = j;
        match diff_type {
            REMOVED => {
                let old_index = if swapped { None } else { Some(a) };
                let new_index = if swapped { Some(a) } else { None };
                let result_type = if swapped {
                    DiffResult::Added
                } else {
                    DiffResult::Removed
                };
                result.push(result_type(DiffElement {
                    old_index,
                    new_index,
                }));
                a = a.wrapping_sub(1);
            }
            ADDED => {
                let old_index = if swapped { Some(b) } else { None };
                let new_index = if swapped { None } else { Some(b) };
                let result_type = if swapped {
                    DiffResult::Removed
                } else {
                    DiffResult::Added
                };
                result.push(result_type(DiffElement {
                    old_index,
                    new_index,
                }));
                b = b.wrapping_sub(1);
            }
            _ => {
                result.push(DiffResult::Common(DiffElement {
                    old_index: Some(a),
                    new_index: Some(b),
                }));
                a = a.wrapping_sub(1);
                b = b.wrapping_sub(1);
            }
        };
        j = routes[prev];
        diff_type = diff_types[prev];
    }
    result.into_iter().rev().collect()
}

fn create_fp(
    fp: &[FarthestPoint],
    base: isize,
    k: isize,
    M: isize,
    ptr: &mut usize,
    routes: &mut [usize],
    diff_types: &mut [u8],
) -> FarthestPoint {
    if base < 1 as isize {
        let base = (base + 1) as usize;
        let prev = fp[base].id;
        let y = fp[base].y + 1;
        *ptr += 1;
        routes[*ptr] = prev;
        diff_types[*ptr] = REMOVED;
        return FarthestPoint { y, id: *ptr };
    } else if base + 1 >= fp.len() as isize {
        let base = (base - 1) as usize;
        let prev = fp[base].id;
        let y = fp[base].y;
        *ptr += 1;
        routes[*ptr] = prev;
        diff_types[*ptr] = ADDED;
        return FarthestPoint { y, id: *ptr };
    }

    let slide = &fp[(base - 1) as usize];
    let down = &fp[(base + 1) as usize];

    if slide.y == -1 && down.y == -1 {
        return FarthestPoint { y: 0, id: 0 };
    }
    *ptr += 1;
    if down.y == -1 || k == M || slide.y > down.y + 1 {
        let prev = slide.id;
        let y = slide.y;
        routes[*ptr] = prev;
        diff_types[*ptr] = ADDED;
        FarthestPoint { y, id: *ptr }
    } else {
        let prev = down.id;
        let y = down.y + 1;
        routes[*ptr] = prev;
        diff_types[*ptr] = REMOVED;
        FarthestPoint { y, id: *ptr }
    }
}

fn snake<T: PartialEq + Clone>(fps: &[FarthestPoint], ctx: &mut Ctx<T>) -> FarthestPoint {
    let M = ctx.A.len() as isize;
    let N = ctx.B.len() as isize;
    if ctx.k + N < 0 || M - ctx.k < 0 {
        return FarthestPoint { y: -1, id: 0 };
    }
    let mut fp = create_fp(fps, ctx.base, ctx.k, M, ctx.ptr, ctx.routes, ctx.diff_types);
    while fp.y + ctx.k < M && fp.y < N && ctx.A[(fp.y + ctx.k) as usize] == ctx.B[fp.y as usize] {
        let prev = fp.id;
        *ctx.ptr += 1;
        fp.id = *ctx.ptr;
        fp.y += 1;
        ctx.routes[*ctx.ptr] = prev;
        ctx.diff_types[*ctx.ptr] = COMMON;
    }
    fp
}

pub fn diff<T: PartialEq + Clone>(old: &[T], new: &[T]) -> Vec<DiffResult> {
    let new_len = new.len();
    let old_len = old.len();
    let common_prefix = old.iter().zip(new).take_while(|p| p.0 == p.1);
    let prefix_size = common_prefix.count();
    let common_suffix = old.iter()
        .rev()
        .zip(new.iter().rev())
        .take(cmp::min(old_len, new_len) - prefix_size)
        .take_while(|p| p.0 == p.1);
    let suffix_size = common_suffix.count();
    let swapped = old_len < new_len;

    let sliced_old = &old[prefix_size..(old_len - suffix_size)];
    let sliced_new = &new[prefix_size..(new_len - suffix_size)];

    let (A, B) = if swapped {
        (sliced_new, sliced_old)
    } else {
        (sliced_old, sliced_new)
    };

    let mut result: Vec<DiffResult> = Vec::new();
    let M = A.len();
    let N = B.len();

    if M == 0 && N == 0 && prefix_size == 0 && suffix_size == 0 {
        return result;
    }

    if N == 0 {
        let mut p = 0;
        while p < prefix_size {
            result.push(DiffResult::Common(DiffElement {
                old_index: Some(p),
                new_index: Some(p),
            }));
            p += 1;
        }

        let mut o = prefix_size;
        while o < M + prefix_size {
            if swapped {
                result.push(DiffResult::Added(DiffElement {
                    old_index: None,
                    new_index: Some(o),
                }));
            } else {
                result.push(DiffResult::Removed(DiffElement {
                    old_index: Some(o),
                    new_index: None,
                }));
            }
            o += 1;
        }

        let mut s = 0;
        let old_offset = sliced_old.len() + prefix_size;
        let new_offset = sliced_new.len() + prefix_size;
        while s < suffix_size {
            let old_index = s + old_offset;
            result.push(DiffResult::Common(DiffElement {
                old_index: Some(old_index),
                new_index: Some(s + new_offset),
            }));
            s += 1;
        }
        return result;
    }

    let offset = N as isize;
    let D = (M - N) as isize;
    let size = M + N + 1;
    let mut fp: Vec<FarthestPoint> = vec![FarthestPoint { y: -1, id: 0 }; size];
    let mut P = 0;
    let mut ptr = 0;
    let mut routes: Vec<usize> = vec![0; M * N + size + 1];
    let mut diff_types: Vec<u8> = vec![0; M * N + size + 1];

    let mut ctx = Ctx {
        k: 0,
        base: 0,
        A,
        B,
        ptr: &mut ptr,
        routes: &mut routes,
        diff_types: &mut diff_types,
    };

    while fp[(D + offset) as usize].y < N as isize {
        let mut k = -(P as isize);
        while k < D as isize {
            let base = k + offset as isize;
            ctx.k = k;
            ctx.base = base;
            fp[base as usize] = snake(&fp, &mut ctx);
            k += 1;
        }
        let mut k = (D + P) as isize;
        while k > D as isize {
            let base = k + offset as isize;
            ctx.k = k;
            ctx.base = base;
            fp[base as usize] = snake(&fp, &mut ctx);
            k -= 1;
        }
        let base = D + offset;
        ctx.k = D;
        ctx.base = base;
        fp[base as usize] = snake(&fp, &mut ctx);
        P += 1;
    }

    let mut result: Vec<DiffResult> = vec![];
    let mut p = 0;
    while p < prefix_size {
        result.push(DiffResult::Common(DiffElement {
            old_index: Some(p),
            new_index: Some(p),
        }));
        p += 1;
    }
    let base = (D + offset) as usize;
    result.extend(back_trace(
        A,
        B,
        &fp[base],
        swapped,
        &ctx.routes,
        &ctx.diff_types,
    ));
    let mut s = 0;
    let old_offset = sliced_old.len() + prefix_size;
    let new_offset = sliced_new.len() + prefix_size;
    while s < suffix_size {
        let old_index = s + old_offset;
        result.push(DiffResult::Common(DiffElement {
            old_index: Some(old_index),
            new_index: Some(new_offset + s),
        }));
        s += 1;
    }
    result
}

#[test]
fn should_return_one_changed() {
    let result = diff(&vec!["a"], &vec!["b"]);
    let expected = vec![
        DiffResult::Added(DiffElement {
            old_index: None,
            new_index: Some(0),
            // data: "b",
        }),
        DiffResult::Removed(DiffElement {
            old_index: Some(0),
            new_index: None,
            // data: "a",
        }),
    ];

    assert_eq!(result, expected);
}

#[test]
fn should_return_empty() {
    let a: Vec<String> = vec![];
    let b: Vec<String> = vec![];
    let result = diff(&a, &b);
    let expected = vec![];
    assert_eq!(result, expected);
}

#[test]
fn should_return_one_common() {
    let result = diff(&vec!["a"], &vec!["a"]);
    let expected = vec![DiffResult::Common(DiffElement {
        old_index: Some(0),
        new_index: Some(0),
        // data: "a",
    })];
    assert_eq!(result, expected);
}

#[test]
fn should_return_one_removed() {
    let result = diff(&vec!["a"], &vec![]);
    let expected = vec![DiffResult::Removed(DiffElement {
        old_index: Some(0),
        new_index: None,
        // data: "a",
    })];
    assert_eq!(result, expected);
}

#[test]
fn should_return_one_added() {
    let result = diff(&vec![], &vec!["a"]);
    let expected = vec![DiffResult::Added(DiffElement {
        old_index: None,
        new_index: Some(0),
        // data: "a",
    })];
    assert_eq!(result, expected);
}

#[test]
fn should_return_two_changed() {
    let result = diff(&vec!["a", "a"], &vec!["b", "b"]);
    let expected = vec![
        DiffResult::Added(DiffElement {
            old_index: None,
            new_index: Some(0),
            // data: "b",
        }),
        DiffResult::Added(DiffElement {
            old_index: None,
            new_index: Some(1),
            // data: "b",
        }),
        DiffResult::Removed(DiffElement {
            old_index: Some(0),
            new_index: None,
            // data: "a",
        }),
        DiffResult::Removed(DiffElement {
            old_index: Some(1),
            new_index: None,
            // data: "a",
        }),
    ];

    assert_eq!(result, expected);
}

#[test]
fn should_create_diff_result_with_added() {
    let result = diff(&vec!["abc", "c"], &vec!["abc", "bcd", "c"]);
    let expected = vec![
        DiffResult::Common(DiffElement {
            old_index: Some(0),
            new_index: Some(0),
            // data: "abc",
        }),
        DiffResult::Added(DiffElement {
            old_index: None,
            new_index: Some(1),
            // data: "bcd",
        }),
        DiffResult::Common(DiffElement {
            old_index: Some(1),
            new_index: Some(2),
            // data: "c",
        }),
    ];

    assert_eq!(result, expected);
}

#[test]
fn should_create_diff_result_with_added_swapped() {
    let result = diff(&vec!["abc", "bcd", "c"], &vec!["abc", "c"]);
    let expected = vec![
        DiffResult::Common(DiffElement {
            old_index: Some(0),
            new_index: Some(0),
            // data: "abc",
        }),
        DiffResult::Removed(DiffElement {
            old_index: Some(1),
            new_index: None,
            // data: "bcd",
        }),
        DiffResult::Common(DiffElement {
            old_index: Some(2),
            new_index: Some(1),
            // data: "c",
        }),
    ];

    assert_eq!(result, expected);
}

#[test]
fn should_create_diff_result_with_removed() {
    let result = diff(&vec!["abc", "bcd", "c"], &vec!["abc", "c"]);
    let expected = vec![
        DiffResult::Common(DiffElement {
            old_index: Some(0),
            new_index: Some(0),
            // data: "abc",
        }),
        DiffResult::Removed(DiffElement {
            old_index: Some(1),
            new_index: None,
            // data: "bcd",
        }),
        DiffResult::Common(DiffElement {
            old_index: Some(2),
            new_index: Some(1),
            // data: "c",
        }),
    ];
    assert_eq!(result, expected);
}

#[test]
fn should_create_diff_result_without_new() {
    let result = diff(&vec!["abc", "bcd", "c"], &vec![]);
    let expected = vec![
        DiffResult::Removed(DiffElement {
            old_index: Some(0),
            new_index: None,
            // data: "abc",
        }),
        DiffResult::Removed(DiffElement {
            old_index: Some(1),
            new_index: None,
            // data: "bcd",
        }),
        DiffResult::Removed(DiffElement {
            old_index: Some(2),
            new_index: None,
            // data: "c",
        }),
    ];
    assert_eq!(result, expected);
}

#[test]
fn should_create_diff_result_without_old() {
    let result = diff(&vec![], &vec!["abc", "bcd", "c"]);
    let expected = vec![
        DiffResult::Added(DiffElement {
            old_index: None,
            new_index: Some(0),
            // data: "abc",
        }),
        DiffResult::Added(DiffElement {
            old_index: None,
            new_index: Some(1),
            // data: "bcd",
        }),
        DiffResult::Added(DiffElement {
            old_index: None,
            new_index: Some(2),
            // data: "c",
        }),
    ];
    assert_eq!(result, expected);
}

#[test]
fn should_create_empty_result_with_empty_input() {
    let result = diff(&vec![0u8; 0], &vec![0u8; 0]);
    let expected = vec![];
    assert_eq!(result, expected);
}

#[test]
fn should_() {
    let result = diff(
        &vec!["abc", "bcd", "c", "aaa", "bbb", "ccc"],
        &vec!["abc", "c", "aaa", "bbb", "ccc"],
    );
    let expected = vec![
        DiffResult::Common(DiffElement {
            old_index: Some(0),
            new_index: Some(0),
            // data: "abc",
        }),
        DiffResult::Removed(DiffElement {
            old_index: Some(1),
            new_index: None,
            // data: "bcd",
        }),
        DiffResult::Common(DiffElement {
            old_index: Some(2),
            new_index: Some(1),
            // data: "c",
        }),
        DiffResult::Common(DiffElement {
            old_index: Some(3),
            new_index: Some(2),
            // data: "aaa",
        }),
        DiffResult::Common(DiffElement {
            old_index: Some(4),
            new_index: Some(3),
            // data: "bbb",
        }),
        DiffResult::Common(DiffElement {
            old_index: Some(5),
            new_index: Some(4),
            // data: "ccc",
        }),
    ];
    assert_eq!(result, expected);
}
