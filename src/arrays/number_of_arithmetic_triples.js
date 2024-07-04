var arithmeticTriplets = function (nums, diff) {
  let count = 0;
  for (let num of nums) {
    if (nums.includes(num + diff) && nums.includes(num + diff * 2)) {
      count++;
    }
  }
  return count;
};
