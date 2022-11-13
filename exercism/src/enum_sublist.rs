#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}


// brute force if/else blocks.........def not ideal...and messy AF  (BUT I GOT IT!)
pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.len() == _second_list.len() {
        if _first_list == _second_list {
            Comparison::Equal
        } else {
            Comparison::Unequal
        }
    } else if _first_list.len() < _second_list.len() {
        if _first_list.len() == 0 {
             Comparison::Sublist 
        } else {
            for win in _second_list.windows(_first_list.len()) {
                if win == _first_list {
                    return Comparison::Sublist;
                }        
            }
            Comparison::Unequal
        }
    } else if _first_list.len() > _second_list.len() {
        if _second_list.len() == 0 { 
            Comparison::Superlist 
        } else {
            for win in _first_list.windows(_second_list.len()) {
                if win == _second_list {
                    return Comparison::Superlist;
                }        
            }
            Comparison::Unequal
        }
    } else {
        Comparison::Unequal
    }
}


// now with pattern matching.  Note to self: Try to recapture that magic you felt with Haskell :D
pub fn sublist_pat_mat<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    match (_first_list.len(), _second_list.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (a, b) if a > b => if _first_list.windows(b).any(|win| win == _second_list) {Comparison::Superlist} else {Comparison::Unequal},
        (a, b) if a < b => if _second_list.windows(a).any(|win| win == _first_list) {Comparison::Sublist} else {Comparison::Unequal},
        (_, _) => if _first_list == _second_list {Comparison::Equal} else {Comparison::Unequal}
    }
}


fn main() {
 
let first_list = [1,2,3,4,5,6];
let second_list = [1,2,3,4,5,6];


sublist(&first_list, &second_list);
sublist_pat_mat(&first_list, &second_list);
}