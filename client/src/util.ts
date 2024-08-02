// Performs a type-agnostic deep comparison of two values by process of elimination. This mainly covers the use cases of Primitives, Objects and Arrays, but not certain builtins like Dates.
export function equals(a: any, b: any): boolean {
  if (typeof a !== typeof b) return false;   // Different types can't be equal, except number and bigint but who cares. Null and undefined are considered different.
  if (typeof a !== 'object') return a === b; // A simple comparison suffices for non-objects
  if (isNullish(a) !== isNullish(b)) return false; // A common use case is checking for something to exist vs. not, so it's covered explicitly here
  if (isNullish(a) && isNullish(b)) return true;
  if (a instanceof Array && !(b instanceof Array)) return false; // Could be generalized with Object.getPrototypeOf, but I think it's preferable to have "pure" and regular objects match (Look up Object.create(null))
  if (a?.length !== b?.length)  return false; // Another not technically necessary but common point of comparison.
  for (const key of new Set(Object.keys(a).concat(Object.keys(b)))) // All there's left is to deep-compare what are clearly objects. The keys of both need to be merged to prevent an extra key on one side being ignored.
    if (!equals(a[key],  b[key])) return false;
  return true; // Only once all points of inequality are rules out can we say for certain that the two values are equal.
}

export function isNullish(value: any) {
  return (value === null || value === undefined);
}

export function variantOf(enumValue: any) {
  if (enumValue === null) return null;
  if (typeof enumValue === 'string') return enumValue;
  else return Object.entries(enumValue)[0][0];
}

export type Index = string | number | symbol;
export type MaybeIndex = Index | null;

export function atPath(value: any, path: string): any {
  for (const segment of path.split('.'))
    value = (value ?? {} as any)[segment];
  return value;
}
