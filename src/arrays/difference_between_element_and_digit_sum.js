/**
 * @param {number[]} nums
 * @return {number}
 */
var differenceOfSum = function (nums) {
  numberSum = 0;
  digitSum = 0;
  for (let num of nums) {
    numberSum += num;
    while (num != 0) {
      digitSum += num % 10;
      num = Math.floor(num / 10);
    }
    console.log(numberSum, digitSum);
  }
  return Math.abs(digitSum - numberSum);
};

differenceOfSum([1, 15, 6, 3]);
