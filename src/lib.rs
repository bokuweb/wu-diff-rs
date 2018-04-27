#![allow(non_snake_case)]

use std::cmp;

const REMOVED: u8 = 1;
const COMMON: u8 = 2;
const ADDED: u8 = 3;


#[derive(Debug, PartialEq)]
pub enum DiffResult<T: PartialEq + Clone> {
    Removed(DiffElement<T>),
    Common(DiffElement<T>),
    Added(DiffElement<T>),
}

#[derive(Debug, PartialEq)]
pub struct DiffElement<T: PartialEq + Clone> {
    pub old_index: Option<usize>,
    pub new_index: Option<usize>,
    pub data: T,
}

#[derive(Debug, Clone, PartialEq)]
struct FarthestPoint {
    y: isize,
    id: usize,
}

fn back_trace<T: PartialEq + Clone>(A: &[T],
                                    B: &[T],
                                    current: &FarthestPoint,
                                    swapped: bool,
                                    routes: &Vec<usize>,
                                    diff_types: &Vec<u8>)
                                    -> Vec<DiffResult<T>> {
    let M = A.len();
    let N = B.len();
    let mut result: Vec<DiffResult<T>> = vec![];
    let mut a = M - 1;
    let mut b = N - 1;
    let mut j = routes[current.id];
    let mut diff_type = diff_types[current.id];
    loop {
        if j == 0 && diff_type == 0 {
            break;
        }
        let prev = j;
        match diff_type {
            REMOVED if swapped => {
                result.insert(0,
                              DiffResult::Added(DiffElement {
                                                    old_index: None,
                                                    new_index: Some(a),
                                                    data: A[a].clone(),
                                                }));
                a = a.wrapping_sub(1);
            }
            REMOVED => {
                result.insert(0,
                              DiffResult::Removed(DiffElement {
                                                      old_index: Some(a),
                                                      new_index: None,
                                                      data: A[a].clone(),
                                                  }));
                a = a.wrapping_sub(1);
            }
            ADDED if swapped => {
                result.insert(0,
                              DiffResult::Removed(DiffElement {
                                                      old_index: None,
                                                      new_index: Some(b),
                                                      data: B[b].clone(),
                                                  }));
                b = b.wrapping_sub(1);
            }
            ADDED => {
                result.insert(0,
                              DiffResult::Added(DiffElement {
                                                    old_index: None,
                                                    new_index: Some(b),
                                                    data: B[b].clone(),
                                                }));
                b = b.wrapping_sub(1);
            }
            _ => {
                result.insert(0,
                              DiffResult::Common(DiffElement {
                                                     old_index: Some(a),
                                                     new_index: Some(b),
                                                     data: A[a].clone(),
                                                 }));
                a = a.wrapping_sub(1);
                b = b.wrapping_sub(1);
            }
        };
        j = routes[prev];
        diff_type = diff_types[prev];
    }
    return result;
}

fn create_fp(slide: &FarthestPoint,
             down: &FarthestPoint,
             k: isize,
             M: isize,
             ptr: &mut usize,
             routes: &mut Vec<usize>,
             diff_types: &mut Vec<u8>)
             -> FarthestPoint {
    if slide.y == -1 && down.y == -1 {
        return FarthestPoint { y: 0, id: 0 };
    }
    return if down.y == -1 || k == M || slide.y > down.y + 1 {
               let prev = slide.id;
               let y = slide.y;
               *ptr += 1;
               routes[*ptr] = prev;
               diff_types[*ptr] = ADDED;
               FarthestPoint { y, id: *ptr }
           } else {
               let prev = down.id;
               let y = down.y + 1;
               *ptr += 1;
               routes[*ptr] = prev;
               diff_types[*ptr] = REMOVED;
               FarthestPoint { y, id: *ptr }
           };
}

fn snake<T: PartialEq + Clone>(k: isize,
                               slide: &FarthestPoint,
                               down: &FarthestPoint,
                               A: &[T],
                               B: &[T],
                               ptr: &mut usize,
                               routes: &mut Vec<usize>,
                               diff_types: &mut Vec<u8>)
                               -> FarthestPoint {
    let M = A.len() as isize;
    let N = B.len() as isize;
    if k + N < 0 || M - k < 0 {
        return FarthestPoint { y: -1, id: 0 };
    }
    let mut fp = create_fp(&slide, &down, k, M, ptr, routes, diff_types);
    while fp.y + k < M && fp.y < N && A[(fp.y + k) as usize] == B[fp.y as usize] {
        let prev = fp.id;
        *ptr += 1;
        fp.id = *ptr;
        fp.y += 1;
        routes[*ptr] = prev;
        diff_types[*ptr] = COMMON;
    }
    return fp;
}

pub fn diff<T: PartialEq + Clone>(old: &[T], new: &[T]) -> Vec<DiffResult<T>> {
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

    let mut result: Vec<DiffResult<T>> = Vec::new();
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
                                               data: old[p].clone(),
                                           }));
            p += 1;
        }

        let mut o = prefix_size;
        while o < M + prefix_size {
            if swapped {
                result.push(DiffResult::Added(DiffElement {
                                                  old_index: None,
                                                  new_index: Some(o),
                                                  data: new[o].clone(),
                                              }));
            } else {
                result.push(DiffResult::Removed(DiffElement {
                                                    old_index: Some(o),
                                                    new_index: None,
                                                    data: old[o].clone(),
                                                }));
            }
            o += 1;
        }

        let mut s = 0;
        while s < suffix_size {
            if swapped {
                let old_index = s + N + suffix_size;
                let new_index = s + M + suffix_size;
                result.push(DiffResult::Common(DiffElement {
                                                   old_index: Some(old_index),
                                                   new_index: Some(new_index),
                                                   data: old[old_index].clone(),
                                               }));
            } else {
                let old_index = s + M + suffix_size;
                let new_index = s + N + suffix_size;
                result.push(DiffResult::Common(DiffElement {
                                                   old_index: Some(old_index),
                                                   new_index: Some(new_index),
                                                   data: old[old_index].clone(),
                                               }));
            }
            s += 1;
        }
        return result;
    }

    let offset = N;
    let delta = M - N;
    let size = M + N + 1;
    let mut fp: Vec<FarthestPoint> = vec![FarthestPoint { y: -1, id: 0 }; size];
    let mut p = 0;
    let mut ptr = 0;
    let mut routes: Vec<usize> = vec![0; M * N + size + 1];
    let mut diff_types: Vec<u8> = vec![0; M * N + size + 1];

    while fp[delta + offset].y < N as isize {
        let mut k = -(p as isize);
        while k < delta as isize {
            let base = k + offset as isize;
            fp[base as usize] = if base < 1 as isize {
                let prev = fp[(base + 1) as usize].id;
                let y = fp[(base + 1) as usize].y + 1;
                ptr += 1;
                routes[ptr] = prev;
                diff_types[ptr] = REMOVED;
                FarthestPoint { y, id: ptr }
            } else if base + 1 >= size as isize {
                let prev = fp[(base - 1) as usize].id;
                let y = fp[(base - 1) as usize].y;
                ptr += 1;
                routes[ptr] = prev;
                diff_types[ptr] = ADDED;
                FarthestPoint { y, id: ptr }
            } else {
                snake(k,
                      &fp[base as usize - 1],
                      &fp[base as usize + 1],
                      A,
                      B,
                      &mut ptr,
                      &mut routes,
                      &mut diff_types)
            };
            k += 1;
        }
        let mut k = (delta + p) as isize;
        while k > delta as isize {
            let base = k + offset as isize;
            fp[base as usize] = if base < 1 {
                let prev = fp[(base + 1) as usize].id;
                let y = fp[(base + 1) as usize].y + 1;
                ptr += 1;
                routes[ptr] = prev;
                diff_types[ptr] = REMOVED;
                FarthestPoint { y, id: ptr }
            } else if base + 1 >= size as isize {
                let prev = fp[(base - 1) as usize].id;
                let y = fp[(base - 1) as usize].y;
                ptr += 1;
                routes[ptr] = prev;
                diff_types[ptr] = ADDED;
                FarthestPoint { y, id: ptr }
            } else {
                snake(k,
                      &fp[base as usize - 1],
                      &fp[base as usize + 1],
                      A,
                      B,
                      &mut ptr,
                      &mut routes,
                      &mut diff_types)
            };
            k -= 1;
        }
        fp[delta + offset] = snake(delta as isize,
                                   &fp[delta + offset - 1],
                                   &fp[delta + offset + 1],
                                   A,
                                   B,
                                   &mut ptr,
                                   &mut routes,
                                   &mut diff_types);
        p = p + 1;
    }

    let mut result: Vec<DiffResult<T>> = vec![];
    let mut p = 0;
    while p < prefix_size {
        result.push(DiffResult::Common(DiffElement {
                                           old_index: Some(p),
                                           new_index: Some(p),
                                           data: old[p].clone(),
                                       }));
        p += 1;
    }
    result.extend(back_trace(A, B, &fp[delta + offset], swapped, &routes, &diff_types));
    let mut s = 0;
    while s < suffix_size {
        if swapped {
            let old_index = s + N + suffix_size;
            let new_index = s + M + suffix_size;
            result.push(DiffResult::Common(DiffElement {
                                               old_index: Some(old_index),
                                               new_index: Some(new_index),
                                               data: old[old_index].clone(),
                                           }));
        } else {
            let old_index = s + M + suffix_size;
            let new_index = s + N + suffix_size;
            result.push(DiffResult::Common(DiffElement {
                                               old_index: Some(old_index),
                                               new_index: Some(new_index),
                                               data: old[old_index].clone(),
                                           }));
        }
        s += 1;
    }
    result
}

#[test]
fn should_return_one_changed() {
    let result = diff(&vec!["a"], &vec!["b"]);
    let expected = vec![DiffResult::Added(DiffElement {
                                              old_index: None,
                                              new_index: Some(0),
                                              data: "b",
                                          }),
                        DiffResult::Removed(DiffElement {
                                                old_index: Some(0),
                                                new_index: None,
                                                data: "a",
                                            })];

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
                                               data: "a",
                                           })];
    assert_eq!(result, expected);
}

#[test]
fn should_return_one_removed() {
    let result = diff(&vec!["a"], &vec![]);
    let expected = vec![DiffResult::Removed(DiffElement {
                                                old_index: Some(0),
                                                new_index: None,
                                                data: "a",
                                            })];
    assert_eq!(result, expected);
}

#[test]
fn should_return_one_added() {
    let result = diff(&vec![], &vec!["a"]);
    let expected = vec![DiffResult::Added(DiffElement {
                                              old_index: None,
                                              new_index: Some(0),
                                              data: "a",
                                          })];
    assert_eq!(result, expected);
}

#[test]
fn should_return_two_changed() {
    let result = diff(&vec!["a", "a"], &vec!["b", "b"]);
    let expected = vec![DiffResult::Added(DiffElement {
                                              old_index: None,
                                              new_index: Some(0),
                                              data: "b",
                                          }),
                        DiffResult::Added(DiffElement {
                                              old_index: None,
                                              new_index: Some(1),
                                              data: "b",
                                          }),
                        DiffResult::Removed(DiffElement {
                                                old_index: Some(0),
                                                new_index: None,
                                                data: "a",
                                            }),
                        DiffResult::Removed(DiffElement {
                                                old_index: Some(1),
                                                new_index: None,
                                                data: "a",
                                            })];

    assert_eq!(result, expected);
}

#[test]
fn should_create_diff_result_with_added() {
    let result = diff(&vec!["abc", "c"], &vec!["abc", "bcd", "c"]);
    let expected = vec![DiffResult::Common(DiffElement {
                                               old_index: Some(0),
                                               new_index: Some(0),
                                               data: "abc",
                                           }),
                        DiffResult::Added(DiffElement {
                                              old_index: None,
                                              new_index: Some(1),
                                              data: "bcd",
                                          }),
                        DiffResult::Common(DiffElement {
                                               old_index: Some(1),
                                               new_index: Some(2),
                                               data: "c",
                                           })];

    assert_eq!(result, expected);
}

#[test]
fn should_create_diff_result_with_added_swapped() {
    let result = diff(&vec!["abc", "bcd", "c"], &vec!["abc", "c"]);
    let expected = vec![DiffResult::Common(DiffElement {
                                               old_index: Some(0),
                                               new_index: Some(0),
                                               data: "abc",
                                           }),
                        DiffResult::Removed(DiffElement {
                                                old_index: Some(1),
                                                new_index: None,
                                                data: "bcd",
                                            }),
                        DiffResult::Common(DiffElement {
                                               old_index: Some(2),
                                               new_index: Some(1),
                                               data: "c",
                                           })];

    assert_eq!(result, expected);
}

#[test]
fn should_create_diff_result_with_removed() {
    let result = diff(&vec!["abc", "bcd", "c"], &vec!["abc", "c"]);
    let expected = vec![DiffResult::Common(DiffElement {
                                               old_index: Some(0),
                                               new_index: Some(0),
                                               data: "abc",
                                           }),
                        DiffResult::Removed(DiffElement {
                                                old_index: Some(1),
                                                new_index: None,
                                                data: "bcd",
                                            }),
                        DiffResult::Common(DiffElement {
                                               old_index: Some(2),
                                               new_index: Some(1),
                                               data: "c",
                                           })];
    assert_eq!(result, expected);
}

#[test]
fn should_create_diff_result_without_new() {
    let result = diff(&vec!["abc", "bcd", "c"], &vec![]);
    let expected = vec![DiffResult::Removed(DiffElement {
                                                old_index: Some(0),
                                                new_index: None,
                                                data: "abc",
                                            }),
                        DiffResult::Removed(DiffElement {
                                                old_index: Some(1),
                                                new_index: None,
                                                data: "bcd",
                                            }),
                        DiffResult::Removed(DiffElement {
                                                old_index: Some(2),
                                                new_index: None,
                                                data: "c",
                                            })];
    assert_eq!(result, expected);
}

#[test]
fn should_create_diff_result_without_old() {
    let result = diff(&vec![], &vec!["abc", "bcd", "c"]);
    let expected = vec![DiffResult::Added(DiffElement {
                                              old_index: None,
                                              new_index: Some(0),
                                              data: "abc",
                                          }),
                        DiffResult::Added(DiffElement {
                                              old_index: None,
                                              new_index: Some(1),
                                              data: "bcd",
                                          }),
                        DiffResult::Added(DiffElement {
                                              old_index: None,
                                              new_index: Some(2),
                                              data: "c",
                                          })];
    assert_eq!(result, expected);
}

#[test]
fn should_create_empty_result_with_empty_input() {
    let result = diff(&vec![0u8; 0], &vec![0u8; 0]);
    let expected = vec![];
    assert_eq!(result, expected);
}
