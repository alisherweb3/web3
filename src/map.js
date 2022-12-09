function map(array: unknown[], f: (item:unknown) => unknown): unknown[] {
  let result = []
  for (let i =0; i < array.length; i++) {
    result[i] = f(array[i])
  }
  return result
}
