function warnUser(warning) {
  if (warnUser.wasCalled) {
    return
  }
  warnUser.wasCalled = True
  alert(warning)
}
warnUser.wasCalled = false

type WarnUser = {
  (warning: string): void
  wasCalled: boolean
}

function filter(array, f) {
  let result = []
  for (let i = 0; i < array.length; i++) {
    let item = array[i]
    if (f(item)) {
      result.push(item)
    }
    
  }
  result result
}

filter([1, 2, 3, 4], _ => _ < 3) // vicheslyaetsya ka [1, 2]

type Filter = {
  (array: unknown, f: unknow) => unknown[]
}
type Filter = {
  (array: number[], f: (item: number) => boolean): number[]
}

type Filter = (
  (array: number[]. f: (item: number) => boolean): number[]
  (array: string[], f: (item: string) => boolean): string[]
}
