#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {

// Create two booleans that check if the lists are empty or if we have a super list or sub list
    
  let superlist = second_list.is_empty() ||
      first_list
      .windows(second_list.len())
      .any(|x| x == second_list);

    let sublist = first_list.is_empty() ||
        second_list
        .windows(first_list.len())
        .any(|x| x == first_list);

    match(superlist, sublist){
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Superlist,
        (false, true) => Comparison::Sublist,
        (false, false) => Comparison::Unequal,
    }
}
