use std::cmp;

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
    tree: Option<Tree>,
}

#[derive(Debug, Clone, PartialEq)]
struct Tree {
    prev: Box<Option<Tree>>,
    diff_type: DiffType,
}

#[derive(Debug, Clone, PartialEq, Copy)]
enum DiffType {
    Removed,
    Common,
    Added,
}

fn back_trace<T: PartialEq + Clone>(A: &[T],
                                    B: &[T],
                                    current: FarthestPoint,
                                    swapped: bool)
                                    -> Vec<DiffResult<T>> {
    let M = A.len();
    let N = B.len();
    let mut result: Vec<DiffResult<T>> = vec![];
    let mut a = M - 1;
    let mut b = N - 1;
    let mut j = current.tree.clone();
    loop {
        match j {
            Some(tree) => {
                match tree.diff_type {
                    DiffType::Removed if swapped => {
                        result.insert(0,
                                      DiffResult::Added(DiffElement {
                                                            old_index: None,
                                                            new_index: Some(a),
                                                            data: A[a].clone(),
                                                        }));
                        a = a.wrapping_sub(1);
                    }
                    DiffType::Removed => {
                        result.insert(0,
                                      DiffResult::Removed(DiffElement {
                                                              old_index: Some(a),
                                                              new_index: None,
                                                              data: A[a].clone(),
                                                          }));
                        a = a.wrapping_sub(1);
                    }
                    DiffType::Added if swapped => {
                        result.insert(0,
                                      DiffResult::Removed(DiffElement {
                                                              old_index: None,
                                                              new_index: Some(b),
                                                              data: B[b].clone(),
                                                          }));
                        b = b.wrapping_sub(1);
                    }
                    DiffType::Added => {
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
                j = *tree.prev;
            }
            _ => break,
        }
    }
    return result;
}

fn create_fp(slide: FarthestPoint,
             down: FarthestPoint,
             k: isize,
             M: isize,
             N: isize)
             -> FarthestPoint {
    if slide.y == -1 && down.y == -1 {
        return FarthestPoint { y: 0, tree: None };
    }
    return if down.y == -1 || k == M || slide.y > down.y + 1 {
               FarthestPoint {
                   y: slide.y,
                   tree: Some(Tree {
                                  diff_type: DiffType::Removed,
                                  prev: Box::new(slide.tree),
                              }),
               }
           } else {
               FarthestPoint {
                   y: down.y + 1,
                   tree: Some(Tree {
                                  diff_type: DiffType::Added,
                                  prev: Box::new(down.tree),
                              }),
               }
           };
}

fn snake<T: PartialEq + Clone>(k: isize,
                               slide: FarthestPoint,
                               down: FarthestPoint,
                               offset: usize,
                               A: &[T],
                               B: &[T])
                               -> FarthestPoint {
    let M = A.len() as isize;
    let N = B.len() as isize;
    if k + N < 0 || M - k < 0 {
        return FarthestPoint { y: -1, tree: None };
    }
    let mut fp = create_fp(slide, down, k, M, N);
    while fp.y + k < M && fp.y < N && A[(fp.y + k) as usize] == B[fp.y as usize] {
        fp.tree = Some(Tree {
                           diff_type: DiffType::Common,
                           prev: Box::new(fp.tree),
                       });
        fp.y += 1;
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
    let mut fp: Vec<FarthestPoint> = vec![FarthestPoint { y: -1, tree: None }; size];
    let mut p = 0;

    while fp[delta + offset].y < N as isize {
        let mut k = -(p as isize);
        while k < delta as isize {
            let base = k + offset as isize;
            if base < 1 || base + 1 >= size as isize {
                fp[base as usize] = FarthestPoint {
                    y: 1,
                    tree: Some(Tree {
                                   diff_type: DiffType::Added,
                                   prev: Box::new(None),
                               }),
                }
            } else if base + 1 >= size as isize {
                let slide = fp[(base - 1) as usize].clone();
                fp[base as usize] = FarthestPoint {
                    y: slide.y,
                    tree: Some(Tree {
                                   diff_type: DiffType::Removed,
                                   prev: Box::new(slide.tree),
                               }),
                }
            } else {
                let base = base as usize;
                fp[base] = snake(k, fp[base - 1].clone(), fp[base + 1].clone(), offset, A, B);
            }
            k += 1;
        }
        let mut k = (delta + p) as isize;
        while k > delta as isize {
            let base = k + offset as isize;
            if base < 1 {
                fp[base as usize] = FarthestPoint {
                    y: 1,
                    tree: Some(Tree {
                                   diff_type: DiffType::Added,
                                   prev: Box::new(None),
                               }),
                }
            } else if base + 1 >= size as isize {
                let slide = fp[(base - 1) as usize].clone();
                fp[base as usize] = FarthestPoint {
                    y: slide.y,
                    tree: Some(Tree {
                                   diff_type: DiffType::Removed,
                                   prev: Box::new(slide.tree),
                               }),
                }
            } else {
                let base = base as usize;
                fp[base] = snake(k, fp[base - 1].clone(), fp[base + 1].clone(), offset, A, B);
            }
            k -= 1;
        }
        fp[delta + offset] = snake(delta as isize,
                                   fp[delta + offset - 1].clone(),
                                   fp[delta + offset + 1].clone(),
                                   offset,
                                   A,
                                   B);
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
    result.extend(back_trace(A, B, fp[delta + offset].clone(), swapped));
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
    let expected = vec![DiffResult::Removed(DiffElement {
                                                old_index: Some(0),
                                                new_index: None,
                                                data: "a",
                                            }),
                        DiffResult::Added(DiffElement {
                                              old_index: None,
                                              new_index: Some(0),
                                              data: "b",
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
    let expected = vec![DiffResult::Removed(DiffElement {
                                                old_index: Some(0),
                                                new_index: None,
                                                data: "a",
                                            }),
                        DiffResult::Removed(DiffElement {
                                                old_index: Some(1),
                                                new_index: None,
                                                data: "a",
                                            }),
                        DiffResult::Added(DiffElement {
                                              old_index: None,
                                              new_index: Some(0),
                                              data: "b",
                                          }),
                        DiffResult::Added(DiffElement {
                                              old_index: None,
                                              new_index: Some(1),
                                              data: "b",
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
