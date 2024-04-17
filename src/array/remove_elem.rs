//!给你一个数组 nums 和一个值 val，你需要 原地 移除所有数值等于 val 的元素，并返回移除后数组的新长度。
//!不要使用额外的数组空间，你必须仅使用 O(1) 额外空间并 原地 修改输入数组。
//!元素的顺序可以改变。你不需要考虑数组中超出新长度后面的元素。
#[inline]
fn find_next(nums: &mut [i32], index: &mut usize) -> Result<i32, usize> {
    *index += 1;
    if *index == nums.len() {
        Err(*index)
    } else {
        Ok(nums[*index])
    }
}
#[inline]
fn find_next_not_target(
    nums: &mut [i32],
    index_first: &mut usize,
    index_scend: &mut usize,
    target_val: i32,
) -> Result<i32, usize> {
    loop {
        match find_next(nums, index_first) {
            Err(_) => {
                break Err(*index_first - *index_scend);
            }
            Ok(tmp) => {
                if tmp == target_val {
                    continue;
                } else {
                    break Ok(tmp);
                }
            }
        }
    }
}
///注意双指针在使用的时候，需要找到下一个不同的数字，而不能单单下一个数字
/// 这样双指针的跳皆才对
pub fn remove_element(nums: &mut [i32], val: i32) -> i32 {
    let mut index_first = 0;
    let mut index_scend = 0;
    let len = nums.len();
    let diff = loop {
        if index_first == len {
            break index_first - index_scend;
        }
        let mut value = nums[index_first];
        if value == val {
            match find_next_not_target(nums, &mut index_first, &mut index_scend, val) {
                Ok(tmp_value) => value = tmp_value,
                Err(e) => {
                    break e;
                }
            }
        }
        nums[index_scend] = value;
        index_first += 1;
        index_scend += 1;
    };
    (len - diff) as i32
}
#[cfg(test)]
pub mod tests {
    use super::remove_element;
    #[test]
    fn test() {
        let mut arr = vec![3, 2, 2, 3, 3, 3, 3, 4, 4, 3, 4];
        let ret = remove_element(&mut arr, 3);
        println!("{} arr {:?}", ret, arr)
    }
}
