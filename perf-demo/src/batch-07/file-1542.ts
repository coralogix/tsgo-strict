import type { Type1541 } from '../batch-07/file-1541.js';
export interface Type1542 {
  id: 1542;
  name: 'File1542';
  next: Type1541;
}

export function make1542(): Type1542 {
  return { id: 1542, name: 'File1542', next: null as unknown as Type1541 };
}
