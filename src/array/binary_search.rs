//！ 一道关于二分查找的题目
//！leetcode连接 https://leetcode.cn/problems/binary-search/description/
//！ 给定一个 n 个元素有序的（升序）整型数组 nums 和一个目标值 target  ，写一个函数搜索 nums 中的 target，如果目标值存在返回下标，否则返回 -1。

///注意这里的left和right不能直接等于 mid ，因为除数取整的原因，会导致无限循环
///要加上对边界条件的判断，如果输入的是空数组，会出错
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    loop {
        let mid = (left + right) / 2;
        if mid > nums.len() {
            return -1;
        }
        if nums[mid] == target {
            break mid as i32;
        }
        if nums[mid] > target {
            right = mid - 1
        }
        if nums[mid] < target {
            left = mid + 1
        }

        if left > right {
            break -1;
        }
        println!("left {} right{} mid {}", left, right, mid)
    }
}
#[cfg(test)]
pub mod tests {
    use super::search;
    #[test]
    fn test() {
        let arr = vec![-1, 0, 3, 5, 9, 12];
        let i = search(arr, 2);
        println!("{}", i)
    }
}
