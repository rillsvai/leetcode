type JSONValue =
  | null
  | boolean
  | number
  | string
  | JSONValue[]
  | { [key: string]: JSONValue };
type OnceFn = (...args: JSONValue[]) => JSONValue | undefined;

function once(fn: Function): OnceFn {
  let hasCalled: boolean = false;
  let onceFunc = (...args: any[]) => {
    hasCalled = true;
    return fn(...args);
  };
  return (...args) => (hasCalled ? undefined : onceFunc(...args));
}

let fn = (a: number, b: number, c: number) => a + b + c;
let onceFn = once(fn);
console.log(onceFn(1, 2, 3)); // 6
console.log(onceFn(2, 3, 6)); // returns undefined without calling fn
