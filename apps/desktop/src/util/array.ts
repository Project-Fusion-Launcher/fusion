export function groupArrayElements<T>(
  array: T[] | undefined,
  n: number,
): T[][] {
  if (!array) return [];

  const result: T[][] = [];

  for (let i = 0; i < array.length; i += n) {
    result.push(array.slice(i, i + n));
  }

  return result;
}
