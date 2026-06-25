
#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

/// Returns true if list_b is sublist of list_a
fn sublist_of_a(list_a: &[i32], list_b: &[i32]) -> bool  {
    if list_b.is_empty() {
        return true;
    }
    let mut i = 0usize;
    while i < list_a.len() {
        let mut j = i;
        let mut k = 0;
        while j < list_a.len() && k < list_b.len() && list_a[j] == list_b[k] {
            j += 1;
            k += 1;
        }
        if k >= list_b.len() {
            return true;
        }
        i += 1;
    }
    false
}

#[test]
fn test_sublist_of_a() {

    let list1 = [1, 2, 3, 4];
    let list2 = [2, 3];
    assert!(sublist_of_a(&list1, &list2));
    let list1 = [1, 2, 3, 4];
    let list2 = [2, 5];
    assert!(!sublist_of_a(&list1, &list2));
    let list1 = [1, 2, 3, 4];
    let list2 = [1, 4];
    assert!(!sublist_of_a(&list1, &list2));

    let list1 = [1, 2, 3, 4];
    let list2 = [];
    assert!(sublist_of_a(&list1, &list2));

    let list1 = [];
    let list2 = [];
    assert!(sublist_of_a(&list1, &list2));

    let list1 = [1, 2, 3, 4, 5, 6, 7];
    let list2 = [2, 3, 4];
    assert!(sublist_of_a(&list1, &list2));

    let list1 = [1, 1, 2];
    let list2 = [0, 1, 1, 1, 2, 1, 2];
    assert!(sublist_of_a(&list2, &list1));
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list.is_empty() && second_list.is_empty() {
        return Comparison::Equal;
    }
    if first_list.is_empty() {
        return Comparison::Sublist;
    }
    if second_list.is_empty() {
        return Comparison::Superlist;
    }

    let first_len = first_list.len();
    let second_len = second_list.len();
    if first_len == second_len {
        if first_list.iter().eq(second_list.iter()) {
            return Comparison::Equal;
        }
        return Comparison::Unequal;
    }
    if first_len > second_len {
        if sublist_of_a(first_list, second_list) {
            return Comparison::Superlist;
        }
    } else {
        if sublist_of_a(second_list, first_list) {
            return Comparison::Sublist;
        }
    }

    Comparison::Unequal

}


#[test]
fn test_sublist() {
    let res = sublist(&[1, 2, 3, 4, 5], &[3, 4]);
    println!("result: {:#?}", res);

    let res = sublist(&[3, 4], &[1, 2, 3, 4, 5]);
    println!("result: {:#?}", res);

    let res = sublist(&[3, 4], &[3, 4]);
    println!("result: {:#?}", res);


    let res = sublist(&[], &[]);
    println!("result: {:#?}", res);

    let res = sublist(&[], &[1, 2, 3]);
    println!("result: {:#?}", res);
    let res = sublist( &[1, 2, 3], &[]);
    println!("result: {:#?}", res);
}
