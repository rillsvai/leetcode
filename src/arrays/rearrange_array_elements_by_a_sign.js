/**
 * @param {number[]} nums
 * @return {number[]}
 */
var rearrangeArray = function (nums) {
  const pos = [];
  const neg = [];

  // Separate positive and negative numbers
  for (const num of nums) {
    if (num > 0) {
      pos.push(num);
    } else {
      neg.push(num);
    }
  }

  const res = new Array(nums.length);
  let k = 0; // index for the result array
  let i = 0; // index for the positive array
  let j = 0; // index for the negative array

  while (i < pos.length && j < neg.length) {
    res[k++] = pos[i++]; // add a positive number
    res[k++] = neg[j++]; // add a negative number
  }

  while (i < pos.length) {
    res[k++] = pos[i++];
  }

  while (j < neg.length) {
    res[k++] = neg[j++];
  }

  return res;
};
