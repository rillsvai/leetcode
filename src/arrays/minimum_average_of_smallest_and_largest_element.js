/**
 * @param {number[]} nums
 * @return {number}
 */
var minimumAverage = function (nums) {
  nums.sort((a, b) => a - b);
  let minAverage = Infinity;
  for (let i = 0, j = nums.length - 1; i < j; i++, j--) {
    const average = (nums[i] + nums[j]) / 2;
    if (average < minAverage) {
      minAverage = average;
    }
  }
  return minAverage;
};

console.log(minimumAverage([7, 8, 3, 4, 15, 13, 4, 1]));
