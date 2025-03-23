import * as R from 'ramda';

export const reorderByIds = <T extends { getId: () => number }>(
  items: T[],
  ids: number[]
) => {
  const idmap = new Map<number, T>();
  items.forEach((item) => idmap.set(item.getId(), R.clone(item)));

  return ids.map((id) => idmap.get(id)!);
};
