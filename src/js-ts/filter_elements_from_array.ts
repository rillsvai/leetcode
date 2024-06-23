type Fn = (n: number, i: number) => any;

function filter(arr: number[], fn: Fn): number[] {
  const filteredArr: number[] = [];
  let i = 0;
  for (const num of arr) {
    if (fn(num, i)) {
      filteredArr.push(num);
    }
    i++;
  }
  return filteredArr;
}
