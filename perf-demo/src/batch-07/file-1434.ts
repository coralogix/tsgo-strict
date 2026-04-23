import type { Type1433 } from '../batch-07/file-1433.js';
export interface Type1434 {
  id: 1434;
  name: 'File1434';
  next: Type1433;
}

export function make1434(): Type1434 {
  return { id: 1434, name: 'File1434', next: null as unknown as Type1433 };
}
