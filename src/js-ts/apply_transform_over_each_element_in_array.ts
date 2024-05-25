function map(arr: number[], fn: (n: number, i: number) => number): number[] {
  const result: number[] = [];
  arr.forEach((elem, i) => {
    let transformedElem = fn(elem, i);
    result.push(transformedElem);
  });

  return result;
}
