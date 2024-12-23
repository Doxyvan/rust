
pub fn convert(s: String, num_rows: i32) -> String {
    let mut index = 0;
    let mut converted_str = String::from("");
    if num_rows == 1{
        return s
    }
    while index < num_rows{
        let mut index_for_finding_letter = index;
        while index_for_finding_letter < s.len() as i32{
                let char_to_push = &s[(index_for_finding_letter as usize)..((index_for_finding_letter+1) as usize)];
                converted_str.push_str(char_to_push);
                let index_for_finding_letter_between_cols = index_for_finding_letter + (2*num_rows-2-index*2);
                if index_for_finding_letter_between_cols < s.len() as i32 && (index != 0 && index != num_rows-1){
                    let char_to_push = &s[(index_for_finding_letter_between_cols as usize)..((index_for_finding_letter_between_cols+1) as usize)];
                    converted_str.push_str(char_to_push);
                }
                index_for_finding_letter += 2*num_rows-2;
                
            }
        index += 1;
        }
        
    
    return converted_str
}


fn main(){
    println!("{}", convert(String::from("ABCDE"), 3));
}