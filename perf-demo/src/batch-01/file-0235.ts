import type { Type234 } from '../batch-01/file-0234.js';
export interface Type235 {
  id: 235;
  name: 'File235';
  next: Type234;
}

export function make235(): Type235 {
  return { id: 235, name: 'File235', next: null as unknown as Type234 };
}
