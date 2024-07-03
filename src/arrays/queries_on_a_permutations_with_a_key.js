function processQueries(queries, m) {
  let result = [];
  let arr = [];
  for (let i = 1; i <= m; i++) {
    arr.push(i);
  }

  for (let query of queries) {
    let index = arr.indexOf(query);
    result.push(index);
    arr = arr.splice(index, 1).concat(arr);
  }
  return result;
}
