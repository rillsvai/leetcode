/**
 * @param {number} n
 * @return {string[]}
 */
var validStrings = function (n) {
  var res = ["0", "1"];
  var tmp = [];
  for (var i = 1; i < n; ++i) {
    for (var x of res) {
      tmp.push(x + "1");
      if (x.endsWith("1")) tmp.push(x + "0");
    }
    [res, tmp] = [tmp, res];
    tmp.length = 0;
  }
  return res;
};
