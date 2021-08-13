#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod functions {
    pub mod numbers {
        //converts a u8 decimal to binary
        pub fn u8_to_binary(number:u8) -> String {
            let mut converted = String::new();
            let mut num = number;
            while num > 0 {
                if num % 2 == 1 {
                    converted = format!("{}{}", "1", converted);
                } else { converted = format!("{}{}", "0", converted); }
                num = num / 2;
            }
            return converted;
        }

        //converts a u16 decimal number to binary
        pub fn u16_to_binary(number:u16) -> String {
            let mut converted = String::new();
            let mut num = number;
            while num > 0 {
                if num % 2 == 1 {
                    converted = format!("{}{}", "1", converted);
                } else { converted = format!("{}{}", "0", converted); }
                num = num / 2;
            }
            return converted;
        }


        
        //sorts a vector of u16 numbers
        pub fn bubble_sort_i16(nums:&Vec<i16>) -> Vec<i16> {
            let mut sorting_nums:Vec<i16>=vec![];
            for i in nums {
                sorting_nums.push(*i);
            }
            let mut right_num:i16;
            let mut sorted:bool = false;
            while !sorted {
                sorted = true;
                for i in 0..sorting_nums.len()-1 {
                    if sorting_nums[i]>sorting_nums[i+1] {
                        sorted=false;
                        right_num=sorting_nums[i];
                        sorting_nums[i]=sorting_nums[i+1];
                        sorting_nums[i+1]=right_num;
                    }
                }
            }
            return sorting_nums
        }

        //sorts a vector of u16 numbers
        pub fn comparison_sort_i16(nums_to_sort: &Vec<i16>) -> Vec<i16> {
            let mut unsorted_nums:Vec<i16> = vec![];
            for i in nums_to_sort {
                unsorted_nums.push(*i);
            }
            let mut sorted_nums:Vec<i16> = vec![];
            let mut next_num:[i16;2];
            while unsorted_nums.len()>0 {
                next_num =comparison_sort_i16_applier(&unsorted_nums);
                sorted_nums.push(next_num[0]);
                unsorted_nums.remove(next_num[1] as usize);
            }
            return sorted_nums
        }

        //sorts a vector of u16 numbers
        fn comparison_sort_i16_applier(num_list: &Vec<i16>) -> [i16;2] {
            let mut unsorted_nums:Vec<i16> = vec![];
            for i in num_list {
                unsorted_nums.push(*i);
            }
            let mut smallest_num = unsorted_nums[0];
            let mut index:i16 = 0;
            for i in 0..unsorted_nums.len() {
                if smallest_num>unsorted_nums[i] {
                    smallest_num=unsorted_nums[i];
                    index=i as i16;
                }
            }
            return [smallest_num,index]
        }

        //sorts a vector of u16 numbers
        use std::collections::HashMap;
        pub fn table_sort_i16(nums: &Vec<i16>) -> HashMap<i16,u16> {
            let mut unsorted_nums:Vec<i16>=vec![];
            for i in nums {
                unsorted_nums.push(*i);
            }
            let mut num_map:HashMap<i16,u16> = HashMap::new();
            for i in unsorted_nums {
                if num_map.contains_key(&i) {num_map.insert(i,num_map[&i]+1);}
                else {num_map.insert(i,1);}
            }
            return num_map

        }

        //sorts a vector of i16 numbers
        pub fn quicksort(nums: &Vec<i16>) -> Vec<i16> {
            let mut unsorted_nums:Vec<i16> = vec![];
            for i in nums {
                unsorted_nums.push(*i);
            }
            if unsorted_nums.len()<2 {return unsorted_nums}
            let pivot:i16 = unsorted_nums[0];
            unsorted_nums.remove(0);
            let mut smaller:Vec<i16> = vec![];
            let mut bigger:Vec<i16> = vec![];
            let mut sorted:Vec<i16> = vec![];
            for i in unsorted_nums {
                if i>pivot {bigger.push(i)}
                else {smaller.push(i)}
            }
            let smaller_nums = quicksort(&smaller);
            let bigger_nums = quicksort(&bigger);
            for i in smaller_nums {sorted.push(i)}
            sorted.push(pivot);
            for i in bigger_nums {sorted.push(i)}
            return sorted
        }
    }


    pub mod input {

        //gets a string from the user
        pub fn str_input(message:&str) -> String {
            use std::io::stdin;
            let mut input = String::new();
            println!("{}",message);
            stdin()
                .read_line(&mut input)
                .expect("Did not enter a correct string");
            return input.trim().to_string()
        }

        //gets a u8 number from the user
        pub fn u8_input(message:&str) -> u8 {
            use std::io::stdin;
            let mut input = String::new();
            println!("{}\n",message);
            stdin()
                .read_line(&mut input)
                .expect("Did not enter a correct string");
            let num = match input.trim().parse::<u8>() {
                Ok(i) => i,
                Err(_) => u8_input("Input an actual number"),//wow I used recursion without realizing
            };
            num
        }

        //stops the program from continuing until the user presses enter
        pub fn stall() {
            #![allow(unused_must_use)]
            use std::io::stdin;
            println!("Press Enter to Continue");
            let mut unused = String::new();
            stdin().read_line(&mut unused);
        }

        //same as stall but allows a string
        pub fn stop_to_read(read_this:&str) {
            #![allow(unused_must_use)]
            use std::io::stdin;
            let mut unused = String::new();
            println!("{} \nPress Enter to Continue", read_this);
            stdin().read_line(&mut unused);
        }

        //gets a u16 number from the user
        pub fn u16_input(message:&str) -> u16 {
            use std::io::stdin;
            let mut input = String::new();
            println!("{}\n",message);
            stdin()
                .read_line(&mut input)
                .expect("Did not enter a correct string");
            let num = match input.trim().parse::<u16>() {
                Ok(i) => i,
                Err(_) => u16_input("Input an actual number"),
            };
            num
        }
        //gets a u32 number from the user
        pub fn u32_input(message:&str) -> u32 {
            use std::io::stdin;
            let mut input = String::new();
            println!("{}\n",message);
            stdin()
                .read_line(&mut input)
                .expect("Did not enter a correct string");
            let num = match input.trim().parse::<u32>() {
                Ok(i) => i,
                Err(_) => u32_input("Input an actual number"),
            };
            num
        }
    }
}