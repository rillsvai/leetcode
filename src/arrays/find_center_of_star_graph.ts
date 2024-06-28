function findCenter(edges: number[][]): number {
  const frequency: { [key: number]: number } = {};
  for (const edge of edges) {
    for (const num of edge) {
      if (!frequency[num]) {
        frequency[num] = 1;
        continue;
      }
      frequency[num] += 1;
    }
  }
  console.log(frequency);
  let result = 0;
  for (const key in frequency) {
    const value = frequency[parseInt(key)];
    if (value > result) {
      result = parseInt(key);
    }
  }
  return result;
}

console.log(
  findCenter([
    [1, 2],
    [2, 3],
    [4, 2],
  ])
);
