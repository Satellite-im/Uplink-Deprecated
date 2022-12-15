export function customPredicateString(
  elementType: string,
  attribute: string,
  value: string,
) {
  const predicateString: string = `-ios predicate string:elementType == ${elementType} AND ${attribute} == '${value}'`
  return predicateString
}
