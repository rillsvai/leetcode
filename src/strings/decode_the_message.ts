function generateAlphabet(): string[] {
  const alphabet: string[] = [];
  for (let i = 0; i < 26; i++) {
    alphabet.push(String.fromCharCode("a".charCodeAt(0) + i));
  }
  return alphabet;
}

function decodeMessage(key: string, message: string): string {
  const alphabet = generateAlphabet();
  console.log(alphabet);
  key = key.replace(/ /g, "");
  const shuffledAlphabet = [...new Set(key)];
  let decodedMessage = "";
  for (let symbol of message) {
    if (symbol === " ") {
      decodedMessage += " ";
      continue;
    }
    decodedMessage += alphabet[shuffledAlphabet.indexOf(symbol)];
  }

  return decodedMessage;
}

console.log(
  decodeMessage(
    "the quick brown fox jumps over the lazy dog",
    "vkbs bs t suepuv"
  )
);
