function minimumOperations(nums: number[]): number {
  let result = 0;
  for (const num of nums) {
    result += num % 3 === 0 ? 0 : 1;
  }

  return result;
}
