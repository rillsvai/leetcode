function countDigits(num: number): number {
  let temp: number = num;
  let result = 0;
  while (temp != 0) {
    const digit = temp % 10;
    if (num % digit == 0) {
      result += 1;
    }
    temp = Math.floor(temp / 10);
    console.log(temp);
  }

  return result;
}

console.log(countDigits(7));
