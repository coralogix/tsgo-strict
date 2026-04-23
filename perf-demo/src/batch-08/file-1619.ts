import type { Type1618 } from '../batch-08/file-1618.js';
export interface Type1619 {
  id: 1619;
  name: 'File1619';
  next: Type1618;
}

export function make1619(): Type1619 {
  return { id: 1619, name: 'File1619', next: null as unknown as Type1618 };
}
